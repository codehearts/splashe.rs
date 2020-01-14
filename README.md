# splashers

A themeable splash page with fuzzy finding

## Installation

For now, you must clone this repo and run `cargo build` to build the binary

## Usage

Use `cargo run` from within this repo to build and run splashers. Your generated splash page will be output in a new `site/` directory, just point your start page to that `index.html` and you'll be good to go!

## Configuration

splashers is controlled by a `splashers.toml` file in the repo directory. By specifying a theme and groups of sites, you can style it as you want and get quick access to your sites with fuzzy finding. Just start typing!

## Example splashers.toml

```toml
theme = "night-goat"

[[site_groups]]
label = "Search"
sites = [
    { label = "DuckDuckGo", url = "https://duckduckgo.com" },
    { label = "Google", url = "https://google.com" },
]

[[site_groups]]
label = "For Fun"
sites = [
    { label = "GitHub", url = "https://github.com" },
    { label = "Reddit", url = "https://reddit.com" },
]
```
