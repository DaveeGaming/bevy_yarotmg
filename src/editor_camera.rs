use bevy::prelude::*;
use bevy_rapier2d::na::clamp;

use crate::{input::Keybinds, states::AppSet};

#[derive(Component)]
pub struct MainCamera;

pub struct EditorCameraPlugin;

impl Plugin for EditorCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_camera.in_set(AppSet::Editor) );
    }
}


fn move_camera(
    time: Res<Time>,
    input: Res<Keybinds>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>
) {
    let cam_t = camera.get_single_mut();
    if cam_t.is_err() { return; }
    let (mut cam_t, mut ortho) = cam_t.unwrap();


    ortho.scale -= 0.05 * input.scroll_wheel;
    ortho.scale = ortho.scale.clamp(0.05, 0.5);

    let mut editor_cam_speed = 100.;
    if input.modifier_key.active {
        editor_cam_speed = editor_cam_speed * 3.;
    }
    let mut veloc = Vec2::default();
    
    if input.key_up.active { veloc.y = 1.; }
    if input.key_down.active { veloc.y = -1.; }
    if input.key_left.active { veloc.x = -1.; }
    if input.key_right.active { veloc.x = 1.; }

    veloc = veloc.normalize_or_zero();
    let distance = Vec2::splat( editor_cam_speed * time.delta_seconds() ) * veloc;
    cam_t.translation += distance.extend(0.);

}