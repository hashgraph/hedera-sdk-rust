use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use tonic::transport::Channel;

use crate::query::{AnyQueryData, QueryExecute, ToQueryProtobuf};
use crate::{ContractBytecodeResponse, ContractId, Query, ToProtobuf};

/// Get information about a smart contract instance.
pub type ContractInfoQuery = Query<ContractInfoQueryData>;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractInfoQueryData {
    /// The contract for which information is requested.
    contract_id: Option<ContractId>,
}

impl ContractInfoQuery {
    /// Sets the contract for which information is requested.
    pub fn contract_id(&mut self, contract_id: ContractId) -> &mut Self {
        self.data.contract_id = Some(contract_id);
        self
    }
}

impl From<ContractInfoQueryData> for AnyQueryData {
    #[inline]
    fn from(data: ContractInfoQueryData) -> Self {
        Self::ContractInfo(data)
    }
}

impl ToQueryProtobuf for ContractInfoQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let contract_id = self.contract_id.as_ref().map(|id| id.to_protobuf());

        services::Query {
            query: Some(services::query::Query::ContractGetInfo(services::ContractGetInfoQuery {
                contract_id,
                header: Some(header),
            })),
        }
    }
}

#[async_trait]
impl QueryExecute for ContractInfoQueryData {
    type Response = ContractBytecodeResponse;

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        SmartContractServiceClient::new(channel).get_contract_info(request).await
    }
}
