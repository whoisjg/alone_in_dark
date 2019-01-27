use crate::components::*;
use specs::prelude::*;

#[derive(Component, Clone, Debug, Default)]
pub struct Wall;

#[derive(Component, Clone, Debug, Default)]
pub struct Lighted {
    pub specular: f32,
}

#[derive(Component, Clone, Debug, Default)]
pub struct PlayerBullet;

#[derive(Component, Clone, Debug, Default)]
pub struct EnemyBullet;

#[derive(Component, Clone, Debug, Default)]
pub struct DontDrawShape;
