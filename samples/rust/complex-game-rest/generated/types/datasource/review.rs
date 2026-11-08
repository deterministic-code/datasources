// schema-version: 1.0
/// Type Review.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct Review {
    pub id: i64,
    pub uuid: String,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub version: Vec<u8>,
    pub game_id: i64,
    pub user_id: i64,
    pub description: String,
    pub rating: String,
    pub is_approved: bool,
}
impl Default for Review {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4().to_string(),
            created: datasources::chrono::Utc::now(),
            updated: datasources::chrono::Utc::now(),
            version: vec![0],
            game_id: 0,
            user_id: 0,
            description: String::new(),
            rating: String::new(),
            is_approved: false,
        }
    }
}
