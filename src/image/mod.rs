use crate::{FormClient, OpenAiClient, JsonRequestClient};
use crate::image::structs::{ImageEditRequest, ImageRequest, ImageResponse, ImageVariationRequest};

pub mod structs;

impl<'a> FormClient<'a,ImageEditRequest,ImageResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/images/edits";
}

impl<'a> FormClient<'a,ImageVariationRequest,ImageResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/images/variations";
}

impl JsonRequestClient<ImageRequest,ImageResponse> for OpenAiClient {
    const ENDPOINT: &'static str = "/images/generations";
}

