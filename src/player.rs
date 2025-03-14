use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Default)]
pub struct PlayerType;

#[derive(Component, Clone, Copy)]
pub enum Direction {
    TopLeft,
    TopRight,
    BotLeft,
    BotRight
}

impl Direction {
    pub fn from(vector: &Vec3) -> Self {
        let x = vector.x;
        let y = vector.y;

        if x >= 0. && y >= 0. {
            Self::TopRight
        }
        else if x < 0. && y >= 0. {
            Self::TopLeft
        }
        else if x < 0. && y < 0. {
            Self::BotLeft
        }
        else if x >= 0. && y < 0. {
            Self::BotRight
        } 
        else {
            unreachable!();
        }
    }
}

#[derive(Bundle)]
pub struct Player {
    id: AccountId,
    kind: PlayerType,
    name: Name,
    experience: Experience,
    health: Health,
    speed: Speed,
    graphic: Graphic,
    sprite: Sprite,
    target: Target,
    direction: Direction,
    transform: Transform,
}

impl Player {

    pub fn new(
        id: i32,
        name: String,
        assets: &Res<AssetServer>,
        atlas: &mut ResMut<Assets<TextureAtlasLayout>>
    ) -> Self {

        let path = "sprites/character2.png";
        let texture: Handle<Image> = assets.load(path);
        let layout = TextureAtlasLayout::from_grid(UVec2::new(255,512), 6, 3, None, None);
        let handle = atlas.add(layout);

        Self {
            id: AccountId(id),
            kind: Default::default(),
            name: Name(name),
            experience: Experience { 
                current: 0, 
                level: 1 
            },
            health: Health {
                current: 100,
                maximum: 100
            },
            speed: Speed {
                walking: 2,
                running: 6
            },
            graphic: Graphic {
                idle: Animation {
                    topleft: vec![2],
                    topright: vec![3],
                    botleft: vec![0],
                    botright: vec![1],
                },
                running: Animation {
                    topleft: vec![8,14],
                    topright: vec![9,15],
                    botleft: vec![6,12],
                    botright: vec![7,13],
                },
                walking: Animation {
                    topleft: vec![8,14],
                    topright: vec![9,15],
                    botleft: vec![6,12],
                    botright: vec![7,13],
                },
                fps: 5.,
                timer: Graphic::timer(5.)
            },
            sprite: Sprite {
                image: texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: handle.clone(),
                    index: 1,
                }),
                ..default()
            },
            target: Target(None),
            direction: Direction::BotRight,
            transform: Transform::from_xyz(0., 0., 2.)
        }
    }

    pub fn with_position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.transform = Transform::from_xyz(x, y, z);
        self
    }

}

#[derive(Component, Debug)]
pub struct AccountId(pub i32);

#[derive(Component, Debug)]
pub struct Name(String);

#[derive(Component, Debug)]
pub struct Experience {
    pub current: usize,
    pub level: usize,
}

#[derive(Component, Debug)]
pub struct Speed {
    pub walking: usize,
    pub running: usize,
}

#[derive(Component, Debug)]
pub struct SpeedValue(pub f32);

#[derive(Component, Debug)]
pub struct Health {
    pub current: usize,
    pub maximum: usize,
}

#[derive(Component, Debug)]
pub struct Target(pub Option<Vec2>);

#[derive(Component, Debug)]
pub struct Graphic {
    pub idle: Animation,
    pub running: Animation,
    pub walking: Animation,
    pub fps: f32,
    pub timer: Timer,
}

impl Graphic {
    pub fn reset(&mut self) {
        self.timer = Self::timer(self.fps);
    }

    fn timer(fps: f32) -> Timer {
        Timer::new(
            Duration::from_secs_f32(1.0 / fps), 
            TimerMode::Once
        )
    }
}

#[derive(Component, Debug)]
pub struct Animation {
    topleft: Vec<usize>,
    topright: Vec<usize>,
    botleft: Vec<usize>,
    botright: Vec<usize>,
}

impl Animation {

    pub fn facing(&self, direction: Direction) -> &Vec<usize> {
        match direction {
            Direction::TopLeft => &self.topleft,
            Direction::TopRight => &self.topright,
            Direction::BotLeft => &self.botleft,
            Direction::BotRight => &self.botright,
        }
    }
    
    pub fn next(&self, direction: Direction, current: usize) -> usize {
        let animation = self.facing(direction);

        let value = animation
            .iter()
            .skip_while(|&&v| v != current)
            .nth(1)
            .cloned();
        
        let first = animation.first().cloned();
        value.unwrap_or(first.unwrap_or(current))
    }

}


