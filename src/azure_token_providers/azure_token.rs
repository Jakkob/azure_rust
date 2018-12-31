use super::weird_azure_date_formater;

use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct AzureToken {
    #[serde(rename = "accessToken")]
    pub access_token: String,

    #[serde(rename = "expiresOn", with = "weird_azure_date_formater")]
    pub expires_on: DateTime<Utc>,

    #[serde(rename = "tokenType")]
    pub token_type: String,

    pub tenant: String,
    pub subscription: String,
}
