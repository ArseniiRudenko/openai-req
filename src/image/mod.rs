use reqwest::Client;
use crate::{FormClient, OpenAiClient, PostClient};
use crate::image::structs::{ImageEditRequest, ImageRequest, ImageResponse, ImageVariationRequest};

pub mod structs;


impl<'a> FormClient<'a,ImageEditRequest,ImageResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/images/edits";

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


impl<'a> FormClient<'a,ImageVariationRequest,ImageResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/images/variations";

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

impl PostClient<ImageRequest,ImageResponse> for OpenAiClient {
    const ENDPOINT: &'static str = "/images/generations";

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

