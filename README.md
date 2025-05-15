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

### Levels and Colors

> [!Note]  
> `lv00` to `lv05` form a linear gradient from background to foreground.

| Color                                              | Name   | Description          | Example Usage           |
| -------------------------------------------------- | ------ | -------------------- | ----------------------- |
| ![](https://placehold.co/30/0A0A0A/0A0A0A/?text=.) | `lv00` | Background           | Default Background      |
| ![](https://placehold.co/30/1A1A1A/1A1A1A/?text=.) | `lv01` | Alternate Background | Floating/UI Background  |
| ![](https://placehold.co/30/2A2A2A/2A2A2A/?text=.) | `lv02` | Selection Background | Marks, Visual Selection |
| ![](https://placehold.co/30/4A4A4A/4A4A4A/?text=.) | `lv03` | Muted foreground     | Inlay hints, Ghost Text |
| ![](https://placehold.co/30/7A7A7A/7A7A7A/?text=.) | `lv04` | Alternate Foreground | Comments                |
| ![](https://placehold.co/30/C1C1C1/C1C1C1/?text=.) | `lv05` | Foreground           | Normal Text             |
| ![](https://placehold.co/30/FF5C57/FF5C57/?text=.) | `clrd` | Red                  | Errors, Diff Delete     |
| ![](https://placehold.co/30/FF9F43/FF9F43/?text=.) | `clor` | Orange               | Constants               |
| ![](https://placehold.co/30/FFD93B/FFD93B/?text=.) | `clyl` | Yellow               | Types, Diff Changed     |
| ![](https://placehold.co/30/5CC26C/5CC26C/?text=.) | `clgn` | Green                | Strings, Diff Add       |
| ![](https://placehold.co/30/3FD1C7/3FD1C7/?text=.) | `clcy` | Cyan                 | Modules                 |
| ![](https://placehold.co/30/4D8EDA/4D8EDA/?text=.) | `clbl` | Blue                 | Functions               |
| ![](https://placehold.co/30/C586C0/C586C0/?text=.) | `clmg` | Magenta              | Keywords                |
| ![](https://placehold.co/30/A9746E/A9746E/?text=.) | `clbn` | Brown                | Special                 |

### Example Configuration

```toml
[[themes]]
name = "example"

[themes.variants.light]
lv00 = "#F7F7F7"
lv01 = "#E4E4E4"
lv02 = "#C8C8C8"
lv03 = "#A0A0A0"
lv04 = "#707070"
lv05 = "#303030"
clrd = "#C4473D"
clor = "#B5671D"
clyl = "#B29A00"
clgn = "#3C8C4D"
clcy = "#2B8C86"
clbl = "#3663AA"
clmg = "#8A5A86"
clbn = "#6E4C47"

[themes.variants.dark]
lv00 = "#0A0A0A"
lv01 = "#1A1A1A"
lv02 = "#2A2A2A"
lv03 = "#4A4A4A"
lv04 = "#7A7A7A"
lv05 = "#C1C1C1"
clrd = "#FF5C57"
clor = "#FF9F43"
clyl = "#FFD93B"
clgn = "#5CC26C"
clcy = "#3FD1C7"
clbl = "#4D8EDA"
clmg = "#C586C0"
clbn = "#A9746E"
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

### Neovim

Use colors in your colorscheme configuration.

```lua
local colors = dofile("/home/<user>/.config/daub/gen/colors.lua")
```

Set colorscheme in your config. Daub sets `vim.g.colors_name` as "daub".

Lua:

```lua
vim.cmd.colorscheme("daub")
```

### Polybar

Symlink generated `.Xresources` to `$HOME`:

```sh
ln -s ~/.config/daub/gen/.Xresources ~/
```

Use ANSI colors via xrdb in your `config.ini`:

```ini
${xrdb:<color>}
```

### Rofi

Include `colors.rasi` in your Rofi config.

```css
@theme "~/.config/daub/gen/colors.rasi";
```
