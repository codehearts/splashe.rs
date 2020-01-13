//! Provides an interface for managing site files on disk

use std::fs::{copy, read_dir, DirBuilder, File};
use std::io::prelude::Write;
use std::path::Path;

/// Creates the site directory on disk if it does not exist
pub fn create() -> Result<(), std::io::Error> {
    DirBuilder::new().recursive(true).create("./site")?;

    copy_file("main.js")?;
    copy_file("style.css")?;
    copy_file(format!("themes/{}", "night-goat").as_str())?;

    Ok(())
}

/// Writes a file to the site directory on disk, overwriting any existing file
pub fn write_file(filename: &str, contents: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(format!("site/{}", filename))?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

/// Copies all files for the selected theme to the site directory
fn copy_file(path: impl AsRef<Path>) -> Result<(), std::io::Error> {
    let path = path.as_ref();

    if path.is_dir() {
        copy_directory_recursively(path, "site/")
    } else {
        copy(
            path,
            format!("site/{:?}", path.file_name().unwrap_or_default()),
        )
        .map(|_| ())
    }
}

/// Recursively copies the contents of the source directory to the destination
fn copy_directory_recursively(
    source: impl AsRef<Path>,
    destination: &str,
) -> Result<(), std::io::Error> {
    let source_string = source.as_ref().to_str().unwrap_or_default();

    for entry in read_dir(&source)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            copy_directory_recursively(&path, destination)?;
        } else if let Some(path_string) = path.to_str() {
            copy(&path, path_string.replace(source_string, destination)).map(|_| ())?
        }
    }
    Ok(())
}
