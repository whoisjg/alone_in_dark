#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
#[repr(u32)]
pub enum Sprites {
    
DOWN_STAND,
RIGHT_WALK1,
RIGHT_WALK2,
LEFT_STAND,
RIGHT_STAND,
DOWN_WALK1,
DOWN_WALK2,
LEFT_WALK2,
LEFT_WALK1,
UP_WALK1,
UP_WALK2,
UP_STAND,
}

impl raylib::Asset for Sprites {
    const ASSETS: &'static [&'static str] = &[ "testing/src/down_stand.png", "testing/src/right_walk1.png", "testing/src/right_walk2.png", "testing/src/left_stand.png", "testing/src/right_stand.png", "testing/src/down_walk1.png", "testing/src/down_walk2.png", "testing/src/left_walk2.png", "testing/src/left_walk1.png", "testing/src/up_walk1.png", "testing/src/up_walk2.png", "testing/src/up_stand.png",];
    
    fn file_for(&self) -> &'static str {
        Self::ASSETS[*self as usize]
    }
}