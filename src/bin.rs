//! Splash page for accessing sites with fuzzy matching

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

use env_logger;
use splashers::create_site;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    create_site(None).map_err(|error| {
        eprintln!("Failed to create site");
        error
    })
}
