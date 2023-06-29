use {crate::states::game_state::GameState, bevy::prelude::*};

pub(super) struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(remove_dead_entities.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Health(pub i32);

fn remove_dead_entities(mut cmds: Commands, hp_qry: Query<(Entity, &Health)>) {
    for id in hp_qry
        .iter()
        .filter(|(_, hp)| !hp.0.is_positive())
        .map(|(id, _)| id)
    {
        cmds.entity(id).despawn_recursive();
    }
}
