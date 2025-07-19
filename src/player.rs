use crate::FacingDirectionState;
use crate::GameState;
use crate::MovementSpeed;
use crate::MovementState;
use crate::setup::AnimationConfig;
use crate::setup::GameAssets;
use bevy::prelude::*;

pub struct PlayerPlugIn;

impl Plugin for PlayerPlugIn {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_systems(Update, animate_player.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component, Reflect)]
#[require(MovementSpeed(500.))]
#[require(StateScoped::<GameState>(GameState::Playing))]
pub struct Player;

fn animate_player(
    mut query: Query<
        (
            &mut Transform,
            &MovementSpeed,
            &mut Sprite,
            &mut AnimationConfig,
        ),
        With<Player>,
    >,
    mut last_movement_state: Local<MovementState>,
    facing_state: Res<State<FacingDirectionState>>,
    game_assets: Res<GameAssets>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut next_facing_state: ResMut<NextState<FacingDirectionState>>,
    mut next_movement_state: ResMut<NextState<MovementState>>,
) {
    for (mut position, movement_speed, mut image, mut animation) in &mut query {
        if key.just_pressed(KeyCode::KeyW) {
            next_facing_state.set(FacingDirectionState::Up);
            next_movement_state.set(MovementState::Moving);
            *last_movement_state = MovementState::Moving;

            *image = Sprite {
                image: game_assets.player_walk.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: 6,
                    layout: game_assets.player_walk_layout.clone(),
                }),
                ..default()
            };

            *animation = AnimationConfig::new(6, 11, 10);
        }
        if key.pressed(KeyCode::KeyW) {
            position.translation.y += movement_speed.0 * time.delta_secs();
        }

        if key.just_pressed(KeyCode::KeyA) {
            next_facing_state.set(FacingDirectionState::Left);
            next_movement_state.set(MovementState::Moving);
            *last_movement_state = MovementState::Moving;

            *image = Sprite {
                image: game_assets.player_walk.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: 12,
                    layout: game_assets.player_walk_layout.clone(),
                }),
                ..default()
            };

            *animation = AnimationConfig::new(12, 17, 10);
        }
        if key.pressed(KeyCode::KeyA) {
            position.translation.x -= movement_speed.0 * time.delta_secs();
        }

        if key.just_pressed(KeyCode::KeyS) {
            next_facing_state.set(FacingDirectionState::Down);
            next_movement_state.set(MovementState::Moving);
            *last_movement_state = MovementState::Moving;

            *image = Sprite {
                image: game_assets.player_walk.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: 0,
                    layout: game_assets.player_walk_layout.clone(),
                }),
                ..default()
            };

            *animation = AnimationConfig::new(0, 5, 10);
        }
        if key.pressed(KeyCode::KeyS) {
            position.translation.y -= movement_speed.0 * time.delta_secs();
        }

        if key.just_pressed(KeyCode::KeyD) {
            next_facing_state.set(FacingDirectionState::Right);
            next_movement_state.set(MovementState::Moving);
            *last_movement_state = MovementState::Moving;

            *image = Sprite {
                image: game_assets.player_walk.clone(),
                texture_atlas: Some(TextureAtlas {
                    index: 18,
                    layout: game_assets.player_walk_layout.clone(),
                }),
                ..default()
            };

            *animation = AnimationConfig::new(18, 23, 10);
        }
        if key.pressed(KeyCode::KeyD) {
            position.translation.x += movement_speed.0 * time.delta_secs();
        }

        if !key.any_pressed([KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD])
            && *last_movement_state == MovementState::Moving
        {
            next_movement_state.set(MovementState::Idle);
            *last_movement_state = MovementState::Idle;

            if *facing_state == FacingDirectionState::Up {
                *image = Sprite {
                    image: game_assets.player_idle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        index: 4,
                        layout: game_assets.player_idle_layout.clone(),
                    }),
                    ..default()
                };

                *animation = AnimationConfig::new(4, 7, 10);
            }

            if *facing_state == FacingDirectionState::Up {
                *image = Sprite {
                    image: game_assets.player_idle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        index: 4,
                        layout: game_assets.player_idle_layout.clone(),
                    }),
                    ..default()
                };

                *animation = AnimationConfig::new(4, 7, 10);
            }

            if *facing_state == FacingDirectionState::Left {
                *image = Sprite {
                    image: game_assets.player_idle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        index: 8,
                        layout: game_assets.player_idle_layout.clone(),
                    }),
                    ..default()
                };

                *animation = AnimationConfig::new(8, 11, 10);
            }

            if *facing_state == FacingDirectionState::Down {
                *image = Sprite {
                    image: game_assets.player_idle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        index: 0,
                        layout: game_assets.player_idle_layout.clone(),
                    }),
                    ..default()
                };

                *animation = AnimationConfig::new(0, 3, 10);
            }

            if *facing_state == FacingDirectionState::Right {
                *image = Sprite {
                    image: game_assets.player_idle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        index: 12,
                        layout: game_assets.player_idle_layout.clone(),
                    }),
                    ..default()
                };

                *animation = AnimationConfig::new(12, 15, 10);
            }
        };
    }
}
