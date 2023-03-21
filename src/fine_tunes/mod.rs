use crate::fine_tunes::structs::{FineTuneCreateRequest, FineTune, FineTuneListResponse, FineTuneEventsResponse, FineTuneGetRequest, FineTuneCancelRequest, FineTuneEventsGetRequest};
use crate::{GetRequest, JsonRequest, ByUrlRequest, OpenAiClient};
pub mod structs;
use reqwest::RequestBuilder;

impl GetRequest for FineTuneListResponse {
    const ENDPOINT: &'static str = "/fine-tunes";
}

impl JsonRequest<FineTune> for FineTuneCreateRequest{
    const ENDPOINT: &'static str = "/fine-tunes";
}

impl ByUrlRequest<FineTune> for FineTuneGetRequest {
    const ENDPOINT: &'static str = "/fine-tunes/";
    const SUFFIX: &'static str = "";
}

impl ByUrlRequest<FineTune> for FineTuneCancelRequest {
    const ENDPOINT: &'static str = "/fine-tunes/";
    const SUFFIX: &'static str = "/cancel";

    fn builder(client:&OpenAiClient, final_url: String) -> RequestBuilder {
        client.client.post(final_url)
    }
}

impl ByUrlRequest<FineTuneEventsResponse> for FineTuneEventsGetRequest{
    const ENDPOINT: &'static str = "/fine-tunes/";
    const SUFFIX: &'static str = "/events";
}


