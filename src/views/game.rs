use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Barrier;
use std::time::Duration;
use chrono::{DateTime, Utc};
use bevy::input::mouse::{AccumulatedMouseScroll, MouseMotion};
use bevy::window::PrimaryWindow;
use bevy_ecs_tiled::prelude::*;
use bevy::prelude::*;
use futures_util::future::{select, Either};
use async_std::task::sleep;
use futures_util::lock::Mutex;
use futures_util::pin_mut;
use once_cell::sync::Lazy;
use tinker_records::messages::{Message, Value};
use async_tungstenite::async_std::connect_async;
use tungstenite as ts;
use futures_util::stream::StreamExt;

use crate::cursor::{Cursor, CursorData, CursorType};
use crate::player::{AccountId, Direction, EntityType, Graphic, Player, PlayerType, Speed, Target};
use crate::state::ConnectionState;
use bevy::tasks::IoTaskPool;

use super::{despawn_view, ViewState};

pub static RUNNING: AtomicBool = AtomicBool::new(false);
pub static SHUTDOWN: AtomicBool = AtomicBool::new(false);
pub static SHUTDOWN_BARRIER: Barrier = Barrier::new(2);

pub static INCOMING_QUEUE: Lazy<Mutex<VecDeque<Message>>> = Lazy::new(|| { Default::default() });
pub static OUTGOING_QUEUE: Lazy<Mutex<VecDeque<Message>>> = Lazy::new(|| { Default::default() });

#[derive(Component)]
pub struct OnGame;

pub fn main_game(app: &mut App) {
    app
        .add_systems(OnEnter(ViewState::Game), socket_connection)
        .add_systems(OnEnter(ViewState::Game), handle_messages)
        .add_systems(Update, process_messages)

        .add_systems(OnEnter(ViewState::Game), game_setup)
        .add_systems(OnExit(ViewState::Game), despawn_view::<OnGame>)

        .add_systems(Update, character_movement.run_if(in_state(ViewState::Game)))

        .add_systems(Update, player_movement.run_if(in_state(ViewState::Game)))
        .add_systems(Update, camera_movement.run_if(in_state(ViewState::Game)))
        .add_systems(Update, player_animation.run_if(in_state(ViewState::Game)))
        .add_systems(Update, cursor_movement.run_if(in_state(ViewState::Game)))
        .add_systems(Update, cursor_animation.run_if(in_state(ViewState::Game)))
        .add_systems(Update, camera_zoom.run_if(in_state(ViewState::Game)));
}

fn broadcast(message: Message) {
    // TODO: change this to use an mpsc channel to enqueue outgoing messages
    // TODO: change the socket_connection_task to use mpsc channel to send incoming messages
    if let Some(mut queue) = OUTGOING_QUEUE.try_lock() {
        queue.push_back(message);
    }
}


#[derive(Resource)]
struct ConnectionChannel {
    receiver: std::sync::Mutex<std::sync::mpsc::Receiver<Message>>,
}

fn handle_messages(
    mut commands: Commands,
) {
    let (sender, receiver) = std::sync::mpsc::channel();
    IoTaskPool::get().spawn(handle_messages_task(sender)).detach();
    commands.insert_resource(ConnectionChannel { receiver: std::sync::Mutex::new(receiver) });
}

fn socket_connection(
    state: Res<ConnectionState>,
) {
    IoTaskPool::get().spawn(socket_connection_task(state.clone())).detach();
}

async fn handle_messages_task(
    sender: std::sync::mpsc::Sender<Message>
) {
    loop {
        for item in INCOMING_QUEUE.lock().await.drain(..) {
            sender.send(item).unwrap();
        }
    }
}

fn process_messages(
    mut query: Query<(
        &AccountId,
        &mut Speed,
        &mut Target,
    ),With<EntityType>>,
    delete_query: Query<(Entity,&AccountId), With<EntityType>>,
    channel: Option<Res<ConnectionChannel>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if let Some(reader) = channel {
        if let Ok(rx) = reader.receiver.lock() {
            while let Ok(item) = rx.try_recv() {
                match item.value {
                    Value::Move(message) => {
                        for (id, mut speed, mut target) in &mut query {
                            if id.0 == item.header.account_id {
                                target.0 = Some(Vec2::new(message.x,message.y));
                                speed.fixed = Some(message.speed);
                            }
                        }
                    },
                    Value::Initial(message) => {
                        for character in message.entities {
                            Player::new::<EntityType>(
                                character.account_id,
                                &asset_server,
                                &mut texture_atlas_layouts
                            )
                            .with_name(character.name.clone())
                            .with_position(character.x, character.y, 2.0)
                            .with_speed(0.0)
                            .build(&mut commands);
                        }
                    },
                    Value::Connect(message) => {
                        Player::new::<EntityType>(
                            item.header.account_id,
                            &asset_server,
                            &mut texture_atlas_layouts
                        )
                        .with_name(message.entity.name.clone())
                        .with_position(message.entity.x, message.entity.y, 2.0)
                        .with_speed(0.0)
                        .build(&mut commands);
                    },
                    Value::Disconnect(_) => {
                        for (entity, id) in &delete_query {
                            if id.0 == item.header.account_id {
                                commands.entity(entity).despawn_recursive();
                                break;
                            }
                        }
                    },
                    _ => ()
                }
            }
        }
    }
}

async fn socket_connection_task(
    state: ConnectionState
) {
    if let Some(token) = state.token {
        let url = format!("ws://localhost:8080/connect/{}",token);
    
        let (mut stream, _) = connect_async(&url)
            .await
            .expect("Failed to connect");

        RUNNING.store(true, Ordering::Relaxed);

        loop {
            // create timeout and stream futures
            let timeout = sleep(Duration::from_millis(100));
            let source = stream.next();

            pin_mut!(timeout);

            // check for incoming messages from the server
            if let Either::Left((result, _)) = select(source,timeout).await {
                if let Some(Ok(ts::Message::Text(value))) = result {
                    let message = serde_json::from_slice(value.as_bytes()).unwrap();
                    INCOMING_QUEUE.lock().await.push_back(message);
                }
            }

            // send outgoing messages to the server
            while let Some(value) = OUTGOING_QUEUE.lock().await.pop_front() {
                let message = serde_json::to_string(&value).unwrap();
                stream.send(ts::Message::text(message)).await.unwrap();
            }

            if SHUTDOWN.load(Ordering::Relaxed) {
                println!("SHUTTING DOWN");
                stream.close(None).await.unwrap();
                SHUTDOWN_BARRIER.wait();
                break;
            }

        }
    }
}

fn game_setup(
    mut commands: Commands,
    state: Res<ConnectionState>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
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

    Player::new::<PlayerType>(
        state.id,
        &asset_server,
        &mut texture_atlas_layouts
    )
    .with_name(state.username.clone())
    .build(&mut commands);

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
    ),Or<(With<PlayerType>,With<EntityType>)>>,
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
    mut query: Query<(
        &mut Speed,
        &mut Target
    ),With<PlayerType>>,
    keys: Res<ButtonInput<KeyCode>>, 
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>
) {
    let (camera, camera_transform) = camera.single();
    let (mut speed, mut target) = query.single_mut();

    if keys.pressed(KeyCode::ShiftLeft) {
        speed.fixed = Some(speed.running as f32);
    } else {
        speed.fixed = Some(speed.walking as f32);
    }
    
    if buttons.pressed(MouseButton::Left) {
        (*target).0 = windows
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
            .map(|ray| ray.origin.truncate())
            .map(|v| Vec2::new(v.x, v.y + 180.0));
    }
}

fn character_movement(
    time: Res<Time>, 
    mut query: Query<(
        &AccountId,
        &mut Transform,
        &mut Speed,
        &mut Target,
        &mut Direction
    )>,
    state: Res<ConnectionState>,
) {
    for (id, mut transform, speed, mut target, mut facing) in &mut query {

        if let Some(point) = target.0 {
            if let Some(speed_value) = speed.fixed {
                let cpos = transform.translation;
                let tpos = Vec3::new(point.x, point.y, cpos.z);
                let direction = (tpos - cpos).normalize();
                let distance = cpos.distance(tpos);
    
                if distance > 10. {
                    let amount = 1000. * time.delta_secs() * (speed_value / 10.0);
                    let npos = cpos + direction * amount;
                    transform.translation = npos;
                    if id.0 == state.id {
                        broadcast(Message::Move(state.id, speed_value, npos.x, npos.y));
                    }
                } else {
                    (*target).0 = None;
                }
    
                // update the facing direction of the character
                *facing = Direction::from(&direction);
            }
        }
    }
}