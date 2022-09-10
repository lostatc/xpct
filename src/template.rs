#![cfg(feature = "handlebars")]

use std::fmt;
use std::sync::Mutex;

use handlebars::{Handlebars, HelperDef};
use once_cell::sync::Lazy;
use serde::Serialize;

static HANDLEBARS_REGISTRY: Lazy<Mutex<Handlebars>> = Lazy::new(|| {
    let mut registry = Handlebars::new();
    registry.set_strict_mode(true);
    registry.register_escape_fn(handlebars::no_escape);
    Mutex::new(registry)
});

pub struct HandlebarsFormat<Data>(Data);

impl<Data> HandlebarsFormat<Data> {
    const TEMPLATE_NAME: &'static str = concat!(env!("CARGO_CRATE_NAME"), "-formatter");

    pub fn new(template: impl AsRef<str>, data: Data) -> anyhow::Result<Self> {
        let mut registry = HANDLEBARS_REGISTRY.lock().unwrap();
        registry.register_template_string(Self::TEMPLATE_NAME, template)?;
        Ok(Self(data))
    }

    pub fn register_helper(name: &str, def: Box<dyn HelperDef + Send + Sync + 'static>) {
        let mut registry = HANDLEBARS_REGISTRY.lock().unwrap();
        registry.register_helper(name, def);
    }
}

impl<Data> fmt::Display for HandlebarsFormat<Data>
where
    Data: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            HANDLEBARS_REGISTRY
                .lock()
                .unwrap()
                .render(Self::TEMPLATE_NAME, &self.0)
                .expect("failed to render handlebars template")
                .as_str(),
        )
    }
}
