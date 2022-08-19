# splashers

A themeable splash page with fuzzy finding

## Installation

For now, you must clone this repo and run `cargo build` to build the binary

## Usage

Use `cargo run` from within this repo to build and run splashers. Your generated splash page will be output in a new `site/` directory, just point your start page to that `index.html` and you'll be good to go!

## Configuration

splashers is controlled by a `splashers.yaml` file in the repo directory. By specifying a theme and groups of sites, you can style it as you want and get quick access to your sites with fuzzy finding. Just start typing!

## Example splashers.yaml

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

## Custom Fonts in Modern Safari

Modern versions of Safari block user-installed fonts from loading to prevent device fingerprinting. This includes local HTML documents, but can be circumvented by copying the fonts to your output directory and loading them with `@font-face` in `theme.css`. This is already done for the following fonts, which can be found at these locations if installed via Font Book:

- Wawati SC: `/System/Library/PrivateFrameworks/FontServices.framework/Versions/A/Resources/Fonts/Subsets/WawaSC-Regular.otf`

## Credit
- "night-goat" theme background by [Mini on pixiv](https://www.pixiv.net/en/artworks/75632428)
- "underwater-train" theme background by [tama on pixiv](https://www.pixiv.net/en/artworks/63318516)
- "underwater-train" theme lace styles from [aegnis on codepen](https://codepen.io/aegnis/pen/bBpWGR)
- "persona-5" theme background owned by Atlus
