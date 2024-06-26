mod input;
mod player;
mod entity;
mod weapon;
mod projectile;
mod health;
mod projectilepattern;
mod states;
mod editor;
mod rapier;
mod stateful;
mod editor_camera;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::states::StateManager;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(Msaa::Off)
        .insert_resource(Time::from_hz(60.))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(StateManager)
        .run();
}