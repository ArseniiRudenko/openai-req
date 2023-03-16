pub mod structs;

use reqwest::Client;
use super::{OpenAiClient, PostClient};
use structs::{CompletionRequest, CompletionSuccess};
use async_trait::async_trait;

#[async_trait(?Send)]
impl PostClient<CompletionRequest, CompletionSuccess> for OpenAiClient {

    const ENDPOINT: &'static str = "/completions";

    fn client(&self) -> Client {
        return self.client.clone()
    }

    fn key(&self) -> &str {
        return self.key.as_str()
    }

    fn url(&self) -> &str {
        return self.url.as_str()
    }
}