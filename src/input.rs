use bevy::prelude::*;
use crate::player;


/// Defined here to be able to change keybinds in runtime
/// TODO: Make keybinds be able to set to any mouse or gamepad button, we could make a struct that stores all 3 for every key
#[derive(Resource)]
struct KeyBinds {
    weapon_fire: KeyCode,

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
                weapon_fire: KeyCode::Space,

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

/// The big reason for having this input manager separately, is so we can save our keypresses \
/// to variables, and update our player and camera in a FixedUpdate loop 
/// 
/// TODO: If this runs before a fixed update, it could eat an input, check the system run order between fixedupdate and normal update
fn input_manager(
    keybinds: Res<KeyBinds>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut player::Player>,
    mut cam: Query<&mut OrthographicProjection, With<Camera>>
) {

    // Errors if we have zero or multiple player components
    let player = player.get_single_mut();
    if player.is_err() {
        warn_once!("Player component not found in input_manager");
        return;
    }
    let mut player = player.unwrap();

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
        // We can manipulate the camera zoom outside the fixed loop because we 
        // increment by set numbers, and only on keyinputs
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

    // Update player component with input values
    player.firing = keyboard.pressed(keybinds.weapon_fire);
    player.velocity = movement_vec;
    player.camera_velocity = camera_mov;
}
