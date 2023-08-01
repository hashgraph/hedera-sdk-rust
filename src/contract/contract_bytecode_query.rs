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

use hedera_proto::services;
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use tonic::transport::Channel;

use crate::ledger_id::RefLedgerId;
use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    BoxGrpcFuture,
    ContractId,
    Error,
    FromProtobuf,
    Query,
    ToProtobuf,
    ValidateChecksums,
};

/// Get the runtime bytecode for a smart contract instance.
pub type ContractBytecodeQuery = Query<ContractBytecodeQueryData>;

#[derive(Default, Debug, Clone)]
pub struct ContractBytecodeQueryData {
    /// The contract for which information is requested.
    contract_id: Option<ContractId>,
}

impl ContractBytecodeQuery {
    /// Gets the contract for which information is requested.
    #[must_use]
    pub fn get_contract_id(&self) -> Option<ContractId> {
        self.data.contract_id
    }

    /// Sets the contract for which information is requested.
    pub fn contract_id(&mut self, contract_id: ContractId) -> &mut Self {
        self.data.contract_id = Some(contract_id);
        self
    }
}

impl From<ContractBytecodeQueryData> for AnyQueryData {
    #[inline]
    fn from(data: ContractBytecodeQueryData) -> Self {
        Self::ContractBytecode(data)
    }
}

impl ToQueryProtobuf for ContractBytecodeQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let contract_id = self.contract_id.to_protobuf();

        services::Query {
            query: Some(services::query::Query::ContractGetBytecode(
                services::ContractGetBytecodeQuery { contract_id, header: Some(header) },
            )),
        }
    }
}

impl QueryExecute for ContractBytecodeQueryData {
    type Response = Vec<u8>;

    fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> BoxGrpcFuture<'_, services::Response> {
        Box::pin(async {
            SmartContractServiceClient::new(channel).contract_get_bytecode(request).await
        })
    }
}

impl ValidateChecksums for ContractBytecodeQueryData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        self.contract_id.validate_checksums(ledger_id)
    }
}

impl FromProtobuf<services::response::Response> for Vec<u8> {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let pb = pb_getv!(pb, ContractGetBytecodeResponse, services::response::Response);

        Ok(pb.bytecode)
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::query::ToQueryProtobuf;
    use crate::{
        ContractBytecodeQuery,
        Hbar,
    };

    #[test]
    fn serialize() {
        expect![[r#"
            Query {
                query: Some(
                    ContractGetBytecode(
                        ContractGetBytecodeQuery {
                            header: Some(
                                QueryHeader {
                                    payment: None,
                                    response_type: AnswerOnly,
                                },
                            ),
                            contract_id: Some(
                                ContractId {
                                    shard_num: 0,
                                    realm_num: 0,
                                    contract: Some(
                                        ContractNum(
                                            5005,
                                        ),
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            }
        "#]]
        .assert_debug_eq(
            &ContractBytecodeQuery::new()
                .contract_id(crate::ContractId {
                    shard: 0,
                    realm: 0,
                    num: 5005,
                    evm_address: None,
                    checksum: None,
                })
                .max_payment_amount(Hbar::from_tinybars(100_000))
                .data
                .to_query_protobuf(Default::default()),
        );
    }
}
