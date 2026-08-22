use crate::reader::Reader;
use crate::record::Record;
use crate::result::Result;

pub async fn collect_reader(reader: &dyn Reader) -> Result<Vec<Record>> {
    let mut items = Vec::new();
    while reader.next().await? {
        if let Some(current) = reader.current() {
            items.push(current);
        }
    }
    Ok(items)
}
