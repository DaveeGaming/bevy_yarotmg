use bevy::prelude::*;

use crate::player::Player;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedPostUpdate, update_entity_rotation);
    }
}

#[derive(Component)]
pub struct EntityRotate;

/// Rotate every entity to the same orientation as the player
fn update_entity_rotation(
    mut entities: Query<&mut Transform, (With<EntityRotate>, Without<Player>)>,
    player: Query<&mut Transform, With<Player>>
) {
    if let Ok(p_transform) = player.get_single() {
        for mut entity in entities.iter_mut() {
            entity.rotation = p_transform.rotation;
        }
    } else {
        warn_once!("Player entity not found in entity_rotation");
    }

}