use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use crate::{
    entity::EntityRotate, 
    health::Health,
    projectile::{ProjectileAsset, ProjectileTargetingType},
    projectilepattern::{CirclePattern, IPPattern},
    weapon::Weapon,
};

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

    // All set by the input system
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

/// This is where we currently spawn the camera, player, 
/// and the other health entities
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
        RigidBody::KinematicPositionBased,
        Collider::cuboid(2., 2.)
    )).id();

    commands.entity(player_id).add_child(cam_id);
    commands.spawn( ( 
        SpriteBundle { texture: char.clone(), ..default()},
        Health::default(),
        EntityRotate,
        RigidBody::KinematicPositionBased,
        Collider::cuboid(5., 5.)
    ));
    commands.spawn( ( 
        SpriteBundle { 
            texture: char,
            transform: Transform::from_translation(Vec3::new(20., 20., 0.)),
            ..default()},
        Health::default(),
        EntityRotate
    ));
}


fn update_weapon(
    time: Res<Time>,
    commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query< (&Camera, &GlobalTransform)>,
    projectile_asset: Res<ProjectileAsset>,
    mut player: Query<(&mut Player, &Transform)>,
) {
    let p = player.get_single_mut();

    // Errors out if we have zero, or multiple player components
    if p.is_err() {
        warn_once!("Unable to find player in update_weapon");
        return;
    }

    let (mut player, transform) = p.unwrap();

    // The reason I implemented weapon this way, 
    // is i want the weapon cooldown to decrement
    // constantly, even when we aren't shooting
    if player.weapon.is_some() {
        // cache out the firing, because we request weapon as mutable later
        let firing = player.firing;
        let time = time.delta().as_secs_f32();
        let wp = player.weapon.as_mut().unwrap();

        wp.increment_attack_timer( time );

        // * firing - updated from attack system
        if firing && wp.can_attack() {
            // Main window, we only have a single one
            let window = window.single(); 
            let (camera, cam_transform) = camera.single();

            // Turn our cursor position into a point in the world
            let mouse_world = window.cursor_position()
            .and_then( |pos| camera.viewport_to_world_2d(cam_transform, pos ));

            // mouse_world is none if our mouse is outside the window
            if mouse_world.is_some() {
                // Get vector pointing in the direction of the 
                // mouse from the player
                let dir = (mouse_world.unwrap() - transform.translation.xy())
                    .normalize_or_zero();
                
                let mut pattern = CirclePattern {
                    amount: 10,
                    dir,
                    targeting: ProjectileTargetingType::PLAYER,
                    ..default()
                };
                let handle = projectile_asset.handle.clone();
                pattern.spawn(commands, transform, handle);
            }
        }
    }

}

fn update_player_transform(
    mut player: Query<(&mut Transform, &Player)>,
) {
    // Errors if we have zero or multiple players
    match player.get_single_mut() {
        Ok( (mut transform, player) ) => {
            // As we are rotating the player, 
            // our "UP" direction changes, we should reflect that in our movement
            let movement = 
                (Vec2::splat(player.movement_speed) * player.velocity)
                    .extend(0.);

            let rotation = transform.rotation;
            transform.translation += Quat::mul_vec3(rotation, movement); 
        },
        Err(_) => warn_once!("No Player found for update_player_transform"),
    }
}

fn update_player_camera(
    mut player: Query<(&mut Transform, &Player)>,
) {

    // Errors if we have zero or multiple players
    match player.get_single_mut() {
        Ok( (mut transform, player) ) => {
            transform.rotate_z( player.camera_velocity * player.camera_rot_speed );
        },
        Err(_) => warn_once!("No Player found for update_player_transform"),
    }
}