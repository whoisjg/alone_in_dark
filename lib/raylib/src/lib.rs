#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

mod app;
mod assets;
mod core;
mod structs;
mod textures;
mod utils;

pub use self::app::*;
pub use self::assets::*;
pub use self::core::*;
pub use self::structs::*;
pub use self::textures::*;
pub use self::utils::*;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {}

}
