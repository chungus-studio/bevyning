#![allow(clippy::type_complexity)]
use crate::{
    menu::MenuPlugIn,
    player::{AnimationConfig, Player, PlayerPlugIn},
    setup::SetupPlugIn,
};
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
        WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        StateInspectorPlugin::<GameState>::default(),
        MenuPlugIn,
        PlayerPlugIn,
        SetupPlugIn,
    ));

    app.register_type::<AnimationConfig>();
    app.register_type::<Player>();
    app.register_type::<GameState>();

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

#[derive(Resource, Default)]
struct GameAssets {
    vampire: Handle<Image>,
    vampire_layout: Handle<TextureAtlasLayout>,
    tree: Handle<Image>,
}

#[derive(Component, Reflect)]
pub struct MovementSpeed(f32);
