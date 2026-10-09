use crate::reader::Reader;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::sync::Mutex;

pub struct FirstResultReader {
    first: Vec<Row>,
    index: Mutex<isize>,
    first_set: Mutex<bool>,
    current: Mutex<Option<Row>>,
    live: Box<dyn Reader>,
}

impl FirstResultReader {
    pub async fn capture(live: Box<dyn Reader>) -> Result<(Box<dyn Reader>, Vec<Row>)> {
        let mut rows = Vec::new();
        while live.next().await? {
            if let Some(row) = live.current() {
                rows.push(row);
            }
        }
        Ok((
            Box::new(Self {
                first: rows.clone(),
                index: Mutex::new(-1),
                first_set: Mutex::new(true),
                current: Mutex::new(None),
                live,
            }) as Box<dyn Reader>,
            rows,
        ))
    }
}

#[async_trait]
impl Reader for FirstResultReader {
    fn current(&self) -> Option<Row> {
        self.current.lock().expect("reader").clone()
    }

    async fn next(&self) -> Result<bool> {
        if !*self.first_set.lock().expect("reader") {
            let moved = self.live.next().await?;
            *self.current.lock().expect("reader") = self.live.current();
            return Ok(moved);
        }
        let mut index = self.index.lock().expect("reader");
        *index += 1;
        let i = *index as usize;
        if i < self.first.len() {
            *self.current.lock().expect("reader") = Some(self.first[i].clone());
            Ok(true)
        } else {
            *self.current.lock().expect("reader") = None;
            Ok(false)
        }
    }

    async fn next_result(&self) -> Result<bool> {
        *self.first_set.lock().expect("reader") = false;
        *self.current.lock().expect("reader") = None;
        self.live.next_result().await
    }
}
