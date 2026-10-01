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
    use crate::create_readonly_repository_options::create_readonly_repository_options;
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
            ("FirstName".to_string(), "given_name".to_string()),
            ("LastName".to_string(), "family_name".to_string()),
            ("Email".to_string(), "email_address".to_string()),
        ]);
        let first_name = mapper.map("FirstName".into());
        let last_name = mapper.map("LastName".into());
        let email = mapper.map("Email".into());
        let phone = mapper.map("Phone".into());
        let from_first = mapper.reverse_map("given_name".into());
        let from_last = mapper.reverse_map("family_name".into());
        let from_email = mapper.reverse_map("email_address".into());
        let from_phone = mapper.reverse_map("Phone".into());
        assert_eq!(first_name, "given_name");
        assert_eq!(last_name, "family_name");
        assert_eq!(email, "email_address");
        assert_eq!(phone, "Phone");
        assert_eq!(from_first, "FirstName");
        assert_eq!(from_last, "LastName");
        assert_eq!(from_email, "Email");
        assert_eq!(from_phone, "Phone");
    }

    #[test]
    fn first_key_wins_on_reverse() {
        let mapper = DictionaryMapper::new([
            ("Home".to_string(), "address".to_string()),
            ("Work".to_string(), "address".to_string()),
        ]);
        let home = mapper.map("Home".into());
        let work = mapper.map("Work".into());
        let address = mapper.reverse_map("address".into());
        assert_eq!(home, "address");
        assert_eq!(work, "address");
        assert_eq!(address, "Home");
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
        let mut options = create_readonly_repository_options::<Account>();
        options.entity = "user".into();
        options.row_converter = Some(RowTypeConverter::of::<Account>());
        options.property_mappings = Box::new(DictionaryMapper::new([
            ("Key".to_string(), "id".to_string()),
            ("Login".to_string(), "username".to_string()),
        ]));
        let repo = ReadonlyRepository::new(connection, options).unwrap();
        let row = repo.read_single(None, None).await.unwrap().unwrap();
        assert_eq!(row.get("Key"), Some(&Value::Int(7)));
        assert_eq!(row.get("Login"), Some(&Value::from("ada")));
        assert_eq!(
            command.reader_logs.lock().unwrap()[0].0,
            "SELECT id, username FROM user"
        );
    }
}
