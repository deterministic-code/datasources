use datasources::RepositoryFactory;

fn register(_factory: &RepositoryFactory) {}

datasources::bind_repositories!(register);

#[test]
fn bind_repositories_compiles_without_direct_ctor() {
    let _ = register as fn(&RepositoryFactory);
}
