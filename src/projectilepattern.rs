use bevy::{math::f32, prelude::*};

use crate::projectile::{PState, Projectile, ProjectileTargetingType};
use bevy_rapier2d::prelude::*;

/// Used for building custom bullet patterns, 
/// its better if we centralize the syntax a bit
pub trait IPPattern {
    fn spawn( 
        &mut self,
        commands: Commands, 
        center: &Transform, 
        sprite: Handle<Image>);
    fn update_state(&mut self);
}

#[derive(Component)]
pub struct PPattern {
    /// Stores the ID-s of every bullet entity
    pub bullets: Vec<Entity>,
    
    //TODO: Make all state changes its own struct?
    pub state_current: usize,
    pub state_duration: f32,
    pub states: Option<Vec<PState>>,
    pub state_repeat: bool,
}


impl Default for PPattern {
    fn default() -> Self {
        PPattern {
            bullets: Vec::new(),

            state_current: 0,
            state_duration: 0.,
            states: None,
            state_repeat: false
        }
    }
}


pub struct CirclePattern{
    pub base: PPattern,
    pub amount: i32,
    pub dir: Vec2,
    pub max_deg: f32,

    pub targeting: ProjectileTargetingType,
}

impl Default for CirclePattern {
    fn default() -> Self {
        CirclePattern {
            base: PPattern::default(),

            amount: 0,
            max_deg: 360.,
            dir: Vec2::new(0.,1.),
            targeting: ProjectileTargetingType::ENVIRONMENT
        }
    }
}

impl IPPattern for CirclePattern {
    fn spawn(
        &mut self,
        mut commands: Commands,
        center: &Transform, 
        sprite: Handle<Image>
    ) {
        // We want the pattern to be centered
        // Get the direction of the pattern, offset it to the right by max_deg/2
        // Then equally spread out the bullets

        let deg = self.max_deg / self.amount as f32;
        let deg_offset = self.max_deg / 2.;

        // Create quat pointing to given direction
        let dir_quat = Quat::from_rotation_arc(Vec3::Y, self.dir.extend(0.));

        // Create transform with quat
        let mut base_transform = center.with_rotation( dir_quat );

        // Rotate transform to the beginning of the pattern
        base_transform.rotate_z( (deg_offset).to_radians() ); 

        // Offset the pattern by half a rotation to the right,
        // because our first bullet is with 0 rotation
        // and the last bullet of the pattern has an empty space after it
        // equaling to deg, we half it and offset the pattern,
        // so we can center it
        base_transform.rotate_z( -(deg / 2.).to_radians() );

        for _ in 1..=self.amount {
            let id = commands.spawn(
                (
                    SpriteBundle {
                        transform: base_transform, 
                        texture: sprite.clone(),
                        ..default()
                    },
                    Projectile {
                        targeting_type: self.targeting,
                        ..default()
                    },
                    Collider::cuboid(1., 4.),
                    RigidBody::KinematicPositionBased,
                    ActiveCollisionTypes::all()          
                )
            ).id();

            // Rotate after spawning first bullet
            base_transform.rotate_z( -deg.to_radians() );

            self.base.bullets.push(id);
        }

        // One possible way of updating the bullets is to reconstruct the base
        // Every spawn iteration, and just spawn a PPattern clone as a component
        // This way every PPatern will have unique entity ID-s

        //TODO: idk xd
        // if self.base_id.is_none() {
        //     let id = commands.spawn(self.base).id();
        //     self.base_id = Some(id);
        // }
    }

    fn update_state(&mut self) {
        todo!()
    }
}