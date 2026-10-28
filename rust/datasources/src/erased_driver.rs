#[macro_export]
macro_rules! export_driver {
    ($symbol:ident, $ty:path) => {
        #[::ctor::ctor]
        fn register_linked_driver() {
            let module = stringify!($symbol);
            let Some(name) = $crate::ProviderCatalog::default()
                .iter()
                .find(|(_, catalog)| catalog.module == module)
                .map(|(name, _)| name.clone())
            else {
                return;
            };
            $crate::register(
                name,
                ::std::sync::Arc::new($ty) as ::std::sync::Arc<dyn $crate::Driver>,
            );
        }
    };
}
