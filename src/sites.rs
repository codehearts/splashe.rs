//! Types for grouping and representing websites

use serde::Serialize;

/// Label and URL for a site
#[derive(Serialize)]
pub struct Site {
    /// Label for the site
    pub label: &'static str,

    /// URL for the site
    pub url: &'static str,
}

/// Grouping of sites under a single label
#[derive(Serialize)]
pub struct SiteGroup {
    /// Label for the site group
    pub label: &'static str,

    /// Sites under the group, in the order to display them
    pub sites: Vec<Site>,
}
