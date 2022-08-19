/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use time::Duration;

use crate::execute::execute;
use crate::query::cost::QueryCost;
use crate::query::payment_transaction::PaymentTransaction;
use crate::{
    AccountId,
    Client,
    Error,
    TransactionId,
};

mod any;
mod cost;
mod execute;
pub(super) mod payment_transaction;
mod protobuf;

pub(crate) use any::AnyQueryData;
pub use any::{
    AnyQuery,
    AnyQueryResponse,
};
pub(crate) use execute::QueryExecute;
pub(crate) use protobuf::ToQueryProtobuf;

/// A query that can be executed on the Hedera network.
#[derive(Debug, Default)]
pub struct Query<D>
where
    D: QueryExecute,
{
    pub(crate) data: D,
    pub(crate) payment: PaymentTransaction,
}

impl<D> Query<D>
where
    D: QueryExecute + Default,
{
    /// Create a new query ready for configuration and execution.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<D> Query<D>
where
    D: QueryExecute,
{
    /// Set the account IDs of the nodes that this query may be submitted to.
    ///
    /// Defaults to the full list of nodes configured on the client; or, the node account IDs
    /// configured on the query payment transaction (if explicitly provided).
    ///
    pub fn node_account_ids(&mut self, ids: impl IntoIterator<Item = AccountId>) -> &mut Self {
        self.payment.node_account_ids(ids);
        self
    }

    /// Set an explicit payment amount for this query.
    ///
    /// The client will submit exactly this amount for the payment of this query. Hedera
    /// will not return any remainder (over the actual cost for this query).
    ///
    // TODO: Use Hbar
    pub fn payment_amount(&mut self, amount: u64) -> &mut Self {
        self.payment.body.data.amount = Some(amount);
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
    /// Set to `None` to allow unlimited payment amounts.
    ///
    // TODO: Use Hbar
    pub fn max_payment_amount(&mut self, max: impl Into<Option<u64>>) -> &mut Self {
        self.payment.body.data.max_amount = max.into();
        self
    }

    /// Sets the duration that the payment transaction is valid for, once finalized and signed.
    ///
    /// Defaults to 120 seconds (or two minutes).
    ///
    pub fn payment_transaction_valid_duration(&mut self, duration: Duration) -> &mut Self {
        self.payment.transaction_valid_duration(duration);
        self
    }

    /// Set the maximum transaction fee the payer account is willing to pay for the query
    /// payment transaction.
    ///
    /// Defaults to 1 hbar.
    ///
    // TODO: Use Hbar
    pub fn max_payment_transaction_fee(&mut self, fee: u64) -> &mut Self {
        self.payment.max_transaction_fee(fee);
        self
    }

    /// Set a note or description that should be recorded in the transaction record (maximum length
    /// of 100 characters) for the payment transaction.
    pub fn payment_transaction_memo(&mut self, memo: impl AsRef<str>) -> &mut Self {
        self.payment.transaction_memo(memo);
        self
    }

    /// Set an explicit transaction ID to use to identify the payment transaction
    /// on this query.
    ///
    /// Overrides payer account defined on this query or on the client.
    ///
    pub fn payment_transaction_id(&mut self, id: TransactionId) -> &mut Self {
        self.payment.transaction_id(id);
        self
    }
}

impl<D> Query<D>
where
    D: QueryExecute,
{
    /// Execute this query against the provided client of the Hedera network.
    pub async fn execute(&mut self, client: &Client) -> crate::Result<D::Response> {
        if self.payment.body.data.amount.is_none() && self.data.is_payment_required() {
            // payment is required but none was specified, query the cost
            let cost = QueryCost::new(self).execute(client).await?;

            if let Some(max_amount) = self.payment.body.data.max_amount {
                if cost > max_amount {
                    return Err(Error::MaxQueryPaymentExceeded {
                        query_cost: cost,
                        max_query_payment: max_amount,
                    });
                }
            }

            self.payment.body.data.amount = Some(cost);
        }

        execute(client, self).await
    }
}
