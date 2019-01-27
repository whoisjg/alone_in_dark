use crate::components::*;
use crate::constants::*;
use specs::prelude::*;

pub struct DamageSys;

impl<'a> System<'a> for DamageSys {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (
        WriteExpect<'a, raylib::AppContext>,
        WriteStorage<'a, Damage>,
        WriteStorage<'a, Shape>,
    );

    fn run(&mut self, (mut ctx, mut damages, mut shapes): Self::SystemData) {
        use specs::Join;

        for (damage, shape) in (&mut damages, &mut shapes).join() {
            damage.vel += ctx.get_frame_time() * raylib::Vector2::new(0.0, 100.0);
            damage.pos += ctx.get_frame_time() * damage.vel;
            shape.pos = damage.pos;
        }
    }
}

pub struct DrawDamageSys;

impl<'a> System<'a> for DrawDamageSys {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (
        WriteExpect<'a, Option<raylib::DrawContext>>,
        ReadStorage<'a, Damage>,
    );

    fn run(&mut self, (mut ctx, damages): Self::SystemData) {
        use specs::Join;
        let ctx = ctx.as_mut().unwrap();

        for (damage,) in (&damages,).join() {
            ctx.draw_text(
                &format!("{}", damage.amount),
                (damage.pos.x * PPT as f32) as i32,
                (damage.pos.y * PPT as f32) as i32,
                15,
                &raylib::Color::WHITE,
            );
        }
    }
}
