use crate::enrichment::Enrichment;
use crate::from_schema::FromSchema;
use crate::result::Result;
use crate::row::Row;
use crate::write_mode::WriteMode;
use async_trait::async_trait;

#[async_trait]
pub trait ChildWrites<T: FromSchema>: Send + Sync {
    async fn create_children(&self, enrichment: &Enrichment, parent: &mut T) -> Result<()>;

    async fn update_children(&self, enrichment: &Enrichment, parent: &mut T) -> Result<()>;

    async fn delete_children(&self, enrichment: &Enrichment, parent: &T) -> Result<()>;

    async fn create_linked(&self, enrichment: &Enrichment, parent: &T) -> Result<()>;

    async fn update_linked(&self, enrichment: &Enrichment, parent: &T) -> Result<()>;

    async fn delete_linked(&self, enrichment: &Enrichment, parent: &T) -> Result<()>;

    async fn write_collected(&self, _children: &mut [Row], _mode: WriteMode) -> Result<bool> {
        Ok(false)
    }
}
