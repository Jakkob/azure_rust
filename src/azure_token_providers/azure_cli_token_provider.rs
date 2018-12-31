use super::azure_token::AzureToken;
use super::TokenProvider;
use std::process::Command;

pub struct CliTokenProvider {
    cached_token: AzureToken,
}

fn get_azure_cli_token() -> AzureToken {
    let output = Command::new("powershell.exe")
        .args(&[
            "-NonInteractive",
            "-Command",
            "az",
            "account",
            "get-access-token",
        ])
        .output()
        .expect("looks like the CLI command didn't work..");

    let std_out = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(&std_out).unwrap()
}

impl CliTokenProvider {
    pub fn new() -> CliTokenProvider {
        CliTokenProvider {
            cached_token: get_azure_cli_token(),
        }
    }
}

impl TokenProvider for CliTokenProvider {
    fn get_token(&mut self) -> &AzureToken {
        if chrono::Utc::now() < self.cached_token.expires_on {
            return &self.cached_token;
        }

        self.cached_token = get_azure_cli_token();
        &self.cached_token
    }
}
