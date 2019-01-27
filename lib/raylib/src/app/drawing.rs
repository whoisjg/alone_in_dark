use crate::AppContext;
use crate::Color;
use crate::*;

use std::ffi::CString;

pub struct DrawContext {
    pub app_context: AppContext,
}

impl DrawContext {
    pub fn clear_background(&mut self, color: &Color) {
        unsafe {
            rl::ClearBackground(color.0);
        }
    }

    pub fn draw_text(&mut self, text: &str, x: i32, y: i32, width: i32, color: &Color) {
        unsafe {
            rl::DrawText(CString::new(text).unwrap().as_ptr(), x, y, width, color.0);
        }
    }

    pub fn draw_rectangle(&mut self, x: i32, y: i32, width: i32, height: i32, color: &Color) {
        unsafe {
            rl::DrawRectangle(x, y, width, height, color.0);
        }
    }

    // void DrawRectangleLinesEx(Rectangle rec, int lineThick, Color color);                               // Draw rectangle outline with extended parameters
    pub fn draw_rectangle_lines_ex(&mut self, rec: &Rectangle, line_thick: i32, color: &Color) {
        unsafe {
            rl::DrawRectangleLinesEx(rec.0, line_thick, color.0);
        }
    }

    //  void DrawCircle(int centerX, int centerY, float radius, Color color);                               // Draw a color-filled circle
    pub fn draw_circle(&mut self, centerX: i32, centerY: i32, radius: f32, color: &Color) {
        unsafe {
            rl::DrawCircle(centerX, centerY, radius, color.0);
        }
    }

    // void DrawRectangleV(Vector2 position, Vector2 size, Color color);                                   // Draw a color-filled rectangle (Vector version)
    pub fn draw_circle_v(&mut self, position: &Vector2, radius: f32, color: &Color) {
        unsafe {
            rl::DrawCircleV(position.0, radius, color.0);
        }
    }
}

impl Drop for DrawContext {
    fn drop(&mut self) {
        unsafe {
            rl::EndDrawing();
        }
    }
}

impl AppContext {
    pub fn draw<F>(self, mut draw: F)
    where
        F: FnMut(DrawContext),
    {
        unsafe {
            rl::BeginDrawing();
        }
        let ctx = DrawContext { app_context: self };
        (draw)(ctx)
    }
}
