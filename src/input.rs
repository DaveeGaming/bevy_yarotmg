use bevy::prelude::*;
use crate::player;

#[derive(Resource)]
struct KeyBinds {
    key_up: KeyCode,
    key_right: KeyCode,
    key_left: KeyCode,
    key_down: KeyCode,
    camera_rot_left: KeyCode,
    camera_rot_right: KeyCode,
    camera_zoom_in: KeyCode,
    camera_zoom_out: KeyCode,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource( 
            KeyBinds {
                key_up: KeyCode::KeyW,            
                key_left: KeyCode::KeyA,            
                key_right: KeyCode::KeyS,            
                key_down: KeyCode::KeyD,            
                camera_rot_left: KeyCode::KeyQ,
                camera_rot_right: KeyCode::KeyE,
                camera_zoom_in: KeyCode::KeyO,
                camera_zoom_out: KeyCode::KeyP,
            });
        // app.add_systems(Startup, setup);
        app.add_systems(Update, input_manager);
    }
}


/* fn setup(
    mut commands: Commands,
) {
} */

fn input_manager(
    keybinds: Res<KeyBinds>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut player::Player>,
    mut cam: Query<&mut OrthographicProjection, With<Camera>>
) {
    let mut movement_vec = Vec2::default();
    let mut camera_mov = 0.;
    for key in keyboard.get_pressed() {
        if *key == keybinds.key_up {
           movement_vec.y = 1.; 
        }
        if *key == keybinds.key_left {
           movement_vec.x = -1.; 
        }
        if *key == keybinds.key_right {
           movement_vec.y = -1.; 
        }
        if *key == keybinds.key_down {
           movement_vec.x = 1.; 
        }
        if *key == keybinds.camera_rot_left {
            camera_mov = 1.;
        }
        if *key == keybinds.camera_rot_right {
            camera_mov = -1.;
        }
    }

    for key in keyboard.get_just_pressed() {
        if let Ok(mut cam_p) = cam.get_single_mut() {
            if *key == keybinds.camera_zoom_in {
                cam_p.scale -= 0.05;
                cam_p.scale = (cam_p.scale * 100.).round() / 100.;
                cam_p.scale = cam_p.scale.clamp(0.05, 0.5);
            }
            if *key == keybinds.camera_zoom_out {
                cam_p.scale += 0.05;
                cam_p.scale = (cam_p.scale * 100.).round() / 100.;
                cam_p.scale = cam_p.scale.clamp(0.05, 0.5);
            }
        }
    }

    movement_vec = movement_vec.normalize_or_zero();
    match player.get_single_mut() {
        Ok(mut v) => { 
            v.velocity = movement_vec;
            v.camera_velocity = camera_mov;
        },
        Err(_) => warn_once!("Player component not found in input_manager"),
    }
}
