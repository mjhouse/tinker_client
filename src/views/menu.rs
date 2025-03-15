use bevy::{
    a11y::Focus, app::AppExit, color::palettes::css::{BLACK, CRIMSON, FIRE_BRICK, WHITE}, prelude::*, ui::{widget::NodeImageMode, FocusPolicy}
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

use crate::{plugins::button::{MyButton, MyButtonLabel}, queries, state::ConnectionState};

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
struct OnError;

#[derive(Clone, Default, Eq, PartialEq, Debug, Hash, Resource)]
struct RegisterInfo {
    username: String,
    password1: String,
    password2: String
}

#[derive(Clone, Default, Eq, PartialEq, Debug, Hash, Resource)]
struct LoginInfo {
    username: String,
    password: String,
}

#[derive(Copy, Clone, PartialEq, Debug, Component)]
enum FormField {
    LoginUsername,
    LoginPassword,
    RegisterUsername,
    RegisterPassword1,
    RegisterPassword2,
}

#[derive(Copy, Clone, PartialEq, Debug, Component)]
enum NextField {
    LoginUsername,
    LoginPassword,
    RegisterUsername,
    RegisterPassword1,
    RegisterPassword2,
}

#[derive(Component, Default)]
enum MenuButtonAction {
    #[default]
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
        // .add_plugins(ButtonPlugin)


        .init_state::<MenuState>()
        .init_resource::<RegisterInfo>()
        .init_resource::<LoginInfo>()
        
        // basic menu layout view
        .add_systems(OnEnter(ViewState::Menu), menu_setup)
        .add_systems(OnExit(ViewState::Menu), despawn_view::<OnMenu>)

        .add_systems(Update, tab_register_system
            .run_if(in_state(ViewState::Menu)))
        .add_systems(Update, tab_login_system
            .run_if(in_state(ViewState::Menu)))

        .add_systems(OnEnter(MenuState::Login), login_setup
            .run_if(in_state(ViewState::Menu)))
        .add_systems(OnExit(MenuState::Login), despawn_view::<OnLogin>)

        .add_systems(OnEnter(MenuState::Register), register_setup
            .run_if(in_state(ViewState::Menu)))
        .add_systems(OnExit(MenuState::Register), despawn_view::<OnRegister>)

        .add_systems(Update, (
            button_system,
            menu_action,
            focus_system,
            handle_tab_key
        )
            .run_if(in_state(ViewState::Menu)))

        .add_systems(Update, form_listener.run_if(in_state(ViewState::Menu)));
}

fn handle_tab_key(
    keys: Res<ButtonInput<KeyCode>>, 
    mut focusable: Query<(&mut TextInputInactive, &mut BorderColor, &NextField, &FormField), With<TextInput>>,
) {
    if keys.just_pressed(KeyCode::Tab) {

        let mut next_field: Option<NextField> = None;

        for (mut item, mut border, next, _) in focusable.iter_mut() {
            if !item.0 {
                border.0 = BORDER_COLOR_INACTIVE;
                item.0 = true;
                next_field = Some(*next);
            }
        }

        if let Some(field) = next_field {
            for (mut item, mut border, _, current) in focusable.iter_mut() {
                let a = field as usize;
                let b = *current as usize;

                if a == b {
                    border.0 = BORDER_COLOR_ACTIVE;
                    item.0 = false;
                }
            }
        }
    }
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

fn form_input(parent: &mut ChildBuilder<'_>, placeholder: &str, field: FormField, next: NextField) {
    parent.spawn((
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
            value: placeholder.to_string(),
            ..default()
        },
        TextInputInactive(true),
        field,
        next
    ));
}

fn form_button(parent: &mut ChildBuilder<'_>, label: &str, action: MenuButtonAction) {
    parent
        .spawn(MyButton::new(action))
        .with_child(MyButtonLabel::new(label));
}

fn login_setup(
    mut commands: Commands, 
    query: Query<Entity, With<TabContainer>>,
) {
    if let Some(container) = query.iter().next() {

        let tab_wrapper = (
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            OnLogin,
        );

        let button_wrapper = Node {
            column_gap: Val::Px(10.0),
            width: Val::Percent(100.0),
            ..default()
        };

        commands
            .entity(container)
            .with_children(|parent| {

                parent 
                    .spawn(tab_wrapper)
                    .with_children(|parent| {

                        form_input(parent, "Username", FormField::LoginUsername, NextField::LoginPassword);
                        form_input(parent, "Password", FormField::LoginPassword, NextField::LoginUsername);

                        parent
                            .spawn(button_wrapper)
                            .with_children(|parent| {

                                parent
                                    .spawn(MyButton::new(MenuButtonAction::Login))
                                    .with_child(MyButtonLabel::new("Login"));

                                parent
                                    .spawn(MyButton::new(MenuButtonAction::Quit))
                                    .with_child(MyButtonLabel::new("Quit"));

                                // form_button(parent, "Login", MenuButtonAction::Login);
                                // form_button(parent, "Quit", MenuButtonAction::Quit);

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

        let tab_wrapper = (
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            OnRegister,
        );

        let button_wrapper = Node {
            column_gap: Val::Px(10.0),
            width: Val::Percent(100.0),
            ..default()
        };

        commands
            .entity(container)
            .with_children(|parent| {

                parent 
                    .spawn(tab_wrapper)
                    .with_children(|parent| {

                        form_input(parent, "Username", FormField::RegisterUsername, NextField::RegisterPassword1);
                        form_input(parent, "Password", FormField::RegisterPassword1, NextField::RegisterPassword2);
                        form_input(parent, "Password (Again)", FormField::RegisterPassword2, NextField::RegisterUsername);

                        parent
                            .spawn(button_wrapper)
                            .with_children(|parent| {

                                form_button(parent, "Register", MenuButtonAction::Register);
                                form_button(parent, "Quit", MenuButtonAction::Quit);

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

    let error_wrapper = (
        Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(WHITE.into())
    );

    let error_text = (
        Text::new(""),
        TextColor(CRIMSON.into()),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        OnError
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
                .spawn(error_wrapper)
                .with_child(error_text);

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

fn form_listener(
    mut query: Query<(&FormField,&TextInputValue),Changed<TextInputValue>>,
    mut register_info: ResMut<RegisterInfo>,
    mut login_info: ResMut<LoginInfo>,
) {
    for (field, input) in &mut query {
        match field {
            FormField::LoginUsername => login_info.username = input.0.clone(),
            FormField::LoginPassword => login_info.password = input.0.clone(),
            FormField::RegisterUsername => register_info.username = input.0.clone(),
            FormField::RegisterPassword1 => register_info.password1 = input.0.clone(),
            FormField::RegisterPassword2 => register_info.password2 = input.0.clone(),
        }
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

fn menu_action(
    mut view_state: ResMut<NextState<ViewState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut error_query: Query<&mut Text, With<OnError>>,
    mut connection_state: ResMut<ConnectionState>,
    mut app_exit_events: EventWriter<AppExit>,
    register_info: Res<RegisterInfo>,
    login_info: Res<LoginInfo>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        let mut error_message = error_query.single_mut();
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                },
                MenuButtonAction::Register => {

                    let info = queries::register(
                        register_info.username.clone(), 
                        register_info.password1.clone(), 
                        register_info.password2.clone()
                    );

                    if info.is_ok() {
                        menu_state.set(MenuState::Login);
                    } else {
                        // display failure in UI
                        dbg!(info);
                        error_message.0 = "Registration failed".into();
                    }

                },
                MenuButtonAction::Login => {

                    let info = queries::login(
                        login_info.username.clone(), 
                        login_info.password.clone(), 
                    );

                    if let Ok(data) = info {
                        connection_state.id = data.id;
                        connection_state.username = data.name;
                        connection_state.token = Some(data.token.clone());
                        view_state.set(ViewState::Game)
                    } else {
                        // display failure in UI
                        error_message.0 = "Login failed".into();
                    }
                    
                },
                _ => ()
            }
        }
    }
}