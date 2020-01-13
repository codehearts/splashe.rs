//! Configurable properties which affect site rendering

use serde::Serialize;

/// Label and URL for a site
#[derive(Serialize)]
struct Site {
    /// Label for the site
    pub label: &'static str,

    /// URL for the site
    pub url: &'static str,
}

/// Grouping of sites under a single label
#[derive(Serialize)]
struct SiteGroup {
    /// Label for the site group
    pub label: &'static str,

    /// Sites under the group, in the order to display them
    pub sites: Vec<Site>,
}

/// Configuration for the rendered site
#[derive(Serialize)]
pub struct Config {
    /// Name of the theme to style splashers with
    theme: String,

    /// Array of site groups to display, in the order to display them
    site_groups: [SiteGroup; 0],
}

impl Config {
    pub fn new() -> Self {
        Self {
            theme: String::from("night-goat"),
            site_groups: [],
        }
    }
}
