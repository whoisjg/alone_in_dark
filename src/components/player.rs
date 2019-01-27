use crate::components::*;

#[derive(Component, Debug, Clone, Default)]
pub struct Player {
    pub time_since_fired: f32,
    pub can_control: bool,
    pub can_fire: bool,
    pub should_die: bool,
}

pub const fn player_shape(pos: raylib::Vector2) -> Shape {
    Shape::new(
        pos,
        ShapeType::Rect(Rect {
            width: 0.9,
            height: 0.9,
        }),
        raylib::Color::WHITE,
    )
}

pub const fn player_light() -> Light {
    Light::new(10.0, raylib::Color::WHITE)
}

pub fn make_player(world: &mut World, pos: raylib::Vector2) {
    world
        .create_entity()
        .with(Player::default())
        .with(player_shape(pos))
        .with(player_light())
        .build();
}

pub fn basic_player<'a>(x: f32, y: f32, p: &Entity, updater: &'a Read<'a, LazyUpdate>) {
    updater.insert(*p, Player::default());
    updater.insert(*p, player_shape(raylib::Vector2::new(x, y)));
    updater.insert(*p, player_light());
}
