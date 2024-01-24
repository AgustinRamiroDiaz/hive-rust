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
  - Once your Queen Bee has been placed (but not before), you can decide whether to use each turn after that to place another tile or to move one of the pieces that have already been placed.
  - If a player can nether place a new piece or move an existing piece, the turn passes to their opponent who then takes their turn again.
- Code quality
  - the Yew state is being cloned a lot. We should find a way to only clone it once, and if possible, don't clone it
