use dirs;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn get_config_file() -> std::io::Result<PathBuf> {
    let config_dir = dirs::home_dir();
    if config_dir.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Home directory not found",
        ));
    }

    let config_dir = config_dir.unwrap().join(".config/gim/");
    let config_file = config_dir.join("config.toml");
    Ok(config_file)
}

pub fn get_config_into() -> std::io::Result<toml::Value> {
    get_config_into_toml(true)
}

pub fn get_config_into_toml(log_dir: bool) -> std::io::Result<toml::Value> {
    let config_file = get_config_file().expect("Failed to get config file");
    if !config_file.exists() {
        if let Some(parent) = config_file.parent() {
            fs::create_dir_all(parent)?;
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "config directory not found",
            ));
        }
        let mut settings_table = toml::map::Map::new();
        settings_table.insert(
            "example".to_string(),
            toml::Value::String("value".to_string()),
        );

        let mut ai_table = toml::map::Map::new();
        ai_table.insert("model".to_string(), toml::Value::String(String::new()));
        ai_table.insert("apikey".to_string(), toml::Value::String(String::new()));
        ai_table.insert("url".to_string(), toml::Value::String(String::new()));
        ai_table.insert("language".to_string(), toml::Value::String("English".to_string()));

        let mut default_content = toml::map::Map::new();
        default_content.insert("settings".to_string(), toml::Value::Table(settings_table));
        default_content.insert("ai".to_string(), toml::Value::Table(ai_table));
        let default_content = toml::to_string(&toml::Value::Table(default_content))
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let mut file = fs::File::create(&config_file)?;
        file.write_all(default_content.as_bytes())?;
    }
    if log_dir {
        println!("Config file is {}", config_file.display());
    }
    let content = fs::read_to_string(&config_file)?;
    let config: toml::Value = toml::from_str(&content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(config)
}

pub fn save_config(config: &toml::Value) {
    let updated_content = toml::to_string(config).expect("Failed to serialize config");
    let config_dir = get_config_file().expect("Failed to get config file");
    fs::write(&config_dir, updated_content).expect("Failed to write config file");
    println!(
        "Successfully updated AI configuration in {}",
        config_dir.display()
    );
}

#[cfg(test)]
mod tests {
    use crate::config::get_config_into;

    #[test]
    fn test_ensure_config_file_exists_creates_file() {
        let parsed = get_config_into().unwrap();
        let settings = parsed.get("settings");
        let ai = parsed.get("ai");
        assert!(settings.is_some(), "Missing settings section");
        assert!(ai.is_some(), "Missing ai section");

        let ai_table = ai.unwrap().as_table().unwrap();
        assert!(ai_table.contains_key("model"), "Missing model field");
        assert!(ai_table.contains_key("apikey"), "Missing apikey field");
        assert!(ai_table.contains_key("url"), "Missing url field");
        assert!(ai_table.contains_key("language"), "Missing language field");
        print!("{:?}", parsed)
    }
}

