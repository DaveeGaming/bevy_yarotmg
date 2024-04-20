use std::borrow::BorrowMut;

use bevy::{math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume}, prelude::*, render::render_resource::encase::rts_array::Length};
use bevy_rapier2d::{geometry::Collider, pipeline::{CollisionEvent, QueryFilter}, plugin::RapierContext, rapier::geometry::CollisionEventFlags};
use crate::{health::{self, Health}, player::Player};

pub struct ProjectilePlugin;

#[derive(Clone, Copy)]
pub struct PState {
    pub angular_velocity: Option<f32>,
    pub speed: Option<f32>,
    pub duration: f32,
}

#[derive(Clone, Copy)]
pub enum ProjectileTargetingType {
    /// Damages enemies
    PLAYER,
    /// Damages player
    ENEMY,
    /// Damages both player and enemies
    ENVIRONMENT
}


#[derive(Component)]
pub struct Projectile {
    pub damage: i32,
    pub targeting_type: ProjectileTargetingType,
    pub angular_velocity: f32,
    pub speed: f32,

    pub state_current: usize,
    pub state_duration: f32,
    pub states: Option<Vec<PState>>,
    pub state_repeat: bool,
    
}


impl Default for Projectile {
    fn default() -> Self {
        Projectile {
            damage: 1,
            angular_velocity: 0.,
            speed: 15.,
            targeting_type: ProjectileTargetingType::ENVIRONMENT,

            state_current: 0,
            states: None,
            state_duration: 0., 
            state_repeat: false,
        }
    }
}

impl Projectile {
    pub fn new(damage: i32, angular_velocity: f32, speed: f32, targeting_type: ProjectileTargetingType) -> Projectile{
        Projectile {
            damage,
            angular_velocity,
            speed,
            targeting_type,

            state_current: 0,
            states: None,
            state_duration: 0., 
            state_repeat: false,
        }
    }

    pub fn from_states(damage: i32, targeting_type: ProjectileTargetingType, states: Vec<PState>, pattern_repeat: bool) -> Projectile {
        let first_pattern = states[0];
        Projectile {
            damage,
            angular_velocity: first_pattern.angular_velocity.unwrap_or_default(),
            speed: first_pattern.speed.unwrap_or_default(),
            targeting_type,

            state_current: 0,
            states: Some(states.clone()),
            state_duration: first_pattern.duration,
            state_repeat: pattern_repeat
        }
    }

}

/// Stores the handle given by asset_server, so we dont load the image in for every projectile
#[derive(Resource)]
pub struct ProjectileAsset {
    pub handle: Handle<Image>,
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, update_states);
        app.add_systems(FixedUpdate, update_projectile_position.after( update_states ) );
        app.add_systems(PostUpdate, 
            (
                player_projectile_detection,
                enemy_projectile_detection
            ) 
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(
        ProjectileAsset {
            handle: asset_server.load("thing.png"),
    })
}


/// Decrements state_duration, and if it reaches zero,  
/// sets the projectile values to the values specified in the next state
fn update_states (
    time: Res<Time>,
    mut projectiles: Query<&mut Projectile>,
) {

    for mut p in projectiles.iter_mut() {

        if p.states.is_some() {
            p.state_duration -= time.delta().as_secs_f32();
            if p.state_duration < 0. {
                let p_length = p.states.as_ref().unwrap().length();
                if p.state_current + 1 < p_length {

                    p.state_current += 1;
                    let next_state =  p.states.as_ref().unwrap()[p.state_current];

                    // Update projectile variables
                    p.state_duration = next_state.duration;
                    p.speed = next_state.speed.unwrap_or( p.speed );
                    p.angular_velocity = next_state.angular_velocity.unwrap_or( p.angular_velocity );

                } else if p.state_repeat && p.state_current + 1 == p_length {

                    p.state_current = 0; 
                    let next_state =  p.states.as_ref().unwrap()[p.state_current];

                    // Update projectile variables
                    p.state_duration = next_state.duration;
                    p.speed = next_state.speed.unwrap_or( p.speed );
                    p.angular_velocity = next_state.angular_velocity.unwrap_or( p.angular_velocity );
                }
            } 
        }
    }
}

/// Updates the transform of every projectile, by what their Projectile struct defines
fn update_projectile_position(
    time: Res<Time>,
    mut projectiles: Query<(&mut Transform, &Projectile)>
) {
    let time = time.delta().as_secs_f32();
    let fm = 60.; 
    let one_frame_offset = 1. / ((fm + 1.) / fm); //FIXME: i hate this shit, i want to kms
    for (mut t, p) in projectiles.iter_mut() { 
        let veloc = p.speed * time * one_frame_offset;
        let rot = t.rotation;
        // Quat::mul_vec3 multiplies the vector by a rotation, this way our velocity vector points
        // to where our sprite is pointing to
        t.translation += Quat::mul_vec3(rot, Vec3::new(0., veloc, 0.));
        if p.states.as_ref().is_some_and( |s| s[p.state_current].duration == 0.) {
            // If the pattern has a duration of zero, we want it to be instant, and not affected
            // by the delta_time
            t.rotate_z( ( p.angular_velocity).to_radians());
        } else {
            t.rotate_z( ( p.angular_velocity).to_radians() * time * one_frame_offset  );
        }
    }
}

//TODO: fix :3
fn update_bullet_collision(
    mut entities: Query<(Entity, &mut Health), Without<Player>>,
    mut projectiles: Query<(Entity, &Projectile)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
) {
    for event in collision_events.read() {
        info!("Received collision event");
        match event  {
            CollisionEvent::Started(e1, e2, args) => {
                if *args & CollisionEventFlags::SENSOR == CollisionEventFlags::SENSOR {
                    if entities.get(*e1).is_ok() || projectiles.get(*e2).is_ok() {
                        let (_, mut health) = entities.get_mut(*e1).unwrap();
                        let (_, mut projectile) = projectiles.get(*e2).unwrap();

                        health.current -= projectile.damage;
                        commands.entity(*e2).despawn();
                    }

                    if entities.get(*e2).is_ok() || projectiles.get(*e1).is_ok() {
                        let (_, mut projectile) = projectiles.get(*e1).unwrap();
                        let (_, mut health) = entities.get_mut(*e2).unwrap();

                        health.current -= projectile.damage;
                        commands.entity(*e1).despawn();
                    }

                    info!("Collision occured");
                }
                info!("Your if is fucked");
            }
            _ => (),
        }
    }
}

fn enemy_projectile_detection(
    mut entities: Query<(Entity, &Collider, &Transform, &mut Health), (Without<Player>, Without<Projectile>)>,
    mut projectiles: Query<(Entity, &Projectile)>,
    mut commands: Commands,
    rapier_ctx: Res<RapierContext>,
) {
    for (e, coll, transform, mut health) in entities.iter_mut() {
        rapier_ctx.intersections_with_shape(
            transform.translation.xy(), //pos
            transform.rotation.to_euler(EulerRot::XYZ).2, //rot
            coll, //shape
            QueryFilter::default(), 
            |entity| {
                let p_e = projectiles.get(entity);
                if p_e.is_ok() {
                    let (id, projectile) = p_e.unwrap();
                    match projectile.targeting_type {
                        ProjectileTargetingType::ENVIRONMENT | ProjectileTargetingType::PLAYER => {
                            health.current -= projectile.damage;
                            commands.entity(id).despawn();
                        }
                        _ => ()
                    }
                }
                true
            }
        )
    }
}

fn player_projectile_detection(
    mut player: Query<(Entity, &Collider, &Transform, &mut Health), With<Player>>, 
    mut projectiles: Query<(Entity, &Projectile)>,
    mut commands: Commands,
    rapier_ctx: Res<RapierContext>,
) {
    let (e, coll, transform, mut health) = player.get_single_mut().expect("Player not found");

    rapier_ctx.intersections_with_shape(
        transform.translation.xy(), //pos
        transform.rotation.to_euler(EulerRot::XYZ).2, //rot
        coll, //shape
        QueryFilter::default(), 
        |entity| {
            let p_e = projectiles.get(entity);
            if p_e.is_ok() {
                let (id, projectile) = p_e.unwrap();
                match projectile.targeting_type {
                    ProjectileTargetingType::ENVIRONMENT | ProjectileTargetingType::ENEMY => {
                        health.current -= projectile.damage;
                        commands.entity(id).despawn();
                    }
                    _ => ()
                }
            }
            true
        }
    )
}