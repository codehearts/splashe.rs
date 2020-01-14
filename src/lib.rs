//! Splash page for accessing sites with fuzzy matching

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

use std::boxed::Box;
use tera::{Context, Tera};

mod site;

mod config;
use crate::config::Config;

/// Creates the site directory on disk with a rendered index.html
/// The chosen theme will be used in the rendering of index.html
pub fn create_site(_theme: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let tera = Tera::new("templates/**/*.html")?;

    let config = Config::try_from_toml("splashers.toml")?;
    let render_context = Context::from_serialize(config)?;
    let rendered_site = tera.render("index.html", &render_context)?;

    site::create()?;
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
