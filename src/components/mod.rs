mod bullet;
mod damage;
mod dialog;
mod enemy;
mod light;
mod markers;
mod paths;
mod player;
mod shape;
mod wall;

pub use self::bullet::*;
pub use self::damage::*;
pub use self::dialog::*;
pub use self::enemy::*;
pub use self::light::*;
pub use self::markers::*;
pub use self::paths::*;
pub use self::player::*;
pub use self::shape::*;
pub use self::wall::*;

use specs::prelude::*;

pub fn register_components(world: &mut specs::World) {
    world.register::<Shape>();
    world.register::<Wall>();
    world.register::<Player>();
    world.register::<Collision>();
    world.register::<Bullet>();
    world.register::<BulletDebris>();
    world.register::<Light>();
    world.register::<Lighted>();
    world.register::<FollowPath>();
    world.register::<Enemy>();
    world.register::<PlayerBullet>();
    world.register::<EnemyBullet>();
    world.register::<Damage>();
    world.register::<DontDrawShape>();
}
