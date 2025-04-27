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

## Support

Currently supports the [kitty](https://sw.kovidgoyal.net/kitty/) terminal and two predefined themes, `3024` and `default` with dark and light variants.
