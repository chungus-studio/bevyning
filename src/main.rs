#![allow(clippy::type_complexity)]
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};
use std::time::Duration;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(ImagePlugin::default_nearest()),
        EguiPlugin::default(),
        WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        StateInspectorPlugin::<GameState>::default(),
    ));

    app.register_type::<AnimationConfig>();
    app.register_type::<Player>();
    app.register_type::<GameState>();

    app.init_state::<GameState>();

    app.add_systems(Startup, (spawn_camera, load_assets).chain());
    app.add_systems(OnEnter(GameState::Playing), spawn_entities);
    app.add_systems(
        Update,
        (move_player, execute_animations, update_camera, trigger_menu)
            .run_if(in_state(GameState::Playing)),
    );
    app.add_systems(OnEnter(GameState::Menu), setup_menu);
    app.add_systems(Update, menu.run_if(in_state(GameState::Menu)));

    app.run();
}

#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug, Reflect)]
#[states(scoped_entities)]
enum GameState {
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
#[require(MovementSpeed(700.))]
#[require(StateScoped::<GameState>(GameState::Playing))]
struct Player;

#[derive(Component, Reflect)]
struct MovementSpeed(f32);

#[derive(Component)]
enum MenuInteraction {
    StartGame,
    ExitGame,
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

fn move_player(
    mut query: Query<(&mut Transform, &MovementSpeed), With<Player>>,
    key: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut position, movement_speed) in &mut query {
        if key.pressed(KeyCode::KeyW) || key.pressed(KeyCode::ArrowUp) {
            position.translation.y += movement_speed.0 * time.delta_secs();
        }
        if key.pressed(KeyCode::KeyA) || key.pressed(KeyCode::ArrowLeft) {
            position.translation.x -= movement_speed.0 * time.delta_secs();
        }
        if key.pressed(KeyCode::KeyS) || key.pressed(KeyCode::ArrowDown) {
            position.translation.y -= movement_speed.0 * time.delta_secs();
        }
        if key.pressed(KeyCode::KeyD) || key.pressed(KeyCode::ArrowRight) {
            position.translation.x += movement_speed.0 * time.delta_secs();
        }
    }
}

fn trigger_menu(mut next_game_state: ResMut<NextState<GameState>>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_released(KeyCode::KeyM) {
        next_game_state.set(GameState::Menu);
    };
}

fn generic_button<I: Component>(text: &str, parent: Entity, menu_interaction: I) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Px(150.),
            height: Val::Px(75.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Name::new(format!("{text} Button")),
        BackgroundColor(NORMAL_BUTTON),
        menu_interaction,
        ChildOf(parent),
        children![(
            Text::new(text),
            TextFont {
                font_size: 25.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextLayout::default().with_justify(JustifyText::Center),
            Name::new(format!("{text} Text")),
        )],
    )
}

fn setup_menu(mut commands: Commands, mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Menu);
    let root = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                right: Val::Px(12.0),
                width: Val::Percent(25.),
                height: Val::Percent(20.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Name::new("MenuUi"),
            StateScoped(GameState::Menu),
        ))
        .id();
    commands.spawn(generic_button(
        "Start Game",
        root,
        MenuInteraction::StartGame,
    ));
    commands.spawn(generic_button("Exit Game", root, MenuInteraction::ExitGame));
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn menu(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuInteraction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, menu_interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match *menu_interaction {
                    MenuInteraction::StartGame => next_game_state.set(GameState::Playing),
                    MenuInteraction::ExitGame => {
                        exit.write(AppExit::Success);
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Component, Reflect)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
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
