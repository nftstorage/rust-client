/*
 * NFT Storage API
 *
 * # API Reference 
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Deal {
    #[serde(rename = "batchRootCid", skip_serializing_if = "Option::is_none")]
    pub batch_root_cid: Option<String>,
    /// This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: YYYY-MM-DDTHH:MM:SSZ.
    #[serde(rename = "lastChange")]
    pub last_change: String,
    /// Miner ID
    #[serde(rename = "miner", skip_serializing_if = "Option::is_none")]
    pub miner: Option<String>,
    /// Filecoin network for this Deal
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<Network>,
    /// Piece CID string
    #[serde(rename = "pieceCid", skip_serializing_if = "Option::is_none")]
    pub piece_cid: Option<String>,
    /// Deal Status
    #[serde(rename = "status")]
    pub status: Status,
    /// Deal Status Description
    #[serde(rename = "statusText", skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,
    /// This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: YYYY-MM-DDTHH:MM:SSZ.
    #[serde(rename = "dealActivation", skip_serializing_if = "Option::is_none")]
    pub deal_activation: Option<String>,
    /// This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: YYYY-MM-DDTHH:MM:SSZ.
    #[serde(rename = "dealExpiration", skip_serializing_if = "Option::is_none")]
    pub deal_expiration: Option<String>,
}

impl Deal {
    pub fn new(last_change: String, status: Status) -> Deal {
        Deal {
            batch_root_cid: None,
            last_change,
            miner: None,
            network: None,
            piece_cid: None,
            status,
            status_text: None,
            deal_activation: None,
            deal_expiration: None,
        }
    }
}

/// Filecoin network for this Deal
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Network {
    #[serde(rename = "nerpanet")]
    Nerpanet,
    #[serde(rename = "mainnet")]
    Mainnet,
}
/// Deal Status
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "proposing")]
    Proposing,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "published")]
    Published,
    #[serde(rename = "terminated")]
    Terminated,
}

