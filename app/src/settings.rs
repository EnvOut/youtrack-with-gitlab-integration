use config::{ConfigError, Value};
use config::FileFormat;
use config::Value as ConfigValue;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use crate::service::definitions::tags::TagDefinition;

lazy_static! {
	 pub static ref SETTINGS: config::Config = {
		  let mut settings = config::Config::default();
		  settings
				// Add in `./Settings.toml`
				.merge(config::File::with_name("settings").format(FileFormat::Yaml)).unwrap()
				// Add in settings from the environment (with a prefix of APP)
				// Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
				.merge(config::Environment::with_prefix("APP")).unwrap();
		  settings
	 };
}

pub fn get_str<'a>(key: &str) -> Result<String, ConfigError> {
    SETTINGS.get_str(key)
}

pub fn get_int(key: &str) -> Result<i64, ConfigError> {
    SETTINGS.get_int(key)
}

pub fn get_map(key: &str) -> Result<HashMap<String, String>, ConfigError> {
    SETTINGS.get_table(key)
        .map(|table| table.iter()
            .map(|(key, value)| (key.clone(), value.to_string())
            ).collect())
}

pub fn get_array<T>(key: &str) -> Result<Vec<T>, ConfigError> where T: From<config::Value> {
    SETTINGS.get_array(key)
        .map(|list| list.iter()
            .map(|value| T::from(value.clone()))
            .collect()
        )
}

pub fn get(key: &str) -> Result<Value, ConfigError> {
    SETTINGS.get(key)
}

pub mod array {
    use config::{ConfigError, Value};

    pub fn get_array_str(root: Value) -> Result<Vec<String>, ConfigError> {
        let str_value_res = root.clone().into_str();
        let vector_of_strings_res = root.clone().into_array()
            .map(|vector| {
                let result: Vec<String> = vector.into_iter()
                    .filter_map(|it| it.into_str().ok()).collect();
                result
            });
        match (str_value_res, vector_of_strings_res) {
            (Ok(op_ref), Err(_)) => Ok(vec![op_ref]),
            (Err(_), Ok(ops_ref_list)) if !ops_ref_list.is_empty() => Ok(ops_ref_list),
            _ => Err(config::ConfigError::Message(format!("Value of {:?} should be string or list of strings", &root)))
        }
    }

    /// # Examples
    /// ```
    /// get_array_def("rrr.rr", "labels")
    /// ```
    pub fn get_array_def(root: Value, defs_path: &str) -> Result<Vec<(String, Value)>, ConfigError> {
        let array_ref_res = get_array_str(root);
        let defs_res = crate::settings::get(defs_path.clone())
            .and_then(|v| v.into_table());
        match (array_ref_res, defs_res) {
            (Ok(array_ref), Ok(defs)) => {
                let results = array_ref.iter()
                    .map(|r| {
                        let value = defs.get(r).ok_or(config::ConfigError::Message(format!("Definition of the reference: {:?} in the {:?} not found", r, defs_path))).unwrap();
                        (r.clone(), value.clone())
                    })
                    .collect::<Vec<_>>();
                Ok(results)
            }
            (arr_ref_res, defs_res) => {
                if let Some(arr_ref_err) = arr_ref_res.err() {
                    log::warn!("{}", arr_ref_err)
                }
                if let Some(defs_err) = defs_res.err() {
                    log::warn!("{}", defs_err)
                }
                Err(ConfigError::Message(format!("Something wrong with mapping to the {:?}", defs_path)))
            }
        }
    }
}

pub fn get_tag_definition(key: &str) -> Result<TagDefinition, ConfigError> {
    let default_style = 13;
    let tag_name = key.to_string();
    let value = SETTINGS.get::<ConfigValue>(format!("tags.{tag_name}", tag_name = tag_name).as_str()).unwrap();
    match (value.clone().into_str(), value.into_table()) {
        (Ok(title), Err(_)) => Ok(TagDefinition::new(tag_name, title, default_style)),
        (Err(_), Ok(map)) => {
            let title = map.get("title")
                .ok_or(ConfigError::Message("Key \"title\" in label configuration is required".to_string()))
                .and_then(|value| value.clone().into_str())
                .unwrap_or(format!(r###"Type of the `title` ({path}) is not string"###, path = key));

            let style_res = map.get("style")
                .and_then(|it| it.clone().into_int().ok())
                .map(|it| it.to_string());
            let style = style_res
                .and_then(|it|it.parse::<u8>().ok())
                .unwrap_or(default_style);
            Ok(TagDefinition::new(tag_name, title, style))
        }
        _ => Err(ConfigError::Message(format!("Undefined problem with configuration of hte label {}", key)))
    }
}
