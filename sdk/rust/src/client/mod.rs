use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::sync::Arc;

use parking_lot::RwLock;

use crate::client::network::{Network, TESTNET};
use crate::{AccountId, TransactionId};

mod network;

#[derive(Clone)]
pub struct Client {
    pub(crate) network: Arc<Network>,
    pub(crate) payer_account_id: Arc<RwLock<Option<AccountId>>>,
    pub(crate) max_transaction_fee: Arc<AtomicU64>,
}

// TODO: Client(Arc<Inner>)

impl Client {
    pub fn for_testnet() -> Self {
        Self {
            network: Arc::new(Network::from_static(TESTNET)),
            payer_account_id: Arc::new(RwLock::new(None)),
            max_transaction_fee: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Sets the account that will, by default, be paying for transactions and queries built with
    /// this client.
    ///
    /// The payer account ID is used to generate the default transaction ID for all transactions
    /// executed with this client.
    ///
    pub fn set_payer_account_id(&self, id: AccountId) {
        *self.payer_account_id.write() = Some(id);
    }
}

impl Client {
    pub(crate) fn generate_transaction_id(&self) -> Option<TransactionId> {
        self.payer_account_id.read().map(TransactionId::generate)
    }
}
