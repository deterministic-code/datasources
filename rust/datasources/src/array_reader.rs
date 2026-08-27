use crate::reader::Reader;
use crate::record::Record;
use crate::result::Result;
use async_trait::async_trait;

pub struct ArrayReader<T = Record> {
    rows: Vec<T>,
    index: std::sync::Mutex<isize>,
    current: std::sync::Mutex<Option<T>>,
}

impl<T> ArrayReader<T> {
    pub fn new(rows: Vec<T>) -> Self {
        Self {
            rows,
            index: std::sync::Mutex::new(-1),
            current: std::sync::Mutex::new(None),
        }
    }
}

#[async_trait]
impl<T> Reader<T> for ArrayReader<T>
where
    T: Clone + Send + Sync,
{
    fn current(&self) -> Option<T> {
        self.current.lock().expect("reader").clone()
    }

    async fn next(&self) -> Result<bool> {
        let mut index = self.index.lock().expect("reader");
        *index += 1;
        let i = *index as usize;
        if i < self.rows.len() {
            *self.current.lock().expect("reader") = Some(self.rows[i].clone());
            Ok(true)
        } else {
            *self.current.lock().expect("reader") = None;
            Ok(false)
        }
    }
}
