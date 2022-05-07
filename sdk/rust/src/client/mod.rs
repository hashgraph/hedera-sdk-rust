use std::sync::Arc;

use tonic::transport::Channel;

use crate::client::network::Network;

mod network;

pub use network::NetworkChannel;

#[derive(Clone)]
pub struct Client {
    pub(crate) network: Arc<Network>,
}
