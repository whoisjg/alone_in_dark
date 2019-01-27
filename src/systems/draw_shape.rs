use crate::components::*;
use crate::constants::*;
use specs::prelude::*;

pub struct DrawShapeSys;

impl<'a> System<'a> for DrawShapeSys {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (
        WriteExpect<'a, Option<raylib::DrawContext>>,
        Entities<'a>,
        ReadStorage<'a, Shape>,
        ReadStorage<'a, Light>,
        ReadStorage<'a, Lighted>,
        ReadStorage<'a, DontDrawShape>,
    );

    fn run(&mut self, (mut ctx, ents, shapes, lights, lighted, dont_draw): Self::SystemData) {
        use specs::Join;
        let ctx = ctx.as_mut().unwrap();
        // The `.join()` combines multiple component storages,
        // so we get access to all entities which have
        // both a position and a velocity.

        let all_lights: Vec<_> = (&lights, &shapes)
            .join()
            .map(|(l, s)| (l, s.center()))
            .collect();

        for (ent, st, _) in (&ents, &shapes, !&dont_draw).join() {
            let mut color = st.color.clone();
            color.a = if let Some(lighted) = lighted.get(ent) {
                // handle lighting effects
                let center = st.center();
                let closest_light = all_lights
                    .iter()
                    .map(|(l, pos)| (l, pos, l.power_at((*pos - center).hypot())))
                    .max_by(|(_, _, a), (_, _, b)| a.partial_cmp(&b).unwrap());

                if let Some((l, lpos, power)) = closest_light {
                    (255.0 * lighted.specular * power).min(255.0) as u8
                } else {
                    0
                }
            } else {
                color.a
            };

            match &st.shape_type {
                ShapeType::Rect(rect) => {
                    ctx.draw_rectangle(
                        (st.pos.x * PPT as f32) as i32,
                        (st.pos.y * PPT as f32) as i32,
                        (rect.width * PPT as f32) as i32,
                        (rect.height * PPT as f32) as i32,
                        &color,
                    );
                }
                ShapeType::Circle(Circle { radius }) => ctx.draw_circle(
                    (st.pos.x * PPT as f32) as i32,
                    (st.pos.y * PPT as f32) as i32,
                    *radius * PPT as f32,
                    &color,
                ),
            }
        }
    }
}
