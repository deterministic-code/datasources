use crate::converters::ConvertSchema;
use crate::readonly_repository_mapping_options::ReadonlyRepositoryMappingOptions;

pub fn create_readonly_repository_mapping_options<T: ConvertSchema>() -> ReadonlyRepositoryMappingOptions {
    let mut options = ReadonlyRepositoryMappingOptions::default();
    options.entity = std::any::type_name::<T>()
        .rsplit("::")
        .next()
        .unwrap_or_default()
        .into();
    options
}
