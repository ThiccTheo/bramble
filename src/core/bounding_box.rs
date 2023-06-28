use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct BoundingBox {
    pub width: f32,
    pub height: f32,
}

impl BoundingBox {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl From<Vec2> for BoundingBox {
    fn from(value: Vec2) -> Self {
        Self {
            width: value.x,
            height: value.y,
        }
    }
}

impl From<BoundingBox> for Vec2 {
    fn from(value: BoundingBox) -> Self {
        Self {
            x: value.width,
            y: value.height,
        }
    }
}
