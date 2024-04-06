use bevy::prelude::*;
use crate::entity::EntityRotate;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, update_player_transform);
        app.add_systems(FixedUpdate, update_player_camera);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer> 
) {    
    let char = asset_server.load("character.png");


    let mut cam = Camera2dBundle::default();
    cam.projection.scale = 0.2;

    let cam_id = commands.spawn( cam ).id();
    let player_id =  commands.spawn( 
    (
        SpriteBundle {
        texture: char.clone(),
        ..default()},
        Player::default() 
    )).id();

    commands.entity(player_id).add_child(cam_id);
    commands.spawn( ( 
        SpriteBundle { texture: char, ..default()},
        EntityRotate
    ));
}

#[derive(Component)]
pub struct Player {
    pub velocity: Vec2,
    pub movement_speed: f32,

    pub camera_velocity: f32,
    pub camera_rot_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            velocity: Default::default(),
            movement_speed: 2.,

            camera_velocity: 0.,
            camera_rot_speed: 0.1,
        }
    }
}

fn update_player_transform(
    mut player: Query<(&mut Transform, &Player)>,
) {
    match player.get_single_mut() {
        Ok( (mut transform, player) ) => {
            // As we are rotating the player, our "UP" direction changes, we should reflect that in our movement
            let movement = (Vec2::splat(player.movement_speed) * player.velocity ).extend(0.);
            let rotation = transform.rotation;
            transform.translation += Quat::mul_vec3(rotation, movement); 
        },
        Err(_) => warn_once!("No Player found for update_player_transform"),
    }
}

fn update_player_camera(
    mut player: Query< (&Player, &mut Transform )>,
) {

    if let Ok(  (player, mut p_trans) ) = player.get_single_mut()  {
        p_trans.rotate_z( player.camera_velocity * player.camera_rot_speed );

    } else {
        warn_once!("Player component not found in player_camera");
    }
}