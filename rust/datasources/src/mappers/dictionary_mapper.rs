use super::Mapper;
use std::collections::HashMap;
use std::hash::Hash;

pub struct DictionaryMapper<T> {
    forward: HashMap<T, T>,
    reverse: HashMap<T, T>,
}

impl<T> DictionaryMapper<T>
where
    T: Clone + Eq + Hash,
{
    pub fn new(map: impl IntoIterator<Item = (T, T)>) -> Self {
        let mut forward = HashMap::new();
        let mut reverse = HashMap::new();
        for (from, to) in map {
            reverse.entry(to.clone()).or_insert_with(|| from.clone());
            forward.insert(from, to);
        }
        Self { forward, reverse }
    }
}

impl<T> Mapper<T, T> for DictionaryMapper<T>
where
    T: Clone + Eq + Hash,
{
    fn map(&self, value: T) -> T {
        self.forward.get(&value).cloned().unwrap_or(value)
    }

    fn reverse_map(&self, value: T) -> T {
        self.reverse.get(&value).cloned().unwrap_or(value)
    }
}

#[cfg(test)]
mod tests {
    use super::DictionaryMapper;
    use crate::converters::{ConvertSchema, RowTypeConverter};
    use crate::create_repository_options::create_repository_options;
    use crate::mappers::Mapper;
    use crate::mocks::{MockCommand, MockConnection};
    use crate::readonly_repository::ReadonlyRepository;
    use crate::repository::IReadonlyRepository;
    use crate::row::Row;
    use crate::value::Value;
    use std::sync::Arc;

    #[test]
    fn maps_known_names_and_passes_through_the_rest() {
        let mapper = DictionaryMapper::new([
            ("Key".to_string(), "id".to_string()),
            ("Login".to_string(), "username".to_string()),
        ]);
        assert_eq!(mapper.map("Key".into()), "id");
        assert_eq!(mapper.reverse_map("id".into()), "Key");
        assert_eq!(mapper.map("other".into()), "other");
        assert_eq!(mapper.reverse_map("other".into()), "other");
    }

    struct Account;

    impl ConvertSchema for Account {
        fn schema() -> Vec<(&'static str, &'static str)> {
            vec![("Key", "INTEGER"), ("Login", "TEXT")]
        }
    }

    #[tokio::test]
    async fn read_reverse_maps_column_names() {
        let command = Arc::new(MockCommand::new());
        command.reader_rows.lock().unwrap().push({
            let mut row = Row::new();
            row.insert("id".into(), Value::Int(7));
            row.insert("username".into(), Value::from("ada"));
            row
        });
        let connection = MockConnection::with_command_provider(command.clone(), "memory");
        let mut options = create_repository_options::<Account>();
        options.entity = "user".into();
        options.row_converter = Some(RowTypeConverter::of::<Account>());
        options.column_mappings = Box::new(DictionaryMapper::new([
            ("Key".to_string(), "id".to_string()),
            ("Login".to_string(), "username".to_string()),
        ]));
        let repo = ReadonlyRepository::new(connection, options.read).unwrap();
        let row = repo.read_single(None, None).await.unwrap().unwrap();
        assert_eq!(row.get("Key"), Some(&Value::Int(7)));
        assert_eq!(row.get("Login"), Some(&Value::from("ada")));
        assert_eq!(
            command.reader_logs.lock().unwrap()[0].0,
            "SELECT id, username FROM user"
        );
    }
}
