use crate::fine_tunes::structs::{FineTuneCreateRequest, FineTune, FineTunesResponse, FineTuneEventsResponse, FineTuneGetRequest, FineTuneCancelRequest, FineTuneEventsGetRequest, FineTuneDeleteRequest};
use crate::{ApiClient, GetClient, OpenAiClient, JsonRequestClient, ByUrlClient};
use crate::structs::DeleteResponse;
pub mod structs;
use reqwest::RequestBuilder;

impl GetClient<FineTunesResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/fine-tunes";
}

impl JsonRequestClient<FineTuneCreateRequest, FineTune> for OpenAiClient{
    const ENDPOINT: &'static str = "/fine-tunes";

}

impl ByUrlClient<FineTuneGetRequest, FineTune> for OpenAiClient {
    const ENDPOINT: &'static str = "/fine-tunes/";
    const SUFFIX: &'static str = "";
}

impl ByUrlClient<FineTuneCancelRequest, FineTune> for OpenAiClient {
    const ENDPOINT: &'static str = "/fine-tunes/";
    const SUFFIX: &'static str = "/cancel";

    fn builder(&self, final_url: String) -> RequestBuilder {
        self.client().post(final_url)
    }
}

impl ByUrlClient<FineTuneEventsGetRequest, FineTuneEventsResponse> for OpenAiClient {
    const ENDPOINT: &'static str = "/fine-tunes/";
    const SUFFIX: &'static str = "/events";
}


impl ByUrlClient<FineTuneDeleteRequest, DeleteResponse> for OpenAiClient {
    const ENDPOINT: &'static str = "/models/";
    const SUFFIX: &'static str = "";

    fn builder(&self, final_url: String) -> RequestBuilder {
        self.client.delete(final_url)
    }
}