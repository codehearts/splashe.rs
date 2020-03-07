//! Splash page for accessing sites with fuzzy matching

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

use std::boxed::Box;
use tera::{Context, Tera};

#[macro_use]
extern crate log;

mod site;

mod config;
use crate::config::try_from_yaml;

/// Creates the site directory on disk with a rendered index.html
/// The chosen theme will be used in the rendering of index.html
///
/// # Errors
///
/// If the config is malformed or a filesystem operation fails
pub fn create_site(_theme: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let tera = Tera::new("templates/**/*.html")?;

    let config = try_from_yaml("splashers.yaml")?;
    let render_context = Context::from_serialize(config.clone())?;
    let rendered_site = tera.render("index.html", &render_context)?;

    site::create(&config)?;
    site::write_file("index.html", rendered_site.as_str())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
