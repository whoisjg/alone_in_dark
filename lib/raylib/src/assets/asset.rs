pub trait Asset {
    // The representation for this asset
    type Item;
    const FILE: &'static str;

    fn asset(&self) -> &Self::Item;
}

pub trait DrawRect {}

pub struct Sprite {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl DrawRect for Sprite {}

impl Sprite {
    const fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Sprite { x, y, w, h }
    }
}

pub struct SpriteSheet {
    pub sprites: &'static [Sprite],
}

impl SpriteSheet {}

pub(crate) struct TestSprite;

impl TestSprite {
    const INFO: Sprite = Sprite {
        x: 0,
        y: 0,
        w: 0,
        h: 0,
    };
}

impl Asset for TestSprite {
    type Item = Sprite;
    const FILE: &'static str = "resources/down_stand.png";

    fn asset(&self) -> &Sprite {
        &Self::INFO
    }
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub(crate) enum TestSpriteSheet {
    First,
    Second,
}

impl TestSpriteSheet {
    const SPRITES: SpriteSheet = SpriteSheet {
        sprites: &[Sprite::new(0, 0, 0, 0)],
    };
}

impl Asset for TestSpriteSheet {
    type Item = Sprite;
    const FILE: &'static str = "resources/animation1";

    fn asset(&self) -> &Sprite {
        &Self::SPRITES.sprites[*self as usize]
    }
}
