# Grumpy Flappy Bird – Rust Edition

A faithful yet lightweight recreation of the classic **Flappy Bird** written in Rust with the [Bevy](https://bevyengine.org) game engine.

---

## Gameplay
* Tap **Space** to flap and gain altitude.
* Fly through an endless procession of randomly–sized pipe pairs.
* Collide with a pipe or the ground and the game ends.

### Core Mechanics (straight from `src/main.rs`)
| Mechanic | Constant | Value |
|----------|----------|-------|
| Pipe movement speed | `PIPE_MOVEMENT_SPEED` | **50.0 px/sec** |
| Horizontal spacing between consecutive pipe pairs | `PIPE_OFFSET` | **300 px** |
| Pipe width | `PIPE_WIDTH` | **75 px** |
| Vertical gap between top & bottom pipes | `PIPE_GAP` | **150 px** |
| Gravity | `player.acceleration.y` | **-20.0** |
| Jump impulse | `JUMP_FORCE` | **5000.0** |

These values can be tweaked in `src/main.rs` to adjust difficulty.

---

## Running the Game
```bash
# Make sure you have Rust installed (https://rustup.rs)
cargo run
```
The first run will take a little longer while Rust downloads dependencies and compiles Bevy.

---

## Project Structure
```
├── assets/                # Sprite frames for the bird
│   └── Grumpy Flappy Bird/
│       ├── frame-1.png
│       └── …
├── src/
│   └── main.rs            # Game code (≈300 lines)
├── Cargo.toml             # Dependency declarations (bevy, rand)
└── README.md              # You are here
```

### Technical Highlights
* **Entity-Component-System (ECS)** – Powered by Bevy; the bird, pipes and floor are entities with components.
* **Procedural pipe generation** – Heights are picked via `rand` each time the rightmost pipe leaves the screen.
* **Axis-aligned bounding box collisions** – Implemented manually with `Rect::intersect` for learning purposes.
* **Camera & rendering** – Utilises Bevy's 2D camera and `Mesh2d` primitives for pipes/floor; sprites for the bird.

---

## Controls
* **Space** – Flap / jump.
* Close the window or hit **Ctrl-C** in the terminal to quit.

---

## Assets & Licensing
The bird sprite frames are provided in `assets/Grumpy Flappy Bird`. They are included for educational use only.

The rest of the code is licensed under the MIT License. See `LICENSE` for details.