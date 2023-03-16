pub mod structs;

use reqwest::Client;
use structs::{ChatRequest, ChatSuccess};
use super::{OpenAiClient, PostClient};
use async_trait::async_trait;

#[async_trait(?Send)]
impl PostClient<ChatRequest, ChatSuccess> for OpenAiClient {

    const ENDPOINT: &'static str = "/chat/completions";

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
