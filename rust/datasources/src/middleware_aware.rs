use crate::middleware_hook::MiddlewareHook;
use async_trait::async_trait;

#[async_trait]
pub trait MiddlewareAware {
    fn use_middleware(&self, before: Option<MiddlewareHook>, after: Option<MiddlewareHook>);
}
