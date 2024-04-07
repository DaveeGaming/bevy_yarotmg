use bevy::{math::bounding::{Aabb2d, IntersectsVolume}, prelude::*, render::render_resource::encase::rts_array::Length};
use crate::{health::Health, player::Player};

pub struct ProjectilePlugin;

#[derive(Clone, Copy)]
pub struct PPattern {
    pub angular_velocity: Option<f32>,
    pub speed: Option<f32>,
    pub duration: f32,
}


#[derive(Component)]
pub struct Projectile {
    pub damage: i32,
    pub angular_velocity: f32,
    pub speed: f32,

    pub current_pattern: usize,
    pub pattern_duration: f32,
    pub patterns: Option<Vec<PPattern>>,
    pub pattern_repeat: bool,
    
}

impl Projectile {
    pub fn new(damage: i32, angular_velocity: f32, speed: f32) -> Projectile{
        Projectile {
            damage,
            angular_velocity,
            speed,

            current_pattern: 0,
            patterns: None,
            pattern_duration: 0., 
            pattern_repeat: false,
        }
    }

    pub fn from_patterns(damage: i32,patterns: Vec<PPattern>, pattern_repeat: bool) -> Projectile {
        let first_pattern = patterns[0];
        Projectile {
            damage,
            angular_velocity: first_pattern.angular_velocity.unwrap_or_default(),
            speed: first_pattern.speed.unwrap_or_default(),

            current_pattern: 0,
            patterns: Some(patterns.clone()),
            pattern_duration: first_pattern.duration,
            pattern_repeat
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
        app.add_systems(FixedUpdate, update_projectile_position);
        app.add_systems(FixedUpdate, update_pattern);
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

fn update_pattern (
    time: Res<Time>,
    mut projectiles: Query<&mut Projectile>,
) {

    for mut p in projectiles.iter_mut() {

        if p.patterns.is_some() {
            if p.pattern_duration < 0. {
                let p_length = p.patterns.as_ref().unwrap().length();
                if p.current_pattern + 1 < p_length {

                    p.current_pattern += 1;
                    let next_pattern =  p.patterns.as_ref().unwrap()[p.current_pattern];

                    // Update projectile variables
                    p.pattern_duration = next_pattern.duration;
                    p.speed = next_pattern.speed.unwrap_or( p.speed );
                    p.angular_velocity = next_pattern.angular_velocity.unwrap_or( p.angular_velocity );

                } else if p.pattern_repeat && p.current_pattern + 1 == p_length {

                    p.current_pattern = 0; 
                    let next_pattern =  p.patterns.as_ref().unwrap()[p.current_pattern];

                    // Update projectile variables
                    p.pattern_duration = next_pattern.duration;
                    p.speed = next_pattern.speed.unwrap_or( p.speed );
                    p.angular_velocity = next_pattern.angular_velocity.unwrap_or( p.angular_velocity );
                }
            } else {
                p.pattern_duration -= time.delta().as_secs_f32();
            }
        }
    }
}

fn update_projectile_position(
    mut projectiles: Query<(&mut Transform, &Projectile)>
) {
    for (mut t, p) in projectiles.iter_mut() { 
        let veloc = (Vec2::splat(p.speed) * Vec2::new(0., 1.)).extend(0.);
        let rot = t.rotation;
        t.translation += Quat::mul_vec3(rot, veloc);
        t.rotate_z(p.angular_velocity.to_radians());
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