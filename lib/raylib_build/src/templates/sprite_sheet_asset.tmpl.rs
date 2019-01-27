#[derive(Debug, Copy, Clone)]
pub enum {0} {{
    {2}
}}

impl {0} {{
    const INFO: SPRITE = {3};
}}

impl raylib::Asset for {0} {{
    type Item = raylib::Sprite;
    const FILE: &'static str = "{1}";
    fn file() -> &'static str {{
        Self::FILE
    }}

    fn asset(&self) -> &raylib::Sprite {{
        match &self {{
            {4}   
        }}
    }}
}}