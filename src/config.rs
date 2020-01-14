//! Configurable properties which affect site rendering

use serde::{Deserialize, Serialize};

/// Label and URL for a site
#[derive(Serialize, Deserialize)]
struct Site {
    /// Label for the site
    pub label: String,

    /// URL for the site
    pub url: String,
}

/// Grouping of sites under a single label
#[derive(Serialize, Deserialize)]
struct SiteGroup {
    /// Label for the site group
    pub label: String,

    /// Sites under the group, in the order to display them
    pub sites: Vec<Site>,
}

/// Configuration for the rendered site
#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Name of the theme to style splashers with
    theme: String,

    /// List of site groups to display, in the order to display them
    site_groups: Vec<SiteGroup>,
}

impl Config {
    /// Creates a new Config object from a toml file
    pub fn try_from_toml(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let toml_contents = std::fs::read_to_string(filename)?;
        let config: Self = toml::from_str(toml_contents.as_str())?;

        Ok(config)
    }
}
