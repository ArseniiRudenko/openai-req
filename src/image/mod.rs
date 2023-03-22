use crate::{ FormRequest, JsonRequest};

use std::io::Error;
use std::io::ErrorKind::NotFound;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use reqwest::multipart::Part;
use async_trait::async_trait;
use tokio::try_join;
use crate::conversions::AsyncTryFrom;
use crate::file_to_part;

///Generates image from text prompt.
///Details at https://platform.openai.com/docs/api-reference/images/create
/// # Usage example
///```
/// use openai_req::image::ImageRequest;
/// use openai_req::JsonRequest;
///
/// let prompt = "cool company logo".to_string();
/// let req = ImageRequest::new(prompt);
/// let res = req.run(&client).await?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageRequest {
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<ImageSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
}

impl JsonRequest<ImageResponse> for ImageRequest{
    const ENDPOINT: &'static str = "/images/generations";
}

impl ImageRequest {
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            n: None,
            size: None,
            response_format: None,
            user: None,
        }
    }

    pub fn n(mut self, n: u32) -> Self {
        self.n = Some(n);
        self
    }

    pub fn size(mut self, size: ImageSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn response_format(mut self, response_format: String) -> Self {
        self.response_format = Some(response_format);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }
}

///Generates image edit for provided image.
///Details at https://platform.openai.com/docs/api-reference/images/create-edit
/// # Usage example
///```
/// use openai_req::image::ImageEditRequest;
/// use std::path::PathBuf;
/// use openai_req::FormRequest;
///
/// let image_path = PathBuf::from("tests/generated.png");
/// let mask_path = PathBuf::from("tests/mask.png");
/// let prompt = "remove text".to_string();
/// let req = ImageEditRequest::new(image_path,prompt)?
///         .mask(mask_path)?;
/// let res = req.run(&client).await?;
/// ```
#[derive(Debug,Clone)]
pub struct ImageEditRequest {
    image: PathBuf,
    mask: Option<PathBuf>,
    prompt: String,
    n: Option<i32>,
    size: Option<ImageSize>,
    response_format: Option<String>,
    user: Option<String>
}

impl FormRequest<ImageResponse> for ImageEditRequest{
    const ENDPOINT: &'static str = "/images/edits";
}

impl ImageEditRequest{

    /// Will check if provided path exists, and return io::Error if it does not.
    pub fn new(image:PathBuf, prompt: String) -> Result<Self,Error> {
        if image.exists() {
            return Ok(
                Self {
                    image,
                    mask: None,
                    prompt,
                    n: None,
                    size: None,
                    response_format: None,
                    user: None,
                })
        }
        Err(Error::new(NotFound, "File does not exist"))
    }

    /// Will check if provided path exists, and return io::Error if it does not.
    pub fn mask(mut self, mask: PathBuf) ->  Result<Self,Error> {
        if mask.exists() {
            self.mask = Some(mask);
            return Ok(self)
        }
        Err(Error::new(NotFound, "File does not exist"))
    }

    pub fn n(mut self, n: i32) -> Self {
        self.n = Some(n);
        self
    }

    pub fn size(mut self, size: ImageSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn response_format(mut self, response_format: String) -> Self {
        self.response_format = Some(response_format);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }

}

#[async_trait]
impl AsyncTryFrom<ImageEditRequest> for reqwest::multipart::Form {

    type Error = Error;

    async fn try_from(request: ImageEditRequest) -> Result<Self, Self::Error> {
        let mut form = reqwest::multipart::Form::new()
            .part("prompt", Part::text(request.prompt));

        if let Some(mask) = request.mask {
            let (mask,image)= try_join!(file_to_part(&mask),file_to_part(&request.image))?;
            form =
                form.part("image", image)
                    .part("mask", mask);
        }else {
            form = form.part("image", file_to_part(&request.image).await?)
        }
        if let Some(n) = request.n {
            form = form.part("n", Part::text(n.to_string()));
        }
        if let Some(size) = request.size {
            form = form.part("size", Part::text(size.to_string()));
        }
        if let Some(response_format) = request.response_format {
            form = form.part("response_format", Part::text(response_format));
        }
        if let Some(user) = request.user {
            form = form.part("user", Part::text(user));
        }
        return Ok(form)
    }
}

///Generates variation for provided image.
///Details at https://platform.openai.com/docs/api-reference/images/create-variation
/// # Usage example
///```
/// use std::path::PathBuf;
/// use openai_req::FormRequest;
/// use openai_req::image::ImageVariationRequest;
///
/// let image_path = PathBuf::from("tests/generated.png");
/// let req = ImageVariationRequest::new(image_path)?;
/// let res = req.run(&client).await?;
/// ```
#[derive(Debug,Clone)]
pub struct ImageVariationRequest {
    image: PathBuf,
    n: Option<u32>,
    size: Option<ImageSize>,
    user: Option<String>
}

impl FormRequest<ImageResponse> for ImageVariationRequest{
    const ENDPOINT: &'static str = "/images/variations";
}

impl ImageVariationRequest {
    pub fn new(image: PathBuf) -> Result<Self,Error> {
        if image.exists() {
            return Ok(
                Self {
                    image,
                    n: None,
                    size: None,
                    user: None,
                }
            )
        }
        Err(Error::new(NotFound, "File does not exist"))
    }

    pub fn n(mut self, n: u32) -> Self {
        self.n = Some(n);
        self
    }

    pub fn size(mut self, size: ImageSize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }
}

#[async_trait]
impl AsyncTryFrom<ImageVariationRequest> for reqwest::multipart::Form {

    type Error = Error;

    async fn try_from(request: ImageVariationRequest) -> Result<Self, Self::Error> {
        let mut form = reqwest::multipart::Form::new()
            .part("image", file_to_part(&request.image).await?);
        if let Some(n) = request.n {
            form = form.part("n", Part::text(n.to_string()));
        }
        if let Some(size) = request.size {
            form = form.part("size", Part::text(size.to_string()));
        }
        if let Some(user) = request.user {
            form = form.part("user", Part::text(user));
        }
        return Ok(form)
    }
}


#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct ImageResponse {
    pub created: i64,
    pub data: Vec<ImageData>,
}

#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct ImageData {
    pub url: String,
}

#[derive(Debug,Clone, Serialize, Deserialize)]
pub enum ImageSize {
    #[serde(rename = "256x256")]
    S256,
    #[serde(rename = "512x512")]
    S512,
    #[serde(rename = "1024x1024")]
    S1024,
}

impl ToString for ImageSize {
    fn to_string(&self) -> String {
        match *self {
            ImageSize::S256 => String::from("256x256"),
            ImageSize::S512 => String::from("512x512"),
            ImageSize::S1024 => String::from("1024x1024"),
        }
    }
}

impl ImageSize {
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "256x256" => Some(ImageSize::S256),
            "512x512" => Some(ImageSize::S512),
            "1024x1024" => Some(ImageSize::S1024),
            _ => None,
        }
    }
}


