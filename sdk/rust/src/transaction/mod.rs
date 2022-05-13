use serde_with::skip_serializing_none;
use time::Duration;

use crate::execute::execute;
use crate::{AccountId, Client, Signer, TransactionId, TransactionResponse};

mod execute;
mod protobuf;

pub(crate) use execute::TransactionExecute;
pub(crate) use protobuf::ToTransactionDataProtobuf;

const DEFAULT_TRANSACTION_VALID_DURATION: Duration = Duration::seconds(120);

#[derive(serde::Serialize)]
pub struct Transaction<D> {
    #[serde(flatten)]
    pub(crate) body: TransactionBody<D>,

    #[serde(skip)]
    signers: Vec<Box<dyn Signer>>,
}

#[skip_serializing_none]
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionBody<D> {
    pub(crate) data: D,
    node_account_ids: Option<Vec<AccountId>>,
    #[serde(with = "crate::serde::duration_opt")]
    transaction_valid_duration: Option<Duration>,
    max_transaction_fee: Option<u64>,
    #[serde(skip_serializing_if = "crate::serde::skip_if_string_empty")]
    transaction_memo: String,
    payer_account_id: Option<AccountId>,
    transaction_id: Option<TransactionId>,
}

impl<D> Default for Transaction<D>
where
    D: Default,
{
    fn default() -> Self {
        Self {
            body: TransactionBody {
                data: D::default(),
                node_account_ids: None,
                transaction_valid_duration: None,
                transaction_memo: String::new(),
                max_transaction_fee: None,
                payer_account_id: None,
                transaction_id: None,
            },
            signers: Vec::new(),
        }
    }
}

impl<D> Transaction<D>
where
    D: Default,
{
    #[inline]
    #[must_use]
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
        self.body.node_account_ids = Some(ids.into_iter().collect());
        self
    }

    /// Sets the duration that this transaction is valid for, once finalized and signed.
    ///
    /// Defaults to 120 seconds (or two minutes).
    ///
    pub fn transaction_valid_duration(&mut self, duration: Duration) -> &mut Self {
        self.body.transaction_valid_duration = Some(duration);
        self
    }

    /// Set the maximum transaction fee the paying account is willing to pay.
    pub fn max_transaction_fee(&mut self, fee: u64) -> &mut Self {
        self.body.max_transaction_fee = Some(fee);
        self
    }

    /// Set a note or description that should be recorded in the transaction record (maximum length
    /// of 100 characters).
    pub fn transaction_memo(&mut self, memo: impl AsRef<str>) -> &mut Self {
        self.body.transaction_memo = memo.as_ref().to_owned();
        self
    }

    /// Sets the account that will be paying for this transaction.
    pub fn payer_account_id(&mut self, id: AccountId) -> &mut Self {
        self.body.payer_account_id = Some(id);
        self
    }

    /// Set an explicit transaction ID to use to identify this transaction.
    ///
    /// Overrides payer account defined on this transaction or on the client.
    ///
    pub fn transaction_id(&mut self, id: TransactionId) -> &mut Self {
        self.body.transaction_id = Some(id);
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
    /// Execute this transaction against the provided client of the Hedera network.
    pub async fn execute(&mut self, client: &Client) -> crate::Result<TransactionResponse> {
        execute(client, self).await
    }
}
