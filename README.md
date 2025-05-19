# gim-config

A configuration management library for GIM applications

## Features

- Load and save configuration in TOML format
- Automatic creation of default configuration file
- Configuration file located in `~/.config/gim/config.toml`

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
gim-config = "0.1.0"
```

Example code:

```rust
use gim_config;

fn main() {
    // Load config
    let config = gim_config::get_config_into().unwrap();
    
    // Modify config
    let mut config = config.clone();
    config["ai"]["model"] = toml::Value::String("gpt-4".to_string());
    
    // Save config
    gim_config::save_config(&config);
}
```

## License

MIT