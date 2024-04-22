mod input;
mod player;
mod entity;
mod weapon;
mod projectile;
mod health;
mod projectilepattern;
mod rapier;
mod states;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::health::HealthPlugin;
use crate::projectile::ProjectilePlugin;
use crate::player::PlayerPlugin;
use crate::input::InputPlugin;
use crate::entity::EntityPlugin;
use crate::rapier::RapierPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa::Off)
        .insert_resource(Time::from_hz(60.))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(InputPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EntityPlugin)
        .add_plugins(ProjectilePlugin)
        .add_plugins(HealthPlugin)
        .add_plugins(RapierPlugin)
        .run();
}