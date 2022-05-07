use tonic::transport::Channel;

mod network;

pub struct Client {
    chan: Channel,
}
