use crate::AccountId;

#[derive(Debug, Default)]
pub struct Query<D> {
    pub(crate) data: D,
    node_account_ids: Option<Vec<AccountId>>,
    // TODO: payment_transaction: Option<TransferTransaction>,
    payment_amount: Option<u64>,
    payment_amount_max: Option<Option<u64>>,
}

impl<D> Query<D>
where
    D: Default,
{
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<D> Query<D> {
    /// Set the account IDs of the nodes that this query may be submitted to.
    ///
    /// Defaults to the full list of nodes configured on the client; or, the node account IDs
    /// configured on the query payment transaction (if explicitly provided).
    ///
    pub fn node_account_ids(&mut self, ids: impl IntoIterator<Item = AccountId>) -> &mut Self {
        self.node_account_ids = Some(ids.into_iter().collect());
        self
    }

    /// Set an explicit payment amount for this query.
    ///
    /// The client will submit exactly this amount for the payment of this query. Hedera
    /// will not return any remainder (over the actual cost for this query).
    ///
    // TODO: Use Hbar
    pub fn payment_amount(&mut self, amount: impl Into<Option<u64>>) -> &mut Self {
        self.payment_amount = amount.into();
        self
    }

    /// Set the maximum payment allowable for this query.
    ///
    /// When a query is executed without an explicit payment amount set,
    /// the client will first request the cost of the given query from the node it will be
    /// submitted to and attach a payment for that amount from the operator account on the client.
    ///
    /// If the returned value is greater than this value, a [`MaxQueryPaymentExceeded`] error
    /// will be returned.
    ///
    /// Defaults to the maximum payment amount configured on the client.
    ///
    /// Set to `None` to disable automatic query payments for this query.
    ///
    pub fn max_payment_amount(&mut self, max: impl Into<Option<u64>>) -> &mut Self {
        self.payment_amount_max = Some(max.into());
        self
    }
}
