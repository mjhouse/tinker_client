use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum ViewState {
    #[default]
    Menu,
    Game
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_view<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub mod menu;
pub mod game;