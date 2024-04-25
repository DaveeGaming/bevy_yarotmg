use bevy::prelude::*;

use crate::{player::Player, states::AppSet};

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, update_entity_rotation.in_set(AppSet::Gameplay) );
    }
}

#[derive(Component)]
pub struct EntityRotate;

/// Rotate every entity to the same orientation as the player
// TODO: Don't use the player component, instead use an EntityRotateRoot component
fn update_entity_rotation(
    mut entities: Query<&mut Transform, (With<EntityRotate>, Without<Player>)>,
    player: Query<&mut Transform, With<Player>>
) {
    if let Ok(p_transform) = player.get_single() {
        for mut entity in entities.iter_mut() {
            entity.rotation = p_transform.rotation;
        }
    } 
}