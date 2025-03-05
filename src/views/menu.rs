use bevy::{app::AppExit, color::palettes::css::{BLACK, CRIMSON, FIRE_BRICK}, ecs::schedule::And, prelude::*, text, ui::{widget::NodeImageMode, FocusPolicy}};
use bevy_simple_text_input::{
    TextInput, TextInputCursorPos, TextInputInactive, TextInputPlaceholder, TextInputPlugin, TextInputSettings, TextInputSubmitEvent, TextInputSystem, TextInputTextColor, TextInputTextFont, TextInputValue
};

use super::{despawn_view, ViewState};


#[derive(Component)]
struct OnMenu;

#[derive(Component)]
struct UsernameInput(String);

#[derive(Component)]
struct PasswordInput(String);

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

const NORMAL_BUTTON: Color = Color::srgb(1.0, 0.84, 0.0);
const HOVERED_BUTTON: Color = Color::srgb(1.0, 0.92, 0.5);
const PRESSED_BUTTON: Color = Color::srgb(1.0, 0.92, 0.5);

const BORDER_COLOR_ACTIVE: Color = Color::srgb(0.75, 0.75, 0.75);
const BORDER_COLOR_INACTIVE: Color = Color::srgb(0.5, 0.2, 0.2);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const INPUT_BACKGROUND_COLOR: Color = Color::srgb(0.598, 0.033, 0.033);

pub fn main_menu(app: &mut App) {
    app
        .add_plugins(TextInputPlugin)
        .add_systems(OnEnter(ViewState::Menu), menu_setup)
        .add_systems(OnExit(ViewState::Menu), despawn_view::<OnMenu>)
        .add_systems(Update, (
            button_system,
            menu_action,
            focus_system,
        ).run_if(in_state(ViewState::Menu)))
        .add_systems(Update, listener_username.run_if(in_state(ViewState::Menu)))
        .add_systems(Update, listener_password.run_if(in_state(ViewState::Menu)));
}

fn listener_username(
    mut input_query: Query<
        (
            &mut UsernameInput,
            &TextInputValue
        ),
        Changed<TextInputValue>
    >,
) {
    for (mut value, text_input) in &mut input_query {
        value.0 = text_input.0.clone();
    } 
}

fn listener_password(
    mut input_query: Query<
        (
            &mut PasswordInput,
            &TextInputValue
        ),
        Changed<TextInputValue>
    >, 
) {
    for (mut value, text_input) in &mut input_query {
        value.0 = text_input.0.clone();
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        *background_color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into()
        }
    }
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    username_input: Query<&UsernameInput>,
    password_input: Query<&PasswordInput>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<ViewState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    println!("username: {}",username_input.single().0.clone());
                    println!("password: {}",password_input.single().0.clone());

                    // 1. try to login to server using username and password
                    // 2. on failure, set error message to display in UI
                    // 3. on success, fetch initial game state from server
                    // 4. create connection to server websocket

                    game_state.set(ViewState::Game);
                }
            }
        }
    }
}

fn focus_system(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = BORDER_COLOR_ACTIVE.into();
                } else {
                    inactive.0 = true;
                    *border_color = BORDER_COLOR_INACTIVE.into();
                }
            }
        }
    }
}


fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let button_node = Node {
        width: Val::Percent(100.0),
        padding: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ImageNode {
                image: asset_server.load("splash.png"),
                image_mode: NodeImageMode::Stretch,
                ..Default::default()
            },
            Interaction::None,
            OnMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                ))
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn((
                        Text::new("Tinker"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        Node {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        },
                    ));
                });

            parent
                .spawn(
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    }
                )
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(400.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(40.0)),
                                ..default()
                            },
                            BackgroundColor(FIRE_BRICK.into()),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Text::new("Play Game"),
                                    TextFont {
                                        font_size: 30.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                    Node {
                                        margin: UiRect::all(Val::Px(20.0)),
                                        ..default()
                                    },
                                ));

                            parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        border: UiRect::all(Val::Px(2.0)),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        margin: UiRect::bottom(Val::Px(20.0)),
                                        ..default()
                                    },
                                    BorderColor(BORDER_COLOR_INACTIVE),
                                    BackgroundColor(INPUT_BACKGROUND_COLOR),
                                    TextInputValue("".to_string()),
                                    FocusPolicy::Block,
                                    TextInput,
                                    TextInputTextFont(TextFont {
                                        font_size: 20.,
                                        ..default()
                                    }),
                                    TextInputTextColor(TextColor(TEXT_COLOR)),
                                    TextInputPlaceholder {
                                        value: "Username".to_string(),
                                        ..default()
                                    },
                                    TextInputInactive(true),
                                    UsernameInput("".into())
                                ));

                            parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        border: UiRect::all(Val::Px(2.0)),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    BorderColor(BORDER_COLOR_INACTIVE),
                                    BackgroundColor(INPUT_BACKGROUND_COLOR),
                                    TextInputValue("".to_string()),
                                    FocusPolicy::Block,
                                    TextInput,
                                    TextInputTextFont(TextFont {
                                        font_size: 20.,
                                        ..default()
                                    }),
                                    TextInputTextColor(TextColor(TEXT_COLOR)),
                                    TextInputPlaceholder {
                                        value: "Password".to_string(),
                                        ..default()
                                    },
                                    TextInputInactive(true),
                                    PasswordInput("".into())
                                ));

                            parent
                                .spawn(
                                    Node {
                                        margin: UiRect::top(Val::Px(20.0)),
                                        width: Val::Percent(100.0),
                                        ..default()
                                    }
                                )
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            Button,
                                            Node {
                                                margin: UiRect::right(Val::Px(10.0)),
                                                ..button_node.clone()
                                            },
                                            BackgroundColor(NORMAL_BUTTON),
                                            MenuButtonAction::Play,
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                Text::new("Play"),
                                                TextColor(BLACK.into()),
                                            ));
                                        });
            
                                    parent
                                        .spawn((
                                            Button,
                                            button_node.clone(),
                                            BackgroundColor(NORMAL_BUTTON),
                                            MenuButtonAction::Quit,
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                Text::new("Quit"),
                                                TextColor(BLACK.into()),
                                            ));
                                        });
                                });


                        });
                });
        });
}

