use std::{fs, io::{Error, ErrorKind, Write as _, Result}};
use std::path::PathBuf;
use toml::{Value, map};

use crate::directory::config_dir;

fn get_config_file() -> Result<PathBuf> {
    let config_dir = config_dir()?;
    let config_file = config_dir.join("config.toml");
    Ok(config_file)
}

pub fn get_config() -> Result<Value> {
    get_config_into_toml(true)
}

pub fn get_config_into_toml(log_dir: bool) -> Result<Value> {
    let config_file = get_config_file().expect("Failed to get config file");
    if !config_file.exists() {
        if let Some(parent) = config_file.parent() {
            fs::create_dir_all(parent)?;
        } else {
            return Err(Error::new(
                ErrorKind::NotFound,
                "config directory not found",
            ));
        }
        let mut update_table = map::Map::new();
        update_table.insert("tried".to_string(), Value::Integer(0));
        update_table.insert("max_try".to_string(), Value::Integer(5));
        update_table.insert(
            "last_try_day".to_string(),
            Value::String("2000-01-01".to_string()),
        );
        update_table.insert("try_interval_days".to_string(), Value::Integer(30));

        let mut ai_table = map::Map::new();
        ai_table.insert("model".to_string(), Value::String(String::new()));
        ai_table.insert("apikey".to_string(), Value::String(String::new()));
        ai_table.insert("url".to_string(), Value::String(String::new()));
        ai_table.insert("language".to_string(), Value::String("English".to_string()));

        let mut default_content = map::Map::new();
        default_content.insert("update".to_string(), Value::Table(update_table));
        default_content.insert("ai".to_string(), Value::Table(ai_table));
        let default_content = toml::to_string(&Value::Table(default_content))
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        let mut file = fs::File::create(&config_file)?;
        file.write_all(default_content.as_bytes())?;
    }
    if log_dir {
        println!("Config file is {}", config_file.display());
    }
    let content = fs::read_to_string(&config_file)?;
    let config: Value = toml::from_str(&content)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    Ok(config)
}

pub fn get_config_value(section: &str, key: &str) -> Result<Value> {
    let config = get_config()?;
    let section_table = config
        .get(section)
        .ok_or_else(|| {
            Error::new(
                ErrorKind::NotFound,
                format!("Section '{}' not found", section),
            )
        })?
        .as_table()
        .ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Section '{}' is not a table", section),
            )
        })?;

    section_table
        .get(key)
        .ok_or_else(|| {
            Error::new(
                ErrorKind::NotFound,
                format!("Key '{}' not found in section '{}'", key, section),
            )
        })
        .map(|v| v.clone())
}

pub fn update_config_value(section: &str, key: &str, value: Value) -> Result<()> {
    let mut config = get_config_into_toml(false)?;
    let section_table = config
        .get_mut(section)
        .ok_or_else(|| {
            Error::new(
                ErrorKind::NotFound,
                format!("Section '{}' not found", section),
            )
        })?
        .as_table_mut()
        .ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                format!("Section '{}' is not a table", section),
            )
        })?;

    if let Some(existing_value) = section_table.get(key) {
        if existing_value == &value {
            return Ok(());
        }
    }

    section_table.insert(key.to_string(), value);
    save_config(&config)?;
    Ok(())
}

pub fn save_config(config: &Value) -> Result<()> {
    let updated_content = toml::to_string(config)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    let config_dir = get_config_file()?;
    fs::write(&config_dir, updated_content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config::get_config;

    #[test]
    fn test_ensure_config_file_exists_creates_file() {
        let parsed = get_config().unwrap();
        let update = parsed.get("update");
        let ai = parsed.get("ai");
        assert!(update.is_some(), "Missing update section");
        assert!(ai.is_some(), "Missing ai section");

        let ai_table = ai.unwrap().as_table().unwrap();
        assert!(ai_table.contains_key("model"), "Missing model field");
        assert!(ai_table.contains_key("apikey"), "Missing apikey field");
        assert!(ai_table.contains_key("url"), "Missing url field");
        assert!(ai_table.contains_key("language"), "Missing language field");
        print!("{:?}", parsed)
    }
}
