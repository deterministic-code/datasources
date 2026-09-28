use std::sync::Arc;

use crate::{Connection, MockConnection, MockTransaction, UnitOfWorkTransaction};

#[tokio::test]
async fn nested_do_work_starts_once() {
    let txn = Arc::new(MockTransaction::new_unbound());
    let connection = MockConnection::with_transaction(txn.clone());
    let shared: Arc<dyn Connection> = connection.clone();
    UnitOfWorkTransaction::new(shared.clone())
        .do_work(async {
            UnitOfWorkTransaction::new(shared.clone())
                .do_work(async { Ok::<_, crate::DatasourceError>(()) })
                .await
        })
        .await
        .expect("join");
    assert_eq!(connection.start_calls(), 1);
    assert_eq!(txn.commit_calls(), 1);
}
