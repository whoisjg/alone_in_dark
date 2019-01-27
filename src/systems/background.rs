use crate::components::*;
use crate::constants::*;
use specs::prelude::*;

pub struct DrawBackground;

impl<'a> System<'a> for DrawBackground {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (
        WriteExpect<'a, Option<raylib::DrawContext>>,
        ReadStorage<'a, Shape>,
        ReadStorage<'a, Light>,
    );

    fn run(&mut self, (mut ctx, shapes, lights): Self::SystemData) {
        let ctx = ctx.as_mut().unwrap();

        // The `.join()` combines multiple component storages,
        // so we get access to all entities which have
        // both a position and a velocity.

        ctx.draw_rectangle(
            0,
            0,
            crate::SCREEN_WIDTH,
            crate::SCREEN_HEIGHT,
            &raylib::Color::BLACK,
        );

        // draw lights above the background using concentric circles
        for (shape, light) in (&shapes, &lights).join() {
            let lr = light.radius as i32;
            for i in (1..).take_while(|i| i < &lr) {
                let r = i as f32;
                let modi = i as f32 + rand::random::<f32>();
                let mut color = light.color.clone();
                color.a = (255.0 * light.power_at(modi)) as u8;
                let pos = shape.center();
                ctx.draw_circle(
                    (pos.x * PPT as f32) as i32,
                    (pos.y * PPT as f32) as i32,
                    r * PPT as f32,
                    &color,
                )
            }
        }
    }
}
