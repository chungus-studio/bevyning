use crate::GameState;
use crate::MovementSpeed;
use bevy::prelude::*;
use std::time::Duration;

pub struct PlayerPlugIn;

impl Plugin for PlayerPlugIn {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_player, execute_animations).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component, Reflect)]
#[require(MovementSpeed(700.))]
#[require(StateScoped::<GameState>(GameState::Playing))]
pub struct Player;

fn move_player(
    mut query: Query<(&mut Transform, &MovementSpeed), With<Player>>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut position, movement_speed) in &mut query {
        if key.pressed(KeyCode::KeyW) || key.pressed(KeyCode::ArrowUp) {
            position.translation.y += movement_speed.0 * time.delta_secs();
        }
        if key.pressed(KeyCode::KeyA) || key.pressed(KeyCode::ArrowLeft) {
            position.translation.x -= movement_speed.0 * time.delta_secs();
        }
        if key.pressed(KeyCode::KeyS) || key.pressed(KeyCode::ArrowDown) {
            position.translation.y -= movement_speed.0 * time.delta_secs();
        }
        if key.pressed(KeyCode::KeyD) || key.pressed(KeyCode::ArrowRight) {
            position.translation.x += movement_speed.0 * time.delta_secs();
        }
    }
}

#[derive(Component, Reflect)]
pub struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(
            Duration::from_secs_f32(1.0 / (fps as f32)),
            TimerMode::Repeating,
        )
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        // We track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    // ...and it IS the last frame, then we move back to the first frame and stop.
                    atlas.index = config.first_sprite_index;
                } else {
                    // ...and it is NOT the last frame, then we move to the next frame...
                    atlas.index += 1;
                    // ...and reset the frame timer to start counting all over again
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}
