# "The Hive" 🐝 board game made in Rust 🦀

Demo at https://agustinramirodiaz.github.io/hive-rust/

# Local development

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Yew](https://yew.rs/docs/getting-started/introduction)

## Run

```bash
trunk serve
```

## TODO

- Add rules to game (taken from https://www.ultraboardgames.com/hive/game-rules.php)
  - If a player can nether place a new piece or move an existing piece, the turn passes to their opponent who then takes their turn again.
  - Handle ties. For example, both bees are together and another bug comes near touching both of them, closing both of them.
- Code quality
  - the Yew state is being cloned a lot. We should find a way to only clone it once, and if possible, don't clone it
