use serde::Serialize;
use starknet_api::core::ChainId;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Deployment<'a> {
    chain_id: ChainId,
    services: &'a [Service],
}

impl<'a> Deployment<'a> {
    pub const fn new(chain_id: ChainId, services: &'a [Service]) -> Self {
        Self { chain_id, services }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Service {
    name: ServiceName,
    config_path: &'static str,
    ingress: bool,
    replicas: Replicas,
    storage: Option<usize>,
}

impl Service {
    pub const fn new(
        name: ServiceName,
        config_path: &'static str,
        ingress: bool,
        replicas: Replicas,
        storage: Option<usize>,
    ) -> Self {
        Self { name, config_path, ingress, replicas, storage }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Replicas {
    Single,
    Multiple,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ServiceName {
    Mempool,
    Gateway,
    Batcher,
}
