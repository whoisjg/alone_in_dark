use crate::*;

pub fn check_collision_recs(rec1: &Rectangle, rec2: &Rectangle) -> bool {
    unsafe { rl::CheckCollisionRecs(rec1.0, rec2.0) }
}

// bool CheckCollisionCircleRec(Vector2 center, float radius, Rectangle rec);                          // Check collision between circle and rectangle
pub fn check_collision_circle_rec(center: &Vector2, radius: f32, rec: &Rectangle) -> bool {
    unsafe { rl::CheckCollisionCircleRec(center.0, radius, rec.0) }
}

// bool CheckCollisionCircles(Vector2 center1, float radius1, Vector2 center2, float radius2);         // Check collision between two circles
pub fn check_collision_circles(
    center1: &Vector2,
    radius1: f32,
    center2: &Vector2,
    radius2: f32,
) -> bool {
    unsafe { rl::CheckCollisionCircles(center1.0, radius1, center2.0, radius2) }
}
