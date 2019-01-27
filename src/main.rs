#[macro_use]
extern crate log;
#[macro_use]
extern crate specs_derive;
use specs::prelude::*;

use raylib::*;

mod components;
mod constants;
mod levels;
mod systems;

use self::components::*;
use self::constants::*;
use self::levels::*;
use self::systems::*;

// struct Game {
struct Game<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    draw_dispatcher: Dispatcher<'a, 'b>,
}

// impl Game {
impl<'a, 'b> Game<'a, 'b> {
    pub fn new() -> Self {
        let mut world = World::new();
        register_components(&mut world);

        let mut dispatcher = DispatcherBuilder::new()
            .with(Level1::new(), "level_1", &[])
            .with(PlayerSys, "player_sys", &[])
            .with(EnemySys::new(), "enemy_sys", &[])
            .with(DialogSys, "dialog_sys", &[])
            .with(BulletSys, "bullet_sys", &[])
            .with(DamageSys, "damage_sys", &[])
            .with(BulletDebrisSys, "bullet_debris_sys", &[])
            .build();

        let mut draw_dispatcher = DispatcherBuilder::new()
            .with_thread_local(DrawBackground)
            .with_thread_local(DrawShapeSys)
            .with_thread_local(DrawDialogSys)
            .with_thread_local(DrawDamageSys)
            .build();

        dispatcher.setup(&mut world.res);
        draw_dispatcher.setup(&mut world.res);
        Game {
            world,
            dispatcher,
            draw_dispatcher,
        }
    }
}

pub fn make_simple_level(world: &mut World) {
    let room_height = VERTICAL_TILES_PER_WINDOW as f32;
    let room_width = room_height * 16.0 / 9.0;
    let wall_color = raylib::Color::WHITE;

    let wall_width = 5.0;

    // make walls
    for x in (0..room_width as i32) {
        for y in (0..room_height as i32) {
            let x = x as f32;
            let y = y as f32;
            // left wall
            if (x == wall_width)
                || (x == room_width - wall_width)
                || y == wall_width
                || y == room_height - wall_width
            {
                make_wall(
                    world.create_entity(),
                    raylib::Vector2::new(x, y),
                    1.0,
                    1.0,
                    wall_color,
                );
            }
        }
    }

    // make middle of room wall
    for x in ((room_width / 3.0) as i32..(room_width / 3.0 + wall_width) as i32) {
        for y in ((room_height / 4.0) as i32..(3.0 * room_height / 4.0) as i32) {
            let x = x as f32;
            let y = y as f32;
            make_wall(
                world.create_entity(),
                raylib::Vector2::new(x, y),
                1.0,
                1.0,
                wall_color,
            );
        }
    }

    // middle of the room circle
    make_circle_wall(
        world.create_entity(),
        raylib::Vector2::new(room_width / 2.0, room_height / 2.0),
        5.0,
        wall_color,
    );

    make_player(world, raylib::Vector2::new(50.0, 10.0))
}

// impl raylib::Game for Game {
impl<'a, 'b> raylib::Game for Game<'a, 'b> {
    fn start(&mut self, mut ctx: raylib::AppContext) {
        ctx.set_target_fps(FPS as i32);
        self.world.add_resource(ctx);
        self.world.add_resource::<Option<raylib::DrawContext>>(None);

        // make_simple_level(&mut self.world);
    }

    fn update(&mut self, mut ctx: raylib::AppContext) -> bool {
        // This dispatches all the systems in.
        self.world.add_resource(ctx.clone());
        self.dispatcher.dispatch(&mut self.world.res);
        self.world.maintain();
        let mut should_quit = false;
        ctx.draw(|ctx| {
            self.world.add_resource(Some(ctx));
            self.draw_dispatcher.dispatch(&mut self.world.res);
            let mut draw = self.world.res.fetch_mut::<Option<raylib::DrawContext>>();
            let c = draw.take().unwrap();
            should_quit = c.app_context.should_quit;
        });
        if should_quit {
            info!("should_quit");
        }
        return should_quit;
    }
}

fn main() {
    // env_logger::init();

    let game = Box::new(Game::new());
    raylib::run(SCREEN_WIDTH, SCREEN_HEIGHT, "Game", game);
}
