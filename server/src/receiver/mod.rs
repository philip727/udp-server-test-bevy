pub mod config;
pub mod constants;
pub mod events;
pub mod helpers;
pub mod resources;
pub mod states;
pub mod systems;

use bevy::prelude::*;

use self::{
    config::UdpReceiverConfig,
    events::UdpPacketReceivedEvent,
    helpers::create_bind_address,
    resources::UdpReceiverManager,
    states::UdpReceiverState,
    systems::{bind_udp_socket, handle_packets, setup_udp_receiver},
};

pub struct ReceiverPlugin {
    pub use_config_file: bool,
    pub config: Option<UdpReceiverConfig>,
}

impl ReceiverPlugin {}

impl Default for ReceiverPlugin {
    fn default() -> Self {
        ReceiverPlugin {
            config: None,
            use_config_file: false,
        }
    }
}

impl Plugin for ReceiverPlugin {
    fn build(&self, app: &mut App) {
        let mut receiver_config = UdpReceiverConfig::new();

        if let Some(c) = &self.config {
            receiver_config = c.clone();
        }

        // Loads the receiver config from file
        if self.use_config_file || self.config.is_none() {
            // Warns the user that no config was provided in the code and no server config file in
            // use was specified, so trying from the server properties file
            if self.config.is_none() && !self.use_config_file {
                info!("No udp receiver config was provided internally, loading from 'config/server.properties'.")
            }

            let file_config = UdpReceiverConfig::from_file();
            if let Err(e) = file_config {
                panic!("{}", e);
            }

            receiver_config = file_config.unwrap();
        }

        // Checks if debugging is provided and is also true
        if receiver_config.debugging.is_some_and(|b| b) {
            info!("Udp receiver is in debugging mode.");
        }

        // Bevy udp receiver building
        app.init_resource::<UdpReceiverManager>()
            .add_state::<UdpReceiverState>()
            .add_event::<UdpPacketReceivedEvent>()
            .add_systems(OnEnter(UdpReceiverState::Setup), setup_udp_receiver)
            .add_systems(OnEnter(UdpReceiverState::Binding), bind_udp_socket)
            .add_systems(
                Update,
                handle_packets.run_if(in_state(UdpReceiverState::Finished)),
            );

        // Creates the bind address for the UdpReceiver
        create_bind_address(
            app,
            &receiver_config
                .port
                .expect("Please provide a port in the server config."),
        );

        let mut udp_receiver_manager = app.world.get_resource_mut::<UdpReceiverManager>().unwrap();
        udp_receiver_manager.set_debug_mode(receiver_config.debugging.unwrap_or(false));
    }
}
