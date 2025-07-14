use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_systems(Startup, (spawn_camera, load_image, setup_sprite_sheet));
    app.add_systems(Update, animate_sprite_sheet);
    app.run();
}

#[derive(Component)]
struct Player;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn load_image(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("trees/PNG/Assets_separately/Trees/Autumn_tree1.png"),
            ..default()
        },
        Transform::from_xyz(-500.0, 250.0, 0.0).with_scale(Vec3::splat(5.0)),
    ));
}

fn load_sprite_sheet(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Sprite {
        image: asset_server.load("vampires/PNG/Vampires1/Idle/Vampires1_Idle_full.png"),
        ..default()
    });
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite_sheet(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if !timer.just_finished() {
            continue;
        }
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn setup_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("vampires/PNG/Vampires1/Idle/Vampires1_Idle_full.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 15 };

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        Transform::from_scale(Vec3::splat(5.0)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));
}
