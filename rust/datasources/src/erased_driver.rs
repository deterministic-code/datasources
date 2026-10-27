use crate::command::Command;
use crate::connection::Connection;
use crate::converters::TypeConverter;
use crate::driver::Driver;
use crate::reader::Reader;
use crate::sabi_driver::{
    SabiCommandBox, SabiConnectionBox, SabiDriverBox, SabiReaderBox, SabiTransactionBox,
};
use crate::sabi_value::{
    decode_options, decode_parameters, decode_value, encode_response, encode_row, encode_value,
    isolation_of, lift,
};
use crate::transaction::Transaction;
use crate::value::Value;
use abi_stable::sabi_trait::TD_Opaque;
use abi_stable::std_types::{RNone, ROption, RSome};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

struct Slots {
    next: AtomicU64,
    connections: Mutex<HashMap<u64, Arc<dyn Connection>>>,
    commands: Mutex<HashMap<u64, Arc<dyn Command>>>,
    transactions: Mutex<HashMap<u64, Arc<dyn Transaction>>>,
    readers: Mutex<HashMap<u64, Arc<dyn Reader>>>,
}

impl Slots {
    fn id(&self) -> u64 {
        self.next.fetch_add(1, Ordering::Relaxed)
    }
}

fn wait<T>(future: impl std::future::Future<Output = T>) -> T {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("runtime")
        .block_on(future)
}

#[derive(Clone)]
struct Erased {
    driver: Arc<dyn Driver>,
    slots: Arc<Slots>,
    converters: Arc<HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>>>,
}

struct ConnKeep {
    slots: Arc<Slots>,
    id: u64,
}

impl Drop for ConnKeep {
    fn drop(&mut self) {
        self.slots.connections.lock().expect("connections").remove(&self.id);
    }
}

struct CommandKeep {
    slots: Arc<Slots>,
    id: u64,
    #[allow(dead_code)]
    connection: Arc<dyn Connection>,
}

impl Drop for CommandKeep {
    fn drop(&mut self) {
        self.slots.commands.lock().expect("commands").remove(&self.id);
    }
}

struct TransactionKeep {
    slots: Arc<Slots>,
    id: u64,
    #[allow(dead_code)]
    connection: Arc<dyn Connection>,
}

impl Drop for TransactionKeep {
    fn drop(&mut self) {
        self.slots.transactions.lock().expect("transactions").remove(&self.id);
    }
}

struct ReaderKeep {
    slots: Arc<Slots>,
    id: u64,
}

impl Drop for ReaderKeep {
    fn drop(&mut self) {
        self.slots.readers.lock().expect("readers").remove(&self.id);
    }
}

#[derive(Clone)]
struct PluginConnection {
    keep: Arc<ConnKeep>,
}

#[derive(Clone)]
struct PluginCommand {
    keep: Arc<CommandKeep>,
}

#[derive(Clone)]
struct PluginTransaction {
    keep: Arc<TransactionKeep>,
}

#[derive(Clone)]
struct PluginReader {
    keep: Arc<ReaderKeep>,
}

impl PluginConnection {
    fn connection(&self) -> Arc<dyn Connection> {
        self.keep
            .slots
            .connections
            .lock()
            .expect("connections")
            .get(&self.keep.id)
            .expect("connection")
            .clone()
    }
}

impl PluginCommand {
    fn command(&self) -> Arc<dyn Command> {
        self.keep
            .slots
            .commands
            .lock()
            .expect("commands")
            .get(&self.keep.id)
            .expect("command")
            .clone()
    }
}

impl PluginTransaction {
    fn transaction(&self) -> Arc<dyn Transaction> {
        self.keep
            .slots
            .transactions
            .lock()
            .expect("transactions")
            .get(&self.keep.id)
            .expect("transaction")
            .clone()
    }
}

impl PluginReader {
    fn reader(&self) -> Arc<dyn Reader> {
        self.keep
            .slots
            .readers
            .lock()
            .expect("readers")
            .get(&self.keep.id)
            .expect("reader")
            .clone()
    }
}

fn store_command(slots: &Arc<Slots>, command: Arc<dyn Command>) -> SabiCommandBox {
    let id = slots.id();
    let connection = command.connection();
    slots.commands.lock().expect("commands").insert(id, command);
    SabiCommandBox::from_value(
        PluginCommand {
            keep: Arc::new(CommandKeep {
                slots: Arc::clone(slots),
                id,
                connection,
            }),
        },
        TD_Opaque,
    )
}

fn store_transaction(slots: &Arc<Slots>, transaction: Arc<dyn Transaction>) -> SabiTransactionBox {
    let id = slots.id();
    let connection = transaction.connection();
    slots
        .transactions
        .lock()
        .expect("transactions")
        .insert(id, transaction);
    SabiTransactionBox::from_value(
        PluginTransaction {
            keep: Arc::new(TransactionKeep {
                slots: Arc::clone(slots),
                id,
                connection,
            }),
        },
        TD_Opaque,
    )
}

fn store_reader(slots: &Arc<Slots>, reader: Box<dyn Reader>) -> SabiReaderBox {
    let id = slots.id();
    slots
        .readers
        .lock()
        .expect("readers")
        .insert(id, Arc::from(reader));
    SabiReaderBox::from_value(
        PluginReader {
            keep: Arc::new(ReaderKeep {
                slots: Arc::clone(slots),
                id,
            }),
        },
        TD_Opaque,
    )
}

impl crate::sabi_driver::SabiReader for PluginReader {
    fn current(&self) -> ROption<abi_stable::std_types::RVec<crate::sabi_value::SabiCell>> {
        self.reader().current().map_or(RNone, |row| RSome(encode_row(&row)))
    }

    fn next(&self) -> abi_stable::std_types::RResult<bool, abi_stable::std_types::RString> {
        lift(wait(self.reader().next()))
    }

    fn next_result(&self) -> abi_stable::std_types::RResult<bool, abi_stable::std_types::RString> {
        lift(wait(self.reader().next_result()))
    }
}

impl crate::sabi_driver::SabiTransaction for PluginTransaction {
    fn isolation(&self) -> u8 {
        crate::sabi_value::isolation_tag(self.transaction().isolation())
    }

    fn commit(&self) -> abi_stable::std_types::RResult<(), abi_stable::std_types::RString> {
        lift(wait(self.transaction().commit()))
    }

    fn abort(&self) -> abi_stable::std_types::RResult<(), abi_stable::std_types::RString> {
        lift(wait(self.transaction().abort()))
    }
}

impl crate::sabi_driver::SabiCommand for PluginCommand {
    fn execute(
        &self,
        text: abi_stable::std_types::RString,
        parameters: ROption<abi_stable::std_types::RVec<crate::sabi_value::SabiParameter>>,
    ) -> abi_stable::std_types::RResult<crate::sabi_value::SabiResponse, abi_stable::std_types::RString>
    {
        let query = text.to_string();
        let params = decode_parameters(&parameters);
        let command = self.command();
        lift(wait(async move {
            command
                .execute_native(&crate::native::Native::new(query), params.as_deref())
                .await
                .map(|response| encode_response(&response))
        }))
    }

    fn execute_reader(
        &self,
        text: abi_stable::std_types::RString,
        parameters: ROption<abi_stable::std_types::RVec<crate::sabi_value::SabiParameter>>,
    ) -> abi_stable::std_types::RResult<SabiReaderBox, abi_stable::std_types::RString> {
        let query = text.to_string();
        let params = decode_parameters(&parameters);
        let command = self.command();
        let slots = Arc::clone(&self.keep.slots);
        lift(wait(async move {
            command
                .execute_reader_native(&crate::native::Native::new(query), params.as_deref())
                .await
                .map(|reader| store_reader(&slots, reader))
        }))
    }
}

impl crate::sabi_driver::SabiConnection for PluginConnection {
    fn slot(&self) -> u64 {
        self.keep.id
    }

    fn provider(&self) -> abi_stable::std_types::RString {
        self.connection().provider().into()
    }

    fn is_open(&self) -> bool {
        self.connection().is_open()
    }

    fn create_command(&self) -> SabiCommandBox {
        store_command(&self.keep.slots, self.connection().create_command())
    }

    fn open(&self) -> abi_stable::std_types::RResult<(), abi_stable::std_types::RString> {
        let connection = self.connection();
        lift(wait(async move { connection.open().await }))
    }

    fn close(&self) -> abi_stable::std_types::RResult<(), abi_stable::std_types::RString> {
        let connection = self.connection();
        lift(wait(async move { connection.close().await }))
    }

    fn start_transaction(
        &self,
        isolation: ROption<u8>,
    ) -> abi_stable::std_types::RResult<SabiTransactionBox, abi_stable::std_types::RString> {
        let connection = self.connection();
        let slots = Arc::clone(&self.keep.slots);
        let level = match isolation {
            RSome(tag) => Some(isolation_of(tag)),
            RNone => None,
        };
        lift(wait(async move {
            connection
                .start_transaction(level)
                .await
                .map(|transaction| store_transaction(&slots, transaction))
        }))
    }
}

impl crate::sabi_driver::SabiDriver for Erased {
    fn connection(&self, options: abi_stable::std_types::RVec<crate::sabi_value::SabiCell>) -> SabiConnectionBox {
        let map = decode_options(&options);
        let connection = self.driver.connection(Some(&map));
        let id = self.slots.id();
        self.slots
            .connections
            .lock()
            .expect("connections")
            .insert(id, connection);
        SabiConnectionBox::from_value(
            PluginConnection {
                keep: Arc::new(ConnKeep {
                    slots: Arc::clone(&self.slots),
                    id,
                }),
            },
            TD_Opaque,
        )
    }

    fn command(&self, slot: u64) -> SabiCommandBox {
        let id = slot;
        let inner = self
            .slots
            .connections
            .lock()
            .expect("connections")
            .get(&id)
            .expect("connection")
            .clone();
        store_command(&self.slots, self.driver.command(inner))
    }

    fn transaction(&self, slot: u64, isolation: u8) -> SabiTransactionBox {
        let id = slot;
        let inner = self
            .slots
            .connections
            .lock()
            .expect("connections")
            .get(&id)
            .expect("connection")
            .clone();
        store_transaction(
            &self.slots,
            self.driver.transaction(inner, isolation_of(isolation)),
        )
    }

    fn column_type(
        &self,
        name: abi_stable::std_types::RString,
    ) -> abi_stable::std_types::RResult<abi_stable::std_types::RString, abi_stable::std_types::RString>
    {
        lift(self.driver.column_type(name.as_str()).map(Into::into))
    }

    fn converter_names(&self) -> abi_stable::std_types::RVec<abi_stable::std_types::RString> {
        self.converters.keys().map(|name| name.as_str().into()).collect()
    }

    fn convert(
        &self,
        name: abi_stable::std_types::RString,
        value: crate::sabi_value::SabiValue,
        reverse: bool,
    ) -> abi_stable::std_types::RResult<crate::sabi_value::SabiValue, abi_stable::std_types::RString>
    {
        let Some(converter) = self.converters.get(name.as_str()) else {
            return abi_stable::std_types::RResult::RErr(
                format!("missing converter {name}").into(),
            );
        };
        let decoded = decode_value(&value);
        let converted = if reverse {
            converter.reverse_convert(&decoded)
        } else {
            converter.convert(&decoded)
        };
        lift(converted.map(|next| encode_value(&next)))
    }
}

pub fn erase_driver(driver: Arc<dyn Driver>) -> SabiDriverBox {
    let converters = driver.row_type_converters();
    SabiDriverBox::from_value(
        Erased {
            driver,
            slots: Arc::new(Slots {
                next: AtomicU64::new(1),
                connections: Mutex::new(HashMap::new()),
                commands: Mutex::new(HashMap::new()),
                transactions: Mutex::new(HashMap::new()),
                readers: Mutex::new(HashMap::new()),
            }),
            converters: Arc::new(converters),
        },
        TD_Opaque,
    )
}

#[macro_export]
macro_rules! export_driver {
    ($symbol:ident, $ty:path) => {
        #[no_mangle]
        extern "C" fn $symbol() -> $crate::SabiDriverBox {
            $crate::erase_driver(
                ::std::sync::Arc::new($ty) as ::std::sync::Arc<dyn $crate::Driver>,
            )
        }

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
