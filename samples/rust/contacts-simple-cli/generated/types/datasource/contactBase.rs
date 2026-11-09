// schema-version: 1.0
/// Type ContactBase.
#[derive(Clone, Debug, PartialEq, datasources::Facet)]
#[facet(crate = ::datasources::facet)]
pub struct ContactBase {
    pub id: i32,
    pub uuid: datasources::uuid::Uuid,
    pub created: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub updated: datasources::chrono::DateTime<datasources::chrono::Utc>,
    pub version: Vec<u8>,
    pub name: String,
    pub given_name: String,
    pub additional_name: String,
    pub pronunciation: String,
    pub family_name: String,
    pub nick_name: String,
    pub job_title: String,
    pub gender_type_id: i32,
    pub organization: String,
    pub is_starred: bool,
    pub is_favorite: bool,
    pub is_deleted: bool,
}
impl Default for ContactBase {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: datasources::uuid::Uuid::new_v4(),
            created: datasources::chrono::Utc::now(),
            updated: datasources::chrono::Utc::now(),
            version: vec![0],
            name: String::new(),
            given_name: String::new(),
            additional_name: String::new(),
            pronunciation: String::new(),
            family_name: String::new(),
            nick_name: String::new(),
            job_title: String::new(),
            gender_type_id: 0,
            organization: String::new(),
            is_starred: false,
            is_favorite: false,
            is_deleted: false,
        }
    }
}
