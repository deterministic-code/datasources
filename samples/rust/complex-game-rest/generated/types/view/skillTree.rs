// schema-version: 1.0
#[allow(unused_imports)]
use super::*;
/// View SkillTree.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct SkillTree {
    pub id: i64,
    pub uuid: datasources::uuid::Uuid,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub version: Vec<u8>,
    pub parent_skill_id: Option<i64>,
    pub name: String,
    pub children: Vec<SkillTree>,
}
impl Default for SkillTree {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4(),
            created: datasources::chrono::Utc::now(),
            updated: datasources::chrono::Utc::now(),
            version: vec![0],
            parent_skill_id: None,
            name: String::new(),
            children: vec![],
        }
    }
}
