use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Item {
    pub can_stack: bool,
    pub id: ItemType,
}

#[derive(Default, PartialEq, Eq)]
pub enum ItemType {
    #[default]
    Null,
}
