use crate::{
    GameAssets, GameState,
    player::{AnimationConfig, Player},
};
use bevy::prelude::*;

pub struct SetupPlugIn;

impl Plugin for SetupPlugIn {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, load_assets).chain());
        app.add_systems(Update, update_camera.run_if(in_state(GameState::Playing)));
        app.add_systems(OnEnter(GameState::Playing), spawn_entities);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn update_camera(
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let Ok(player_transform) = player_query.single() else {
        warn!("Multiple or no player found");
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        warn!("Multiple or no camera found");
        return;
    };
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let player = asset_server.load("vampires/PNG/Vampires1/Idle/Vampires1_Idle_full.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let tree = asset_server.load("trees/PNG/Assets_separately/Trees/Autumn_tree1.png");
    commands.insert_resource(GameAssets {
        tree,
        vampire: player,
        vampire_layout: texture_atlas_layout,
    });
    next_game_state.set(GameState::Playing);
}

fn spawn_entities(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Player,
        Sprite {
            image: game_assets.vampire.clone(),
            texture_atlas: Some(TextureAtlas {
                index: 0,
                layout: game_assets.vampire_layout.clone(),
            }),
            ..default()
        },
        Transform::default().with_scale(Vec3::splat(5.0)),
        Name::new("Player"),
        AnimationConfig::new(0, 3, 10),
    ));

    commands.spawn((
        Sprite {
            image: game_assets.tree.clone(),
            ..default()
        },
        Transform::from_xyz(-500.0, 250.0, 0.0).with_scale(Vec3::splat(5.0)),
        Name::new("Tree"),
        StateScoped(GameState::Playing),
    ));
}
