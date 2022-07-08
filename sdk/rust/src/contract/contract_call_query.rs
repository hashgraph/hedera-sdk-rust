use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use serde::{
    Deserialize,
    Serialize,
};
use serde_with::base64::Base64;
use serde_with::{
    serde_as,
    skip_serializing_none,
};
use tonic::transport::Channel;

use crate::query::{
    AnyQueryData,
    QueryExecute,
    ToQueryProtobuf,
};
use crate::{
    AccountAddress,
    ContractCallResponse,
    ContractId,
    Query,
    ToProtobuf,
};

/// Call a function of the given smart contract instance.
pub type ContractCallQuery = Query<ContractCallQueryData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContractCallQueryData {
    contract_id: Option<ContractId>,

    gas_limit: u64,

    #[serde_as(as = "Base64")]
    data: Vec<u8>,

    sender_id: Option<AccountAddress>,
}

impl ContractCallQuery {
    /// Sets the contract to make a static call against.
    pub fn contract_id(&mut self, contract_id: ContractId) -> &mut Self {
        self.data.contract_id = Some(contract_id);
        self
    }

    /// Sets the gas limit for this call.
    pub fn gas_limit(&mut self, gas: u64) -> &mut Self {
        self.data.gas_limit = gas;
        self
    }

    /// Sets the data for this transaction.
    pub fn data(&mut self, data: Vec<u8>) -> &mut Self {
        self.data.data = data;
        self
    }

    /// Sets the sender for this transaction.
    pub fn sender_id(&mut self, sender_id: impl Into<AccountAddress>) -> &mut Self {
        self.data.sender_id = Some(sender_id.into());
        self
    }
}

impl From<ContractCallQueryData> for AnyQueryData {
    #[inline]
    fn from(data: ContractCallQueryData) -> Self {
        Self::ContractCall(data)
    }
}

impl ToQueryProtobuf for ContractCallQueryData {
    fn to_query_protobuf(&self, header: services::QueryHeader) -> services::Query {
        let contract_id = self.contract_id.as_ref().map(|id| id.to_protobuf());
        let sender_id = self.sender_id.as_ref().map(|id| id.to_protobuf());

        services::Query {
            query: Some(services::query::Query::ContractCallLocal(
                #[allow(deprecated)]
                services::ContractCallLocalQuery {
                    contract_id,
                    gas: self.gas_limit as i64,
                    function_parameters: self.data.clone(),
                    max_result_size: 0,
                    header: Some(header),
                    sender_id,
                },
            )),
        }
    }
}

#[async_trait]
impl QueryExecute for ContractCallQueryData {
    type Response = ContractCallResponse;

    async fn execute(
        &self,
        channel: Channel,
        request: services::Query,
    ) -> Result<tonic::Response<services::Response>, tonic::Status> {
        SmartContractServiceClient::new(channel).contract_call_local_method(request).await
    }
}
