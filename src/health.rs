use bevy::prelude::*;

/// Marks entities with Text that are the child of Health component entities
#[derive(Component)]
pub struct HealthText;

/// Marks entities with a Health bar \
/// This is what we filter collisions by currently
#[derive(Component)]
pub struct Health {
    pub max: i32,
    pub current: i32,
} 


impl Default for Health {
    fn default() -> Self {
        Health {
            max: 10,
            current: 5,
        }
    }
}
pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, create_health_text);
        app.add_systems(FixedUpdate, update_health_text);
    }
}

/// Checks for entities that got a Health component \
/// If they got one, we place a text component on them as a child, with a HealthText component
fn create_health_text(
    mut commands: Commands,
    mut health_event: Query<(Entity, &Handle<Image>), (Added<Health>, With<Sprite>)>   ,
    assets: Res<Assets<Image>>
) {
    for (entity, sprite) in health_event.iter_mut() {
        let sprite_size = assets.get(sprite.id()).unwrap().size_f32();
        let text = commands.spawn( 
            (
            Text2dBundle {
                text: Text::from_section( "None", TextStyle { font_size: 10., ..default()}),
                transform: Transform::from_translation( Vec3::new(0., -sprite_size.y, 0.)),
                ..default()
            },
            HealthText
            )
        ).id();
        commands.entity(entity).add_child(text);
    }
}


/// For every HealthText, get their text, and their parent, and update the shown health
fn update_health_text(
    mut text: Query<(&mut Text, &Parent), With<HealthText>>,
    health_entities: Query<&Health>
) {
    for (mut text, parent) in text.iter_mut() {
        if let Ok(health) = health_entities.get(parent.get()) {
            text.sections[0].value = format!("{}/{}", health.current, health.max); 
        }
    
    }
}