use crate::converters::ConvertSchema;
use crate::readonly_repository_options::ReadonlyRepositoryOptions;

pub fn create_readonly_repository_options<T: ConvertSchema>() -> ReadonlyRepositoryOptions {
    let mut options = ReadonlyRepositoryOptions::default();
    options.entity = std::any::type_name::<T>()
        .rsplit("::")
        .next()
        .unwrap_or_default()
        .into();
    options
}
