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
    movement_state: Res<State<MovementState>>,
    facing_state: Res<State<FacingDirectionState>>,
    game_assets: Res<GameAssets>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut next_facing_state: ResMut<NextState<FacingDirectionState>>,
    mut next_movement_state: ResMut<NextState<MovementState>>,
) {
    for (mut position, movement_speed, mut sprite, mut animation) in &mut query {
        if key.just_pressed(KeyCode::KeyW) {
            next_facing_state.set(FacingDirectionState::Up);
            next_movement_state.set(MovementState::Moving);

            set_sprite_and_animation(
                &game_assets,
                &movement_state,
                &mut sprite,
                &mut animation,
                6,
                11,
                10,
            );
        }
        if key.pressed(KeyCode::KeyW) {
            position.translation.y += movement_speed.0 * time.delta_secs();
        }

        if key.just_pressed(KeyCode::KeyA) {
            next_facing_state.set(FacingDirectionState::Left);
            next_movement_state.set(MovementState::Moving);

            *sprite = Sprite {
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

            *sprite = Sprite {
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

            *sprite = Sprite {
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
            && *movement_state == MovementState::Moving
        {
            next_movement_state.set(MovementState::Idle);

            if *facing_state == FacingDirectionState::Up {
                *sprite = Sprite {
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
                *sprite = Sprite {
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
                *sprite = Sprite {
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
                *sprite = Sprite {
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
                *sprite = Sprite {
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

fn set_sprite_and_animation(
    game_assets: &Res<GameAssets>,
    movement_state: &Res<State<MovementState>>,
    sprite: &mut Sprite,
    animation: &mut AnimationConfig,
    first_animation_index: usize,
    last_animation_index: usize,
    animation_fps: u8,
) {
    if **movement_state == MovementState::Idle {
        *sprite = Sprite {
            image: game_assets.player_idle.clone(),
            texture_atlas: Some(TextureAtlas {
                index: first_animation_index,
                layout: game_assets.player_idle_layout.clone(),
            }),
            ..default()
        };

        *animation =
            AnimationConfig::new(first_animation_index, last_animation_index, animation_fps);
    } else if **movement_state == MovementState::Moving {
        // It could just be "else", I put this in case MovementState includes something else in the future
        *sprite = Sprite {
            image: game_assets.player_walk.clone(),
            texture_atlas: Some(TextureAtlas {
                index: first_animation_index,
                layout: game_assets.player_walk_layout.clone(),
            }),
            ..default()
        };

        *animation =
            AnimationConfig::new(first_animation_index, last_animation_index, animation_fps);
    }
}
