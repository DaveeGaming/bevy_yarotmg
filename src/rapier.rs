use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


pub struct RapierPlugin;

impl Plugin for RapierPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.));
        app.add_plugins(RapierDebugRenderPlugin::default());
        // app.add_systems(Startup, rapier_start);
    }
}

fn rapier_start(
    mut rapier_config: ResMut<RapierConfiguration>
) {
    rapier_config.gravity = Vec2::ZERO;
}