use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub struct SpriteAssetBuilder {
    input_dir: PathBuf,
    output_dir: PathBuf,
    out_file: PathBuf,
    check_rerun: bool,
}

impl SpriteAssetBuilder {
    pub fn new(
        input_dir: impl Into<PathBuf>,
        output_dir: impl Into<PathBuf>,
        out_file: impl Into<PathBuf>,
        rerun_on_change: bool,
    ) -> Self {
        let input_dir = input_dir.into();
        let output_dir = output_dir.into();
        let out_file = out_file.into();
        if !input_dir.exists() {
            panic!("input_dir {} does not exist", input_dir.display());
        }
        if !input_dir.is_dir() {
            panic!("input_dir {} is not a directory", input_dir.display());
        }
        if !output_dir.exists() {
            panic!("output_dir {} does not exist", output_dir.display());
        }
        if !output_dir.is_dir() {
            panic!("output_dir {} is not a directory", output_dir.display());
        }
        if out_file.extension().expect("outfile has no extension") != OsStr::new("rs") {
            panic!(
                "out_file {} is not a rust file: {:?}",
                out_file.display(),
                out_file.extension()
            );
        }
        SpriteAssetBuilder {
            input_dir,
            output_dir,
            out_file,
            check_rerun: rerun_on_change,
        }
    }

    pub fn build(&self) {
        // rebuild if changed for
        rerun_print(self.check_rerun, &self.input_dir);
        rerun_print(self.check_rerun, &self.output_dir);
        let mut images: Vec<(String, PathBuf)> = self
            .input_dir
            .read_dir()
            .expect("failed to read sprite directory")
            .map(|image_path| image_path.expect("failed to get direntry").path())
            .filter(|image_path| {
                image_path.is_file() && image_path.extension() == Some(OsStr::new("png"))
            })
            .map(|image_path| {
                rerun_print(self.check_rerun, &image_path);
                let name = image_path
                    .file_stem()
                    .expect("failed to form get basename of image")
                    .to_str()
                    .expect("failed to get basename of image");
                // TODO check name validity.
                (name.to_owned().to_uppercase(), image_path)
            })
            .collect();
        // TODO form texture atlas and all that good stuff.

        let template: (String, String) =
            images
                .into_iter()
                .fold((String::new(), String::new()), |acc, (name, image_path)| {
                    let out_path = self
                        .input_dir
                        .join(image_path.file_name().expect("could not name output file"));
                    fs::copy(&image_path, &out_path).expect("could not copy file");
                    rerun_print(self.check_rerun, &out_path);
                    // add comment after enum name;
                    let enum_names = format!("{}\n{},", acc.0, name);
                    let path_list = format!(
                        "{} {:?},",
                        acc.1,
                        out_path.to_str().expect("could not print path")
                    );
                    (enum_names, path_list)
                });

        fs::write(
            &self.out_file,
            format!(
                include_str!("templates/sprite_asset.tmpl.rs"),
                "Sprites", template.0, template.1
            ),
        )
        .expect("failed to write out file");
    }
}

fn rerun_print(check_rerun: bool, path: &Path) {
    if check_rerun {
        println!(
            "cargo:rerun-if-changed={}",
            path.to_str()
                .expect("path could not be converted to string")
        );
    }
}
