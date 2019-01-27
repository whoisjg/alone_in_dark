use inflector::Inflector;

use std::ffi::{OsStr, OsString};
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
pub enum AssetType {
    Sprites,
}

impl AssetType {
    pub fn allowed_extensions(&self) -> Vec<OsString> {
        vec![OsString::from("png")]
    }
}

#[derive(Debug, Clone)]
pub struct AssetBuilder {
    search_dir: PathBuf,
    kind: AssetType,
    watch: bool,
}

impl AssetBuilder {
    pub fn new(
        search_dir: impl Into<PathBuf>,
        kind: AssetType,
        watch: bool,
    ) -> Result<Self, String> {
        let search_dir = search_dir.into();
        validate(&search_dir, true, None)?;
        Ok(AssetBuilder {
            search_dir,
            kind,
            watch,
        })
    }

    // takes the asset builder and tries to find files it can use to build stuff with.
    // if it comes across sub folders, it recurses on them.
    pub fn check(&self) -> Result<Vec<AssetFolder>, String> {
        // files in this folder. needs to be filtered
        let mut my_files = Vec::new();
        // grabs files in this directory and subfolders recursively
        let entries = self
            .search_dir
            .read_dir()
            .map_err(|e| format!("can't read search_dir {}", e))?;

        let entries: Result<Vec<_>, _> = entries.map(|entry| entry).collect();

        let entries = entries.map_err(|e| format!("not all files readable {}", e))?;

        let builders: Result<Vec<_>, _> = entries
            .into_iter()
            .filter_map(|entry| {
                if entry.path().is_file() {
                    my_files.push(entry);
                    return None;
                } else {
                    Some(AssetBuilder::new(entry.path(), self.kind, self.watch))
                }
            })
            .collect();

        let builders = builders?;

        let sub_folders: Result<Vec<_>, _> = builders
            .into_iter()
            .map(|builder| builder.check())
            .collect();

        let sub_folders = sub_folders?;

        let mut sub_folders: Vec<_> = sub_folders.into_iter().flat_map(|f| f).collect();

        // Show the folders
        let allowed_extensions = self.kind.allowed_extensions();
        let filtered_files: Vec<DirEntry> = my_files
            .into_iter()
            .filter(|f| f.path().extension().is_some())
            .filter(|f| {
                allowed_extensions
                    .iter()
                    .any(|e| f.path().extension().unwrap() == e)
            })
            .collect();

        if filtered_files.len() > 0 {
            let folder = AssetFolder::new(
                self.search_dir.clone(),
                self.kind,
                self.watch,
                filtered_files,
            );

            sub_folders.push(folder);
        }
        Ok(sub_folders)
    }
}

#[derive(Debug)]
pub struct AssetFolder {
    pub folder: PathBuf,
    pub kind: AssetType,
    pub watch: bool,
    pub files: Vec<DirEntry>,

    pub output_file: Option<PathBuf>,
    pub trim: bool,
}

#[derive(Debug)]
pub struct SpriteInfo {
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl SpriteInfo {
    pub fn from_rect(rect: rect_packer::Rect, name: String) -> SpriteInfo {
        SpriteInfo {
            name,
            x: rect.x as u32,
            y: rect.y as u32,
            w: rect.width as u32,
            h: rect.height as u32,
        }
    }
}

#[derive(Debug)]
pub struct Atlas {
    pub out_file: PathBuf,
    pub image: image::RgbaImage,
    pub sprites: Vec<SpriteInfo>,
}

impl Atlas {
    pub fn save_image(&self, p: impl AsRef<Path>) -> Result<(), std::io::Error> {
        self.image.save(p)
    }

    pub fn template(&self) -> String {
        let sprites_templates = self
            .sprites
            .iter()
            .map(|s| {
                format!(
                    include_str!("templates/sprite_asset.tmpl.rs"),
                    s.name.to_pascal_case(),
                    self.out_file.display(),
                    format!("Sprite::new({}, {}, {}, {})", s.x, s.y, s.w, s.h)
                )
            })
            .fold(String::new(), |acc, s| format!("{}\n{}", acc, s));

        let enum_dec = self
            .sprites
            .iter()
            .map(|s| {
                format!(
                    "{}({}),\n",
                    s.name.to_pascal_case(),
                    s.name.to_pascal_case()
                )
            })
            .fold(String::new(), |acc, s| format!("{}\n{}", acc, s));

        let enum_match = self
            .sprites
            .iter()
            .map(|s| format!("{}(s) => s.asset(),\n", s.name.to_pascal_case(),))
            .fold(String::new(), |acc, s| format!("{}\n{}", acc, s));

        let atlas_name = self
            .out_file
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_pascal_case();
        let file_name = self.out_file.display();
        let atlas_sprite_info = format!(
            "Sprite::new({}, {}, {}, {})",
            0,
            0,
            self.image.width(),
            self.image.height()
        );

        let atlas_template = format!(
            include_str!("templates/sprite_sheet_asset.tmpl.rs"),
            atlas_name, file_name, enum_dec, atlas_sprite_info, enum_match,
        );

        format!("{}\n{}", sprites_templates, atlas_template)
    }
}

impl AssetFolder {
    pub fn new(
        folder: impl Into<PathBuf>,
        kind: AssetType,
        watch: bool,
        files: Vec<DirEntry>,
    ) -> Self {
        AssetFolder {
            folder: folder.into(),
            kind,
            watch,
            files,

            output_file: None,
            trim: false,
        }
    }

    pub fn collect(&self) -> Result<Atlas, String> {
        // get the images
        let out_file = self.output_file.as_ref().ok_or("no outdir set")?;
        watch_print(self.watch, &self.folder);

        let entries = self
            .folder
            .read_dir()
            .map_err(|e| format!("failed to read dir {}", e));
        let entries = entries?;

        let entries: Result<Vec<_>, _> = entries.collect();
        let entries = entries.map_err(|e| format!("can't read dir {}", e))?;
        let paths: Vec<_> = entries.into_iter().map(|e| e.path()).collect();

        let allowed_extensions = self.kind.allowed_extensions();

        let images: Result<Vec<(String, image::RgbaImage)>, String> = paths
            .iter()
            .filter(|p| {
                p.is_file()
                    && allowed_extensions
                        .iter()
                        .any(|ext| ext == p.extension().unwrap())
            })
            .map(|p| {
                let image = image::open(p)
                    .map(|i| i.to_rgba())
                    .map_err(|e| format!("can't open image {}", e));
                let image = image?;

                let name = p
                    .file_stem()
                    .map(|n| n.to_str().unwrap().to_owned())
                    .ok_or(format!("file has no stem {}", p.display()))?;
                Ok((name, image))
            })
            .collect();
        let images = images?;

        // get the max dimension of the image
        let max_dim = images
            .iter()
            .fold(0, |acc, (_, i)| acc.max(i.width()).max(i.height()));

        let border_padding = 5;
        let rectangle_padding = 5;

        // pack rectangles with a decent width
        let start_height = (0..32)
            .find(|i| 1 << i > 2 * max_dim as i32)
            .expect("dim too big");
        let pow_2_width = (0..32)
            .skip_while(|i| 1 << i < max_dim as i32)
            .find(|w| {
                let conf = rect_packer::Config {
                    width: 1 << w,
                    height: 1 << start_height,
                    border_padding,
                    rectangle_padding,
                };

                let mut packer = rect_packer::Packer::new(conf);

                images.iter().all(|(_, ref i)| {
                    packer
                        .pack(i.width() as i32, i.height() as i32, false)
                        .is_some()
                })
            })
            .expect("cannot fit all these images man!");

        // pack rectangles height wise
        let mut max_height = start_height;
        let mut max_width = pow_2_width;

        println!("start, {} {}", max_width, max_height);

        let _pow_2_height = ((start_height + 1)..pow_2_width).find(|h| {
            if *h >= max_width {
                return true;
            }
            let conf = rect_packer::Config {
                width: 1 << (pow_2_width - (h - start_height)),
                height: 1 << h,
                border_padding,
                rectangle_padding,
            };

            let mut packer = rect_packer::Packer::new(conf);

            let can_fit = images.iter().all(|(_, ref i)| {
                packer
                    .pack(i.width() as i32, i.height() as i32, false)
                    .is_some()
            });

            if can_fit {
                max_width = pow_2_width - (h - start_height);
                max_height = *h;
                false
            } else {
                true
            }
        });

        println!("end, {} {}", max_width, max_height);

        let max_width = 1 << max_width;
        let max_height = 1 << max_height;

        let conf = rect_packer::Config {
            width: max_width as i32,
            height: max_height as i32,

            border_padding,
            rectangle_padding,
        };

        let mut packer = rect_packer::Packer::new(conf);
        let rects: Result<Vec<_>, _> = images
            .iter()
            .map(|(_, ref i)| {
                packer
                    .pack(i.width() as i32, i.height() as i32, false)
                    .ok_or(format!("can't pack"))
            })
            .collect();

        // recalculate max width and height
        let rects = rects?;
        let (max_width, max_height) = rects.iter().fold((0, 0), |(mw, mh), r| {
            let w = (r.x + r.width) as u32;
            let h = (r.y + r.height) as u32;
            (mw.max(w), mh.max(h))
        });

        let mut atlas_image = image::RgbaImage::new(max_width.max(1), max_height.max(1));

        let sprites: Vec<_> = images
            .into_iter()
            .enumerate()
            .map(|(i, (name, image))| {
                let rect = rects[i];

                for (x, y, p) in image.enumerate_pixels() {
                    atlas_image.put_pixel(rect.x as u32 + x, rect.y as u32 + y, *p);
                }

                SpriteInfo::from_rect(rect, name)
            })
            .collect();

        Ok(Atlas {
            out_file: PathBuf::from(out_file),
            image: atlas_image,
            sprites,
        })
    }
}

struct AssetWriter {
    pub out_dir: PathBuf,
    pub folders: Vec<AssetFolder>,
}

impl AssetWriter {
    pub fn new(out_dir: impl Into<PathBuf>, folders: Vec<AssetFolder>) -> Self {
        AssetWriter {
            out_dir: out_dir.into(),
            folders,
        }
    }

    pub fn write() -> Result<String, String> {
        Ok("".to_owned())
    }
}

fn trim(image: &image::RgbaImage) -> (u32, u32, u32, u32) {
    let rows = 0..image.height();
    let cols = 0..image.width();
    let row_has_pixel = |&row: &u32| cols.clone().any(|col| image.get_pixel(col, row)[3] != 0);
    let col_has_pixel = |&col: &u32| rows.clone().any(|row| image.get_pixel(col, row)[3] != 0);

    let top = rows
        .clone()
        .find(&row_has_pixel)
        .expect("image contains no pixels with non-zero alpha");
    let left = cols.clone().find(&col_has_pixel).unwrap();
    let bottom = rows.clone().rev().find(&row_has_pixel).unwrap() + 1;
    let right = cols.clone().rev().find(&col_has_pixel).unwrap() + 1;

    (top, left, right - left, bottom - top)
}

fn validate<'a>(
    path: &PathBuf,
    is_dir: bool,
    extensions: impl Into<Option<&'a Vec<OsString>>>,
) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("{} does not exist", path.display()));
    }
    Ok(())
}

fn watch_print(watch: bool, path: &Path) {
    if watch {
        println!(
            "cargo:rerun-if-changed={}",
            path.to_str()
                .expect("path could not be converted to string")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_enumerate() {
        let folders = AssetBuilder::new("resources/animals", AssetType::Sprites, false)
            .unwrap()
            .check()
            .unwrap();
        println!("Assets: {:?}.", folders);
    }

    #[test]
    fn can_write() {
        let mut folders = AssetBuilder::new("resources/chara2_1", AssetType::Sprites, false)
            .unwrap()
            .check()
            .unwrap();
        assert_eq!(folders.len(), 1);
        let mut f = &mut folders[0];
        f.output_file = Some("resources/out/atlas.png".into());

        let atlas = f.collect().unwrap();
        // println!("Asset: {:?}.", atlas);
        println!("w, h: ({}, {})", atlas.image.width(), atlas.image.height());
        atlas.image.save("resources/out/atlas.png");
        std::fs::write("resources/out/atlas.rs", atlas.template());
        // println!("{:?}", atlas.template());
    }
}
