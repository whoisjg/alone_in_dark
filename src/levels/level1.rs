use crate::components::*;
use crate::constants::*;
use specs::prelude::*;
use specs::world::EntitiesRes;

type L1SD<'a> = <Level1 as System<'a>>::SystemData;

pub struct Level1 {
    pub phase: Level1Phase,
    pub done: bool,
    pub time: f32,
}

impl Level1 {
    pub fn new() -> Level1 {
        Level1 {
            phase: Level1Phase::Setup,
            done: false,
            time: 0.0,
        }
    }
}

#[derive(Debug)]
pub enum Level1Phase {
    Setup,
    IntroSetup(IntroSetup),
    Intro,
}

#[derive(Debug)]
pub enum IntroSetup {
    Start,
    WaitForTutDialog,
    SpawnEnemy,
    WaitToKillEnemy,
    PlayerDead,
}

impl<'a> System<'a> for Level1 {
    type SystemData = (
        WriteExpect<'a, raylib::AppContext>,
        Entities<'a>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Shape>,
        WriteStorage<'a, Dialog>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        match &self.phase {
            Level1Phase::Setup => {
                info!("Setup");
                self.setup_level(data);
                self.done = false;
                self.phase = Level1Phase::IntroSetup(IntroSetup::Start);
            }
            Level1Phase::IntroSetup(_) => {
                // info!("[{}]Intro Setup: {:?}", data.0.frame, self.phase);
                self.start_intro(data);
                if self.done {
                    self.phase = Level1Phase::Intro;
                }
            }
            Intro => self.run_intro(),
        }
    }
}

impl<'a> Level1 {
    pub fn setup_level(&mut self, (mut ctx, ents, mut players, mut shapes, _, updater): L1SD) {
        build_level(LEVEL_1_LAYOUT, ents, updater);
    }
    pub fn start_intro(&mut self, (mut ctx, ents, mut player, mut shapes, dialogs, updater): L1SD) {
        // Add dialog to player
        if let Level1Phase::IntroSetup(setup) = &self.phase {
            match setup {
                IntroSetup::Start => {
                    let (ent, _, _) = (&ents, &player, !&dialogs).join().next().unwrap();
                    let say: DialogCharSeq =
                        ("Where am I? It's so dark. I should speak with ENTER. I should move with WASD. And Shoot my way out of here with mouse 1. In that order.")
                            .into();
                    updater.insert(
                        ent,
                        Dialog::new(DialogThumbnail::new(DialogImage::Player), say.0),
                    );
                    self.phase = Level1Phase::IntroSetup(IntroSetup::WaitForTutDialog);
                }
                IntroSetup::WaitForTutDialog => {
                    // wait for done

                    match (&ents, &player, &dialogs).join().next() {
                        None => {
                            // wait 5 seconds to spawn the enemy
                            // self.time += ctx.get_frame_time();
                            // if self.time > 1.0 {
                            self.phase = Level1Phase::IntroSetup(IntroSetup::SpawnEnemy);
                            // }
                        }
                        _ => (),
                    }
                }
                IntroSetup::SpawnEnemy => {
                    // spawn enemy
                    let enemy = ents.create();
                    basic_enemy(20.0, 50.0, &enemy, &updater);
                    self.phase = Level1Phase::IntroSetup(IntroSetup::WaitToKillEnemy);
                }
                IntroSetup::WaitToKillEnemy => {
                    let (ent, player) = (&ents, &player).join().next().unwrap();
                    if player.should_die {
                        let say: DialogCharSeq = ("I'm hit. I guess I'll die now.").into();
                        updater.insert(
                            ent,
                            Dialog::new(DialogThumbnail::new(DialogImage::Player), say.0),
                        );
                        self.phase = Level1Phase::IntroSetup(IntroSetup::PlayerDead);
                    }
                }
                IntroSetup::PlayerDead => match (&ents, &player, &dialogs).join().next() {
                    None => {
                        ctx.should_quit = true;
                    }
                    _ => (),
                },
            }
        }
    }
    pub fn run_intro(&mut self) {}
}

// KEEP 16/9 aspect ratio
const LEVEL_1_LAYOUT: &'static str = r#"/--------------\
|              |
|         -    |
|         |    |
|    X    |    |
|         -    |
|              |
|              |
\--------------/
"#;

pub fn build_level<'a>(level: &str, ents: Read<'a, EntitiesRes>, updater: Read<'a, LazyUpdate>) {
    let room_height = VERTICAL_TILES_PER_WINDOW as f32;
    let room_width = room_height * SCREEN_ASPECT_RATIO;
    let wall_color = raylib::Color::WHITE;

    let (level_width, level_height) =
        level
            .lines()
            .enumerate()
            .fold((0, 0), |(lw, lh), (i, line)| {
                if i > 0 && lw != line.len() {
                    panic!(
                        "line {} has a different length from the others. Expected {}",
                        i, lw
                    );
                }

                (line.len(), i + 1)
            });

    let level_width_f = level_width as f32;
    let level_height_f = level_height as f32;

    let block_at_i = level_width_f / room_width;
    let block_at_j = level_height_f / room_height;

    let block_w = room_width / level_width_f;
    let block_h = room_height / level_height_f;

    let level: Vec<_> = level
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    // info!(
    //     "build level room: {:?}, level: {:?}, scaling: {:?}, inverse: {:?}",
    //     (room_width, room_height),
    //     (level_width, level_height),
    //     (block_w, block_h),
    //     (block_at_i, block_at_j),
    // );

    let mut made_player = false;

    // Go to string and build level
    for i in (0..room_width as i32) {
        for j in (0..room_height as i32) {
            let x = i as f32;
            let y = j as f32;
            let x_lerp = x * block_at_i;
            let y_lerp = y * block_at_j;
            let col = (x_lerp) as usize;
            let row = (y_lerp) as usize;

            let c = level[row][col];
            match c {
                ' ' => (),
                '|' => {
                    // vertical walls do not tile horizontally
                    if f32_about_eq(x_lerp.round(), x_lerp, block_at_i) {
                        // info!(
                        //     "found vertical tile at {:?}, {:?}, {:?},",
                        //     (i, j),
                        //     (x_lerp, y_lerp),
                        //     (col, row),
                        // );
                        // place wall here

                        let w = ents.create();
                        basic_wall(x, y, &w, &updater);
                    }
                }
                '-' => {
                    // horizontal walls do not tile vertically
                    if f32_about_eq(y_lerp.round(), y_lerp, block_at_i) {
                        // info!(
                        //     "found horizontal tile at {:?}, {:?}, {:?},",
                        //     (i, j),
                        //     (x_lerp, y_lerp),
                        //     (col, row),
                        // );
                        // place wall here

                        let w = ents.create();
                        basic_wall(x, y, &w, &updater);
                    }
                }
                'X' => {
                    // place player exactly. floor don't round
                    if !made_player {
                        let p = ents.create();
                        basic_player(x, y, &p, &updater);
                        made_player = true;
                    }
                }
                '/' | '\\' => {
                    // found corner tile
                    // tiles vertically and horizontally
                    if f32_about_eq(y_lerp.round(), y_lerp, block_at_i)
                        || f32_about_eq(x_lerp.round(), x_lerp, block_at_j)
                    {
                        // info!(
                        //     "found corner tile at {:?}, {:?}, {:?},",
                        //     (i, j),
                        //     (x_lerp, y_lerp),
                        //     (col, row),
                        // );
                        // place wall here
                        let w = ents.create();
                        basic_wall(x, y, &w, &updater);
                    }
                }
                _ => panic!("un recognized char {}", c),
            }
        }
    }
}

pub fn f32_about_eq(x: f32, y: f32, e: f32) -> bool {
    (x - y).abs() < e
}
