# 🕹️ bevyning

A learning repository for exploring the **basics of Bevy** — the modular, data-driven game engine built in Rust.

This project is organized around building a series of **minimum viable projects (MVPs)** to progressively gain familiarity with Bevy concepts such as systems, plugins, asset loading, and physics.

---

## 🔗 Useful Links

- 📘 [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- 📁 [Bevy Official Examples](https://github.com/bevyengine/bevy/tree/main/examples)
- ⚙️ [Avian 2D (Physics Library)](https://docs.rs/avian2d/latest/avian2d/)

---

## 🛠️ Development Rules

> ✅ **All work must be done on branches via pull requests.**  
> 🚫 **No direct commits to `main`.**

---

## 🎯 MVPs

Each MVP focuses on a specific set of Bevy features. Treat them as progressive, hands-on mini-projects.

---

### MVP 1 — Basic 2D Game

#### 🧪 1. Hello World

Set up a minimal Bevy app with [`DefaultPlugins`](https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html) and a blank window.

> 💡 *WindowPlugin* This plugin allows you to customize the window title, size and modes (e.g `WindowMode::Windowed`).

---

#### 🎥 2. Camera Setup & Asset Loading

- Spawn a 2D camera to render the scene.
- Make sure to set the [`ImagePlugin`](https://docs.rs/bevy/latest/bevy/prelude/struct.ImagePlugin.html) to [`default_nearest`](https://docs.rs/bevy/latest/bevy/prelude/struct.ImagePlugin.html#method.default_nearest) as we are rendering pixel art.
- Load a minimum of two assets:
  - A standalone image texture.
  - A spritesheet (texture atlas) with multiple indexed sprites. (for the player, make sure there are multiple frames related to a player action, like walking, running, jumping, etc)

> 🎨 Use any number of sprites you like, but at least one full image and one spritesheet.

---

#### 🎮 3. Basic Movement

- Designate the spritesheet sprite as the "player". (animations will be covered in another MVP)
- Implement movement using any input method (`WASD`, arrow keys, or click-to-move).
- Handle screen wrapping if not using click-to-move.

> ⚠️ *Tip:* Always apply **delta time** for framerate-independent movement.

---

#### 📋 4. Game State & Menu

- Create two states: `Menu` and `Playing`.
- Add a basic UI menu:
  - `Start Game` → enters `Playing` state.
  - `Exit` → quits the game.
- Press `Esc` during gameplay to return to the menu.

---

### MVP 2 — Physics & Animations

---
#### 1. Basic Physics with [`Avian2D`](https://docs.rs/avian2d/latest/avian2d/)

- Setup gravity (zero if top down, positive if bottom up).
- Add physics components to the player & other necessary entities.
- Collisions:
    - The player should be restricted to move in certain spots ([`RigidBody::Static`](https://docs.rs/avian2d/latest/avian2d/dynamics/rigid_body/enum.RigidBody.html))
    - Dynamic collisions should interact with the player or other entities. The logic of what involves collisions is up to you.
    - The player should be able to interact with 1 object in the game (via intersection detection).
---

#### 2. Animations
- Add some entity state to the player representing the state you want to animate.
- Add a system to animate the player's spritesheet based on the given state. Again, this implementation is up to you.
---

### MVP 3 - Custom Assets & Shaders

---
#### 1. Implement a Custom Asset Loader

Right now, Bevy specific assets and loaders were used (`Image`, `TextureAtlasLayout`) . In order to have dynamic configurations for game objects, using Assets are usually a great place to get this behaviour.
This also allows the game to hot-reload assets as they change without restarting the game (for this make sure the `file_watcher` feature is enabled in you `Cargo.toml`)

For this step, choose an area of your game that you would like to be able to configure from outside of the game (`.ron`, `.json` file) and implement a custom asset loader for it (docs: [here](https://docs.rs/bevy/0.16.1/bevy/asset/trait.AssetLoader.html), example : [here](https://github.com/bevyengine/bevy/blob/main/examples/asset/custom_asset.rs)).

By default, using an Asset will not necessarily make it automatically reload in game. Make sure this is the case for this specific Asset. I should be able to change the file and whatever this Asset does should now include the new configuration in it's behaviour.

#### 2. Shaders

Bevy has a powerful yet pretty simple shader pipeline setup for you to include your own Materials in the game world or the UI world.

For this step, implement both a [UIMaterial]() and a [Material2d]() shader of your choosing. It can be for a player/prop effect, grayscale shader, projectile effect, anything.


