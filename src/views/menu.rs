use bevy::{app::AppExit, color::palettes::css::{CRIMSON,FIRE_BRICK,BLACK}, prelude::*};
use super::{despawn_view, ViewState};

#[derive(Component)]
struct OnMenu;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

const NORMAL_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);
const HOVERED_BUTTON: Color = Color::srgb(0.45, 0.45, 0.45);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn main_menu(app: &mut App) {
    app
        .add_systems(OnEnter(ViewState::Menu), menu_setup)
        .add_systems(OnExit(ViewState::Menu), despawn_view::<OnMenu>)
        .add_systems(Update, (button_system,menu_action).run_if(in_state(ViewState::Menu)));
        // .add_systems(OnExit(MenuButtonAction::Quit),despawn_screen::<OnSettingsMenuScreen>);
}

// This system handles changing all buttons color based on mouse interaction
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
                    game_state.set(ViewState::Game);
                }
            }
        }
    }
}


fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
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
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
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
                                    Button,
                                    button_node.clone(),
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
}

