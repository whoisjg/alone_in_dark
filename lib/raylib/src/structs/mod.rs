mod color;
mod image;
mod texture;
mod vec2;
mod rectangle;

pub use self::color::*;
pub use self::image::*;
pub use self::texture::*;
pub use self::vec2::*;
pub use self::rectangle::*;

use std::ops::{Deref, DerefMut};

macro_rules! impl_wrapper {
    ($name:ident, $t:ty, $rawfield:tt) => {
        impl Deref for $name {
            type Target = $t;
            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$rawfield
            }
        }

        impl DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$rawfield
            }
        }
    };
}

macro_rules! make_thin_wrapper {
    ($name:ident, $t:ty) => {
        #[repr(transparent)]
        #[derive(Copy, Clone, Debug)]
        pub struct $name(pub(crate) $t);

        impl_wrapper!($name, $t, 0);
    };
}

make_thin_wrapper!(Vector2, rl::Vector2);
make_thin_wrapper!(Rectangle, rl::Rectangle);

make_thin_wrapper!(Color, rl::Color);

impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Vector2(rl::Vector2 { x, y })
    }
}

impl Rectangle {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rectangle(rl::Rectangle {
            x,
            y,
            width,
            height,
        })
    }
}
