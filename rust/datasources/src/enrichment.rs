use crate::association::Association;
use crate::datasource_error::DatasourceError;
use crate::enrichment_property_type::EnrichmentPropertyType;
use crate::result::Result;
use crate::schema::Schema;

#[derive(Clone)]
pub struct Enrichment {
    pub destination_property: String,
    pub source_property: String,
    pub related_key: String,
    pub related_property: Option<String>,
    pub r#type: EnrichmentPropertyType,
    pub association: Option<Association>,
    pub dictionary_key: Option<String>,
}

impl Enrichment {
    pub fn new(
        destination_property: impl Into<String>,
        source_property: impl Into<String>,
        related_key: impl Into<String>,
        related_property: Option<String>,
        r#type: EnrichmentPropertyType,
    ) -> Self {
        Self {
            destination_property: destination_property.into(),
            source_property: source_property.into(),
            related_key: related_key.into(),
            related_property,
            r#type,
            association: None,
            dictionary_key: None,
        }
    }

    pub fn linked(mut self, association: Association) -> Self {
        self.r#type = EnrichmentPropertyType::List;
        self.association = Some(association);
        self
    }

    pub fn with_dictionary_key(mut self, name: impl Into<String>) -> Self {
        self.dictionary_key = Some(name.into());
        self
    }

    pub fn map_key(&self) -> Result<&str> {
        self.dictionary_key.as_deref().ok_or_else(|| {
            DatasourceError::message(format!("{} requires dictionary_key", self.destination_property))
        })
    }

    pub fn value_columns(&self, schema: &Schema) -> Result<Vec<String>> {
        let map_key = self.map_key()?;
        Ok(schema
            .columns
            .iter()
            .filter(|(name, column)| !column.is_id && *name != &self.related_key && *name != map_key)
            .map(|(name, _)| name.clone())
            .collect())
    }
}
