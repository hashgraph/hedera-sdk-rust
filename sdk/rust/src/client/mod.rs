use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use parking_lot::RwLock;
use tokio::sync::{OwnedRwLockReadGuard, RwLock as AsyncRwLock};
use tokio::task::block_in_place;

use self::mirror_network::MirrorNetwork;
use crate::client::network::{Network, TESTNET};
use crate::{AccountId, Signer, TransactionId};

mod mirror_network;
mod network;

/// Managed client for use on the Hedera network.
#[derive(Clone)]
pub struct Client {
    network: Arc<Network>,
    mirror_network: Arc<MirrorNetwork>,
    payer_account_id: Arc<RwLock<Option<AccountId>>>,
    default_signers: Arc<AsyncRwLock<Vec<Box<dyn Signer>>>>,
    max_transaction_fee: Arc<AtomicU64>,
}

impl Client {
    /// Construct a Hedera client pre-configured for testnet access.
    pub fn for_testnet() -> Self {
        Self {
            network: Arc::new(Network::from_static(TESTNET)),
            mirror_network: Arc::new(MirrorNetwork::from_static(&[mirror_network::TESTNET])),
            payer_account_id: Arc::new(RwLock::new(None)),
            max_transaction_fee: Arc::new(AtomicU64::new(0)),
            default_signers: Arc::new(AsyncRwLock::new(Vec::with_capacity(1))),
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

    /// Adds a signer that will, by default, sign for all transactions and queries built
    /// with this client.
    ///
    pub fn add_default_signer<S>(&self, signer: S)
    where
        S: Signer,
    {
        block_in_place(|| {
            self.default_signers.blocking_write().push(Box::new(signer));
        });
    }

    /// Removes all default signers from this client.
    pub fn clear_default_signers(&self) {
        block_in_place(|| {
            self.default_signers.blocking_write().clear();
        });
    }

    /// Generate a new transaction ID from the stored payer account ID, if present.
    pub(crate) fn generate_transaction_id(&self) -> Option<TransactionId> {
        self.payer_account_id.read().map(TransactionId::generate)
    }

    /// Gets a reference to the configured network.
    pub(crate) fn network(&self) -> &Network {
        &self.network
    }

    /// Gets a reference to the configured mirror network.
    pub(crate) fn mirror_network(&self) -> &MirrorNetwork {
        &self.mirror_network
    }

    /// Gets the maximum transaction fee the paying account is willing to pay.
    pub(crate) fn max_transaction_fee(&self) -> &AtomicU64 {
        &self.max_transaction_fee
    }

    /// Gets a list of the default signers.
    pub(crate) async fn default_signers(&self) -> OwnedRwLockReadGuard<Vec<Box<dyn Signer>>> {
        Arc::clone(&self.default_signers).read_owned().await
    }
}
