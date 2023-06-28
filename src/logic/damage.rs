use bevy::prelude::*;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageDealtEvent>();
    }
}

#[derive(Component)]
pub struct Damage(pub i32);

pub struct DamageDealtEvent {
    pub damage_dealt: i32,
    pub target_id: Entity,
}
