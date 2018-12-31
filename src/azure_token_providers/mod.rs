mod azure_cli_token_provider;
mod azure_token;
mod service_principal_token_provider;
mod weird_azure_date_formater;

use self::azure_cli_token_provider::CliTokenProvider;
pub use self::azure_token::AzureToken;
use self::service_principal_token_provider::ServicePrincipalTokenProvider;

use self::service_principal_token_provider::has_env_vars;

trait TokenProvider {
    fn get_token(&mut self) -> &AzureToken;
}

pub struct TokenResolver {
    provider: Box<dyn TokenProvider>,
}

impl TokenResolver {
    pub fn new() -> Self {
        if has_env_vars() {
            Self {
                provider: Box::new(ServicePrincipalTokenProvider::new()),
            }
        } else {
            Self {
                provider: Box::new(CliTokenProvider::new()),
            }
        }
    }

    pub fn resolve_token(&mut self) -> &AzureToken {
        &mut self.provider.get_token()
    }
}
