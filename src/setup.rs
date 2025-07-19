use std::time::Duration;

use crate::{
    EntityState, GameState,
    player::{Player, PlayerAnimationConfig},
};
use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub struct SetupPlugIn;

impl Plugin for SetupPlugIn {
    fn build(&self, app: &mut App) {
        app.add_plugins((ResourceInspectorPlugin::<GameAssets>::default(),));

        app.register_type::<AnimationConfig>();

        app.add_systems(Startup, (spawn_camera, load_assets))
            .add_systems(Update, (update_camera).run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Playing), spawn_entities);
    }
}

#[derive(Resource, Default, Reflect)]
pub struct GameAssets {
    pub player_idle: Handle<Image>,
    pub player_walk: Handle<Image>,
    pub player_idle_layout: Handle<TextureAtlasLayout>,
    pub player_walk_layout: Handle<TextureAtlasLayout>,
    tree: Handle<Image>,
}

impl GameAssets {
    pub fn sheet_for_state(
        &self,
        state: EntityState,
    ) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
        match state {
            EntityState::Idle => (self.player_idle.clone(), self.player_idle_layout.clone()),
            EntityState::Moving => (self.player_walk.clone(), self.player_walk_layout.clone()),
        }
    }
}

#[derive(Component, Reflect)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
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
    let tree = asset_server.load("trees/PNG/Assets_separately/Trees/Autumn_tree1.png");

    let vampire_idle = asset_server.load("vampires/PNG/Vampires1/Idle/Vampires1_Idle_full.png");
    let layout_idle = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 4, None, None);
    let texture_atlas_idle_layout = texture_atlas_layouts.add(layout_idle);

    let vampire_walk = asset_server.load("vampires/PNG/Vampires1/Walk/Vampires1_Walk_full.png");
    let layout_walk = TextureAtlasLayout::from_grid(UVec2::splat(64), 6, 4, None, None);
    let texture_atlas_walk_layout = texture_atlas_layouts.add(layout_walk);

    commands.insert_resource(GameAssets {
        tree,
        player_idle: vampire_idle,
        player_walk: vampire_walk,
        player_idle_layout: texture_atlas_idle_layout,
        player_walk_layout: texture_atlas_walk_layout,
    });
    next_game_state.set(GameState::Playing);
}

fn spawn_entities(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        Player,
        Sprite {
            image: game_assets.player_idle.clone(),
            texture_atlas: Some(TextureAtlas {
                index: 0,
                layout: game_assets.player_idle_layout.clone(),
            }),
            ..default()
        },
        Transform::default().with_scale(Vec3::splat(5.0)),
        Name::new("Player"),
        AnimationConfig::new(0, 3, 10),
        PlayerAnimationConfig {
            idle_down: UVec2::new(0, 3),
            idle_up: UVec2::new(4, 7),
            idle_left: UVec2::new(8, 11),
            idle_right: UVec2::new(12, 15),

            walk_down: UVec2::new(0, 5),
            walk_up: UVec2::new(6, 11),
            walk_left: UVec2::new(12, 17),
            walk_right: UVec2::new(18, 23),
        },
    ));

    commands.spawn((
        Sprite {
            image: game_assets.tree.clone(),
            ..default()
        },
        Transform::from_xyz(-500.0, 250.0, 0.0).with_scale(Vec3::splat(5.0)),
        Name::new("Tree"),
        //Added because there is no struct for this tree upon which I can
        //#[require(StateScoped::<GameState>(GameState::Playing))]
        StateScoped(GameState::Playing),
    ));
}
