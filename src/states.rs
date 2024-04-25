use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

use crate::editor_camera::EditorCameraPlugin;
use crate::health::HealthPlugin;
use crate::projectile::ProjectilePlugin;
use crate::player::PlayerPlugin;
use crate::input::{InputPlugin, Keybinds};
use crate::entity::EntityPlugin;
use crate::rapier::RapierPlugin;
// TODO: Plugin bundle for a gameplay, and an editor state

#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppStates {
    Editor,
    Gameplay
}

impl AppStates {
    pub fn next(&self) -> AppStates {
        match self {
            AppStates::Editor => AppStates::Gameplay,
            AppStates::Gameplay => AppStates::Editor, 
        }
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppSet {
    Editor,
    Gameplay
}

struct GlobalPlugins;
struct GameplayPlugins;
struct EditorPlugins;

pub struct StateManager;

impl PluginGroup for GlobalPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InputPlugin)
    }
}
impl PluginGroup for EditorPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(EditorCameraPlugin)
    }
}
impl PluginGroup for GameplayPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ProjectilePlugin)
            .add(RapierPlugin)
            .add(EntityPlugin)
            .add(HealthPlugin)
            .add(PlayerPlugin)
    }
}

impl Plugin for StateManager {
    fn build(&self, app: &mut App) {
        app.insert_state(AppStates::Gameplay);
        app.add_plugins(GlobalPlugins);
        app.add_plugins(GameplayPlugins);
        app.add_plugins(EditorPlugins);
        app.add_systems(Update, change_state);
        app.configure_sets(Startup, 
            (
              AppSet::Gameplay.run_if( in_state(AppStates::Gameplay)),
              AppSet::Editor.run_if( in_state(AppStates::Editor))  
            )
        );
        app.configure_sets(Update, 
            (
              AppSet::Gameplay.run_if( in_state(AppStates::Gameplay)),
              AppSet::Editor.run_if( in_state(AppStates::Editor))  
            )
        );
        app.configure_sets(PostUpdate, 
            (
              AppSet::Gameplay.run_if( in_state(AppStates::Gameplay)),
              AppSet::Editor.run_if( in_state(AppStates::Editor))  
            )
        );
        app.configure_sets(FixedUpdate, 
            (
              AppSet::Gameplay.run_if( in_state(AppStates::Gameplay)),
              AppSet::Editor.run_if( in_state(AppStates::Editor))  
            )
        );
    }
}

fn change_state(
    input: Res<Keybinds>,
    state: Res<State<AppStates>>,
    mut next: ResMut<NextState<AppStates>>
) {
    if input.change_state.active {
        next.set( state.next() );
    }
}