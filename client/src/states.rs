use bevy::prelude::States;

#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub enum AppState {
    #[default]
    Menu,
    Game,
}
