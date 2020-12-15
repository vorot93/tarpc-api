use primitive_types::H256;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForkData {
    pub genesis: H256,
    pub forks: BTreeSet<u64>,
}

#[tarpc::service]
pub trait EthApi {
    async fn forks() -> Result<ForkData, String>;
}
