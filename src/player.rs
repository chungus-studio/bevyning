use crate::FacingDirectionState;
use crate::GameState;
use crate::MovementSpeed;
use crate::MovementState;
use bevy::prelude::*;

pub struct PlayerPlugIn;

impl Plugin for PlayerPlugIn {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)));
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
    mut next_player_facing_state: ResMut<NextState<FacingDirectionState>>,
    mut next_movement_state: ResMut<NextState<MovementState>>,
) {
    for (mut position, movement_speed) in &mut query {
        if key.pressed(KeyCode::KeyW) || key.pressed(KeyCode::ArrowUp) {
            next_player_facing_state.set(FacingDirectionState::Up);
            position.translation.y += movement_speed.0 * time.delta_secs();
        }
        if key.pressed(KeyCode::KeyA) || key.pressed(KeyCode::ArrowLeft) {
            next_player_facing_state.set(FacingDirectionState::Left);
            position.translation.x -= movement_speed.0 * time.delta_secs();
        }
        if key.pressed(KeyCode::KeyS) || key.pressed(KeyCode::ArrowDown) {
            next_player_facing_state.set(FacingDirectionState::Down);
            position.translation.y -= movement_speed.0 * time.delta_secs();
        }
        if key.pressed(KeyCode::KeyD) || key.pressed(KeyCode::ArrowRight) {
            next_player_facing_state.set(FacingDirectionState::Right);
            position.translation.x += movement_speed.0 * time.delta_secs();
        }
    }
}
