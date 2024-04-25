use bevy::prelude::*;
// TODO: Plugin bundle for a gameplay, and an editor state

#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppStates {
    Editor,
    Gameplay
}

struct GlobalPlugins;
struct GameplayPlugins;
struct EditorPlugins;

struct StateManager;

impl PluginGroup for GlobalPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        todo!()
    }
}
impl PluginGroup for EditorPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        todo!()
    }
}
impl PluginGroup for GameplayPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        todo!()
    }
}

impl Plugin for StateManager {
    fn build(&self, app: &mut App) {
        app.insert_state(AppStates::Gameplay);
    }
}