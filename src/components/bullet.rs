use specs::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Bullet {
    pub dir: raylib::Vector2,
    pub speed: f32,
}

impl Bullet {
    pub const BULLET_FLICKER_DUR: u32 = 60;
    pub fn new(dir: raylib::Vector2, speed: f32) -> Self {
        Bullet { dir, speed }
    }
}

#[derive(Component, Clone, Debug)]
pub struct BulletDebris {
    pub lifetime: f32,
    pub gravity: raylib::Vector2,
    pub dir: raylib::Vector2,
    pub start_pos: raylib::Vector2,
}
