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
pub struct UploadResponse {
    #[serde(rename = "ok", skip_serializing_if = "Option::is_none")]
    pub ok: Option<bool>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<crate::models::Nft>>,
}

impl UploadResponse {
    pub fn new() -> UploadResponse {
        UploadResponse {
            ok: None,
            value: None,
        }
    }
}

