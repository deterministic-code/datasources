use crate::connection::Connection;
use std::sync::Arc;

pub fn downcast_connection<T: Connection + 'static>(
    connection: Arc<dyn Connection>,
) -> Option<Arc<T>> {
    if connection.as_any().is::<T>() {
        Some(unsafe { Arc::from_raw(Arc::into_raw(connection) as *const T) })
    } else {
        None
    }
}
