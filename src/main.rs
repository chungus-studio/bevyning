#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
use crate::{menu::MenuPlugIn, player::PlayerPlugIn, setup::SetupPlugIn};
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

mod menu;
mod player;
mod setup;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        EguiPlugin::default(),
        WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::AltLeft)),
        StateInspectorPlugin::<GameState>::default(),
        StateInspectorPlugin::<FacingDirectionState>::default(),
        StateInspectorPlugin::<MovementState>::default(),
        MenuPlugIn,
        PlayerPlugIn,
        SetupPlugIn,
    ));

    app.register_type::<GameState>();

    app.init_state::<GameState>()
        .init_state::<FacingDirectionState>()
        .init_state::<MovementState>();

    app.run();
}

#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug, Reflect)]
#[states(scoped_entities)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    Menu,
}

#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug, Reflect)]
#[states(scoped_entities)]
pub enum FacingDirectionState {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug, Reflect)]
#[states(scoped_entities)]
pub enum MovementState {
    #[default]
    Idle,
    Moving,
}

#[derive(Component, Reflect)]
pub struct MovementSpeed(f32);
