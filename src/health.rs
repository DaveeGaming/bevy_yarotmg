use std::process::Child;

use bevy::prelude::*;

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

fn create_health_text(
    mut commands: Commands,
    mut health_event: Query<Entity, Added<Health>>   
) {
    for entity in health_event.iter_mut() {
        let text = commands.spawn( 
            Text2dBundle {
                text: Text::from_section( "None", TextStyle { font_size: 10., ..default()}),
                transform: Transform::from_translation( Vec3::new(0., -10., 0.)),
                ..default()
            }
        ).id();
        commands.entity(entity).add_child(text);
    }
}

fn update_health_text(
    mut text: Query<(&mut Text, &Parent)>,
    health_entities: Query<&Health>
) {
    for (mut text, parent) in text.iter_mut() {
        if let Ok(health) = health_entities.get(parent.get()) {
            text.sections[0].value = format!("{}/{}", health.current, health.max); 
        }
    
    }
}