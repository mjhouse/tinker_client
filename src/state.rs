use bevy::ecs::system::Resource;


#[derive(Resource,Debug,Clone,Default)]
pub struct ConnectionState {
    pub id: i32,
    pub username: String,
    pub token: Option<String>
}
