#[macro_export]
macro_rules! bind_repositories {
    ($register:expr) => {
        #[$crate::ctor]
        fn __datasources_bind_repositories() {
            $crate::RepositoryFactory::bind($register);
        }
    };
}
