use reqwest::Client;
use structs::FilesResponse;
use super::{GetClient, OpenAiClient};
use async_trait::async_trait;
pub mod structs;


#[async_trait(?Send)]
impl GetClient<FilesResponse> for OpenAiClient {
    const ENDPOINT: &'static str = "/files";

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