use serde_json::Value;
use triomphe::Arc;

use crate::{
    Client,
    Error,
    LedgerId,
};

const LOCAL_NODE_PORT: &str = "5551";

use crate::ArcSwap;

#[derive(Default)]
pub struct MirrorNodeGateway {
    inner: ArcSwap<MirrorNodeGatewayInner>,
}

// todo: Add client
#[derive(Clone, Default)]
pub struct MirrorNodeGatewayInner {
    mirror_node_url: String,
}

impl MirrorNodeGateway {
    // Set client in the outer later
    pub fn for_client(client: Client) -> Self {
        let binding = client.ledger_id_internal();
        let ledger_id = binding.as_ref();

        let mirror_node_url =
            MirrorNodeRouter::get_mirror_node_url(client.mirror_network(), ledger_id).unwrap();

        // initiate Mirror node gateway
        MirrorNodeGateway {
            inner: ArcSwap::new(Arc::new(MirrorNodeGatewayInner { mirror_node_url })),
        }
    }

    fn for_network(&self, mirror_network: Vec<String>, ledger_id: Option<&Arc<LedgerId>>) -> Self {
        let mirror_node_url =
            MirrorNodeRouter::get_mirror_node_url(mirror_network, ledger_id).unwrap();

        // initiate Mirror node gateway
        MirrorNodeGateway {
            inner: ArcSwap::new(Arc::new(MirrorNodeGatewayInner { mirror_node_url })),
        }
    }

    // Gets called in MirrorNodeGateway
    async fn get_account_info(&self, id_or_address: String) -> Value {
        let api_url = MirrorNodeRouter::build_api_url(
            self.inner.load().mirror_node_url.clone(),
            MirrorRoutes::AccountsRoute,
            id_or_address,
        );

        let response_body = self.query_from_mirror_node(api_url).await;

        let info = serde_json::from_str(&response_body).unwrap();
        info
    }

    // todo: Return Result instead of unwrapping
    async fn query_from_mirror_node(&self, api_url: String) -> String {
        reqwest::get(api_url).await.unwrap().text().await.unwrap()
    }
}

struct MirrorNodeRouter {}

enum MirrorRoutes {
    AccountsRoute,
    ContractsRoute,
    AccountTokensRoute,
}

impl MirrorRoutes {
    fn to_route(&self, id: String) -> String {
        match self {
            MirrorRoutes::AccountsRoute => format!("/accounts/{}", id),
            MirrorRoutes::ContractsRoute => format!("/contracts/{}", id),
            MirrorRoutes::AccountTokensRoute => format!("/accounts/{}/tokens", id),
        }
    }
}

impl MirrorNodeRouter {
    fn get_mirror_node_url(
        mirror_network: Vec<String>,
        ledger_id: Option<&Arc<LedgerId>>,
    ) -> crate::Result<String> {
        let mut address: Vec<String> = Vec::new();

        for url in mirror_network {
            if let Some(it) = url.find(":") {
                address.push(url[..it].to_string());
                break;
            }
        }

        if address.is_empty() {
            return crate::Result::Err(crate::Error::AddressNotFound);
        }

        if ledger_id.is_some() {
            return Ok(format!("https://{}", address[0]));
        } else {
            return Ok(format!("http://{}:{}", address[0], LOCAL_NODE_PORT));
        }
    }

    fn build_api_url(mirror_node_url: String, route: MirrorRoutes, id: String) -> String {
        format!("{mirror_node_url}/api/v1{}", route.to_route(id))
    }
}

pub struct MirrorNodeService {
    mirror_node_gateway: MirrorNodeGateway,
}

impl MirrorNodeService {
    pub fn new(mirror_node_gateway: MirrorNodeGateway) -> Self {
        MirrorNodeService { mirror_node_gateway }
    }

    pub async fn get_account_num(&self, evm_address: String) -> crate::Result<u64> {
        let expecting = || Error::basic_parse(format!("Could not parse data"));

        let account_info = self.mirror_node_gateway.get_account_info(evm_address).await;

        let num = account_info.get("account").map(|it| it.as_str().unwrap()).unwrap();

        if let Some(index) = num.rfind('.') {
            let substring = &num[index + 1..];
            substring.parse::<u64>().map_err(|e| crate::Error::BasicParse(Box::new(e)))
        } else {
            crate::Result::Err(expecting())
        }
    }

    // async fn get_account_evm_adress(&self, num: u64) -> EvmAddress {

    // }
}
