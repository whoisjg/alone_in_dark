use specs::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Shape {
    pub pos: raylib::Vector2,
    pub shape_type: ShapeType,
    pub color: raylib::Color,
}

#[derive(Component, Clone, Debug)]
pub struct Collision {
    pub a_shape: Shape,
    pub b_shape: Shape,
}

impl Collision {
    pub fn new(a_shape: Shape, b_shape: Shape) -> Self {
        Collision { a_shape, b_shape }
    }
}

impl Shape {
    pub const fn new(pos: raylib::Vector2, shape_type: ShapeType, color: raylib::Color) -> Self {
        Shape {
            pos,
            shape_type,
            color,
        }
    }

    // for broad_phase_collisiosn
    pub fn bounding_radius(&self) -> f32 {
        match self.shape_type {
            ShapeType::Circle(Circle { radius }) => radius,
            ShapeType::Rect(Rect { width, height }) => width.max(height),
        }
    }

    pub fn to_rect(
        &self,
        new_dim: impl Into<Option<(f32, f32)>>,
        new_color: impl Into<Option<raylib::Color>>,
    ) -> Self {
        let new_dim = new_dim.into();
        let color = match new_color.into() {
            Some(c) => c,
            None => self.color,
        };

        match self.shape_type {
            ShapeType::Rect(_) => {
                let st = match new_dim {
                    Some((w, h)) => ShapeType::Rect(Rect::new(w, h)),
                    None => self.shape_type.clone(),
                };

                return Shape::new(self.pos, st, color);
            }
            ShapeType::Circle(Circle { radius }) => {
                let (pos, st) = match new_dim {
                    Some((w, h)) => (self.pos, ShapeType::Rect(Rect::new(w, h))),
                    None => (
                        self.pos - raylib::Vector2::new(-radius, -radius),
                        ShapeType::Rect(Rect::new(2.0 * radius, 2.0 * radius)),
                    ),
                };

                return Shape::new(pos, st, color);
            }
        }
    }

    pub fn to_circle(
        &self,
        new_radius: impl Into<Option<f32>>,
        new_color: impl Into<Option<raylib::Color>>,
    ) -> Self {
        let new_radius = new_radius.into();
        let color = match new_color.into() {
            Some(c) => c,
            None => self.color,
        };

        match self.shape_type {
            ShapeType::Circle(_) => {
                let st = match new_radius {
                    Some(r) => ShapeType::Circle(Circle::new(r)),
                    None => self.shape_type.clone(),
                };

                return Shape::new(self.pos, st, color);
            }
            ShapeType::Rect(Rect { width, height }) => {
                let pos = self.pos + raylib::Vector2::new(width / 2.0, height / 2.0);
                let st = match new_radius {
                    Some(r) => ShapeType::Circle(Circle::new(r)),
                    None => ShapeType::Circle(Circle::new(width.max(height))),
                };

                return Shape::new(self.pos, st, color);
            }
        }
    }

    pub fn center(&self) -> raylib::Vector2 {
        match self.shape_type {
            ShapeType::Circle(_) => self.pos,
            ShapeType::Rect(Rect { width, height }) => {
                self.pos + raylib::Vector2::new(width / 2.0, height / 2.0)
            }
        }
    }

    pub fn bounding_box(&self) -> raylib::Rectangle {
        match self.shape_type {
            ShapeType::Rect(Rect { width, height }) => {
                raylib::Rectangle::new(self.pos.x, self.pos.y, width, height)
            }
            ShapeType::Circle(Circle { radius }) => raylib::Rectangle::new(
                self.pos.x - radius,
                self.pos.y - radius,
                2.0 * radius,
                2.0 * radius,
            ),
        }
    }

    pub fn collides_with(&self, other: &Shape) -> bool {
        match self.shape_type {
            ShapeType::Rect(ref a) => match other.shape_type {
                ShapeType::Circle(Circle { radius: br }) => {
                    let arec = a.to_rect(self.pos);
                    return raylib::check_collision_circle_rec(&other.pos, br, &arec);
                }
                ShapeType::Rect(ref b) => {
                    let arec = a.to_rect(self.pos);
                    let brec = b.to_rect(other.pos);
                    return raylib::check_collision_recs(&arec, &brec);
                }
            },
            ShapeType::Circle(Circle { radius: ar }) => match other.shape_type {
                ShapeType::Circle(Circle { radius: br }) => {
                    return raylib::check_collision_circles(&self.pos, ar, &other.pos, br);
                }
                ShapeType::Rect(ref b) => {
                    let brec = b.to_rect(other.pos);
                    return raylib::check_collision_circle_rec(&self.pos, ar, &brec);
                }
            },
        }
    }

    pub fn collision_normal(&self, other: &Shape) -> Option<raylib::Vector2> {
        match self.shape_type {
            ShapeType::Rect(ref a) => match other.shape_type {
                ShapeType::Rect(ref b) => {
                    let arec = a.to_rect(self.pos);
                    let brec = b.to_rect(other.pos);

                    return arec.collision_normal_rec(&brec);
                }
                ShapeType::Circle(Circle { radius }) => {
                    let arec = a.to_rect(self.pos);
                    return arec.collision_normal_circle(other.pos, radius);
                }
            },
            ShapeType::Circle(Circle { radius }) => match other.shape_type {
                ShapeType::Rect(ref b) => {
                    let brec = b.to_rect(other.pos);
                    return Some(-1.0 * brec.collision_normal_circle(self.pos, radius).unwrap());
                }
                _ => None,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum ShapeType {
    Rect(Rect),
    Circle(Circle),
}

#[derive(Clone, Debug, Default)]
pub struct Rect {
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(width: f32, height: f32) -> Self {
        return Rect { width, height };
    }
    pub fn to_rect(&self, pos: raylib::Vector2) -> raylib::Rectangle {
        raylib::Rectangle::new(pos.x, pos.y, self.width, self.height)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Circle { radius }
    }
}
