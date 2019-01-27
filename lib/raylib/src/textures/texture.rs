use crate::{Asset, Texture2D};

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ffi::CString;
use std::sync::Mutex;

lazy_static! {
    static ref TEXTURE_MAP: Mutex<HashMap<String, TextureMapEntry>> = Mutex::new(HashMap::new());
}

struct TextureMapEntry {
    file: std::ffi::CString,
    count: u32,
    texture: Texture2D,
}

impl TextureMapEntry {
    pub(crate) fn decrement(&mut self) {
        self.count -= 1;
    }

    pub(crate) fn increment(&mut self) {
        self.count += 1;
    }

    pub(crate) fn texture(&self) -> Texture2D {
        self.texture.clone()
    }
}

pub struct Texture2DHandle {
    file: &'static str,
    texture: Texture2D,
}

impl Texture2DHandle {
    pub(crate) fn new(f: &'static str, t: Texture2D) -> Self {
        Texture2DHandle {
            file: f,
            texture: t,
        }
    }
}

impl Drop for Texture2DHandle {
    fn drop(&mut self) {
        TEXTURE_MAP
            .lock()
            .unwrap()
            .get_mut(self.file)
            .unwrap()
            .decrement();
    }
}

pub fn load_texture<A>() -> Texture2DHandle
where
    A: Asset,
{
    let f = A::FILE;

    match TEXTURE_MAP.lock().unwrap().entry(f.to_string()) {
        Entry::Occupied(mut e) => {
            e.get_mut().increment();
            return Texture2DHandle::new(f, e.get().texture.clone());
        }
        Entry::Vacant(e) => {
            let file = CString::new(f).unwrap();
            // info!("Loading texture {}", f);
            let tex = unsafe { rl::LoadTexture(file.as_ptr()) };
            let handle = Texture2DHandle::new(f, Texture2D(tex));
            e.insert(TextureMapEntry {
                file: file,
                count: 1,
                texture: handle.texture.clone(),
            });
            handle
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestSprite;
    #[test]
    fn can_load() {
        let handle = load_texture::<TestSprite>();
        println!("{:?}", handle.file);
    }
}
