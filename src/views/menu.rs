use bevy::{
    app::AppExit, color::palettes::css::{BLACK, CRIMSON, FIRE_BRICK, GREEN, WHITE}, prelude::*, ui::{widget::NodeImageMode, FocusPolicy}
};
use bevy_simple_text_input::{
    TextInput, 
    TextInputInactive, 
    TextInputPlaceholder, 
    TextInputPlugin, 
    TextInputTextColor, 
    TextInputTextFont, 
    TextInputValue
};


use crate::queries;

use super::{despawn_view, ViewState};

#[derive(Component)]
struct TabLoginButton;

#[derive(Component)]
struct TabRegisterButton;

#[derive(Component)]
struct TabContainer;

#[derive(Component)]
struct OnMenu;

#[derive(Component)]
struct OnLogin;

#[derive(Component)]
struct OnRegister;

#[derive(Component)]
struct LoginUsername(String);

#[derive(Component)]
struct LoginPassword(String);

#[derive(Component)]
struct RegisterUsername(String);

#[derive(Component)]
struct RegisterPassword1(String);

#[derive(Component)]
struct RegisterPassword2(String);

#[derive(Component)]
enum MenuButtonAction {
    Login,
    Register,
    LoginTab,
    RegisterTab,
    Quit,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    #[default]
    None,
    Login,
    Register,
}

const TAB_BUTTON: Color = Color::srgba(0., 0., 0., 0.);
const TAB_BUTTON_HOVER: Color = Color::srgba(1.0, 1.0, 1.0, 0.05);

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
        .init_state::<MenuState>()
        
        // basic menu layout view
        .add_systems(OnEnter(ViewState::Menu), menu_setup)
        .add_systems(OnExit(ViewState::Menu), despawn_view::<OnMenu>)

        // These run in the GameSet::Second, which comes after GameSet::First
        .add_systems(Update, tab_register_system
            .run_if(in_state(ViewState::Menu)))
        .add_systems(Update, tab_login_system
            .run_if(in_state(ViewState::Menu)))

        // Login setup system runs in GameSet::Second, after menu_setup
        .add_systems(OnEnter(MenuState::Login), login_setup
            .run_if(in_state(ViewState::Menu)))
        .add_systems(OnExit(MenuState::Login), despawn_view::<OnLogin>)

        // Register setup system runs in GameSet::Second, after menu_setup
        .add_systems(OnEnter(MenuState::Register), register_setup
            .run_if(in_state(ViewState::Menu)))
        .add_systems(OnExit(MenuState::Register), despawn_view::<OnRegister>)

        // Update systems run in GameSet::Second
        .add_systems(Update, (
            button_system,
            menu_action,
            focus_system,
        )
            .run_if(in_state(ViewState::Menu)))

        .add_systems(Update, listener_register_username.run_if(in_state(ViewState::Menu)))
        .add_systems(Update, listener_register_password1.run_if(in_state(ViewState::Menu)))
        .add_systems(Update, listener_register_password2.run_if(in_state(ViewState::Menu)))

        .add_systems(Update, listener_login_username.run_if(in_state(ViewState::Menu)))
        .add_systems(Update, listener_login_password.run_if(in_state(ViewState::Menu)));
}

fn tab_register_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<TabRegisterButton>),
    >,
    mut game_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        *background_color = match *interaction {
            Interaction::Pressed => {
                game_state.set(MenuState::Register);
                TAB_BUTTON_HOVER.into()
            },
            Interaction::Hovered => TAB_BUTTON_HOVER.into(),
            Interaction::None => TAB_BUTTON.into()
        }
    }
}

fn tab_login_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<TabLoginButton>),
    >,
    mut game_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        *background_color = match *interaction {
            Interaction::Pressed => {
                game_state.set(MenuState::Login);
                TAB_BUTTON_HOVER.into()
            },
            Interaction::Hovered => TAB_BUTTON_HOVER.into(),
            Interaction::None => TAB_BUTTON.into()
        }
    }
}

fn login_setup(
    mut commands: Commands, 
    query: Query<Entity, With<TabContainer>>,
) {
    if let Some(container) = query.iter().next() {

        let button_node = Node {
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let tab_wrapper = (
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            OnLogin,
        );

        commands
            .entity(container)
            .with_children(|parent| {

                parent 
                    .spawn(tab_wrapper)
                    .with_children(|parent| {
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
                                LoginUsername("".into())
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
                                    value: "Password".to_string(),
                                    ..default()
                                },
                                TextInputInactive(true),
                                LoginPassword("".into())
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
                                        MenuButtonAction::Login,
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Text::new("Login"),
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
    }
}

fn register_setup(
    mut commands: Commands, 
    query: Query<Entity, With<TabContainer>>,
) {
    if let Some(container) = query.iter().next() {

        let button_node = Node {
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let tab_wrapper = (
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            OnRegister,
        );

        commands
            .entity(container)
            .with_children(|parent| {

                parent 
                    .spawn(tab_wrapper)
                    .with_children(|parent| {
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
                                RegisterUsername("".into())
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
                                    value: "Password".to_string(),
                                    ..default()
                                },
                                TextInputInactive(true),
                                RegisterPassword1("".into())
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
                                    value: "Password (Again)".to_string(),
                                    ..default()
                                },
                                TextInputInactive(true),
                                RegisterPassword2("".into())
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
                                        MenuButtonAction::Register,
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Text::new("Register"),
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
    }
}

fn menu_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<NextState<MenuState>>,
) {

    let button_node = Node {
        width: Val::Percent(100.0),
        padding: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let background = (
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
    );

    let banner = (
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(CRIMSON.into()),
    );

    let title = (
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
    );

    let dialog_wrapper = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let dialog_background = (
        Node {
            width: Val::Px(400.0),
            height: Val::Px(400.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(FIRE_BRICK.into()),
    );

    let tab_button_wrapper = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        },
        // BackgroundColor(BLACK.into()),
    );

    let login_button = (
        Button,
        TabLoginButton,
        button_node.clone(),
        BackgroundColor(TAB_BUTTON),
        MenuButtonAction::LoginTab,
    );

    let login_text = (
        Text::new("Login"),
        TextColor(WHITE.into()),
        TextFont {
            font_size: 20.0,
            ..default()
        }
    );

    let register_button = (
        Button,
        TabRegisterButton,
        button_node.clone(),
        BackgroundColor(TAB_BUTTON),
        MenuButtonAction::RegisterTab,
    );

    let register_text = (
        Text::new("Register"),
        TextColor(WHITE.into()),
        TextFont {
            font_size: 20.0,
            ..default()
        }
    );

    let tab_container = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        TabContainer,
        BackgroundColor(Color::srgba(1.,1.,1.,0.02).into()),
    );


    commands
        .spawn(background)
        .with_children(|parent| {
            parent
                .spawn(banner)
                .with_child(title);

            parent
                .spawn(dialog_wrapper)
                .with_children(|parent| {
                    parent
                        .spawn(dialog_background)
                        .with_children(|parent| {
                            parent.spawn(tab_button_wrapper)
                            .with_children(|parent| {
                                parent
                                    .spawn(login_button)
                                    .with_child(login_text);
                                parent
                                    .spawn(register_button)
                                    .with_child(register_text);
                            });
                            parent.spawn(tab_container);
                        });
                });
        });

    game_state.set(MenuState::Login);
}


fn listener_login_username(
    mut input_query: Query<(&mut LoginUsername,&TextInputValue),Changed<TextInputValue>>,
) {
    for (mut value, text_input) in &mut input_query {
        value.0 = text_input.0.clone();
    } 
}

fn listener_login_password(
    mut input_query: Query<(&mut LoginPassword,&TextInputValue),Changed<TextInputValue>>, 
) {
    for (mut value, text_input) in &mut input_query {
        value.0 = text_input.0.clone();
    }
}

fn listener_register_username(
    mut input_query: Query<(&mut RegisterUsername,&TextInputValue),Changed<TextInputValue>>,
) {
    for (mut value, text_input) in &mut input_query {
        value.0 = text_input.0.clone();
    } 
}

fn listener_register_password1(
    mut input_query: Query<(&mut RegisterPassword1,&TextInputValue),Changed<TextInputValue>>, 
) {
    for (mut value, text_input) in &mut input_query {
        value.0 = text_input.0.clone();
    }
}

fn listener_register_password2(
    mut input_query: Query<(&mut RegisterPassword2,&TextInputValue),Changed<TextInputValue>>, 
) {
    for (mut value, text_input) in &mut input_query {
        value.0 = text_input.0.clone();
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, Without<TabLoginButton>, Without<TabRegisterButton>),
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
    login_username: Query<&LoginUsername>,
    login_password: Query<&LoginPassword>,
    register_username: Query<&RegisterUsername>,
    register_password1: Query<&RegisterPassword1>,
    register_password2: Query<&RegisterPassword2>,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<ViewState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                },
                MenuButtonAction::Register => {
                    let username = register_username.single().0.clone();
                    let password1 = register_password1.single().0.clone();
                    let password2 = register_password2.single().0.clone();

                    println!("username:  {}",&username);
                    println!("password1: {}",&password1);
                    println!("password2: {}",&password2);

                    let info = queries::register(
                        username, 
                        password1, 
                        password2
                    );

                    dbg!(info);

                },
                MenuButtonAction::Login => {
                    let username = login_username.single().0.clone();
                    let password = login_password.single().0.clone();

                    println!("username: {}",username);
                    println!("password: {}",password);

                    let info = queries::login(
                        username, 
                        password, 
                    );

                    dbg!(info);

                    // 1. try to login to server using username and password
                    // 2. on failure, set error message to display in UI
                    // 3. on success, fetch initial game state from server
                    // 4. create connection to server websocket

                    // game_state.set(ViewState::Game);
                },
                _ => ()
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