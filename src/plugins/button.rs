use bevy::{color::palettes::css::BLACK, prelude::*};

#[derive(Bundle,Default)]
pub struct MyButton<T>
where
    T: Component + Default
{
    pub _button: Button,
    pub node: Node,
    pub background_color: BackgroundColor,
    pub action: T
}

#[derive(Bundle,Default)]
pub struct MyButtonLabel {
    text: Text,
    color: TextColor,
}

impl MyButtonLabel {
    pub fn new(label: &str) -> Self {
        Self {
            text: Text::new(label),
            color: TextColor(BLACK.into())
        }
    }
}

impl<T> MyButton<T>
where 
    T: Component + Default
{
    pub fn new(action: T) -> Self {
        Self { 
            action,
            node: Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        }
    }
}

// action: MenuButtonAction

// NODE: {
//     width: Val::Percent(100.0),
//     padding: UiRect::all(Val::Px(10.0)),
//     justify_content: JustifyContent::Center,
//     align_items: AlignItems::Center,
//     ..default()
// },

// pub struct ButtonPlugin;

// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut App) {
        
//     }
// }
