use crate::command::Command;
use crate::connection::Connection;
use crate::converters::{DefaultValueConverter, DeterministicTypeConverter, TypeConverter};
use crate::information::Information;
use crate::isolation_level::IsolationLevel;
use crate::statements::StatementTextRenderer;
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

    fn column_type(&self, name: &str) -> crate::Result<String> {
        Ok(name.to_string())
    }

    fn deterministic_type_converter(&self) -> Option<&DeterministicTypeConverter> {
        None
    }

    fn default_value_converter(&self) -> Option<&DefaultValueConverter> {
        None
    }

    fn statement_renderer(&self) -> Option<&StatementTextRenderer> {
        None
    }

    fn information(&self) -> Option<&Information> {
        None
    }
}
