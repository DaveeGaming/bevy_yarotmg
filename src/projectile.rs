use std::borrow::BorrowMut;

use bevy::{math::bounding::{Aabb2d, IntersectsVolume}, prelude::*, render::primitives::Aabb, sprite};

use crate::{health::Health, player::Player};

pub struct ProjectilePlugin;

#[derive(Component)]
pub struct Projectile {
    pub damage: i32,
    pub velocity: Vec2,
    pub speed: f32,
}

#[derive(Resource)]
pub struct ProjectileAsset {
    pub handle: Handle<Image>,
}

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, update_projectile_position);
        app.add_systems(FixedUpdate, update_bullet_collision);
    }
}

fn setup(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>
) {
    commands.insert_resource(
        ProjectileAsset {
            handle: asset_server.load("projectile.png"),
    })
}

fn update_projectile_position(
    mut projectiles: Query<(&mut Transform, &Projectile)>
) {
    for (mut t, p) in projectiles.iter_mut() {
        t.translation += (Vec2::splat(p.speed) * p.velocity).extend(0.);
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
        let p_next_position = p_transform.translation.xy() + (p_data.velocity * Vec2::splat(p_data.speed));

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

            if (projectile_aabb.intersects(&e_aabb)) {
                e_health.current -= p_data.damage;
                commands.entity(p_entity).despawn();
            }
        }
    }
}