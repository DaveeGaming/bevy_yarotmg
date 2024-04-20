use bevy::prelude::*;


pub struct ColliderPlugin;

impl Plugin for ColliderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            (
                draw_sprite_debug
            )
        );
    }
}

#[derive(Component)]
pub struct SpriteRectCollider {
    /// Draw rectangle Collider for testing
    pub debug: bool,
}

impl Default for SpriteRectCollider {
    fn default() -> Self {
        SpriteRectCollider { 
            debug: false 
        }
    }
}

fn draw_sprite_debug (
    entities: Query<(&Transform, &SpriteRectCollider, &Handle<Image>)>,
    mut gizmos: Gizmos,
    sprites: Res<Assets<Image>>
) {
    for (e_transform, e_collider, e_sprite) in entities.iter() {
        // If we want to draw the collider for testing
        if e_collider.debug {

            // Get handle to our sprite
            let e_sprite = sprites.get(e_sprite).expect("No sprite found for SpriteCollider Entity");
            let e_sprite_size = e_sprite.size_f32(); 

            let first_corner = e_transform.translation.xy();
            let second_corner = first_corner + e_sprite_size;

            gizmos.rect_2d(first_corner, e_transform.rotation.to_euler(EulerRot::XYZ).2, e_sprite_size, Color::RED);
        }
    }
}