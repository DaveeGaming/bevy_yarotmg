use bevy::{prelude::*, window::PrimaryWindow};
use crate::{entity::EntityRotate, health::Health, projectile::{PState, Projectile, ProjectileAsset}, weapon::Weapon};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, update_player_transform);
        app.add_systems(FixedUpdate, update_player_camera);
        app.add_systems(FixedUpdate, update_weapon);
    }
}


#[derive(Component)]
pub struct Player {
    pub weapon: Option<Weapon>,
    pub firing: bool,

    pub velocity: Vec2,
    pub movement_speed: f32,

    pub camera_velocity: f32,
    pub camera_rot_speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            weapon: Some(Weapon::default()),
            firing: false,

            velocity: Default::default(),
            movement_speed: 2.,

            camera_velocity: 0.,
            camera_rot_speed: 0.1,
        }
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
        Player::default(),
        Health::default(),
    )).id();

    commands.entity(player_id).add_child(cam_id);
    commands.spawn( ( 
        SpriteBundle { texture: char.clone(), ..default()},
        Health::default(),
        EntityRotate
    ));
    commands.spawn( ( 
        SpriteBundle { texture: char, transform: Transform::from_translation(Vec3::new(20., 20., 0.)), ..default()},
        Health::default(),
        EntityRotate
    ));
}


fn update_weapon(
    time: Res<Time>,
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query< (&Camera, &GlobalTransform)>,
    projectile_asset: Res<ProjectileAsset>,
    mut player: Query<(&mut Player, &Transform)>,
) {
    let p = player.get_single_mut();

    if p.is_err() {
        warn_once!("Unable to find player in update_weapon");
        return;
    }

    let (mut player, transform) = p.unwrap();
    
    if player.weapon.is_some() {
        let firing = player.firing;
        let time = time.delta().as_secs_f32();
        let wp = player.weapon.as_mut().unwrap();
        wp.update_attack( time );
        if firing && wp.attack() {
            let window = window.single();
            let (camera, cam_transform) = camera.single();

            let mouse_world = window.cursor_position()
            .and_then( |pos| camera.viewport_to_world_2d(cam_transform, pos ));

            if mouse_world.is_some() {
                let dir = (mouse_world.unwrap() - transform.translation.xy()).normalize_or_zero();
                let dir_rot = Quat::from_rotation_arc(Vec3::Y, dir.extend(0.));
                commands.spawn( (
                    SpriteBundle {
                        transform: transform.with_rotation(dir_rot),
                        texture: projectile_asset.handle.clone(),
                        ..default()
                    },
                    Projectile::from_states( wp.damage,
                         Vec::from( [
                            PState{speed: Some(10.), angular_velocity: Some(90.), duration: 1.},
                            PState{speed: Some(10.), angular_velocity: Some(0.), duration: 1.},
                        ]), true)
                ));
            }
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