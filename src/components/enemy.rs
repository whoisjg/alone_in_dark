use crate::*;
use specs::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Enemy {
    pub last_fired: f32,
    pub count: i32,
}

impl Enemy {
    pub fn new() -> Self {
        Enemy {
            last_fired: 0.0,
            count: 0,
        }
    }
}

pub fn basic_enemy<'a>(x: f32, y: f32, e: &Entity, updater: &'a Read<'a, LazyUpdate>) {
    updater.insert(*e, Enemy::new());
    updater.insert(
        *e,
        Shape::new(
            raylib::Vector2::new(x, y),
            ShapeType::Circle(Circle::new(4.0)),
            raylib::Color::WHITE,
        ),
    );
    updater.insert(*e, Lighted { specular: 50.0 });
}
