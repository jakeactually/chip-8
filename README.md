# chip-8

A chip 8 emulator made in rust and piston

# How to

You must have [rust](https://www.rust-lang.org/es-ES/install.html) installed. Then in shell run:

```
cargo run
```

And by default it will run Pong. You can change the game in src/main.rs

```rust
if args.len() < 2 {
    run::run("roms/Pong (1 player).ch8", false, false);
    return
}
```

with any of the availables in the roms folder.

The roms/roms.json file has info about the game, if it were to have "quirks", they should be provided as booleans in the "run" function.

```json
{
    "title": "BLINKY",
    "file": "Blinky [Hans Christian Egeberg, 1991].ch8",
    "quirks": { "loadStore": true, "shift": true },
    "description": "Blinky (1991), by Hans Christian Egeberg<br/><br/>Pacman clone.<br/>3, 6 - down/up. 7, 8 - left/right"
}
```

```rust
pub fn run(rom: &str, shift_hack: bool, memory_hack: bool)
```

Most games though run with false, false.

# Screenshots

Blinky

![blinky](https://jakeactually.com/github/chip-8/1.JPG "Blinky")

Pong

![pong](https://jakeactually.com/github/chip-8/2.JPG "Pong")

Airplane

![airplane](https://jakeactually.com/github/chip-8/1.JPG "Airplane")
