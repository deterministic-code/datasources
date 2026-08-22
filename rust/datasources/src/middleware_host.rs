use crate::middleware_context::MiddlewareContext;
use crate::middleware_hook::MiddlewareHook;
use crate::parameter::Parameter;
use crate::result::Result;
use std::future::Future;
use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct MiddlewareHost {
    befores: Arc<Mutex<Vec<MiddlewareHook>>>,
    afters: Arc<Mutex<Vec<MiddlewareHook>>>,
}

impl MiddlewareHost {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn use_middleware(
        &self,
        before: Option<MiddlewareHook>,
        after: Option<MiddlewareHook>,
    ) -> &Self {
        if let Some(before) = before {
            self.befores.lock().expect("middleware").push(before);
        }
        if let Some(after) = after {
            self.afters.lock().expect("middleware").push(after);
        }
        self
    }

    pub async fn run<T, F, Fut>(
        &self,
        operation: &str,
        extras: Option<(Option<String>, Option<Vec<Parameter>>)>,
        fn_op: F,
    ) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let mut ctx = MiddlewareContext::new(operation);
        if let Some((text, parameters)) = extras {
            ctx.text = text;
            ctx.parameters = parameters;
        }

        let befores = self.befores.lock().expect("middleware").clone();
        for before in befores {
            before(ctx.clone()).await?;
        }

        match fn_op().await {
            Ok(value) => {
                let afters = self.afters.lock().expect("middleware").clone();
                for after in afters {
                    after(ctx.clone()).await?;
                }
                Ok(value)
            }
            Err(error) => {
                ctx.error = Some(error.to_string());
                let afters = self.afters.lock().expect("middleware").clone();
                for after in afters {
                    after(ctx.clone()).await?;
                }
                Err(error)
            }
        }
    }
}
