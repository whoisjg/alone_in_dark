use rect_packer::Packer;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_packing() {
        let config = rect_packer::Config {
            width: 1024,
            height: 1024,

            border_padding: 5,
            rectangle_padding: 10,
        };

        let rectangles = [(50, 70), (350, 210), (255, 410)];

        let mut packer = Packer::new(config);
        for &(width, height) in &rectangles {
            if let Some(rect) = packer.pack(width, height, false) {
                println!(
                    "Rectangle is at position ({}, {}) within the encompassing rectangle",
                    rect.x, rect.y
                );
            }
        }
    }
}
