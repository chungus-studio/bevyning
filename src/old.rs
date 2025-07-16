/// ------------------------------------ ANIMATION FOR IDLE ------------------------------------
// #[derive(Component, Reflect)]
// struct AnimationIndices {
//     first: usize,
//     last: usize,
// }

// #[derive(Component, Deref, DerefMut, Reflect)]
// struct AnimationTimer(Timer);

// fn animate_sprite_sheet(
//     time: Res<Time>,
//     mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
// ) {
//     for (indices, mut timer, mut sprite) in &mut query {
//         timer.tick(time.delta());

//         if !timer.just_finished() {
//             continue;
//         }
//         if let Some(atlas) = &mut sprite.texture_atlas {
//             atlas.index = if atlas.index == indices.last {
//                 indices.first
//             } else {
//                 atlas.index + 1
//             };
//         }
//     }
// }

// fn setup_sprite_sheet(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     let texture = asset_server.load("vampires/PNG/Vampires1/Idle/Vampires1_Idle_full.png");
//     let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 4, 4, None, None);
//     let texture_atlas_layout = texture_atlas_layouts.add(layout);
//     // Use only the subset of sprites in the sheet that make up the run animation
//     let animation_indices = AnimationIndices { first: 0, last: 15 };

//     commands.spawn((
//         Sprite::from_atlas_image(
//             texture,
//             TextureAtlas {
//                 layout: texture_atlas_layout,
//                 index: animation_indices.first,
//             },
//         ),
//         Transform::from_scale(Vec3::splat(5.0)),
//         animation_indices,
//         AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
//         Name::new("Vampire"),
//     ));
// }


/// ------------------------------------ ANIMATION FOR ALL SIDES ------------------------------------
    // app.register_type::<LeftSprite>();
    // app.register_type::<RightSprite>();
    // app.register_type::<UpSprite>();
    // app.register_type::<DownSprite>();

// app.add_systems(
//         Update,
//         (
//             // Press the right arrow key to animate the right sprite
//             trigger_animation::<RightSprite>.run_if(input_just_pressed(KeyCode::KeyD)),
//             // Press the left arrow key to animate the left sprite
//             trigger_animation::<LeftSprite>.run_if(input_just_pressed(KeyCode::KeyA)),
//             // Press the up arrow key to animate the up sprite
//             trigger_animation::<UpSprite>.run_if(input_just_pressed(KeyCode::KeyW)),
//             // Press the down arrow key to animate the down sprite
//             trigger_animation::<DownSprite>.run_if(input_just_pressed(KeyCode::KeyS)),
//         ),
//     );

// #[derive(Component, Reflect)]
// struct LeftSprite;

// #[derive(Component, Reflect)]
// struct RightSprite;

// #[derive(Component, Reflect)]
// struct UpSprite;

// #[derive(Component, Reflect)]
// struct DownSprite;

// fn walk_animations(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     // Create a minimal UI explaining how to interact with the example
//     commands.spawn((
//         Text::new("W: Animate Up Sprite\nA: Animate Left Sprite\nS: Animate Down Sprite\nD: Animate Right Sprite"),
//         Node {
//             position_type: PositionType::Absolute,
//             top: Val::Px(12.0),
//             left: Val::Px(12.0),
//             ..default()
//         },
//     ));

//     // Load the sprite sheet using the `AssetServer`
//     let texture = asset_server.load("vampires/PNG/Vampires1/Walk/Vampires1_Walk_full.png");

//     // The sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
//     let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 6, 4, None, None);
//     let texture_atlas_layout = texture_atlas_layouts.add(layout);

//     // The first (left-hand) sprite runs at 10 FPS
//     let animation_config_left = AnimationConfig::new(12, 17, 10);

//     // Create the first (left-hand) sprite
//     commands.spawn((
//         Sprite {
//             image: texture.clone(),
//             texture_atlas: Some(TextureAtlas {
//                 layout: texture_atlas_layout.clone(),
//                 index: animation_config_left.first_sprite_index,
//             }),
//             ..default()
//         },
//         Transform::from_scale(Vec3::splat(5.0)).with_translation(Vec3::new(-70.0, 0.0, 0.0)),
//         LeftSprite,
//         animation_config_left,
//     ));