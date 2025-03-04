use bevy::input::mouse::{AccumulatedMouseScroll, MouseMotion};
use bevy::window::PrimaryWindow;
use bevy_ecs_tiled::prelude::*;
use bevy::prelude::*;

use crate::cursor::{Cursor, CursorData, CursorType};
use crate::player::{Direction, Graphic, Player, PlayerType, Speed, Target};

use super::{despawn_view, ViewState};

#[derive(Component)]
struct OnGame;

pub fn main_game(app: &mut App) {
    app
        .add_systems(OnEnter(ViewState::Game), game_setup)
        .add_systems(OnExit(ViewState::Game), despawn_view::<OnGame>)
        .add_systems(Update, player_movement.run_if(in_state(ViewState::Game)))
        .add_systems(Update, camera_movement.run_if(in_state(ViewState::Game)))
        .add_systems(Update, player_animation.run_if(in_state(ViewState::Game)))
        .add_systems(Update, cursor_movement.run_if(in_state(ViewState::Game)))
        .add_systems(Update, cursor_animation.run_if(in_state(ViewState::Game)))
        .add_systems(Update, camera_zoom.run_if(in_state(ViewState::Game)));
}

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let text_font = TextFont {
        font_size: 50.0,
        ..default()
    };
    let text_justification = JustifyText::Center;

    // Load the map: ensure any tile / tileset paths are relative to assets/ folder
    let map_handle: Handle<TiledMap> = asset_server.load("maps/tinker.tmx");

    // Spawn the map with default options
    commands.spawn((
        TiledMapHandle(map_handle),
        TiledMapSettings {
            layer_positioning: LayerPositioning::Centered,
            ..default()
        },
        OnGame
    ));

    commands.spawn((Player::new(
        "Mike".into(),
        &asset_server,
        &mut texture_atlas_layouts
    ), OnGame)
    ).with_child((
        Text2d::new("Mike"),
        text_font.clone(),
        Transform::from_translation(Vec3::new(0.0, 260.0, 1.0)),
        TextLayout::new_with_justify(text_justification),
    ));
    commands.spawn((Cursor::new(
        &asset_server,
        &mut texture_atlas_layouts
    ), OnGame));

}

fn camera_zoom(
    mut camera: Query<(&mut OrthographicProjection, &Camera2d)>,
    scroll: Res<AccumulatedMouseScroll>,
) {
    let (mut projection, _) = camera.single_mut();
    let zoom = -scroll.delta.y * 0.2 + 1.;
    let scale = projection.scale;
    projection.scale = (scale * zoom).clamp(0.1,10.0);
}

fn cursor_animation(
    time: Res<Time>, 
    mut query: Query<(
        &mut Sprite,
        &mut CursorData,
    ),With<CursorType>>,
) {
    for (mut sprite, mut data) in &mut query {
        data.timer.tick(time.delta());
        
        if data.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = data.next(atlas.index);
                if atlas.index != 4 {
                    data.reset();
                }
            }
        }
    }
}

fn player_animation(
    time: Res<Time>, 
    keys: Res<ButtonInput<KeyCode>>, 
    mut query: Query<(
        &mut Graphic,
        &mut Sprite,
        &mut Target,
        &mut Direction
    ),With<PlayerType>>,
) {
    for (mut graphic, mut sprite, target, direction) in &mut query {
        graphic.timer.tick(time.delta());
        
        let animation = if target.0.is_none() {
            &graphic.idle
        } else {
            if keys.pressed(KeyCode::ShiftLeft) {
                &graphic.running
            } else {
                &graphic.walking
            }
        };

        if graphic.timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = animation.next(*direction, atlas.index);
            }
            graphic.reset();
        }
    }
}

fn cursor_movement(
    mut query: Query<(
        &mut Sprite,
        &mut Transform,
        &mut CursorData,
    ),With<CursorType>>,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.single();

    for (mut sprite, mut transform, mut data) in &mut query {
        
        if buttons.just_pressed(MouseButton::Left) {
            if let Some(point) = windows
                .single()
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
                .map(|ray| Vec3::new(ray.origin.x,ray.origin.y,transform.translation.z))
            {
                transform.translation = point;
                if let Some(atlas) = &mut sprite.texture_atlas {
                    data.reset();
                    atlas.index = 0;
                }
            }
        }
    }
}

fn camera_movement(
    buttons: Res<ButtonInput<MouseButton>>,
    mut motion: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    if buttons.pressed(MouseButton::Middle) {
        for mut transform in &mut query {
            for event in motion.read() {
                transform.translation.x -= event.delta.x * 4.0;
                transform.translation.y += event.delta.y * 4.0;
            }
        }
    }

}

fn player_movement(
    time: Res<Time>, 
    mut query: Query<(
        &mut Transform,
        &mut Speed,
        &mut Target,
        &mut Direction
    ),With<PlayerType>>,
    keys: Res<ButtonInput<KeyCode>>, 
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera.single();

    for (mut transform, speed, mut target, mut facing) in &mut query {
        let mut movement = (speed.walking as f32) / 10.0;
        
        if buttons.pressed(MouseButton::Left) {
            (*target).0 = windows
                .single()
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
                .map(|ray| ray.origin.truncate())
                .map(|v| Vec2::new(v.x, v.y + 180.0));
        }

        if keys.pressed(KeyCode::ShiftLeft) {
            movement = (speed.running as f32) / 10.0;
        }

        if let Some(point) = target.0 {
            let cpos = transform.translation;
            let tpos = Vec3::new(point.x, point.y, cpos.z);
            let direction = (tpos - cpos).normalize();
            let distance = cpos.distance(tpos);

            if distance > 10. {
                let amount = 1000. * time.delta_secs() * movement;
                let npos = cpos + direction * amount;
                transform.translation = npos;
            } else {
                (*target).0 = None;
            }

            // update the facing direction of the player
            *facing = Direction::from(&direction);
        }
    }
}