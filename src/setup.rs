use std::time::Duration;

use crate::{GameState, player::Player};
use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub struct SetupPlugIn;

impl Plugin for SetupPlugIn {
    fn build(&self, app: &mut App) {
        app.add_plugins((ResourceInspectorPlugin::<GameAssets>::default(),));

        app.register_type::<AnimationConfig>();

        app.add_systems(Startup, (spawn_camera, load_assets))
            .add_systems(
                Update,
                (update_camera, execute_animations).run_if(in_state(GameState::Playing)),
            )
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
