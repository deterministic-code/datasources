use crate::middleware_context::MiddlewareContext;
use crate::middleware_hook::MiddlewareHook;
use crate::result::Result;
use std::future::Future;
use std::sync::Arc;

pub fn hook<F, Fut>(f: F) -> MiddlewareHook
where
    F: Fn(MiddlewareContext) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    Arc::new(move |ctx| Box::pin(f(ctx)))
}
