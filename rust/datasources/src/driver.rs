use crate::command::Command;
use crate::connection::Connection;
use crate::converters::TypeConverter;
use crate::isolation_level::IsolationLevel;
use crate::transaction::Transaction;
use crate::value::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub trait Driver: Send + Sync {
    fn connection(&self, options: Option<&HashMap<String, String>>) -> Arc<dyn Connection>;
    fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command>;
    fn transaction(
        &self,
        connection: Arc<dyn Connection>,
        isolation: IsolationLevel,
    ) -> Arc<dyn Transaction>;
    fn row_type_converters(
        &self,
    ) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>;

    fn generated_id(&self, id: Value) -> Value {
        id
    }
}
