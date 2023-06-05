use {
    super::*,
    crate::{main_camera::data::MainCamera, physics::data::BoundingBox},
    bevy::{prelude::*, sprite::collide_aabb, window::PrimaryWindow},
};

pub(super) fn highlight_target_on_hover(
    interactable_qry: Query<(&BoundingBox, &Transform), With<Interactable>>,
    cam_qry: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    win_qry: Query<&Window, With<PrimaryWindow>>,
) {
    let (cam, cam_transform) = cam_qry.single();
    let win = win_qry.single();
    let Some(mouse_pos) = win.cursor_position().and_then(|pos| cam.viewport_to_world_2d(cam_transform, pos)) else { return };

    let Some((_interactable_hitbox, interactable_transform)) = interactable_qry
        .iter()
        .find(|(hitbox, transform)| {
            collide_aabb::collide(
                mouse_pos.extend(transform.translation.z),
                Vec2::ONE,
                transform.translation,
                Vec2::new(hitbox.width, hitbox.height),
            )
            .is_some()
        }) else { return };
}
