use bevy::prelude::*;
// use bytes::Bytes;

// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// pub struct Connection {
//     pub addr: SocketAddr,
//     pub socket: SocketHandle,
// }

#[derive(Debug)]
pub enum NetworkEvent {
    Connected(),
    Disconnected(),
    // Message(Connection, Bytes),
    // SendError(NetworkError),
}

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let network_resource = worker::start_worker_thread();

        app.add_event::<NetworkEvent>()
            .add_resource(network_resource)
            .add_system(process_network_events.system());
    }
}

