pub const SCREEN_WIDTH: i32 = 1280;
pub const SCREEN_HEIGHT: i32 = 720;

pub const FPS: u32 = 60;

// KEEP DIVISIBLE BY 9 FOR EAZY CALLCULATIONS
pub const VERTICAL_TILES_PER_WINDOW: i32 = 63;

pub const DIALOG_HEIGHT: i32 = 150;
pub const PLAY_WIDTH: i32 = SCREEN_WIDTH;
pub const PLAY_HEIGHT: i32 = SCREEN_HEIGHT - DIALOG_HEIGHT;
pub const PLAY_ASPECT_RATIO: f32 = PLAY_WIDTH as f32 / PLAY_HEIGHT as f32;
pub const SCREEN_ASPECT_RATIO: f32 = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;

pub const PPT: i32 = PLAY_HEIGHT / VERTICAL_TILES_PER_WINDOW;

pub const PLAYER_BULLET_SPEED: f32 = 2.5;
pub const PLAYER_FIRE_RATE: f32 = 20.0;
pub const LIGHT_FALLOFF: f32 = 5.0;
