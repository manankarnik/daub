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

### Example Configuration

```toml
[[themes]]
name = "example"

[themes.variants.light]
background = "#e6e6e6"
background_alt = "#cecece"
background_selection = "#919191"
foreground_invisible = "#6e6e6e"
foreground_dark = "#3e3e3e"
foreground = "#232323"
foreground = "#121212"
red = "#8a2f3a"
orange = "#804a1f"
yellow = "#8a6a2a"
green = "#4f7b2b"
cyan = "#277e89"
blue = "#2369a2"
purple = "#753a92"
brown = "#6b241c"

[themes.variants.dark]
background = "#121212"
background_alt ="#232323"
background_selection = "#3e3e3e"
foreground_invisible = "#6e6e6e"
foreground_dark = "#919191"
foreground = "#cecece"
foreground = "#e6e6e6"
red = "#e06c75"
orange = "#d19a66"
yellow = "#e5c07b"
green = "#98c379"
cyan = "#56b6c2"
blue = "#61afef"
purple = "#c678dd"
brown = "#be5046"
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
