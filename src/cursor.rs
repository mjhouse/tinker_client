use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Debug, Default)]
pub struct CursorType;

#[derive(Component, Debug)]
pub struct CursorData {
    pub fps: f32,
    pub timer: Timer,
}

#[derive(Bundle, Debug)]
pub struct Cursor {
    pub kind: CursorType,
    pub sprite: Sprite,
    pub data: CursorData,
    pub transform: Transform,
}

impl Cursor {

    pub fn new(
        assets: &Res<AssetServer>,
        atlas: &mut ResMut<Assets<TextureAtlasLayout>>
    ) -> Self {

        let path = "sprites/cursor.png";
        let texture: Handle<Image> = assets.load(path);
        let layout = TextureAtlasLayout::from_grid(UVec2::new(255,255), 5, 1, None, None);
        let handle = atlas.add(layout);

        Self {
            kind: Default::default(),
            sprite: Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.clone(),
                    index: 4,
                }),
                ..default()
            },
            data: CursorData {
                fps: 20.,
                timer: CursorData::timer(20.),
            },
            transform: Transform::from_xyz(0., 0., 1.)
        }
    }
}

impl CursorData {
    pub fn reset(&mut self) {
        self.timer = Self::timer(self.fps);
    }

    fn timer(fps: f32) -> Timer {
        Timer::new(
            Duration::from_secs_f32(1.0 / fps), 
            TimerMode::Once
        )
    }

    pub fn next(&self, index: usize) -> usize {
        if index < 4 {
            index + 1
        } else {
            4
        }
    }
}
