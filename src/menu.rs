use bevy::prelude::*;

use crate::GameState;

pub struct MenuPlugIn;

impl Plugin for MenuPlugIn {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, trigger_menu.run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, menu.run_if(in_state(GameState::Menu)));
    }
}

#[derive(Component)]
enum MenuInteraction {
    StartGame,
    ExitGame,
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
