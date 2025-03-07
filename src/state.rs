use bevy::ecs::system::Resource;


#[derive(Resource,Debug,Default)]
pub struct ConnectionState {
    pub id: i32,
    pub username: String,
    pub token: Option<String>
}
