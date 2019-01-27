use crate::*;
use specs::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Damage {
    pub amount: i32,
    pub pos: raylib::Vector2,
    pub vel: raylib::Vector2,
}

pub fn basic_damage<'a>(
    x: f32,
    y: f32,
    damage: i32,
    e: &Entity,
    updater: &'a Read<'a, LazyUpdate>,
) {
    updater.insert(
        *e,
        Damage {
            amount: damage,
            pos: raylib::Vector2::new(x, y),
            vel: 10.0 * raylib::Vector2::new(rand::random::<f32>(), 0.0),
        },
    );
    updater.insert(
        *e,
        Shape::new(
            raylib::Vector2::new(x, y),
            ShapeType::Circle(Circle::new(1.0)),
            raylib::Color::new(0, 0, 0, 0),
        ),
    );
    updater.insert(*e, Light::new(2.0, raylib::Color::WHITE));
    updater.insert(*e, DontDrawShape);
}
