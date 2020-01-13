//! Splash page for accessing sites with fuzzy matching

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

use splashers::create_site;
use std::boxed::Box;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_site(None).map_err(|error| {
        eprintln!("Failed to create site");
        error
    })
}
