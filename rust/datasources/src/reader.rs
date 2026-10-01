use crate::result::Result;
use crate::row::Row;
use async_trait::async_trait;
use futures_core::Stream;
use futures_util::stream::try_unfold;
use std::pin::Pin;

#[async_trait]
pub trait Reader<T: Send = Row>: Send + Sync {
    fn current(&self) -> Option<T>;
    async fn next(&self) -> Result<bool>;
    async fn next_result(&self) -> Result<bool> {
        Ok(false)
    }

    fn read(&self) -> Pin<Box<dyn Stream<Item = Result<T>> + Send + '_>> {
        Box::pin(try_unfold(self, |reader| async move {
            loop {
                if !reader.next().await? {
                    return Ok(None);
                }
                if let Some(current) = reader.current() {
                    return Ok(Some((current, reader)));
                }
            }
        }))
    }
}
