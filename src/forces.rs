use bevy::prelude::*;

pub struct ForcesPlugin;

impl Plugin for ForcesPlugin {
    fn build(&self, app: &mut App) {}
}

fn apply_forces(mut physics_qry: Query<()>) {}
