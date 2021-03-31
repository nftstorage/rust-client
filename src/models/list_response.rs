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
pub struct ListResponse {
    #[serde(rename = "ok", skip_serializing_if = "Option::is_none")]
    pub ok: Option<bool>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<crate::models::Nft>>,
}

impl ListResponse {
    pub fn new() -> ListResponse {
        ListResponse {
            ok: None,
            value: None,
        }
    }
}

