# bevyning

Learning repository for Bevy basics


## Useful links
- [Bevy Book](https://bevy-cheatbook.github.io/)
- [Bevy examples repo](https://github.com/bevyengine/bevy/tree/main/examples)
- [Avian 2D docs (physics library)](https://docs.rs/avian2d/latest/avian2d/)

## MVPs

Note that each of the main MVPs will require a branch to work on and then a PR to be merged. No commits to the main branch are allowed :).

### Basic little game
1. Bevy Hello World

This involves the minimum setup to create a Bevy app with a blank screen and the [`DefaultPlugins`](https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html) enabled.
It can be useful to read about how the plugins are defined and why they can be useful but it is not necessary for this step.

2. Camera setup and asset loading/spawning

This game will be a 2D rendered game and in order to render anything on the screen a camera needs to be spawned.
After having a camera, the setup for this step is to load your assets.

For this step, you need to load a minimum of two assets:
- An image that represents the entire texture
- A spritesheet that contains multiple images in a single texture that will be indexed by a number (like an array)

The next step is to spawn them in the world as `Sprite`s. The usage of each is up to you.

3. Movement

Design one of your sprites to be a player and make it moveable with the input system you want (wasd, arrows, or click to move).

***Be careful about delta time, if you don't take it into account, the movement will depend on the framerate which is not what you want.***

If it is anything else than click to move make the player wrap around the screen when it reaches the edge (it doesn't have to be perfect).

