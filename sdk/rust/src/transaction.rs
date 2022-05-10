use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use hedera_proto::services;
use prost::Message;
use sha2::{Digest, Sha384};
use time::Duration;
use tonic::transport::Channel;
use tonic::{Response, Status};

use crate::execute::{execute, Execute};
use crate::{AccountId, Client, Error, Signer, ToProtobuf, TransactionId, TransactionResponse};

pub struct Transaction<D> {
    pub(crate) data: D,
    node_account_ids: Option<Vec<AccountId>>,
    transaction_valid_duration: Duration,
    max_transaction_fee: Option<u64>,
    transaction_memo: String,
    signers: Vec<Box<dyn Signer>>,
    // TODO: payer_account_id: Option<AccountId>
    // TODO: transaction_id: Option<TransactionId>
    //  While transaction IDs are generated on-demand by using the operator, if you don't have
    //  an operator configured, you are going to have a bad day
}

impl<D> Transaction<D>
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

#[async_trait]
pub trait TransactionExecute {
    fn default_max_transaction_fee() -> u64 {
        2 * 100_000_000 // 2 hbar
    }

    async fn execute(
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status>;
}

impl<D> Transaction<D>
where
    D: ToProtobuf<Protobuf = services::transaction_body::Data>,
    Self: TransactionExecute,
{
    #[allow(deprecated)]
    fn to_transaction_body_protobuf(
        &self,
        node_account_id: AccountId,
        transaction_id: TransactionId,
        client_max_transaction_fee: &Arc<AtomicU64>,
    ) -> services::TransactionBody {
        let data = self.data.to_protobuf();

        let max_transaction_fee = self.max_transaction_fee.unwrap_or_else(|| {
            // no max has been set on the *transaction*
            // check if there is a global max set on the client
            match client_max_transaction_fee.load(Ordering::Relaxed) {
                max if max > 1 => max,

                // no max has been set on the client either
                // fallback to the hard-coded default for this transaction type
                _ => Self::default_max_transaction_fee(),
            }
        });

        services::TransactionBody {
            data: Some(data),
            transaction_id: Some(transaction_id.to_protobuf()),
            transaction_valid_duration: Some(self.transaction_valid_duration.into()),
            memo: self.transaction_memo.clone(),
            node_account_id: Some(node_account_id.to_protobuf()),
            generate_record: false,
            transaction_fee: max_transaction_fee,
        }
    }
}

#[async_trait]
impl<D> Execute for Transaction<D>
where
    D: ToProtobuf<Protobuf = services::transaction_body::Data>,
    Self: TransactionExecute,
{
    type GrpcRequest = services::Transaction;

    type GrpcResponse = services::TransactionResponse;

    type RequestContext = ();

    type ResponseContext = [u8; 48];

    type Response = TransactionResponse;

    fn node_account_ids(&self) -> Option<&[AccountId]> {
        self.node_account_ids.as_deref()
    }

    fn transaction_id(&self) -> Option<TransactionId> {
        // TODO: explicit
        None
    }

    fn requires_transaction_id() -> bool {
        true
    }

    async fn make_request(
        &self,
        client: &Client,
        transaction_id: Option<TransactionId>,
        node_account_id: AccountId,
        _context: &Self::RequestContext,
    ) -> crate::Result<(Self::GrpcRequest, Self::ResponseContext)> {
        let transaction_id = transaction_id.unwrap();

        let transaction_body = self.to_transaction_body_protobuf(
            node_account_id,
            transaction_id.clone(),
            &client.max_transaction_fee,
        );

        let body_bytes = transaction_body.encode_to_vec();

        let mut signatures = Vec::with_capacity(self.signers.len());

        for signer in &self.signers {
            // TODO: should we run the signers in parallel?
            let signature = signer.sign(&body_bytes).await.map_err(Error::signature)?;

            signatures.push(signature.to_protobuf());
        }

        let signed_transaction = services::SignedTransaction {
            body_bytes,
            sig_map: Some(services::SignatureMap { sig_pair: signatures }),
        };

        let signed_transaction_bytes = signed_transaction.encode_to_vec();

        let transaction_hash = Sha384::digest(&signed_transaction_bytes);

        let transaction =
            services::Transaction { signed_transaction_bytes, ..services::Transaction::default() };

        Ok((transaction, transaction_hash.into()))
    }

    async fn execute(
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> Result<Response<Self::GrpcResponse>, Status> {
        <Self as TransactionExecute>::execute(channel, request).await
    }

    fn make_response(
        _response: Self::GrpcResponse,
        transaction_hash: Self::ResponseContext,
        node_account_id: AccountId,
        transaction_id: Option<TransactionId>,
    ) -> crate::Result<Self::Response> {
        Ok(TransactionResponse {
            node_account_id,
            transaction_id: transaction_id.unwrap(),
            transaction_hash,
        })
    }

    fn response_pre_check_status(response: &Self::GrpcResponse) -> crate::Result<i32> {
        Ok(response.node_transaction_precheck_code)
    }
}

impl<D> Transaction<D>
where
    D: ToProtobuf<Protobuf = services::transaction_body::Data>,
    Self: TransactionExecute,
{
    pub async fn execute(&self, client: &Client) -> crate::Result<TransactionResponse> {
        execute(client, self, ()).await
    }
}
