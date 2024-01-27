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
- Handle new bugs from DLC
  - this is also a nice improvement to do in the code in order to handle any bug as long as they implement some Bug trait
- Code quality
  - the Yew state is being cloned a lot. We should find a way to only clone it once, and if possible, don't clone it
