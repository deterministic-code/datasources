use std::sync::Arc;

use crate::{Connection, MockConnection, MockTransaction, UnitOfWork};

#[tokio::test]
async fn nested_in_transaction_starts_once() {
    let txn = Arc::new(MockTransaction::new_unbound());
    let connection = MockConnection::with_transaction(txn.clone());
    let shared: Arc<dyn Connection> = connection.clone();
    UnitOfWork::new(shared.clone())
        .in_transaction(async {
            UnitOfWork::new(shared.clone())
                .in_transaction(async { Ok::<_, crate::DatasourceError>(()) })
                .await
        })
        .await
        .expect("join");
    assert_eq!(connection.start_calls(), 1);
    assert_eq!(txn.commit_calls(), 1);
}
