use serde_yaml::Value;
use std::collections::HashMap;
use std::sync::OnceLock;

static TRANSLATIONS: OnceLock<HashMap<String, String>> = OnceLock::new();

fn load_translations() -> &'static HashMap<String, String> {
    TRANSLATIONS.get_or_init(|| {
        let root: Value = serde_yaml::from_str(include_str!("../locales/en.yaml"))
            .expect("failed to parse embedded English locale");
        let mut map = HashMap::new();
        flatten_value(None, &root, &mut map);
        map
    })
}

fn flatten_value(prefix: Option<&str>, value: &Value, out: &mut HashMap<String, String>) {
    match value {
        Value::Mapping(mapping) => {
            for (key, child) in mapping {
                let Some(key) = key.as_str() else {
                    continue;
                };
                let next = match prefix {
                    Some(prefix) => format!("{prefix}.{key}"),
                    None => key.to_string(),
                };
                flatten_value(Some(&next), child, out);
            }
        }
        Value::String(text) => {
            if let Some(prefix) = prefix {
                out.insert(prefix.to_string(), text.clone());
            }
        }
        Value::Number(number) => {
            if let Some(prefix) = prefix {
                out.insert(prefix.to_string(), number.to_string());
            }
        }
        Value::Bool(boolean) => {
            if let Some(prefix) = prefix {
                out.insert(prefix.to_string(), boolean.to_string());
            }
        }
        _ => {}
    }
}

pub fn set_locale(_locale: &str) {}

pub fn translate(key: &str, params: &[(&str, String)]) -> String {
    let mut text = load_translations()
        .get(key)
        .cloned()
        .unwrap_or_else(|| key.to_string());

    for (name, value) in params {
        let needle = format!("%{{{name}}}");
        text = text.replace(&needle, value);
    }

    text
}

#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::rust_i18n::translate($key, &[])
    };
    ($key:expr, $($name:ident = $value:expr),+ $(,)?) => {
        $crate::rust_i18n::translate(
            $key,
            &[$((stringify!($name), ($value).to_string())),+]
        )
    };
}

pub use crate::t;
