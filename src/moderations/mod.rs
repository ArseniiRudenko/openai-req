use reqwest::Client;
use crate::moderations::structs::{ModerationRequest, ModerationResponse};
use crate::{OpenAiClient, PostClient};

pub mod structs;

impl PostClient<ModerationRequest,ModerationResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/moderations";

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