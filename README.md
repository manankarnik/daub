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

## How to

### BSPWM

Source `colors.sh` and use variables in your `bspwmrc`.

```sh
. "$HOME/.config/daub/colors.sh"
bspc config normal_border_color "$color8"
bspc config active_border_color "$color4"
bspc config focused_border_color "$color4"
bspc config presel_feedback_color "$color0"
```

### Kitty

Include `colors.conf` in your `kitty.conf`.

```sh
include ~/.config/daub/colors.conf
```

### Rofi

Include `colors.rasi` in your Rofi config.

```css
@theme "~/.config/daub/colors.rasi";
```
