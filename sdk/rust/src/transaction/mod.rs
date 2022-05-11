use hedera_proto::services;
use time::Duration;

use crate::execute::execute;
use crate::{AccountId, Client, Signer, ToProtobuf, TransactionId, TransactionResponse};

mod execute;
mod protobuf;

pub(crate) use execute::TransactionExecute;
pub(crate) use protobuf::ToTransactionDataProtobuf;

pub struct Transaction<D> {
    pub(crate) data: D,
    node_account_ids: Option<Vec<AccountId>>,
    transaction_valid_duration: Duration,
    max_transaction_fee: Option<u64>,
    transaction_memo: String,
    signers: Vec<Box<dyn Signer>>,
    payer_account_id: Option<AccountId>,
    transaction_id: Option<TransactionId>,
}

impl<D> Default for Transaction<D>
where
    D: Default,
{
    fn default() -> Self {
        Self {
            data: D::default(),
            node_account_ids: None,
            transaction_valid_duration: Duration::seconds(120),
            transaction_memo: String::new(),
            max_transaction_fee: None,
            signers: Vec::new(),
            payer_account_id: None,
            transaction_id: None,
        }
    }
}

impl<D> Transaction<D>
where
    D: Default,
{
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<D> Transaction<D> {
    /// Set the account IDs of the nodes that this transaction may be submitted to.
    ///
    /// Defaults to the full list of nodes configured on the client.
    ///
    pub fn node_account_ids(&mut self, ids: impl IntoIterator<Item = AccountId>) -> &mut Self {
        self.node_account_ids = Some(ids.into_iter().collect());
        self
    }

    /// Sets the duration that this transaction is valid for, once finalized and signed.
    ///
    /// Defaults to 120 seconds (or two minutes).
    ///  
    pub fn transaction_valid_duration(&mut self, duration: Duration) -> &mut Self {
        self.transaction_valid_duration = duration;
        self
    }

    /// Set the maximum transaction fee the operator (paying account) is willing to pay.
    pub fn max_transaction_fee(&mut self, fee: u64) -> &mut Self {
        self.max_transaction_fee = Some(fee);
        self
    }

    /// Set a note or description that should be recorded in the transaction record (maximum length
    /// of 100 characters).
    pub fn transaction_memo(&mut self, memo: impl AsRef<str>) -> &mut Self {
        self.transaction_memo = memo.as_ref().to_owned();
        self
    }

    /// Sets the account that will be paying for this transaction.
    pub fn payer_account_id(&mut self, id: AccountId) -> &mut Self {
        self.payer_account_id = Some(id);
        self
    }

    /// Set an explicit transaction ID to use to identify this transaction.
    ///
    /// Overrides payer account defined on this transaction or on the client.
    ///
    pub fn transaction_id(&mut self, id: TransactionId) -> &mut Self {
        self.transaction_id = Some(id);
        self
    }

    /// Adds the signer to the list of signers that will sign this transaction before sending
    /// to the network.
    pub fn signer<S>(&mut self, signer: &S) -> &mut Self
    where
        S: Signer + Clone,
    {
        self.signers.push(Box::new(signer.clone()));
        self
    }
}

impl<D> Transaction<D>
where
    D: ToTransactionDataProtobuf,
    Self: TransactionExecute,
{
    pub async fn execute(&mut self, client: &Client) -> crate::Result<TransactionResponse> {
        execute(client, self).await
    }
}
