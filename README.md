# splashers

A themeable splash page with fuzzy finding

## Installation

For now, you must clone this repo and run `cargo build` to build the binary

## Usage

Use `cargo run` from within this repo to build and run splashers. Your generated splash page will be output in a new `site/` directory, just point your start page to that `index.html` and you'll be good to go!

## Configuration

splashers is controlled by a `splashers.yaml` file in the repo directory. By specifying a theme and groups of sites, you can style it as you want and get quick access to your sites with fuzzy finding. Just start typing!

## Example splashers.toml

```yaml
theme: night-goat
site_groups:
    Search:
        DuckDuckGo: https://duckduckgo.com
        Google: https://google.com
    For Fun:
        GitHub: https://github.com
        Reddit: https://reddit.com
```
