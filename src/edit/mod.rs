pub mod structs;

use reqwest::Client;
use crate::{OpenAiClient, PostClient};
use async_trait::async_trait;
use self::structs::{EditRequest,EditResponse};

#[async_trait(?Send)]
impl<'a> PostClient<'a,EditRequest, EditResponse> for OpenAiClient {

    const ENDPOINT: &'static str = "/edits";

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