//! Splash page for accessing sites with fuzzy matching

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

use std::boxed::Box;
use std::fs::{copy, DirBuilder, File};
use std::io::prelude::Write;
use tera::{Context, Tera};

pub mod sites;

mod copy;

mod config;
use crate::config::site_groups;

/// Creates the site/ directory on disk with a rendered index.html
/// The given theme will be used in the rendering of index.html
pub fn create_site(_theme: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut render_context = Context::new();
    render_context.insert("theme", "night-goat");
    render_context.insert("site_groups", &site_groups());

    let tera = Tera::new("templates/**/*.html")?;
    let rendered_site = tera.render("index.html", &render_context)?;

    create_site_directory()?;
    write_site_file("index.html", rendered_site.as_str())?;
    copy_theme_files("night-goat")?;
    Ok(())
}

/// Creates the site/ directory on disk if it does not exist
fn create_site_directory() -> Result<(), std::io::Error> {
    copy("style.css", "site/style.css").map(|_| ())?;
    copy("main.js", "site/main.js").map(|_| ())?;
    DirBuilder::new().recursive(true).create("./site")
}

/// Writes a file to the site/ directory on disk, overwriting any existing file
fn write_site_file(filename: &str, contents: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(format!("site/{}", filename))?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

/// Copies all files for the selected theme to the site directory
fn copy_theme_files(theme: &str) -> Result<(), std::io::Error> {
    copy::directory_recursively(format!("themes/{}", theme).as_str(), "site/");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
