pub mod structs;

use reqwest::Client;
use structs::{ChatRequest, ChatResponse};
use super::{OpenAiClient, PostClient};
use async_trait::async_trait;

#[async_trait(?Send)]
impl<'a> PostClient<'a,ChatRequest, ChatResponse> for OpenAiClient {

    const ENDPOINT: &'static str = "/chat/completions";

    fn get_client(&self) -> Client {
        return self.client.clone()
    }

    fn get_key(&self) -> &str {
        return self.key.as_str()
    }

    fn get_url(&self) -> &str {
        return self.url.as_str()
    }
}
