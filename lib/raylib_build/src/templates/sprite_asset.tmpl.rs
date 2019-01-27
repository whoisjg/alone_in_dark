#[derive(Debug, Copy, Clone)]
pub struct {0};

impl {0} {{
    const INFO: raylib::Sprite = {2};
}}

impl raylib::Asset for {0} {{
    type Item = raylib::Sprite;
    const FILE: &'static str = "{1}";
    fn file() -> &'static str {{
        Self::FILE
    }}

    fn asset(&self) -> &raylib::Sprite {{
        &Self::INFO
    }}
}}