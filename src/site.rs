//! Provides an interface for managing site files on disk

use std::fs::{copy, read_dir, DirBuilder, File};
use std::io::prelude::Write;
use std::path::Path;

/// Creates the site directory on disk if it does not exist
pub fn create(config: &serde_yaml::Value) -> Result<(), std::io::Error> {
    debug!("Creating site directory: {}", "./site");
    DirBuilder::new().recursive(true).create("./site")?;

    let theme = config
        .get("theme")
        .and_then(|theme| theme.as_str())
        .unwrap_or("default");

    copy_file("main.js")?;
    copy_file("style.css")?;
    copy_file(format!("themes/{}", theme).as_str())?;

    Ok(())
}

/// Writes a file to the site directory on disk, overwriting any existing file
pub fn write_file(filename: &str, contents: &str) -> Result<(), std::io::Error> {
    let destination = &format!("site/{}", filename);

    log::debug!("Creating file {}", destination);
    File::create(destination)?.write_all(contents.as_bytes())?;
    log::info!("Created {}", destination);

    Ok(())
}

/// Copies all files for the selected theme to the site directory
fn copy_file(path: impl AsRef<Path>) -> Result<(), std::io::Error> {
    let path = path.as_ref();

    if path.is_dir() {
        copy_directory_recursively(path, "site/")?;
    } else {
        let destination = format!(
            "site/{}",
            path.file_name().unwrap_or_default().to_string_lossy()
        );

        log::debug!("Copying {:?} to {}", path, destination);
        copy(&path, &destination)?;
        log::info!("Copied {} to {}", path.to_string_lossy(), destination);
    }

    Ok(())
}

/// Recursively copies the contents of the source directory to the destination
fn copy_directory_recursively(
    source: impl AsRef<Path>,
    destination: &str,
) -> Result<(), std::io::Error> {
    for entry in read_dir(&source)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            copy_directory_recursively(&path, destination)?;
        } else {
            copy_file(&path)?;
        }
    }

    Ok(())
}
