#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
#[repr(u32)]
pub enum Sprites {
    
TURTLE,
TITLE,
GFRAME,
KOMIKA,
GAMERA,
SHARK,
ORCA,
SEA,
SKY,
MOUNTAINS,
SWHALE,
FISH,
}

impl raylib::Asset for Sprites {
    const ASSETS: &'static [&'static str] = &[ "resources/sprites/turtle.png", "resources/sprites/title.png", "resources/sprites/gframe.png", "resources/sprites/komika.png", "resources/sprites/gamera.png", "resources/sprites/shark.png", "resources/sprites/orca.png", "resources/sprites/sea.png", "resources/sprites/sky.png", "resources/sprites/mountains.png", "resources/sprites/swhale.png", "resources/sprites/fish.png",];
    
    fn file_for(&self) -> &'static str {
        Self::ASSETS[*self as usize]
    }
}