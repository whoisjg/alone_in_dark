use crate::components::*;
use crate::constants::*;
use specs::prelude::*;

pub struct DialogSys;

impl<'a> System<'a> for DialogSys {
    type SystemData = (
        WriteExpect<'a, raylib::AppContext>,
        Entities<'a>,
        WriteStorage<'a, Dialog>,
        ReadStorage<'a, Player>,
        // lazy add dialog
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (mut ctx, mut entities, mut dialog, player, updater): Self::SystemData) {
        use raylib::input::*;
        use specs::Join;

        // check if player has no dialog, if he does
        // quit it.
        if ctx.is_key_pressed(KEY_ENTER) {
            // for (ent, _, _) in (&entities, &player, !&dialog).join() {
            //     // info!("Added dialog box");
            //     let say: DialogCharSeq = include_str!("../dialog/test.txt").into();
            //     updater.insert(
            //         ent,
            //         Dialog::new(DialogThumbnail::new(DialogImage::Player), say.0),
            //     );
            //     return;
            // }
            // remove dialogbox
            for (ent, _, dialog) in (&entities, &player, &mut dialog).join() {
                if dialog.is_done() {
                    // info!("Removed dialog box");
                    updater.remove::<Dialog>(ent);
                    return;
                }
                if dialog.should_clear() {
                    dialog.clear();
                    return;
                }
            }
        }
    }
}

pub struct DrawDialogSys;

impl<'a> System<'a> for DrawDialogSys {
    type SystemData = (
        WriteExpect<'a, Option<raylib::DrawContext>>,
        WriteStorage<'a, Dialog>,
    );

    fn run(&mut self, (mut ctx, mut dialog): Self::SystemData) {
        use specs::Join;
        let ctx = ctx.as_mut().unwrap();

        // Thumbnail Region
        let (ml, mb, mr, mt) = (20.0, 20.0, 20.0, 20.0);
        let dialog_height = DIALOG_HEIGHT as f32;

        let sw = crate::PLAY_WIDTH as f32;
        let sh = crate::SCREEN_HEIGHT as f32;

        let tx = ml;
        let th = dialog_height - mt - mb;
        let ty = sh - th - mb;
        let tw = sw / 4.0;
        let thumb_rect = raylib::Rectangle::new(tx, ty, tw, th);

        let text_rect = raylib::Rectangle::new(tx + tw + ml, ty, 1200.0, th);

        for (dialog,) in (&mut dialog,).join() {
            ctx.draw_rectangle(
                0,
                crate::SCREEN_HEIGHT - DIALOG_HEIGHT,
                crate::PLAY_WIDTH,
                dialog_height as i32,
                &raylib::Color::WHITE,
            );

            dialog.thumbnail.draw(ctx, &thumb_rect);

            dialog.draw_text(ctx, &text_rect)
            // ctx.draw_text(
            //     &dialog.say,
            //     (tx + tw + ml) as i32,
            //     ty as i32,
            //     40,
            //     &raylib::Color::BLACK,
            // );
        }
    }
}
