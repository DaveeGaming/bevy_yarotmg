use bevy::{math::bounding::{Aabb2d, IntersectsVolume}, prelude::*, render::render_resource::encase::rts_array::Length};
use crate::{health::Health, player::Player};

pub struct ProjectilePlugin;

#[derive(Clone, Copy)]
pub struct PState {
    pub angular_velocity: Option<f32>,
    pub speed: Option<f32>,
    pub duration: f32,
}

#[derive(Component)]
pub struct Projectile {
    pub damage: i32,
    pub angular_velocity: f32,
    pub speed: f32,

    pub state_current: usize,
    pub state_duration: f32,
    pub states: Option<Vec<PState>>,
    pub state_repeat: bool,
    
}

impl Projectile {
    pub fn new(damage: i32, angular_velocity: f32, speed: f32) -> Projectile{
        Projectile {
            damage,
            angular_velocity,
            speed,

            state_current: 0,
            states: None,
            state_duration: 0., 
            state_repeat: false,
        }
    }

    pub fn from_states(damage: i32,states: Vec<PState>, pattern_repeat: bool) -> Projectile {
        let first_pattern = states[0];
        Projectile {
            damage,
            angular_velocity: first_pattern.angular_velocity.unwrap_or_default(),
            speed: first_pattern.speed.unwrap_or_default(),

            state_current: 0,
            states: Some(states.clone()),
            state_duration: first_pattern.duration,
            state_repeat: pattern_repeat
        }
    }

}

#[derive(Resource)]
pub struct ProjectileAsset {
    pub handle: Handle<Image>,
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, update_states);
        app.add_systems(FixedUpdate, update_projectile_position.after( update_states ) );
        app.add_systems(FixedUpdate, update_bullet_collision);
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

fn update_projectile_position(
    time: Res<Time>,
    mut projectiles: Query<(&mut Transform, &Projectile)>
) {
    let time = time.delta().as_secs_f32();
    let fm = 60.; 
    let one_frame_offset = 1. / ((fm + 1.) / fm); //TODO: i hate this shit, i want to kms
    for (mut t, p) in projectiles.iter_mut() { 
        let veloc = p.speed * time * one_frame_offset;
        let rot = t.rotation;
        t.translation += Quat::mul_vec3(rot, Vec3::new(0., veloc, 0.));
        if p.states.as_ref().is_some_and( |s| s[p.state_current].duration == 0.) {
            t.rotate_z( ( p.angular_velocity).to_radians());
        } else {
            t.rotate_z( ( p.angular_velocity).to_radians() * time * one_frame_offset  );
        }
    }
}

fn update_bullet_collision(
    mut entities: Query<(&Transform, &mut Health, &Handle<Image>), (Without<Player>,With<Sprite>)>,
    mut projectiles: Query<(Entity, &Transform, &Projectile, &Handle<Image>), With<Sprite>>,
    mut commands: Commands,
    assets: Res<Assets<Image>>
) {
    for (p_entity, p_transform, p_data, p_sprite) in projectiles.iter_mut() {
        let p_sprite_size = assets.get(p_sprite).unwrap().size_f32(); 
        let p_next_position = p_transform.translation.xy(); // + (p_data.velocity * Vec2::splat(p_data.speed));

        let projectile_aabb = Aabb2d {
            min: p_next_position,
            max: p_next_position + p_sprite_size, 
        };

        for (e_transform,mut e_health, e_sprite) in entities.iter_mut() {
            let e_sprite_size = assets.get(e_sprite).unwrap().size_f32(); 
            let e_aabb = Aabb2d {
                min: e_transform.translation.xy(),
                max: e_transform.translation.xy() + e_sprite_size
            };

            if projectile_aabb.intersects(&e_aabb) {
                e_health.current -= p_data.damage;
                commands.entity(p_entity).despawn();
            }
        }
    }
}