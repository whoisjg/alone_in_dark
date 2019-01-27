use crate::*;

pub use rl::KeyboardKey::Type as KeyboardKey;
pub use rl::KeyboardKey::*;

pub use rl::MouseButton::Type as MouseButton;
pub use rl::MouseButton::*;

impl AppContext {
    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        unsafe { rl::IsKeyPressed(key as i32) }
    }

    pub fn is_key_down(&self, key: KeyboardKey) -> bool {
        unsafe { rl::IsKeyDown(key as i32) }
    }

    pub fn is_key_released(&self, key: KeyboardKey) -> bool {
        unsafe { rl::IsKeyReleased(key as i32) }
    }

    pub fn is_key_up(&self, key: KeyboardKey) -> bool {
        unsafe { rl::IsKeyUp(key as i32) }
    }

    // bool IsMouseButtonPressed(int button);                                  // Detect if a mouse button has been pressed once
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        unsafe { rl::IsMouseButtonPressed(button as i32) }
    }
    // bool IsMouseButtonDown(int button);                                     // Detect if a mouse button is being pressed
    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        unsafe { rl::IsMouseButtonDown(button as i32) }
    }

    // bool IsMouseButtonReleased(int button);                                 // Detect if a mouse button has been released once
    pub fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        unsafe { rl::IsMouseButtonReleased(button as i32) }
    }
    // bool IsMouseButtonUp(int button);                                       // Detect if a mouse button is NOT being pressed
    pub fn is_mouse_button_up(&self, button: MouseButton) -> bool {
        unsafe { rl::IsMouseButtonUp(button as i32) }
    }
    // int GetMouseX(void);                                                    // Returns mouse position X
    pub fn get_mouse_x(&self) -> i32 {
        unsafe { rl::GetMouseX() }
    }
    // int GetMouseY(void);                                                    // Returns mouse position Y
    pub fn get_mouse_y(&self) -> i32 {
        unsafe { rl::GetMouseY() }
    }
    // Vector2 GetMousePosition(void);                                         // Returns mouse position XY
    pub fn get_mouse_position(&self) -> Vector2 {
        let pos = unsafe { rl::GetMousePosition() };
        Vector2(pos)
    }
    // void SetMousePosition(Vector2 position);                                // Set mouse position XY
    // int GetMouseWheelMove(void);
}
