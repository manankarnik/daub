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

Each variant is defined under `themes.variants.{variantName}`, where `{variantName}` can be any name (e.g., `light`, `dark`, `day`, `night` etc.).

### Variant Keys

| Key                   | Required | Description                                                                                                           |
| --------------------- | -------- | --------------------------------------------------------------------------------------------------------------------- |
| `mode`                | Yes      | Must be either `"light"` or `"dark"`. This determines the how the alternate foreground and background colors are set. |
| `color0` to `color7`  | Yes      | Core color palette used for primary theme elements.                                                                   |
| `foreground`          | Yes      | Text (foreground) color.                                                                                              |
| `background`          | Yes      | Background color.                                                                                                     |
| `color8` to `color15` | No       | Extended color palette. If any of the keys are not defined, `color0` through `color7` are used as fallback.           |
| `cursor`              | No       | Cursor color. If not defined, `foreground` is used instead.                                                           |

### Example Configuration

```toml
[[themes]]
name = "default"

[themes.variants.light]
mode = "light"
color0 = "#151515"
color1 = "#ac4142"
color2 = "#90a959"
color3 = "#f4bf75"
color4 = "#6a9fb5"
color5 = "#aa759f"
color6 = "#75b5aa"
color7 = "#d0d0d0"
color8 = "#505050"
color9 = "#ac4142"
color10 = "#90a959"
color11 = "#f4bf75"
color12 = "#6a9fb5"
color13 = "#aa759f"
color14 = "#75b5aa"
color15 = "#f5f5f5"
foreground = "#303030"
background = "#f5f5f5"
cursor = "#303030"

[themes.variants.dark]
mode = "dark"
color0 = "#151515"
color1 = "#ac4142"
color2 = "#90a959"
color3 = "#f4bf75"
color4 = "#6a9fb5"
color5 = "#aa759f"
color6 = "#75b5aa"
color7 = "#d0d0d0"
color8 = "#505050"
color9 = "#ac4142"
color10 = "#90a959"
color11 = "#f4bf75"
color12 = "#6a9fb5"
color13 = "#aa759f"
color14 = "#75b5aa"
color15 = "#f5f5f5"
foreground = "#d0d0d0"
background = "#151515"
cursor = "#d0d0d0"
```

## How to Configure...

### BSPWM

Source `colors.sh` and use variables in your `bspwmrc`.

```sh
. "$HOME/.config/daub/gen/ccolors.sh"
bspc config normal_border_color "$background1"
bspc config active_border_color "$color4"
bspc config focused_border_color "$color4"
bspc config presel_feedback_color "$background1"
```

### Kitty

Include `colors.conf` in your `kitty.conf`.

```sh
include ~/.config/daub/gen/colors.conf
```

### Rofi

Include `colors.rasi` in your Rofi config.

```css
@theme "~/.config/daub/gen/ccolors.rasi";
```
