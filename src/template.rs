#![cfg(feature = "handlebars")]

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandlebarsTemplate {
    name: String,
}

impl HandlebarsTemplate {
    pub fn new(name: &str, template: impl AsRef<str>) -> Self {
        HANDLEBARS_REGISTRY
            .lock()
            .unwrap()
            .register_template_string(name, template.as_ref())
            .expect("failed to parse handlebars template");
        Self {
            name: name.to_string(),
        }
    }

    pub fn render<T: Serialize>(&self, data: &T) -> String {
        HANDLEBARS_REGISTRY
            .lock()
            .unwrap()
            .render(self.name.as_ref(), data)
            .expect("failed to render handlebars template")
    }

    pub fn register_helper(name: &str, def: Box<dyn HelperDef + Send + Sync + 'static>) {
        HANDLEBARS_REGISTRY
            .lock()
            .unwrap()
            .register_helper(name, def);
    }
}
