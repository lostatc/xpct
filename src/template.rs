#![cfg(feature = "fmt")]

use std::sync::Mutex;
use std::collections::HashMap;

use handlebars::Handlebars;
use once_cell::sync::Lazy;

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

    pub fn render(&self, data: &HashMap<&str, &str>) -> String {
        HANDLEBARS_REGISTRY
            .lock()
            .unwrap()
            .render(self.name.as_ref(), data)
            .expect("failed to render handlebars template")
    }
}
