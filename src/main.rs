mod input;
mod player;
mod entity;
mod weapon;
mod projectile;
mod health;

use bevy::prelude::*;
use crate::health::HealthPlugin;
use crate::projectile::ProjectilePlugin;
use crate::player::PlayerPlugin;
use crate::input::InputPlugin;
use crate::entity::EntityPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa::Off)
        .add_plugins(InputPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EntityPlugin)
        .add_plugins(ProjectilePlugin)
        .add_plugins(HealthPlugin)
        .run();
}