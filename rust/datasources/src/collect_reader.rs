use crate::reader::Reader;
use crate::result::Result;

pub async fn collect_reader<T>(reader: &dyn Reader<T>) -> Result<Vec<T>> {
    let mut items = Vec::new();
    while reader.next().await? {
        if let Some(current) = reader.current() {
            items.push(current);
        }
    }
    Ok(items)
}
