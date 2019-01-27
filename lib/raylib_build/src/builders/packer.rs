use image::{self, RgbaImage};
use rect_packer::{self, Rect};

/// convert image to a rectangle at 0, 0
pub fn image_rect(i: &RgbaImage) -> Rect {
    Rect::new(0, 0, i.width() as i32, i.height() as i32)
}

pub fn optimal_packing(
    rects: &Vec<Rect>,
    border_padding: impl Into<Option<i32>>,
    rectangle_padding: impl Into<Option<i32>>,
) -> Vec<Rect> {
    let border_padding = border_padding.into().unwrap_or(0);
    let rectangle_padding = rectangle_padding.into().unwrap_or(0);

    let max_dim = rects
        .iter()
        .fold(0, |acc, r| acc.max(r.width).max(r.height));

    // starting values
    let start_width = (0..32)
        .find(|i| 1 << i > max_dim)
        .expect(&format!("images width to big {}", max_dim));
    // find a length wise rectangle that fits all the images
    // in a row or two
    let start_height = (0..32)
        .skip_while(|i| 1 << i < max_dim)
        // find the first rectangle that works.
        .find(|h| {
            let conf = rect_packer::Config {
                width: 1 << start_width,
                height: 1 << h,
                border_padding,
                rectangle_padding,
            };
            let mut packer = rect_packer::Packer::new(conf);

            rects
                .iter()
                .all(|r| packer.pack(r.width, r.height, false).is_some())
        })
        .expect("impossible to pack. Too many images to pack");

    // we now have a length wise rectangle.
    // make it more and more square until it is a perfect square.
    // this reduces the overal perimater but keeps the area the same
    // so things still work.

    let mut max_height = start_height;
    let mut max_width = start_width;

    // println!("\nestimate {} {}\n", max_width, max_height);

    // find the first square that fails to pack.
    ((start_width + 1)..start_height).find(|w| {
        // or terminate if a perfect square
        if *w >= max_height {
            return true;
        }

        let conf = rect_packer::Config {
            height: 1 << (start_height - (w - start_width)),
            width: 1 << w,
            border_padding,
            rectangle_padding,
        };

        let mut packer = rect_packer::Packer::new(conf);

        let can_fit = rects
            .iter()
            .all(|r| packer.pack(r.width, r.height, false).is_some());

        if can_fit {
            // update estimates
            max_height = start_height - (w - start_width);
            max_width = *w;
            false
        } else {
            true
        }
    });

    // println!("\nestimate {} {}\n", max_width, max_height);

    // actually pack the rects
    let conf = rect_packer::Config {
        width: 1 << max_width,
        height: 1 << max_height,

        border_padding,
        rectangle_padding,
    };

    let mut packer = rect_packer::Packer::new(conf);
    rects
        .into_iter()
        .map(|r| packer.pack(r.width, r.height, false).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_packing() {
        let rects = vec![Rect::new(0, 0, 20, 10), Rect::new(0, 0, 10, 20)];
        let total: Vec<_> = rects.into_iter().cycle().take(10000).collect();
        optimal_packing(&total, 0, 0);
    }
}
