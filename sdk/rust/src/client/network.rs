use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::Client;

impl Client {
    pub fn for_testnet() -> Self {
        Client {
            chan: Channel::from_static("tcp://0.testnet.hedera.com:50211").connect_lazy(),
        }
    }

    pub(crate) fn crypto(&self) -> CryptoServiceClient<Channel> {
        // FIXME: should we be making these on demand?
        CryptoServiceClient::new(self.chan.clone())
    }
}
