use super::azure_token::AzureToken;
use super::TokenProvider;

use std::collections::HashMap;
use std::env;
use std::io;

pub struct ServicePrincipalTokenProvider {
    cached_token: AzureToken,
    client: reqwest::Client,
}

#[allow(non_upper_case_globals)]
mod env_var_keys {
    pub const tenant_id: &str = "TENANT_ID";
    pub const client_id: &str = "CLIENT_ID";
    pub const client_secret: &str = "CLIENT_SECRET";
}

pub fn has_env_vars() -> bool {
    match ServicePrincipalEnvVars::get_env_vars() {
        Ok(_) => true,
        Err(_) => false,
    }
}

struct ServicePrincipalEnvVars {
    pub tenant_id: String,
    pub client_id: String,
    pub client_secret: String,
}

impl ServicePrincipalEnvVars {
    pub fn get_env_vars() -> Result<ServicePrincipalEnvVars, env::VarError> {
        return Ok(ServicePrincipalEnvVars {
            tenant_id: env::var(env_var_keys::tenant_id)?,
            client_id: env::var(env_var_keys::client_id)?,
            client_secret: env::var(env_var_keys::client_secret)?,
        });
    }
}

fn get_service_principal_access_token() -> Result<AzureToken, io::Error> {
    let env_vars: ServicePrincipalEnvVars = ServicePrincipalEnvVars::get_env_vars().unwrap();
    let endpoint: String = format!(
        "https://login.microsoftonline.com/{}/oauth2/token",
        &env_vars.tenant_id
    );

    let endpoint_ref: &str = endpoint.as_ref();

    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    params.insert("resource", "https://management.azure.com/");
    params.insert("client_id", &env_vars.client_id);
    params.insert("client_secret", &env_vars.client_secret);

    let client = reqwest::Client::new();
    let response = client
        .post(endpoint_ref)
        .form(&params)
        .send()
        .unwrap()
        .json::<AzureToken>()
        .unwrap();

    return Ok(response);
}

impl ServicePrincipalTokenProvider {
    pub fn new(_client: reqwest::Client) -> ServicePrincipalTokenProvider {
        ServicePrincipalTokenProvider {
            cached_token: get_service_principal_access_token().unwrap(),
            client: _client,
        }
    }
}

impl TokenProvider for ServicePrincipalTokenProvider {
    fn get_token(&mut self) -> &AzureToken {
        if chrono::Utc::now() < self.cached_token.expires_on {
            return &self.cached_token;
        }

        self.cached_token = get_service_principal_access_token().unwrap();
        return &self.cached_token;
    }
}
