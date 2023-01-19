/// Component that gives entities a visual appearance.
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum Sprite {
    Rect,
    Circle,
}