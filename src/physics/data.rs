use bevy::prelude::*;

#[derive(Component)]
pub struct BoundingBox {
    pub width: f32,
    pub height: f32,
}

impl BoundingBox {
    pub fn new(w: f32, h: f32) -> Self {
        Self {
            width: w,
            height: h,
        }
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
