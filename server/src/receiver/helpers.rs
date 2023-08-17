use bevy::prelude::*;

use super::resources::UdpReceiverManager;

/// Create a bind address
/// This will modify the udp receiver manager
pub fn create_bind_address(app: &mut App, port: &u16) {
    let mut udp_receiver_manager = app.world.get_resource_mut::<UdpReceiverManager>().unwrap();

    // Create bind address on localhost
    let mut bind_address = String::from("127.0.0.1:");
    bind_address.push_str(&port.to_string());

    udp_receiver_manager.set_bind_address(bind_address);
}
