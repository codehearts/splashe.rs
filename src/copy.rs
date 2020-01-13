use std::fs::{copy, read_dir};
use std::path::Path;

/// Recursively copies the contents of the source directory to the destination
pub fn directory_recursively(
    source: impl AsRef<Path>,
    destination: &str,
) -> Result<(), std::io::Error> {
    let source_string = source.as_ref().to_str().unwrap_or_default();

    for entry in read_dir(&source)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            directory_recursively(&path, destination)?;
        } else if let Some(path_string) = path.to_str() {
            copy(&path, path_string.replace(source_string, destination)).map(|_| ())?
        }
    }
    Ok(())
}
