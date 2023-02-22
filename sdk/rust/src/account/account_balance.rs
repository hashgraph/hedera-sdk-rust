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

use std::collections::HashMap;

use hedera_proto::services;
use prost::Message;

use crate::protobuf::ToProtobuf;
use crate::{
    AccountId,
    FromProtobuf,
    Hbar,
    Tinybar,
    TokenId,
};

/// Response from [`AccountBalanceQuery`][crate::AccountBalanceQuery].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct AccountBalance {
    /// The account that is being referenced.
    pub account_id: AccountId,

    /// Current balance of the referenced account.
    pub hbars: Hbar,

    /// Token balances for the referenced account.
    #[deprecated = "use a mirror query"]
    #[allow(deprecated)]
    pub tokens: HashMap<TokenId, u64>,

    /// Token decimals for the referenced account.
    #[deprecated = "use a mirror query"]
    #[allow(deprecated)]
    pub token_decimals: HashMap<TokenId, u32>,
}

impl AccountBalance {
    /// Create a new `AccountBalance` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::<services::CryptoGetAccountBalanceResponse>::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        #[allow(deprecated)]
        services::CryptoGetAccountBalanceResponse {
            header: None,
            account_id: Some(self.account_id.to_protobuf()),
            balance: self.hbars.to_tinybars() as u64,
            token_balances: Vec::default(),
        }
        .encode_to_vec()
    }
}

impl FromProtobuf<services::CryptoGetAccountBalanceResponse> for AccountBalance {
    #[allow(deprecated)]
    fn from_protobuf(pb: services::CryptoGetAccountBalanceResponse) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let account_id = pb_getf!(pb, account_id)?;
        let account_id = AccountId::from_protobuf(account_id)?;

        let balance = Hbar::from_tinybars(pb.balance as Tinybar);

        let mut tokens = HashMap::with_capacity(pb.token_balances.len());
        let mut token_decimals = HashMap::with_capacity(pb.token_balances.len());

        for token in pb.token_balances {
            let token_id = TokenId::from_protobuf(pb_getf!(token, token_id)?)?;

            tokens.insert(token_id, token.balance);
            token_decimals.insert(token_id, token.decimals);
        }

        Ok(Self { account_id, hbars: balance, tokens, token_decimals })
    }
}

impl FromProtobuf<services::response::Response> for AccountBalance {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self> {
        let response = pb_getv!(pb, CryptogetAccountBalance, services::response::Response);

        Self::from_protobuf(response)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ffi")]
    mod ffi {
        use std::collections::HashMap;

        use crate::{
            AccountBalance,
            Hbar,
        };

        #[test]
        #[allow(deprecated)]
        fn serialize() {
            expect_test::expect![[r#"
                {
                  "accountId": "0.0.3",
                  "hbars": 200000000,
                  "tokens": {},
                  "tokenDecimals": {}
                }"#]].assert_eq(
                &serde_json::to_string_pretty(&AccountBalance {
                    account_id: 3.into(),
                    hbars: Hbar::new(2),
                    tokens: HashMap::new(),
                    token_decimals: HashMap::new(),
                })
                .unwrap(),
            )
        }
    }
}
