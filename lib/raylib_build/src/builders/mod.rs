mod packer;
mod sprite_atlas;
mod util;

pub use self::packer::*;
pub use self::sprite_atlas::*;
pub use self::util::*;

use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum AssetType {
    Sprites,
}

impl AssetType {
    pub fn extensions(&self) -> Vec<OsString> {
        match self {
            AssetType::Sprites => vec![OsString::from("png")],
        }
    }
}

#[derive(Debug)]
pub struct AssetFolder {
    pub folder: PathBuf,
    pub files: Vec<fs::DirEntry>,

    pub kind: AssetType,
}

impl AssetFolder {
    pub fn new(folder: impl Into<PathBuf>, files: Vec<fs::DirEntry>, kind: AssetType) -> Self {
        Self {
            folder: folder.into(),
            files,
            kind,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirSearch {
    pub search_dir: PathBuf,
    pub recurse: bool,
    pub kind: AssetType,
}

impl DirSearch {
    // create a new directory searcher
    pub fn new(
        search_dir: impl Into<PathBuf>,
        kind: AssetType,
        recurse: bool,
    ) -> Result<Self, String> {
        let search_dir = search_dir.into();
        validate(&search_dir, true, None)?;
        Ok(DirSearch {
            search_dir,
            kind,
            recurse,
        })
    }

    pub fn search(&self) -> Result<Vec<AssetFolder>, String> {
        // Grab all files in this folder.
        let mut my_files = Vec::new();

        // make sure we can read the entries.
        let entries = self
            .search_dir
            .read_dir()
            .map_err(|e| format!("can't read search_dir {}", e))?;

        // exit if we can't read everything
        let entries: Result<Vec<_>, _> = entries.map(|entry| entry).collect();
        let entries = entries.map_err(|e| format!("not all files readable {}", e))?;

        // if we are willing to recurse check sub folders
        let searchers: Result<Vec<_>, _> = entries
            .into_iter()
            .filter_map(|entry| {
                if entry.path().is_file() {
                    my_files.push(entry);
                    return None;
                } else if self.recurse {
                    return Some(DirSearch::new(entry.path(), self.kind, self.recurse));
                }
                None
            })
            .collect();
        let searchers = searchers?;

        // for each asset_folder, run search and collect the results
        let sub_folders: Result<Vec<_>, _> = searchers
            .into_iter()
            .map(|builder| builder.search())
            .collect();
        let sub_folders = sub_folders?;
        let mut sub_folders: Vec<_> = sub_folders.into_iter().flat_map(|f| f).collect();

        // Now process our own files.
        let exts = self.kind.extensions();
        let filtered_files: Vec<_> = my_files
            .into_iter()
            .filter(|f| f.path().extension().is_some())
            .filter(|f| exts.iter().any(|e| f.path().extension() == Some(e)))
            .collect();

        if filtered_files.len() > 0 {
            sub_folders.push(AssetFolder::new(
                self.search_dir.clone(),
                filtered_files,
                self.kind,
            ));
        }

        Ok(sub_folders)
    }
}
