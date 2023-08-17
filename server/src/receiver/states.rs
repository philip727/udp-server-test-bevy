use bevy::prelude::*;

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub enum UdpReceiverState {
    #[default]
    Setup,
    Binding,
    Finished
}
