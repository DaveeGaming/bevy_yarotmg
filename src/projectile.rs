use bevy::prelude::*;
use bevy_rapier2d::{geometry::Collider, pipeline::QueryFilter, plugin::RapierContext};
use crate::{
    health::Health, 
    player::Player, 
    stateful::{State, StateDuration, Stateful, StatefulEvent}, states::AppSet
};

pub struct ProjectilePlugin;

#[derive(Clone, Copy)]
pub struct PState {
    pub angular_velocity: Option<f32>,
    pub speed: Option<f32>,
    pub duration: StateDuration,
}

impl State for PState {
    fn get_duration(&self) -> StateDuration {
        self.duration
    }
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
}


impl Default for Projectile {
    fn default() -> Self {
        Projectile {
            damage: 1,
            angular_velocity: 0.,
            speed: 15.,
            targeting_type: ProjectileTargetingType::ENVIRONMENT,
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
        app.add_event::<StatefulEvent<PState>>();
        app.add_systems(FixedUpdate, 
            (
                increment_states,
                update_states.after( increment_states ),
                update_projectile_position.after( update_states ),
            ).in_set(AppSet::Gameplay)
        );
        app.add_systems(PostUpdate, 
            (
                // update_bullet_collision, // It works, i will keep it for an example
                player_projectile_detection,
                enemy_projectile_detection
            ).in_set(AppSet::Gameplay)
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    info!("CALLED!!!");
    commands.insert_resource(
        ProjectileAsset {
            handle: asset_server.load("thing.png"),
    })
}


fn increment_states (
    time: Res<Time>,
    mut state_event: EventWriter<StatefulEvent<PState>>,
    mut projectiles: Query<(Entity, &mut Stateful<PState>), With<Projectile>>,
) { 
    for (id, mut state) in projectiles.iter_mut() {
        let event = state.update_state(time.delta_seconds());
        

        match event {
            Some(mut e) => { 
               e.entity_id = id; 
               state_event.send(e);
             },
            _ => ()
        }
    }
}

fn update_states (
    mut state_event: EventReader<StatefulEvent<PState>>,
    mut projectiles: Query<&mut Projectile, With<Stateful<PState>>>,
) { 
    for event in state_event.read() {
        let projectile = projectiles.get_mut( event.entity_id );

        match projectile {
            Ok( mut p ) => {
               p.speed = event.state.speed.unwrap_or( p.speed );
               p.angular_velocity = event.state.angular_velocity.unwrap_or( p.angular_velocity );
            },
            Err(err) => warn!("update_projectile_state lost entity from statefulevent, with error {}", err),
        }
    }
}

/// Updates the transform of every projectile, by what their Projectile struct defines
fn update_projectile_position(
    time: Res<Time>,
    mut projectiles: Query<(Entity,&mut Transform, &Projectile)>,
    state_projectiles: Query<&Stateful<PState>, With<Projectile>>
) {
    let time = time.delta().as_secs_f32();
    let fm = 60.; 
    let one_frame_offset = 1. / ((fm + 1.) / fm); //FIXME: i hate this shit, i want to kms
    for (e,mut t, p) in projectiles.iter_mut() { 
        let veloc = p.speed * time * one_frame_offset;
        let rot = t.rotation;
        // Quat::mul_vec3 multiplies the vector by a rotation, this way our velocity vector points
        // to where our sprite is pointing to
        t.translation += Quat::mul_vec3(rot, Vec3::new(0., veloc, 0.));

        //TODO: implement stateduration::stretch
        if state_projectiles.get( e ).is_ok_and( |s| s.state_duration == StateDuration::Instant) {
            // If the pattern has a duration of zero, we want it to be instant, and not affected
            // by the delta_time
            t.rotate_z( ( p.angular_velocity).to_radians());
        } else {
            t.rotate_z( ( p.angular_velocity).to_radians() * time * one_frame_offset  );
        }
    }
}


// SOME BIG INFO:

// Only gets called if entities have the ActiveEvents::COLLIDE_EVENTS flag
// And dont forget to set the ActiveCollisionType flags, if its two kinematic
// bodies.

// Leaving this here for learning and keepsake purposes

// fn update_bullet_collision(
//     mut entities: Query<(Entity, &mut Health), Without<Player>>,
//     mut projectiles: Query<(Entity, &Projectile)>,
//     mut collision_events: EventReader<CollisionEvent>,
//     mut commands: Commands,
// ) {
//     for event in collision_events.read() {
//         match event  {
//             CollisionEvent::Started(e1, e2, args) => {
//                 if args.intersects( CollisionEventFlags::SENSOR )  {
//                     if entities.get(*e1).is_ok() && projectiles.get(*e2).is_ok() {
//                         let (_, mut health) = entities.get_mut(*e1).unwrap();
//                         let (_, mut projectile) = projectiles.get(*e2).unwrap();

//                         health.current -= projectile.damage;
//                         commands.entity(*e2).despawn();
//                     }

//                     if entities.get(*e2).is_ok() && projectiles.get(*e1).is_ok() {
    //                         let (_, mut projectile) = projectiles.get(*e1).unwrap();
    //                         let (_, mut health) = entities.get_mut(*e2).unwrap();

//                         health.current -= projectile.damage;
//                         commands.entity(*e1).despawn();
//                     }
//                 }
//             }
//             _ => (),
//         }
//     }
// }


//TODO: Make a separate enemy and enviroment struct
fn enemy_projectile_detection(
    mut entities: Query<(Entity, &Collider, &Transform, &mut Health), (Without<Player>, Without<Projectile>)>,
    projectiles: Query<(Entity, &Projectile)>,
    mut commands: Commands,
    rapier_ctx: Res<RapierContext>,
) {
    for (_, coll, transform, mut health) in entities.iter_mut() {
        rapier_ctx.intersections_with_shape(
            transform.translation.xy(), //pos
            transform.rotation.to_euler(EulerRot::XYZ).2, //rot
            coll, //shape
            QueryFilter::default(),  // TODO: Figure out how to narrow down the query
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
    projectiles: Query<(Entity, &Projectile)>,
    mut commands: Commands,
    rapier_ctx: Res<RapierContext>,
) {
    if let Ok( (_, coll, transform, mut health) ) = player.get_single_mut() {

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

}