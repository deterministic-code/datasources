use crate::middleware_context::MiddlewareContext;
use crate::result::Result;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type MiddlewareFuture = Pin<Box<dyn Future<Output = Result<()>> + Send>>;
pub type MiddlewareHook = Arc<dyn Fn(MiddlewareContext) -> MiddlewareFuture + Send + Sync>;
