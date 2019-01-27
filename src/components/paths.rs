use specs::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct FollowPath {
    pub start_pos: raylib::Vector2,
    pub target_pos: raylib::Vector2,
    pub speed: f32,
}
