use crate::components::*;
use specs::prelude::*;

pub struct BulletSys;

impl<'a> System<'a> for BulletSys {
    type SystemData = (
        ReadExpect<'a, raylib::AppContext>,
        Entities<'a>,
        ReadStorage<'a, Wall>,
        ReadStorage<'a, Bullet>,
        ReadStorage<'a, PlayerBullet>,
        ReadStorage<'a, EnemyBullet>,
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Shape>,
        WriteStorage<'a, Collision>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            mut ctx,
            ents,
            walls,
            bullets,
            player_bullets,
            enemy_bullets,
            enemies,
            mut players,
            mut shapes,
            mut collisions,
            updater,
        ): Self::SystemData,
    ) {
        use specs::Join;

        let walls: Vec<_> = (&walls, &shapes).join().map(|(_, s)| s).collect();

        let hit_bullets: Vec<_> = (&ents, &bullets, &shapes, !&collisions)
            .join()
            // broad phase collisions detection check if wall is in the circle.
            .filter_map(|(e, b, s, _)| {
                let s_radius = s.bounding_radius();
                let candidates: Vec<_> = walls
                    .iter()
                    .filter(|w| (s.pos - w.pos).hypot2() < (s_radius + w.bounding_radius()).powi(2))
                    .collect();
                if candidates.len() > 0 {
                    Some((e, b, s, candidates))
                } else {
                    None
                }
            })
            // narrow phase
            .filter_map(|(e, b, s, _)| {
                if let Some(wall) = walls.iter().find(|bshape| s.collides_with(bshape)) {
                    Some((e, s.clone(), (**wall).clone()))
                } else {
                    None
                }
            })
            .collect();

        // mark the collisions
        for (ent, shape, wall) in hit_bullets.into_iter() {
            updater.insert(ent, Collision::new(shape, wall));
        }

        // move those that aren't colliding
        for (bullet, mut shape, ()) in (&bullets, &mut shapes, !&collisions).join() {
            shape.pos += bullet.dir * bullet.speed;
        }

        // remove those that collided
        for (ent, bullet, _, collision) in (&ents, &bullets, &shapes, &collisions).join() {
            ents.delete(ent);
            // Debris callculation
            let norm = collision
                .a_shape
                .collision_normal(&collision.b_shape)
                .unwrap();
            let bb = collision.a_shape.bounding_box();
            let br = collision.a_shape.bounding_radius();

            for _ in 0..2 {
                let debris = ents.create();

                let debrisComp = BulletDebris {
                    lifetime: 0.2,
                    gravity: -150.0 * norm,
                    dir: 40.0
                        * raylib::Vector2::from_angle(
                            norm.atan2() + (rand::random::<f32>() - 0.5) * std::f32::consts::PI,
                        ),
                    start_pos: collision.a_shape.pos,
                };

                updater.insert(debris, debrisComp);
                // debris
                let debris_st = match collision.a_shape.shape_type {
                    ShapeType::Rect(_) => {
                        ShapeType::Rect(Rect::new(bb.width / 3.0, bb.height / 3.0))
                    }
                    ShapeType::Circle(_) => ShapeType::Circle(Circle::new(br / 3.0)),
                };
                updater.insert(
                    debris,
                    Shape::new(
                        collision.a_shape.pos + norm * bb.width,
                        debris_st,
                        raylib::Color::WHITE,
                    ),
                );
                // light debree
                updater.insert(debris, Light::new(1.0, raylib::Color::WHITE));
            }
            // shape.color.a = (20 * ctx.frame % 255) as u8;
            // if (ctx.frame % 2 * Bullet::BULLET_FLICKER_DUR) < Bullet::BULLET_FLICKER_DUR {
            //     shape.color = raylib::Color::BLACK;
            // } else {
            //     shape.color = raylib::Color::WHITE;
            // }
            return;
        }

        // do collisions with enemies
        let all_enemies: Vec<_> = (&shapes, &enemies).join().collect();

        for (ent, bullet, shape, _) in (&ents, &bullets, &shapes, &player_bullets).join() {
            if let Some((e_shape, _)) = all_enemies
                .iter()
                .find(|(e_shape, _)| e_shape.collides_with(shape))
            {
                ents.delete(ent);
                // info!("Hit Enemy: {:?}", e_shape);
                let d = ents.create();
                basic_damage(
                    shape.pos.x + 2.0 * rand::random::<f32>(),
                    shape.pos.y,
                    1,
                    &d,
                    &updater,
                );
            }
        }

        if let Some((p_ent, player, p_shape)) = (&ents, &mut players, &shapes).join().next() {
            for (ent, bullet, shape, _) in (&ents, &bullets, &shapes, &enemy_bullets).join() {
                if shape.collides_with(p_shape) {
                    ents.delete(ent);
                    player.should_die = true;
                }
            }
        }
    }
}

pub struct BulletDebrisSys;

impl<'a> System<'a> for BulletDebrisSys {
    type SystemData = (
        ReadExpect<'a, raylib::AppContext>,
        Entities<'a>,
        ReadStorage<'a, Wall>,
        WriteStorage<'a, BulletDebris>,
        WriteStorage<'a, Shape>,
    );

    fn run(&mut self, (ctx, ents, walls, mut debris, mut shapes): Self::SystemData) {
        use specs::Join;

        let dt = ctx.get_frame_time();

        for (ent, mut trash, mut shape) in (&ents, &mut debris, &mut shapes).join() {
            // if position is below the plane defined by gravity and the start position, remove this
            let gravity =
                if (-1.0 * trash.gravity.normalize()).dot(shape.pos - trash.start_pos) < 0.0 {
                    // ents.delete(ent);
                    trash.dir = raylib::Vector2::new(0.0, 0.0);
                    // -1.0 * trash.gravity.hypot() * (shape.pos - trash.start_pos).normalize()
                    raylib::Vector2::new(0.0, 0.0)
                } else {
                    trash.gravity
                };
            trash.dir += gravity * dt;
            shape.pos += trash.dir * dt;

            trash.lifetime -= dt;
            if trash.lifetime < 0.0 {
                ents.delete(ent).unwrap();
            }
        }
    }
}
