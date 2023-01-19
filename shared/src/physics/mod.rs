use vek::*;

#[derive(Debug, Default, Clone)]
pub struct Transform {
    pub translation: Vec2<f32>,
    pub rotation: f32,
}