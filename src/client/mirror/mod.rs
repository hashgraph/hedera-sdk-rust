use std::net::SocketAddr;

use arc_swap::access::Access;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{
    Json,
    Router,
};
use hyper::Response;
use serde_json::Value;
use triomphe::Arc;

use crate::client::MirrorNetworkData;
use crate::{
    ledger_id,
    AccountId,
    AccountInfo,
    Client,
    EvmAddress,
    LedgerId,
};

const LOCAL_NODE_PORT: &str = "5551";

use std::ops::Deref;

use crate::ArcSwap;

#[derive(Default)]
pub(crate) struct MirrorNodeGateway {
    inner: ArcSwap<MirrorNodeGatewayInner>,
}

// todo: Add client
#[derive(Clone, Default)]
pub(crate) struct MirrorNodeGatewayInner {
    mirror_node_url: String,
}

impl MirrorNodeGateway {
    // Set client in the outer later
    fn for_client(&self, client: Client) -> Self {
        let ledger_id: Option<LedgerId> = client.ledger_id_internal().load().as_ref().map(|id| (**id).clone());

        let mirror_node_url = MirrorNodeRouter::get_mirror_node_url(
            client.mirror_network(),
            ledger_id        )
        .unwrap();

        // initiate Mirror node gateway
        MirrorNodeGateway {
            inner: ArcSwap::new(Arc::new(MirrorNodeGatewayInner {
                mirror_node_url,
            })),
        }    
    }

    fn for_network(&self, mirror_network: Vec<String>, ledger_id: Option<LedgerId>) -> Self {
        let mirror_node_url =
            MirrorNodeRouter::get_mirror_node_url(mirror_network, ledger_id).unwrap();

        // initiate Mirror node gateway
        // initiate Mirror node gateway
        MirrorNodeGateway {
            inner: ArcSwap::new(Arc::new(MirrorNodeGatewayInner {
                mirror_node_url,
            })),
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

        let info: Value = serde_json::from_str(&response_body).unwrap();
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
            MirrorRoutes::AccountsRoute => "/accounts/{id}".to_string(),
            MirrorRoutes::ContractsRoute => "/contracts/{id}".to_string(),
            MirrorRoutes::AccountTokensRoute => "/accounts/{id}/tokens".to_string(),
        }
    }
}

impl MirrorNodeRouter {
    fn get_mirror_node_url(
        mirror_network: Vec<String>,
        ledger_id: Option<LedgerId>,
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

enum MirrorNodeRoutes {
    Accounts,
    Contracts,
    AccountTokens,
}
