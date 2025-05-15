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

> **Note:** `lv00` to `lv05` form a linear gradient from background to foreground.

| Name   | Description          |
| ------ | -------------------- |
| `lv00` | Background           |
| `lv01` | Alternate Background |
| `lv02` | Selection Background |
| `lv03` | Muted foreground     |
| `lv04` | Alternate Foreground |
| `lv05` | Foreground           |
| `clrd` | Red                  |
| `clor` | Orange               |
| `clyl` | Yellow               |
| `clgn` | Green                |
| `clcy` | Cyan                 |
| `clbl` | Blue                 |
| `clmg` | Magenta              |
| `clbn` | Brown                |

### Example Configuration

```toml
[[themes]]
name = "example"

[themes.variants.light]
lv00 = "#e6e6e6"
lv01 = "#cecece"
lv02 = "#919191"
lv03 = "#6e6e6e"
lv04 = "#3e3e3e"
lv05 = "#232323"
clrd = "#8a2f3a"
clor = "#804a1f"
clyl = "#8a6a2a"
clgn = "#4f7b2b"
clcy = "#277e89"
clbl = "#2369a2"
clmg = "#753a92"
clbn = "#6b241c"

[themes.variants.dark]
lv00 = "#121212"
lv01 = "#232323"
lv02 = "#3e3e3e"
lv03 = "#6e6e6e"
lv04 = "#919191"
lv05 = "#cecece"
clrd = "#e06c75"
clor = "#d19a66"
clyl = "#e5c07b"
clgn = "#98c379"
clcy = "#56b6c2"
clbl = "#61afef"
clmg = "#c678dd"
clbn = "#be5046"
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
