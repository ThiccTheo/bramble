use {crate::core::game_state::GameState, bevy::prelude::*};

// Subject to change
#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub enum HealthSystem {
    DealDamage,
}

// Subject to change
pub(super) struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>().add_system(
            deal_damage
                .in_set(HealthSystem::DealDamage)
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Health(pub i32);

pub struct DamageEvent {
    pub damage_dealt: i32,
    pub target_id: Entity,
}

fn deal_damage(
    mut cmds: Commands,
    mut dmg_evr: EventReader<DamageEvent>,
    mut hp_qry: Query<&mut Health>,
) {
    for DamageEvent {
        damage_dealt,
        target_id,
    } in dmg_evr.iter()
    {
        let Ok(mut target_hp) = hp_qry.get_mut(*target_id) else { continue };
        target_hp.0 -= *damage_dealt;

        if target_hp.0 <= 0 {
            cmds.entity(*target_id).despawn_recursive();
        }
    }
}
