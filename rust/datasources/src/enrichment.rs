use crate::association::Association;
use crate::enrichment_property_type::EnrichmentPropertyType;

#[derive(Clone)]
pub struct Enrichment {
    pub destination_property: String,
    pub source_property: String,
    pub related_key: String,
    pub related_property: Option<String>,
    pub r#type: EnrichmentPropertyType,
    pub association: Option<Association>,
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
        }
    }

    pub fn linked(mut self, association: Association) -> Self {
        self.r#type = EnrichmentPropertyType::List;
        self.related_property = None;
        self.association = Some(association);
        self
    }
}
