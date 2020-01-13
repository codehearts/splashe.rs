//! Configurable properties which affect site rendering

use crate::sites::SiteGroup;

/// Name of the theme to style splashers with
pub const theme: &str = "night-goat";

/// Array of site groups to display, in the order to display them
pub fn site_groups() -> [SiteGroup; 0] {
    []
}
