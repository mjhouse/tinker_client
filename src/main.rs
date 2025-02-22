use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_tilemap::prelude::*;
use std::time::Duration;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin{
                primary_window: Some(Window {
                    title: String::from(
                        "Tinker",
                    ),
                    ..Default::default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()))
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .add_systems(Update, animation)
        .add_systems(Update, zoom)
        .run();
}

#[derive(Component)]
pub enum Facing {
    TopLeft,
    TopRight,
    BotLeft,
    BotRight
}

impl Facing {
    pub fn from_vector(vector: &Vec3) -> Self {
        let x = vector.x;
        let y = vector.y;

        if x > 0. && y > 0. {
            Self::TopRight
        }
        else if x < 0. && y > 0. {
            Self::TopLeft
        }
        else if x < 0. && y < 0. {
            Self::BotLeft
        }
        else if x > 0. && y < 0. {
            Self::BotRight
        } 
        else {
            Self::BotRight
        }
    }
}

#[derive(Component)]
pub struct Speed(f32);

#[derive(Component, Debug)]
pub struct Target(Option<Vec2>);

#[derive(Component)]
struct AnimationConfig {
    standing: [usize;4],

    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            standing: [0,1,2,3],

            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    
    let texture: Handle<Image> = asset_server.load("sprites/character2.png");
    let config = AnimationConfig::new(1, 3, 4);
    
    
    // 255 x 512
    let layout = TextureAtlasLayout::from_grid(UVec2::new(255,512), 6, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: config.first_sprite_index,
            }),
            ..default()
        },
        Transform::from_xyz(0., 0., 1.),
        Speed(1.),
        Target(None),
        Facing::BotRight,
        config,
    ));

}


fn animation(
    time: Res<Time>, 
    mut query: Query<(&mut AnimationConfig, &mut Sprite, &mut Target, &mut Facing)>,
) {
    for (mut config, mut sprite, target, facing) in &mut query {
        config.frame_timer.tick(time.delta());

        let (standing, run1, run2) = match *facing {
            Facing::TopLeft => {
                let s = config.standing[2];
                (s, 8, 14)
            },
            Facing::TopRight => {
                let s = config.standing[3];
                (s, 9, 15)
            },
            Facing::BotLeft => {
                let s = config.standing[0];
                (s, 6, 12)
            },
            Facing::BotRight => {
                let s = config.standing[1];
                (s, 7, 13)
            },
        };

        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if target.0.is_none() {
                    atlas.index = standing;
                }
                else {
                    if atlas.index == run1 {
                        atlas.index = run2;
                    } else {
                        atlas.index = run1;
                    }
                }
            }
            config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
        }

    }
}

fn zoom(
    mut camera: Query<(&mut OrthographicProjection, &Camera2d)>,
    scroll: Res<AccumulatedMouseScroll>,
) {
    let (mut projection, _) = camera.single_mut();
    let zoom = -scroll.delta.y * 0.2 + 1.;
    let scale = projection.scale;
    projection.scale = (scale * zoom).clamp(0.1,10.0);
}

fn movement(
    time: Res<Time>, 
    mut query: Query<(&mut Sprite, &mut Transform, &mut Speed, &mut Target, &mut Facing)>, 
    keys: Res<ButtonInput<KeyCode>>, 
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    scroll: Res<AccumulatedMouseScroll>,
) {
    let (camera, camera_transform) = camera.single();

    for (_, mut transform, speed, mut target, mut facing) in &mut query {
        

        if buttons.just_pressed(MouseButton::Left) {
            (*target).0 = windows
                .single()
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
                .map(|ray| ray.origin.truncate());     
        }
        else {
            if let Some(point) = target.0 {
                let current_position = transform.translation;
                let target_position = Vec3::new(point.x, point.y, current_position.z);
                let direction = (target_position - current_position).normalize();
                *facing = Facing::from_vector(&direction);
    
                let distance = current_position.distance(target_position);
                if distance > 10. {
                    let translation_amount = 200. * time.delta_secs();
                    let new_position = current_position + direction * translation_amount;
                    transform.translation = new_position; // Update the entity's position
                } else {
                    (*target).0 = None;
                }
            }
        }


        // if keys.pressed(KeyCode::KeyP) {
        //     (*speed).0 += 0.1;
        // }
        // if keys.pressed(KeyCode::KeyM) {
        //     (*speed).0 -= 0.1;
        // }
        // if keys.pressed(KeyCode::KeyW) {
        //     transform.translation.y += 150. * time.delta_secs() * speed.0;
        // }
        // if keys.pressed(KeyCode::KeyS) {
        //     transform.translation.y -= 150. * time.delta_secs() * speed.0;
        // }
        // if keys.pressed(KeyCode::KeyD) {
        //     transform.translation.x += 150. * time.delta_secs() * speed.0;
        // }
        // if keys.pressed(KeyCode::KeyA) {
        //     transform.translation.x -= 150. * time.delta_secs() * speed.0;
        // }
    }
}