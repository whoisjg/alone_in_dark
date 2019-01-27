use crate::*;
use specs::prelude::*;

pub fn make_wall(
    builder: EntityBuilder,
    pos: raylib::Vector2,
    width: f32,
    height: f32,
    color: raylib::Color,
) {
    builder
        .with(Wall)
        .with(Shape::new(
            pos,
            ShapeType::Rect(Rect { width, height }),
            color,
        ))
        .with(Lighted { specular: 10.0 })
        .build();
}

pub fn basic_wall<'a>(x: f32, y: f32, e: &Entity, updater: &'a Read<'a, LazyUpdate>) {
    updater.insert(*e, Wall);
    updater.insert(
        *e,
        Shape::new(
            raylib::Vector2::new(x, y),
            ShapeType::Rect(Rect::new(1.0, 1.0)),
            raylib::Color::WHITE,
        ),
    );
    updater.insert(*e, Lighted { specular: 10.0 });
}

pub fn make_circle_wall(
    builder: EntityBuilder,
    pos: raylib::Vector2,
    radius: f32,
    color: raylib::Color,
) {
    builder
        .with(Wall)
        .with(Shape::new(pos, ShapeType::Circle(Circle { radius }), color))
        .with(Lighted { specular: 100.0 })
        .build();
}
