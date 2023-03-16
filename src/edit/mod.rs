pub mod structs;

use reqwest::Client;
use crate::{OpenAiClient, PostClient};
use async_trait::async_trait;
use self::structs::{EditRequest,EditResponse};

#[async_trait(?Send)]
impl PostClient<EditRequest, EditResponse> for OpenAiClient {

    const ENDPOINT: &'static str = "/edits";

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