use crate::command::Command;
use crate::command_middleware::CommandMiddleware;
use crate::command_middleware_host::CommandMiddlewareHost;
use crate::connection::Connection;
use crate::converters::TypeConverter;
use crate::datasource_error::DatasourceError;
use crate::driver::Driver;
use crate::isolation_level::IsolationLevel;
use crate::native::Native;
use crate::parameter::Parameter;
use crate::reader::Reader;
use crate::response::Response;
use crate::result::Result;
use crate::row::Row;
use crate::sabi_driver::{SabiCommandBox, SabiConnectionBox, SabiDriverBox, SabiReaderBox, SabiTransactionBox};
use crate::sabi_value::{
    decode_response, decode_row, decode_value, encode_options, encode_parameters, encode_value,
    isolation_of, isolation_tag, lower,
};
use crate::transaction::Transaction;
use crate::value::Value;
use async_trait::async_trait;
use libloading::Library;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

pub struct LoadedLibrary {
    pub sabi: Arc<SabiDriverBox>,
    _lib: Library,
}

pub struct LoadedDriver {
    library: Arc<LoadedLibrary>,
}

impl LoadedDriver {
    pub fn new(sabi: SabiDriverBox, lib: Library) -> Self {
        Self {
            library: Arc::new(LoadedLibrary {
                sabi: Arc::new(sabi),
                _lib: lib,
            }),
        }
    }
}

struct RemoteConverter {
    driver: Arc<SabiDriverBox>,
    name: String,
}

impl TypeConverter<Value, Value> for RemoteConverter {
    fn convert(&self, from: &Value) -> Result<Value> {
        lower(self.driver.convert(self.name.as_str().into(), encode_value(from), false)).map(|value| decode_value(&value))
    }

    fn reverse_convert(&self, to: &Value) -> Result<Value> {
        lower(self.driver.convert(self.name.as_str().into(), encode_value(to), true)).map(|value| decode_value(&value))
    }
}

impl Driver for LoadedDriver {
    fn connection(&self, options: Option<&HashMap<String, String>>) -> Arc<dyn Connection> {
        Arc::new(LoadedConnection {
            library: Arc::clone(&self.library),
            sabi: Arc::new(self.library.sabi.connection(encode_options(options))),
        })
    }

    fn command(&self, connection: Arc<dyn Connection>) -> Arc<dyn Command> {
        let loaded = connection
            .as_any()
            .downcast_ref::<LoadedConnection>()
            .expect("loaded connection");
        Arc::new(LoadedCommand {
            library: Arc::clone(&self.library),
            sabi: Arc::new(self.library.sabi.command(loaded.sabi.slot())),
            host: CommandMiddlewareHost::new(),
        })
    }

    fn transaction(
        &self,
        connection: Arc<dyn Connection>,
        isolation: IsolationLevel,
    ) -> Arc<dyn Transaction> {
        let loaded = connection
            .as_any()
            .downcast_ref::<LoadedConnection>()
            .expect("loaded connection");
        Arc::new(LoadedTransaction {
            library: Arc::clone(&self.library),
            sabi: Arc::new(self.library.sabi.transaction(loaded.sabi.slot(), isolation_tag(isolation))),
        })
    }

    fn row_type_converters(
        &self,
    ) -> HashMap<String, Box<dyn TypeConverter<Value, Value> + Send + Sync>> {
        self.library
            .sabi
            .converter_names()
            .iter()
            .map(|name| {
                let name = name.to_string();
                (
                    name.clone(),
                    Box::new(RemoteConverter {
                        driver: Arc::clone(&self.library.sabi),
                        name,
                    }) as Box<dyn TypeConverter<Value, Value> + Send + Sync>,
                )
            })
            .collect()
    }

    fn column_type(&self, name: &str) -> Result<String> {
        lower(self.library.sabi.column_type(name.into())).map(|value| value.to_string())
    }
}

struct LoadedConnection {
    library: Arc<LoadedLibrary>,
    sabi: Arc<SabiConnectionBox>,
}

struct LoadedCommand {
    library: Arc<LoadedLibrary>,
    sabi: Arc<SabiCommandBox>,
    host: CommandMiddlewareHost,
}

struct LoadedTransaction {
    library: Arc<LoadedLibrary>,
    sabi: Arc<SabiTransactionBox>,
}

struct LoadedReader {
    library: Arc<LoadedLibrary>,
    sabi: Arc<SabiReaderBox>,
    current: std::sync::Mutex<Option<Row>>,
}

async fn block<T: Send + 'static>(work: impl FnOnce() -> T + Send + 'static) -> Result<T> {
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle
            .spawn_blocking(work)
            .await
            .map_err(|error| DatasourceError::message(error.to_string())),
        Err(_) => Ok(work()),
    }
}

#[async_trait]
impl Connection for LoadedConnection {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn provider(&self) -> &str {
        // The name is owned by the plugin string. Callers only need it for the duration of the call.
        // Store it by leaking per call is wrong. Cache on first use via a side channel.
        // Return a static by interning in a process map keyed by the plugin string.
        intern(self.sabi.provider().to_string())
    }

    fn is_open(&self) -> bool {
        self.sabi.is_open()
    }

    fn create_command(&self) -> Arc<dyn Command> {
        Arc::new(LoadedCommand {
            library: Arc::clone(&self.library),
            sabi: Arc::new(self.sabi.create_command()),
            host: CommandMiddlewareHost::new(),
        })
    }

    async fn open(&self) -> Result<()> {
        let sabi = Arc::clone(&self.sabi);
        block(move || lower(sabi.open())).await?
    }

    async fn close(&self) -> Result<()> {
        let sabi = Arc::clone(&self.sabi);
        block(move || lower(sabi.close())).await?
    }

    async fn start_transaction(
        &self,
        isolation: Option<IsolationLevel>,
    ) -> Result<Arc<dyn Transaction>> {
        let sabi = self.sabi.clone();
        let library = Arc::clone(&self.library);
        let tag = isolation.map(isolation_tag);
        let started = block(move || {
            let option = tag.map_or(abi_stable::std_types::ROption::RNone, abi_stable::std_types::ROption::RSome);
            lower(sabi.start_transaction(option))
        })
        .await??;
        let _ = &library.sabi;
        Ok(Arc::new(LoadedTransaction {
            library,
            sabi: Arc::new(started),
        }))
    }
}

fn intern(name: String) -> &'static str {
    use std::sync::Mutex as StdMutex;
    static NAMES: StdMutex<Option<HashMap<String, &'static str>>> = StdMutex::new(None);
    let mut guard = NAMES.lock().expect("names");
    let map = guard.get_or_insert_with(HashMap::new);
    if let Some(found) = map.get(&name) {
        return found;
    }
    let leaked: &'static str = Box::leak(name.clone().into_boxed_str());
    map.insert(name, leaked);
    leaked
}

#[async_trait]
impl Command for LoadedCommand {
    fn add_middleware(&self, middleware: Arc<dyn CommandMiddleware>) {
        self.host.add_middleware(middleware);
    }

    async fn execute_reader_native(
        &self,
        text: &Native,
        parameters: Option<&[Parameter]>,
    ) -> Result<Box<dyn Reader>> {
        let query = text.text().to_string();
        let params = parameters.map(<[Parameter]>::to_vec);
        let sabi = self.sabi.clone();
        let library = Arc::clone(&self.library);
        self.host
            .run(text.text(), parameters, move |_query, _params| {
                let sabi = sabi.clone();
                let library = Arc::clone(&library);
                async move {
                    let reader = block(move || {
                        lower(sabi.execute_reader(query.as_str().into(), encode_parameters(params.as_deref())))
                    })
                    .await??;
                    Ok((
                        Box::new(LoadedReader {
                            library,
                            sabi: Arc::new(reader),
                            current: std::sync::Mutex::new(None),
                        }) as Box<dyn Reader>,
                        Vec::new(),
                    ))
                }
            })
            .await
    }

    async fn execute_native(
        &self,
        text: &Native,
        parameters: Option<&[Parameter]>,
    ) -> Result<Response> {
        let query = text.text().to_string();
        let params = parameters.map(<[Parameter]>::to_vec);
        let sabi = Arc::clone(&self.sabi);
        self.host
            .run(text.text(), parameters, move |_query, _params| {
                let sabi = sabi.clone();
                async move {
                    let response = block(move || {
                        lower(sabi.execute(query.as_str().into(), encode_parameters(params.as_deref())))
                    })
                    .await??;
                    Ok((decode_response(&response), Vec::new()))
                }
            })
            .await
    }
}

#[async_trait]
impl Transaction for LoadedTransaction {
    fn isolation(&self) -> IsolationLevel {
        isolation_of(self.sabi.isolation())
    }

    async fn commit(&self) -> Result<()> {
        let _ = &self.library.sabi;
        let sabi = Arc::clone(&self.sabi);
        block(move || lower(sabi.commit())).await?
    }

    async fn abort(&self) -> Result<()> {
        let _ = &self.library.sabi;
        let sabi = Arc::clone(&self.sabi);
        block(move || lower(sabi.abort())).await?
    }
}

#[async_trait]
impl Reader for LoadedReader {
    fn current(&self) -> Option<Row> {
        self.current.lock().expect("current").clone()
    }

    async fn next(&self) -> Result<bool> {
        let _ = &self.library.sabi;
        let sabi = Arc::clone(&self.sabi);
        let advanced = block(move || lower(sabi.next())).await??;
        if advanced {
            let sabi = Arc::clone(&self.sabi);
            let row = block(move || sabi.current()).await?;
            *self.current.lock().expect("current") = match row {
                abi_stable::std_types::ROption::RSome(cells) => Some(decode_row(&cells)),
                abi_stable::std_types::ROption::RNone => None,
            };
        }
        Ok(advanced)
    }

    async fn next_result(&self) -> Result<bool> {
        let _ = &self.library.sabi;
        let sabi = Arc::clone(&self.sabi);
        block(move || lower(sabi.next_result())).await?
    }
}
