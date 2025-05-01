# Daub

> To apply (a substance, color, etc.) to a surface in a quick or rough manner.

Apply and switch color schemes in real-time!

## Quickstart

### Clone Repository

```sh
git clone https://github.com/manankarnik/daub
```

### Change Directory

```sh
cd daub
```

### Run with Arguments

Use `--` to pass arguments to `daub` when invoking via `cargo`

```sh
cargo run -- help
```

## Commands

### Set

Set a theme

```sh
cargo run -- set <theme> <variant>
```

### Clean

Clean generated files

```sh
cargo run -- clean
```

### List

List themes and variants

```sh
cargo run -- list
```

## Configuration

Custom themes can be configured at:

```sh
$XDG_CONFIG_HOME/daub/daub.toml
```

Multiple themes can be defined in the same config file. Each theme is listed under the `[[themes]]` array and contains one or more variants.

### Variants

Each variant is defined under `themes.variants.{variant}`, where `{variant}` can be any name (e.g., `light`, `dark`, `day`, `night` etc.).

### Variant Keys

The following table is adapted from [Base16 Styling guidelines](https://github.com/tinted-theming/home/blob/main/styling.md).

| Color                                                | base0X | ANSI     | Terminal                 | Text Editor                                                         |
| ---------------------------------------------------- | ------ | -------- | ------------------------ | ------------------------------------------------------------------- |
| ![#](https://placehold.co/25/282c34/000000?text=%2B) | base00 | 0        | Black (Background)       | Default Background                                                  |
| ![#](https://placehold.co/25/3f4451/000000?text=%2B) | base01 | 18       | (Darkest Gray)           | Lighter Background (Used for status bars)                           |
| ![#](https://placehold.co/25/4f5666/000000?text=%2B) | base02 | 19       | (Dark Gray)              | Selection Background                                                |
| ![#](https://placehold.co/25/545862/000000?text=%2B) | base03 | 8        | Bright Black (Gray)      | Comments, Invisibles, Line Highlighting                             |
| ![#](https://placehold.co/25/9196a1/000000?text=%2B) | base04 | 20       | (Light Gray)             | Dark Foreground (Used for status bars)                              |
| ![#](https://placehold.co/25/abb2bf/000000?text=%2B) | base05 | 21       | Foreground               | Default Foreground, Caret, Delimiters, Operators                    |
| ![#](https://placehold.co/25/e6e6e6/000000?text=%2B) | base06 | 7        | White                    | Light Foreground                                                    |
| ![#](https://placehold.co/25/ffffff/000000?text=%2B) | base07 | 15       | Bright White             | The Lightest Foreground                                             |
| ![#](https://placehold.co/25/e06c75/000000?text=%2B) | base08 | 1 and 9  | Red and Bright Red       | Variables, XML Tags, Markup Link Text, Markup Lists, Diff Deleted   |
| ![#](https://placehold.co/25/d19a66/000000?text=%2B) | base09 | 16       | (Orange)                 | Integers, Boolean, Constants, XML Attributes, Markup Link Url       |
| ![#](https://placehold.co/25/e5c07b/000000?text=%2B) | base0A | 3 and 11 | Yellow and Bright Yellow | Classes, Markup Bold, Search Text Background                        |
| ![#](https://placehold.co/25/98c379/000000?text=%2B) | base0B | 2 and 10 | Green and Bright Green   | Strings, Inherited Class, Markup Code, Diff Inserted                |
| ![#](https://placehold.co/25/56b6c2/000000?text=%2B) | base0C | 6 and 14 | Cyan and Bright Cyan     | Support, Regular Expressions, Escape Characters, Markup Quotes      |
| ![#](https://placehold.co/25/61afef/000000?text=%2B) | base0D | 4 and 12 | Blue and Bright Blue     | Functions, Methods, Attribute IDs, Headings                         |
| ![#](https://placehold.co/25/c678dd/000000?text=%2B) | base0E | 5 and 13 | Purple and Bright Purple | Keywords, Storage, Selector, Markup Italic, Diff Changed            |
| ![#](https://placehold.co/25/be5046/000000?text=%2B) | base0F | 17       | (Dark Red or Brown)      | Deprecated, Opening/Closing Embedded Language Tags, e.g. `<?php ?>` |

### Example Configuration

```toml
[[themes]]
name = "example"

[themes.variants.light]
base00 = "#ffffff"
base01 = "#e6e6e6"
base02 = "#abb2bf"
base03 = "#9196a1"
base04 = "#545862"
base05 = "#4f5666"
base06 = "#3f4451"
base07 = "#282c34"
base08 = "#e06c75"
base09 = "#d19a66"
base0A = "#e5c07b"
base0B = "#98c379"
base0C = "#56b6c2"
base0D = "#61afef"
base0E = "#c678dd"
base0F = "#be5046"

[themes.variants.dark]
base00 = "#282c34"
base01 = "#3f4451"
base02 = "#4f5666"
base03 = "#545862"
base04 = "#9196a1"
base05 = "#abb2bf"
base06 = "#e6e6e6"
base07 = "#ffffff"
base08 = "#e06c75"
base09 = "#d19a66"
base0A = "#e5c07b"
base0B = "#98c379"
base0C = "#56b6c2"
base0D = "#61afef"
base0E = "#c678dd"
base0F = "#be5046"
```

## How to Configure...

### BSPWM

Source `colors.sh` and use variables in your `bspwmrc`.

```sh
. "$HOME/.config/daub/gen/colors.sh"
```

### Kitty

Include `colors.conf` in your `kitty.conf`.

```sh
include ~/.config/daub/gen/colors.conf
```

### Neovim (or Vim)

Create a colorscheme file `{VIM/NEOVIM CONFIG DIR}/colors/daub.vim` with the following content:

```vim
source ~/.config/daub/gen/colors.vim
```

Set colorscheme in your config.

Lua:

```lua
vim.cmd("colorscheme daub")
```

Vimscript:

```vim
colorscheme daub
```

### Rofi

Include `colors.rasi` in your Rofi config.

```css
@theme "~/.config/daub/gen/colors.rasi";
```
