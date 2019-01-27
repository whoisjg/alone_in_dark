use crate::*;

impl Rectangle {
    pub fn collision_normal_rec(&self, other: &Rectangle) -> Option<Vector2> {
        let dx = (self.x + self.width / 2.0) - (other.x + other.width / 2.0);
        let dy = (self.y + self.height / 2.0) - (other.y + other.height / 2.0);
        let width = (self.width + other.width) / 2.0;
        let height = (self.height + other.height) / 2.0;
        let cross_width = width * dy;
        let cross_height = height * dx;

        if dx.abs() <= width && dy.abs() <= height {
            if (cross_width > cross_height) {
                if cross_width > -1.0 * cross_height {
                    return Some(Vector2::new(0.0, 1.0));
                } else {
                    return Some(Vector2::new(-1.0, 0.0));
                }
            } else {
                if cross_width > -1.0 * cross_height {
                    return Some(Vector2::new(1.0, 0.0));
                } else {
                    return Some(Vector2::new(0.0, -1.0));
                }
            }
        }
        None
    }

    pub fn collision_normal_circle(&self, pos: Vector2, radius: f32) -> Option<Vector2> {
        let nearest_x = self.x.max(pos.x.min(self.x + self.width));
        let nearest_y = self.y.max(pos.y.min(self.y + self.height));

        let dist = pos - Vector2::new(nearest_x, nearest_y);

        // let pen_depth = radius - dist.hypot();
        let pen_vec = -1.0 * dist.normalize();
        Some(pen_vec)
    }
}
