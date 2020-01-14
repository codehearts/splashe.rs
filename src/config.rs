//! Configurable properties which affect site rendering

use serde_yaml::Value as YamlValue;
use std::fs::File;

/// Creates a new Config object from a toml file
pub fn try_from_yaml(filename: &str) -> Result<YamlValue, Box<dyn std::error::Error>> {
    log::debug!("Reading config from {}", filename);
    let yaml_file = File::open(filename)?;
    let config: YamlValue = serde_yaml::from_reader(yaml_file)?;
    log::info!("Read config {}", filename);

    Ok(config)
}
