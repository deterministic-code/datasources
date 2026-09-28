use std::sync::Arc;

use crate::{Connection, IUnitOfWork, MockConnection, MockTransaction, UnitOfWorkTransaction};

#[tokio::test]
async fn nested_begin_starts_once() {
    let txn = Arc::new(MockTransaction::new_unbound());
    let connection = MockConnection::with_transaction(txn.clone());
    let shared: Arc<dyn Connection> = connection.clone();
    let mut outer = UnitOfWorkTransaction::new(shared.clone());
    outer.begin().await.expect("begin");
    let mut inner = UnitOfWorkTransaction::new(shared);
    inner.begin().await.expect("join");
    inner.commit().await.expect("join commit");
    outer.commit().await.expect("commit");
    assert_eq!(connection.start_calls(), 1);
    assert_eq!(txn.commit_calls(), 1);
    assert_eq!(txn.abort_calls(), 0);
}

#[tokio::test]
async fn nested_abort_rolls_back_once() {
    let txn = Arc::new(MockTransaction::new_unbound());
    let connection = MockConnection::with_transaction(txn.clone());
    let shared: Arc<dyn Connection> = connection.clone();
    let mut outer = UnitOfWorkTransaction::new(shared.clone());
    outer.begin().await.expect("begin");
    let mut inner = UnitOfWorkTransaction::new(shared);
    inner.begin().await.expect("join");
    inner.abort().await.expect("join abort");
    outer.abort().await.expect("abort");
    assert_eq!(connection.start_calls(), 1);
    assert_eq!(txn.commit_calls(), 0);
    assert_eq!(txn.abort_calls(), 1);
}
