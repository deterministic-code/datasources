use crate::reader::Reader;
use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use std::sync::Mutex;

pub struct FirstResultReader {
    sets: Vec<Vec<Row>>,
    set: Mutex<usize>,
    index: Mutex<isize>,
    current: Mutex<Option<Row>>,
}

impl FirstResultReader {
    pub async fn capture(live: Box<dyn Reader>) -> Result<(Box<dyn Reader>, Vec<Row>)> {
        let captured = async {
            let mut sets = Vec::new();
            loop {
                let mut rows = Vec::new();
                while live.next().await? {
                    if let Some(row) = live.current() {
                        rows.push(row);
                    }
                }
                sets.push(rows);
                if !live.next_result().await? {
                    break;
                }
            }
            Ok(sets)
        }
        .await;
        live.close().await?;
        let sets = captured?;
        let first = sets.first().cloned().unwrap_or_default();
        Ok((
            Box::new(Self {
                sets,
                set: Mutex::new(0),
                index: Mutex::new(-1),
                current: Mutex::new(None),
            }) as Box<dyn Reader>,
            first,
        ))
    }
}

#[async_trait]
impl Reader for FirstResultReader {
    fn current(&self) -> Option<Row> {
        self.current.lock().expect("reader").clone()
    }

    async fn next(&self) -> Result<bool> {
        let set = *self.set.lock().expect("reader");
        if set >= self.sets.len() {
            *self.current.lock().expect("reader") = None;
            return Ok(false);
        }
        let mut index = self.index.lock().expect("reader");
        *index += 1;
        let i = *index as usize;
        if i < self.sets[set].len() {
            *self.current.lock().expect("reader") = Some(self.sets[set][i].clone());
            Ok(true)
        } else {
            *self.current.lock().expect("reader") = None;
            Ok(false)
        }
    }

    async fn next_result(&self) -> Result<bool> {
        *self.current.lock().expect("reader") = None;
        *self.index.lock().expect("reader") = -1;
        let mut set = self.set.lock().expect("reader");
        *set += 1;
        Ok(*set < self.sets.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::sync::{Arc, Mutex};
    use std::thread::JoinHandle;

    struct Held {
        stop: Mutex<Option<mpsc::Sender<()>>>,
        finished: Arc<Mutex<bool>>,
        task: Mutex<Option<JoinHandle<()>>>,
    }

    impl Held {
        fn start() -> Self {
            let (tx, rx) = mpsc::channel();
            let finished = Arc::new(Mutex::new(false));
            let done = finished.clone();
            Self {
                stop: Mutex::new(Some(tx)),
                finished,
                task: Mutex::new(Some(std::thread::spawn(move || {
                    let _ = rx.recv();
                    *done.lock().expect("reader") = true;
                }))),
            }
        }
    }

    #[async_trait]
    impl Reader for Held {
        fn current(&self) -> Option<Row> {
            None
        }

        async fn next(&self) -> Result<bool> {
            Ok(false)
        }

        async fn close(&self) -> Result<()> {
            if let Some(stop) = self.stop.lock().expect("reader").take() {
                let _ = stop.send(());
            }
            if let Some(task) = self.task.lock().expect("reader").take() {
                let _ = task.join();
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn capture_awaits_close_before_return() {
        let live = Held::start();
        let finished = live.finished.clone();
        FirstResultReader::capture(Box::new(live)).await.unwrap();
        assert!(*finished.lock().expect("reader"));
    }
}
