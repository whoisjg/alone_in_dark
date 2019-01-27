use crate::*;
use std::path::PathBuf;

const BORDER_PADDING: i32 = 0;
const RECTANGLE_PADDING: i32 = 0;

/// SpriteCollection collects files, in various AssetFolders, and
/// loads them into memory
#[derive(Debug, Clone)]
pub struct SpriteCollection {
    pub name: String,
    pub path: PathBuf,
    pub sprites: Vec<SpriteImage>,
}

#[derive(Debug, Clone)]
pub struct SpriteImage {
    pub name: String,
    pub path: PathBuf,
    pub image: image::RgbaImage,
}

impl SpriteImage {
    pub fn new(name: String, path: impl Into<PathBuf>, image: image::RgbaImage) -> Self {
        SpriteImage {
            name,
            path: path.into(),
            image,
        }
    }
}

impl SpriteCollection {
    pub fn load(folder: AssetFolder) -> Result<Self, String> {
        let name = folder
            .folder
            .file_stem()
            .ok_or("has no stem")
            .map(|s| s.to_str().ok_or(format!("has no stem bad stem")))?;
        let name = name?.to_owned();

        let images: Result<Vec<SpriteImage>, String> = folder
            .files
            .iter()
            .map(|entry| {
                let i = image::open(entry.path())
                    .map(|i| i.to_rgba())
                    .map_err(|e| format!("can't open image{}", e));
                let i = i?;

                let name = entry
                    .path()
                    .file_stem()
                    .map(|n| n.to_str().unwrap().to_owned())
                    .ok_or(format!("file has no stem {}", entry.path().display()))?;

                Ok(SpriteImage::new(name, entry.path(), i))
            })
            .collect();
        let images = images?;

        Ok(SpriteCollection {
            name,
            path: folder.folder,
            sprites: images,
        })
    }

    pub fn pack(&self) -> SpriteAtlas {
        let rects: Vec<_> = self.sprites.iter().map(|i| image_rect(&i.image)).collect();

        let packing = optimal_packing(&rects, BORDER_PADDING, RECTANGLE_PADDING);

        let (max_width, max_height) = packing.iter().fold((1, 1), |(w, h), r| {
            (w.max(r.x + r.width), h.max(r.y + r.height))
        });

        let mut atlas_image = image::RgbaImage::new(max_width as u32, max_height as u32);

        let sprites: Vec<_> = self
            .sprites
            .iter()
            .zip(packing.iter())
            .map(|(s, r)| {
                for (x, y, p) in s.image.enumerate_pixels() {
                    atlas_image.put_pixel(r.x as u32 + x, r.y as u32 + y, *p);
                }

                SpriteInfo::from_rect(s.name.clone(), s.path.clone(), r)
            })
            .collect();

        SpriteAtlas::new(self.name.clone(), sprites, atlas_image)
    }
}

/// SpriteCollection collects files, in various AssetFolders, and
/// loads them into memory
#[derive(Debug, Clone)]
pub struct SpriteAtlas {
    pub name: String,
    pub sprites: Vec<SpriteInfo>,
    pub image: image::RgbaImage,
}

#[derive(Debug, Clone)]
pub struct SpriteInfo {
    pub name: String,
    /// original path of the sprite
    pub path: PathBuf,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl SpriteInfo {
    fn from_rect(name: String, path: PathBuf, rect: &rect_packer::Rect) -> Self {
        SpriteInfo {
            name,
            path,
            x: rect.x as u32,
            y: rect.y as u32,
            w: rect.width as u32,
            h: rect.width as u32,
        }
    }
}

impl SpriteAtlas {
    pub fn new(name: String, sprites: Vec<SpriteInfo>, image: image::RgbaImage) -> Self {
        SpriteAtlas {
            name,
            sprites,
            image,
        }
    }
    pub fn merge(self, name: String, other: SpriteAtlas) -> Self {
        let rects = vec![image_rect(&self.image), image_rect(&other.image)];

        let packing = optimal_packing(&rects, BORDER_PADDING, RECTANGLE_PADDING);

        let (max_width, max_height) = packing.iter().fold((1, 1), |(w, h), r| {
            (w.max(r.x + r.width), h.max(r.y + r.height))
        });

        let mut atlas_image = image::RgbaImage::new(max_width as u32, max_height as u32);

        let all_sprites: Vec<_> = vec![self, other];

        // move the sprites to new position based on packing
        let sprites_fixed: Vec<_> = all_sprites
            .into_iter()
            .zip(packing.into_iter())
            .map(|(s, r)| {
                for (x, y, p) in s.image.enumerate_pixels() {
                    atlas_image.put_pixel(r.x as u32 + x, r.y as u32 + y, *p);
                }

                s.sprites
                    .into_iter()
                    .map(|mut s| {
                        s.x += r.x as u32;
                        s.y += r.y as u32;
                        s
                    })
                    .collect()
            })
            .flat_map(|s: Vec<SpriteInfo>| s)
            .collect();

        SpriteAtlas::new(name, sprites_fixed, atlas_image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_atlas_gen() {
        let cols: Result<Vec<_>, _> = DirSearch::new("resources/animals", AssetType::Sprites, true)
            .expect("can't create dir search")
            .search()
            .expect("dir search faield")
            .into_iter()
            .map(|f| SpriteCollection::load(f))
            .collect();

        let mut first: Vec<_> = cols
            .expect("failed to process all dirs")
            .into_iter()
            .map(|c| c.pack())
            .collect();

        let rest = first.split_off(1);

        let first = first.pop().unwrap();

        let atlas = rest
            .into_iter()
            .fold(first, |acc, a| acc.merge("end".to_owned(), a));

        atlas
            .image
            .save("resources/out/atlas.png")
            .expect("can't save file");

        // std::fs::write("resources/out/atlas.rs", atlas.template());
    }
}
