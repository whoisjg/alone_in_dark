
#[derive(Debug, Copy, Clone)]
pub struct DownStand;

impl DownStand {
    const INFO: raylib::Sprite = Sprite::new(5, 5, 17, 31);
}

impl raylib::Asset for DownStand {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct RightWalk1;

impl RightWalk1 {
    const INFO: raylib::Sprite = Sprite::new(27, 5, 15, 30);
}

impl raylib::Asset for RightWalk1 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Shake1;

impl Shake1 {
    const INFO: raylib::Sprite = Sprite::new(47, 5, 17, 31);
}

impl raylib::Asset for Shake1 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Shake3;

impl Shake3 {
    const INFO: raylib::Sprite = Sprite::new(69, 5, 17, 31);
}

impl raylib::Asset for Shake3 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct RightWalk2;

impl RightWalk2 {
    const INFO: raylib::Sprite = Sprite::new(91, 5, 15, 30);
}

impl raylib::Asset for RightWalk2 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Shake2;

impl Shake2 {
    const INFO: raylib::Sprite = Sprite::new(111, 5, 17, 31);
}

impl raylib::Asset for Shake2 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct LeftStand;

impl LeftStand {
    const INFO: raylib::Sprite = Sprite::new(133, 5, 15, 31);
}

impl raylib::Asset for LeftStand {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct RightStand;

impl RightStand {
    const INFO: raylib::Sprite = Sprite::new(153, 5, 15, 31);
}

impl raylib::Asset for RightStand {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct DownWalk1;

impl DownWalk1 {
    const INFO: raylib::Sprite = Sprite::new(173, 5, 16, 30);
}

impl raylib::Asset for DownWalk1 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct DownWalk2;

impl DownWalk2 {
    const INFO: raylib::Sprite = Sprite::new(194, 5, 16, 30);
}

impl raylib::Asset for DownWalk2 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct LeftWalk2;

impl LeftWalk2 {
    const INFO: raylib::Sprite = Sprite::new(215, 5, 15, 30);
}

impl raylib::Asset for LeftWalk2 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Pose1;

impl Pose1 {
    const INFO: raylib::Sprite = Sprite::new(173, 40, 22, 30);
}

impl raylib::Asset for Pose1 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct LeftWalk1;

impl LeftWalk1 {
    const INFO: raylib::Sprite = Sprite::new(235, 5, 15, 30);
}

impl raylib::Asset for LeftWalk1 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Pose3;

impl Pose3 {
    const INFO: raylib::Sprite = Sprite::new(200, 40, 22, 30);
}

impl raylib::Asset for Pose3 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Pose2;

impl Pose2 {
    const INFO: raylib::Sprite = Sprite::new(227, 40, 22, 30);
}

impl raylib::Asset for Pose2 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct UpWalk1;

impl UpWalk1 {
    const INFO: raylib::Sprite = Sprite::new(27, 40, 15, 30);
}

impl raylib::Asset for UpWalk1 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Surprise;

impl Surprise {
    const INFO: raylib::Sprite = Sprite::new(91, 41, 17, 31);
}

impl raylib::Asset for Surprise {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Nod2;

impl Nod2 {
    const INFO: raylib::Sprite = Sprite::new(5, 41, 17, 31);
}

impl raylib::Asset for Nod2 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Nod3;

impl Nod3 {
    const INFO: raylib::Sprite = Sprite::new(47, 41, 17, 31);
}

impl raylib::Asset for Nod3 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct UpWalk2;

impl UpWalk2 {
    const INFO: raylib::Sprite = Sprite::new(69, 41, 15, 30);
}

impl raylib::Asset for UpWalk2 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Nod1;

impl Nod1 {
    const INFO: raylib::Sprite = Sprite::new(113, 41, 17, 31);
}

impl raylib::Asset for Nod1 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct UpStand;

impl UpStand {
    const INFO: raylib::Sprite = Sprite::new(135, 41, 17, 31);
}

impl raylib::Asset for UpStand {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Laugh3;

impl Laugh3 {
    const INFO: raylib::Sprite = Sprite::new(157, 75, 17, 31);
}

impl raylib::Asset for Laugh3 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Laugh2;

impl Laugh2 {
    const INFO: raylib::Sprite = Sprite::new(179, 75, 17, 31);
}

impl raylib::Asset for Laugh2 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Laugh1;

impl Laugh1 {
    const INFO: raylib::Sprite = Sprite::new(201, 75, 17, 31);
}

impl raylib::Asset for Laugh1 {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        &Self::INFO
    }
}
#[derive(Debug, Copy, Clone)]
pub enum Atlas {
    
DownStand(DownStand),

RightWalk1(RightWalk1),

Shake1(Shake1),

Shake3(Shake3),

RightWalk2(RightWalk2),

Shake2(Shake2),

LeftStand(LeftStand),

RightStand(RightStand),

DownWalk1(DownWalk1),

DownWalk2(DownWalk2),

LeftWalk2(LeftWalk2),

Pose1(Pose1),

LeftWalk1(LeftWalk1),

Pose3(Pose3),

Pose2(Pose2),

UpWalk1(UpWalk1),

Surprise(Surprise),

Nod2(Nod2),

Nod3(Nod3),

UpWalk2(UpWalk2),

Nod1(Nod1),

UpStand(UpStand),

Laugh3(Laugh3),

Laugh2(Laugh2),

Laugh1(Laugh1),

}

impl Atlas {
    const INFO: SPRITE = Sprite::new(0, 0, 250, 106);
}

impl raylib::Asset for Atlas {
    type Item = raylib::Sprite;
    const FILE: &'static str = "resources/out/atlas.png";
    fn file() -> &'static str {
        Self::FILE
    }

    fn asset(&self) -> &raylib::Sprite {
        match &self {
            
DownStand(s) => s.asset(),

RightWalk1(s) => s.asset(),

Shake1(s) => s.asset(),

Shake3(s) => s.asset(),

RightWalk2(s) => s.asset(),

Shake2(s) => s.asset(),

LeftStand(s) => s.asset(),

RightStand(s) => s.asset(),

DownWalk1(s) => s.asset(),

DownWalk2(s) => s.asset(),

LeftWalk2(s) => s.asset(),

Pose1(s) => s.asset(),

LeftWalk1(s) => s.asset(),

Pose3(s) => s.asset(),

Pose2(s) => s.asset(),

UpWalk1(s) => s.asset(),

Surprise(s) => s.asset(),

Nod2(s) => s.asset(),

Nod3(s) => s.asset(),

UpWalk2(s) => s.asset(),

Nod1(s) => s.asset(),

UpStand(s) => s.asset(),

Laugh3(s) => s.asset(),

Laugh2(s) => s.asset(),

Laugh1(s) => s.asset(),
   
        }
    }
}