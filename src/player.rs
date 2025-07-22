use crate::{
    EntityState, FacingDirection, GameState, MovementSpeed,
    setup::{AnimationConfig, GameAssets},
};
use bevy::prelude::*;

pub struct PlayerPlugIn;

impl Plugin for PlayerPlugIn {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>().add_systems(
            Update,
            (move_player, animate_player)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Component, Reflect)]
#[require(MovementSpeed(500.))]
#[require(StateScoped::<GameState>(GameState::Playing))]
#[require(FacingDirection)]
#[require(EntityState)]
pub struct Player;

#[derive(Component, Reflect)]
pub struct PlayerAnimationConfig {
    pub idle_right: UVec2,
    pub idle_up: UVec2,
    pub idle_down: UVec2,
    pub idle_left: UVec2,

    pub walk_right: UVec2,
    pub walk_up: UVec2,
    pub walk_down: UVec2,
    pub walk_left: UVec2,
}

fn move_player(
    mut player_q: Query<(
        &mut Transform,
        &MovementSpeed,
        &mut FacingDirection,
        &mut EntityState,
    )>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut tf, speed, mut facing, mut state)) = player_q.single_mut() else {
        return;
    };

    let mut dir = Vec2::ZERO;

    if key.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if key.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if key.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if key.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    if dir == Vec2::ZERO {
        *state = EntityState::Idle;
        return;
    }

    let displacement = dir.normalize() * speed.0 * time.delta_secs();
    tf.translation.x += displacement.x;
    tf.translation.y += displacement.y;

    *state = EntityState::Moving;

    *facing = if dir.x.abs() > dir.y.abs() {
        if dir.x > 0.0 {
            FacingDirection::Right
        } else {
            FacingDirection::Left
        }
    } else if dir.y > 0.0 {
        FacingDirection::Up
    } else {
        FacingDirection::Down
    };
}

fn animate_player(
    mut player_q: Query<
        (
            &PlayerAnimationConfig,
            &FacingDirection,
            &EntityState,
            &mut AnimationConfig,
            &mut Sprite,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    game_assets: Res<GameAssets>,
) {
    for (clips, facing, state, mut anim, mut sprite) in player_q.iter_mut() {
        let range = match (*state, *facing) {
            (EntityState::Idle, FacingDirection::Down) => clips.idle_down,
            (EntityState::Idle, FacingDirection::Up) => clips.idle_up,
            (EntityState::Idle, FacingDirection::Left) => clips.idle_left,
            (EntityState::Idle, FacingDirection::Right) => clips.idle_right,

            (EntityState::Moving, FacingDirection::Down) => clips.walk_down,
            (EntityState::Moving, FacingDirection::Up) => clips.walk_up,
            (EntityState::Moving, FacingDirection::Left) => clips.walk_left,
            (EntityState::Moving, FacingDirection::Right) => clips.walk_right,
        };

        let desired_first = range.x as usize;
        let desired_last = range.y as usize;

        let (desired_image, desired_layout) = game_assets.sheet_for_state(*state);

        let must_swap_sheet = sprite.image != desired_image
            || anim.first_sprite_index != desired_first
            || anim.last_sprite_index != desired_last;

        if must_swap_sheet {
            *sprite = Sprite {
                image: desired_image,
                texture_atlas: Some(TextureAtlas {
                    index: desired_first,
                    layout: desired_layout,
                }),
                ..Default::default()
            };
            *anim = AnimationConfig::new(desired_first, desired_last, 10);
        }

        if anim.frame_timer.tick(time.delta()).just_finished() {
            if let Some(atlas) = sprite.texture_atlas.as_mut() {
                atlas.index += 1;
                if atlas.index > anim.last_sprite_index {
                    atlas.index = anim.first_sprite_index;
                }
            }
        }
    }
}
