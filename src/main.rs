#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
use crate::{menu::MenuPlugIn, player::PlayerPlugIn, setup::SetupPlugIn};
use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

mod menu;
mod player;
mod setup;

fn main() {
    let mut app = App::new();
    app.register_type::<GameState>()
        .register_type::<FacingDirection>()
        .register_type::<EntityState>()
        .register_type::<MovementSpeed>();

    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        EguiPlugin::default(),
        WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::AltLeft)),
        StateInspectorPlugin::<GameState>::default(),
        MenuPlugIn,
        PlayerPlugIn,
        SetupPlugIn,
    ));

    app.init_state::<GameState>();

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

#[derive(Component, Reflect)]
pub struct MovementSpeed(f32);

#[derive(Component, Default, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum FacingDirection {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

#[derive(Component, Default, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[reflect(Component)]
pub enum EntityState {
    #[default]
    Idle,
    Moving,
}
