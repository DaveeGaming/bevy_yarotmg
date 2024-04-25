use bevy::prelude::*;
use crate::player;


/// Defined here to be able to change keybinds in runtime
/// TODO: Make keybinds be able to set to any mouse or gamepad button, we could make a struct that stores all 3 for every key

pub struct Key {
    pub active: bool,
    pub key: KeyType,
}

pub enum KeyType {
    Keyboard(KeyCode),
    Mouse(MouseButton),
    Gamepad(GamepadButton)
}

pub struct InputSystem<'a> {
    keyboard: Res<'a, ButtonInput<KeyCode>>,
    mouse: Res<'a, ButtonInput<MouseButton>>,
    gamepad: Res<'a, ButtonInput<GamepadButton>>,
}

impl Key {
    pub fn keyboard(key: KeyCode) -> Self {
        Key { active: false, key: KeyType::Keyboard(key) }
    }
    pub fn mouse(key: MouseButton) -> Self {
        Key { active: false, key: KeyType::Mouse(key) }
    }
    pub fn gamepad(key: GamepadButton) -> Self {
        Key { active: false, key: KeyType::Gamepad(key) }
    }

    pub fn pressed(&mut self,sys: &InputSystem) -> bool {
        match self.key {
            KeyType::Keyboard( key ) => { 
                let i = sys.keyboard.pressed( key );
                self.active = i;
                return i;
            },
            KeyType::Mouse( key ) => { 
                let i = sys.mouse.pressed( key );
                self.active = i;
                return i;
            },
            KeyType::Gamepad( key ) => { 
                let i = sys.gamepad.pressed( key );
                self.active = i;
                return i;
            },
        }
    }

    pub fn just_pressed(&mut self,sys: &InputSystem) -> bool {
        match self.key {
            KeyType::Keyboard( key ) => { 
                let i = sys.keyboard.just_pressed( key );
                self.active = i;
                return i;
            },
            KeyType::Mouse( key ) => { 
                let i = sys.mouse.just_pressed( key );
                self.active = i;
                return i;
            },
            KeyType::Gamepad( key ) => { 
                let i = sys.gamepad.just_pressed( key );
                self.active = i;
                return i;
            },
        }
    }
}


#[derive(Resource)]
pub struct Keybinds {
    pub weapon_fire: Key,

    pub key_up: Key,
    pub key_right: Key,
    pub key_left: Key,
    pub key_down: Key,

    pub camera_rot_left: Key,
    pub camera_rot_right: Key,
    pub camera_zoom_in: Key,
    pub camera_zoom_out: Key,
    pub camera_reset: Key,
}


pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource( 
            Keybinds {
                weapon_fire: Key::mouse(MouseButton::Left),

                key_up: Key::keyboard(KeyCode::KeyW),            
                key_left: Key::keyboard(KeyCode::KeyA),            
                key_right: Key::keyboard(KeyCode::KeyS),            
                key_down: Key::keyboard(KeyCode::KeyD),            

                camera_rot_left: Key::keyboard(KeyCode::KeyQ),
                camera_rot_right: Key::keyboard(KeyCode::KeyE),
                camera_zoom_in: Key::keyboard(KeyCode::KeyO),
                camera_zoom_out: Key::keyboard(KeyCode::KeyP),
                camera_reset: Key::keyboard(KeyCode::KeyR),
            });
        // app.add_systems(Startup, setup);
        app.add_systems(Update, input_manager );
    }
}

/// The big reason for having this input manager separately, is so we can save our keypresses \
/// to variables, and update our player and camera in a FixedUpdate loop 
/// 
/// TODO: If this runs before a fixed update, it could eat an input, check the system run order between fixedupdate and normal update
fn input_manager(
    mut keybinds: ResMut<Keybinds>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    gamepad: Res<ButtonInput<GamepadButton>>,
) {
    let system = InputSystem {
        keyboard: keyboard,
        mouse: mouse,
        gamepad: gamepad,
    };

    keybinds.weapon_fire.pressed( &system );

    keybinds.key_up.pressed( &system );
    keybinds.key_down.pressed( &system );
    keybinds.key_left.pressed( &system );
    keybinds.key_right.pressed( &system );

    keybinds.camera_rot_left.pressed( &system );
    keybinds.camera_rot_right.pressed( &system );


    keybinds.camera_reset.just_pressed( &system );
    keybinds.camera_zoom_in.just_pressed( &system );
    keybinds.camera_zoom_out.just_pressed( &system );

    // We can manipulate the camera zoom outside the fixed loop because we 
    // increment by set numbers, and only on keyinputs
}
