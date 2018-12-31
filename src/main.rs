#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod azure_token_providers;

use azure_token_providers::TokenResolver;

fn main() {
    let mut token_resolver = TokenResolver::new();
    let token = token_resolver.resolve_token();
    println!("Exp: {}\r\nToken: {}", token.expires_on, token.access_token);
}
