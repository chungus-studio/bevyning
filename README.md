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

> 💡 *WindowPlugin* This plugin allows you to customize the window title, size and modes (e.g `WindowMode::Borderless`).

---

#### 🎥 2. Camera Setup & Asset Loading

- Spawn a 2D camera to render the scene.
- Make sure to set the [`ImagePlugin`](https://docs.rs/bevy/latest/bevy/prelude/struct.ImagePlugin.html) to [`default_nearest`](https://docs.rs/bevy/latest/bevy/prelude/struct.ImagePlugin.html#method.default_nearest) as we are rendering pixel art.
- Load a minimum of two assets:
  - A standalone image texture.
  - A spritesheet (texture atlas) with multiple indexed sprites.

> 🎨 Use any number of sprites you like, but at least one full image and one spritesheet.

---

#### 🎮 3. Basic Movement

- Designate one sprite as the "player".
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

### MVP 2 — Physics

(Coming soon...)

---
