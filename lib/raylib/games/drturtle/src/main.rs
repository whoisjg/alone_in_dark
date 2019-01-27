#[macro_use]
extern crate log;
// const screen_width: i32 = 1280;
// const screen_height: i32 = 720;

use std::rc::Rc;

// include!("sprites.rs");

enum GameScreen {
    Title,
    Gameplay,
    Ending,
}

struct Game {
    pub screen_height: i32,
    pub screen_width: i32,
    pub screen: GameScreen,
}

impl Game {
    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        Game {
            screen_width,
            screen_height,
            screen: GameScreen::Title,
        }
    }
}

impl raylib::Game for Game {
    fn start(&mut self, mut ctx: raylib::AppContext) {
        ctx.set_target_fps(60);
    }

    fn update(&mut self, mut ctx: raylib::AppContext) {
        use self::GameScreen::*;
        match self.screen {
            Title => {
                if ctx.is_key_pressed(raylib::input::KEY_ENTER) {
                    self.screen = Gameplay;
                }
            }
            Gameplay => {
                if ctx.is_key_pressed(raylib::input::KEY_ENTER) {
                    self.screen = Ending;
                }
            }
            Ending => {
                if ctx.is_key_pressed(raylib::input::KEY_ENTER) {
                    self.screen = Title;
                }
            }
        }

        ctx.begin_drawing(|ctx| match self.screen {
            Title => {
                ctx.draw_rectangle(
                    0,
                    0,
                    self.screen_width,
                    self.screen_height,
                    &raylib::Color::GREEN,
                );
                ctx.draw_text("Title Screen", 20, 20, 40, &raylib::Color::DARKGREEN);
            }
            Gameplay => {
                ctx.draw_rectangle(
                    0,
                    0,
                    self.screen_width,
                    self.screen_height,
                    &raylib::Color::RED,
                );
                ctx.draw_text("Gameplay Screen", 20, 20, 40, &raylib::Color::MAROON);
            }
            Ending => {
                ctx.draw_rectangle(
                    0,
                    0,
                    self.screen_width,
                    self.screen_height,
                    &raylib::Color::BLUE,
                );
                ctx.draw_text("Ending Screen", 20, 20, 40, &raylib::Color::DARKBLUE);
            }
        });
    }
}

fn main() {
    let game = Box::new(Game::new(1280, 720));
    raylib::run(
        game.screen_width,
        game.screen_height,
        "Dr. Turtle & Mr. GAMERA",
        game,
    );
    // raylib::run(
    //     screen_width,
    //     screen_height,
    //     "Dr. Turtle & Mr. GAMERA",
    //     |ctx| {
    //         // let screen_width = ctx.screen_width;
    //         // let screen_height = ctx.screen_height;
    //         ctx.begin_drawing(|ctx| {
    //             ctx.draw_rectangle(0, 0, screen_width, screen_height, &raylib::Color::GREEN);
    //             ctx.draw_text("Title Screen", 20, 20, 40, &raylib::Color::DARKGREEN);
    //         });
    //     },
    // );
}
