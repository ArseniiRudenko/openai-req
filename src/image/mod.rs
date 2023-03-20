use crate::{ FormRequest, JsonRequest};
use crate::image::structs::{ImageEditRequest, ImageRequest, ImageResponse, ImageVariationRequest};

pub mod structs;

impl FormRequest<ImageResponse> for ImageEditRequest{
    const ENDPOINT: &'static str = "/images/edits";
}

impl FormRequest<ImageResponse> for ImageVariationRequest{
    const ENDPOINT: &'static str = "/images/variations";
}

impl JsonRequest<ImageResponse> for ImageRequest{
    const ENDPOINT: &'static str = "/images/generations";
}

