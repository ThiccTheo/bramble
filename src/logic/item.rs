use bevy::prelude::*;

pub const MAX_ITEM_STACK: usize = 64;

#[derive(Component, Default)]
pub struct Item {
    pub can_stack: bool,
    pub id: ItemType,
}

#[allow(unused)]
#[derive(Default, PartialEq, Eq, Debug)]
pub enum ItemType {
    #[default]
    Null,
    Stick,
    WoodPlank,
    WoodLog,
    WoodPickaxe,
    WoodSword,
    WoodAxe,
    WoodHammer,
}
