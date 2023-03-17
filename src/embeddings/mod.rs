use reqwest::Client;
use crate::embeddings::structs::{EmbeddingRequest, EmbeddingResponse};
use crate::{OpenAiClient, PostClient};

pub mod structs;

impl PostClient<EmbeddingRequest,EmbeddingResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/embeddings";

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
