use crate::components::*;
use crate::constants::*;
use specs::prelude::*;

pub struct EnemySys {}

impl EnemySys {
    pub fn new() -> Self {
        EnemySys {}
    }
}

impl<'a> System<'a> for EnemySys {
    type SystemData = (
        WriteExpect<'a, raylib::AppContext>,
        Entities<'a>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, Bullet>,
        WriteStorage<'a, Shape>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (mut ctx, ents, mut enemies, bullets, mut shapes, updater): Self::SystemData,
    ) {
        use specs::Join;

        let dt = ctx.get_frame_time();
        let frame = ctx.frame;

        for (enemy, shape) in (&mut enemies, &mut shapes).join() {
            enemy.last_fired += dt;
            if enemy.last_fired > 1.0 {
                enemy.last_fired = 0.0;
                enemy.count += 1;
                // spawn bullets
                for i in (0..4) {
                    let bpos = shape.pos
                        + 2.0
                            * shape.bounding_radius()
                            * raylib::Vector2::from_angle(
                                // (enemy.count as f32 * 15.0) / 180.0 +
                                (i as f32) / 2.0 * std::f32::consts::PI,
                            );
                    let bdir = (bpos - shape.pos).normalize();
                    let bullet = ents.create();
                    // update position
                    updater.insert(
                        bullet,
                        Shape::new(
                            bpos,
                            ShapeType::Circle(Circle::new(2.0)),
                            raylib::Color::WHITE,
                        ),
                    );
                    // add bullet component
                    updater.insert(bullet, Bullet::new(bdir, PLAYER_BULLET_SPEED / 10.0));
                    // add light component
                    updater.insert(bullet, Light::new(2.0, raylib::Color::WHITE));
                    // add player bullet
                    updater.insert(bullet, EnemyBullet);
                }
            }
        }
    }
}
