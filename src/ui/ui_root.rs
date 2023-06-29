use {crate::states::game_state::GameState, bevy::prelude::*};

pub(super) struct UiRootPlugin;

impl Plugin for UiRootPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ui_root.in_schedule(OnEnter(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct UiRoot;

fn spawn_ui_root(mut cmds: Commands) {
    cmds.spawn(NodeBundle::default());
}
