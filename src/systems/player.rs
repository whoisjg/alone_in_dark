use crate::components::*;
use crate::constants::*;
use specs::prelude::*;

pub struct PlayerSys;

impl<'a> System<'a> for PlayerSys {
    type SystemData = (
        WriteExpect<'a, raylib::AppContext>,
        Entities<'a>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Shape>,
        ReadStorage<'a, Wall>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (mut ctx, ents, mut player, mut shapes, walls, updater): Self::SystemData) {
        use raylib::input::*;
        use specs::Join;

        // Movement check

        let (dx, dy) = match (
            ctx.is_key_down(KEY_A),
            ctx.is_key_down(KEY_D),
            ctx.is_key_down(KEY_W),
            ctx.is_key_down(KEY_S),
        ) {
            (true, _, _, _) => (-1.0, 0.0),
            (_, true, _, _) => (1.0, 0.0),
            (_, _, true, _) => (0.0, -1.0),
            (_, _, _, true) => (0.0, 1.0),
            _ => (0.0, 0.0),
        };

        let walls: Vec<_> = (&walls, &shapes).join().map(|(_, s)| s).collect();

        let (player_ent, mut player, mut shape) = match (&ents, &mut player, &shapes)
            .join()
            .map(|(p, player, s)| (p, player, s.clone()))
            .next()
        {
            Some(joins) => joins,
            None => return,
        };

        let old_pos = shape.pos;
        shape.pos.x = (shape.pos.x + dx).max(0.0);
        shape.pos.y = (shape.pos.y + dy).max(0.0);

        if walls.iter().any(|bshape| shape.collides_with(bshape)) {
            shape.pos = old_pos;
        }

        let (p_width, p_height) = match shape.shape_type {
            ShapeType::Rect(Rect { width, height }) => (width, height),
            _ => (0.0, 0.0),
        };

        // Fire bullet
        let dt = ctx.get_frame_time();
        player.time_since_fired += dt;
        if player.time_since_fired > 1.0 / PLAYER_FIRE_RATE
            && ctx.is_mouse_button_down(raylib::MOUSE_LEFT_BUTTON)
        {
            player.time_since_fired = 0.0;
            let mut bpos = ctx.get_mouse_position();
            bpos.x = bpos.x / PPT as f32;
            bpos.y = bpos.y / PPT as f32;

            let center = shape.pos + 0.5 * raylib::Vector2::new(p_width, p_height);
            let bdir = (bpos - center).normalize();

            let bullet_width = 1.0;

            // spawn four times away from here
            let mut bpos = center + bdir * 4.0 * p_width;
            // randomize how far away to spawn the bullet
            bpos += bdir * 2.0 * bullet_width * (1.0 + (ctx.frame as f32).cos());

            let bullet = ents.create();
            // update position
            updater.insert(
                bullet,
                Shape::new(
                    bpos,
                    ShapeType::Rect(Rect {
                        width: bullet_width,
                        height: bullet_width,
                    }),
                    raylib::Color::WHITE,
                ),
            );
            // add bullet component
            updater.insert(bullet, Bullet::new(bdir, PLAYER_BULLET_SPEED));
            // add light component
            updater.insert(bullet, Light::new(2.0, raylib::Color::WHITE));
            // add player bullet
            updater.insert(bullet, PlayerBullet);
        }

        *shapes.get_mut(player_ent).unwrap() = shape;
    }
}
