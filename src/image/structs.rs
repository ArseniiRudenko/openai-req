use std::io::Error;
use std::io::ErrorKind::NotFound;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use reqwest::multipart::Part;
use async_trait::async_trait;
use crate::{AsyncTryFrom, file_to_part};


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

#[derive(Debug,Clone, Serialize)]
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

#[derive(Debug,Clone)]
pub struct ImageVariationRequest {
    image: PathBuf,
    n: Option<u32>,
    size: Option<String>,
    user: Option<String>
}

#[derive(Debug,Clone,Deserialize)]
pub struct ImageResponse {
    pub created: i64,
    pub data: Vec<ImageData>,
}

#[derive(Debug,Clone,Deserialize)]
pub struct ImageData {
    pub url: String,
}

#[derive(Debug,Clone, Serialize)]
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

impl ImageEditRequest{
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

   pub fn with_mask(mut self, mask: PathBuf) ->  Result<Self,Error> {
        if mask.exists() {
            self.mask = Some(mask);
            return Ok(self)
        }
        Err(Error::new(NotFound, "File does not exist"))
   }

   pub fn with_n(mut self, n: i32) -> Self {
        self.n = Some(n);
        self
   }

   pub fn with_size(mut self, size: ImageSize) -> Self {
        self.size = Some(size);
        self
   }

   pub fn with_response_format(mut self, response_format: String) -> Self {
        self.response_format = Some(response_format);
        self
   }

   pub fn with_user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
   }

}



#[async_trait]
impl AsyncTryFrom<ImageEditRequest> for reqwest::multipart::Form {

    type Error = Error;

    async fn try_from(request: ImageEditRequest) -> Result<Self, Self::Error> {
        let mut form = reqwest::multipart::Form::new()
            .part("image", file_to_part(&request.image).await?)
            .part("prompt", Part::text(request.prompt));

        if let Some(mask) = request.mask {
            form = form.part("mask", file_to_part(&mask).await?);
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


impl ImageVariationRequest {
    pub fn new(image: PathBuf) -> Self {
        ImageVariationRequest {
            image,
            n: None,
            size: None,
            user: None,
        }
    }

    pub fn n(mut self, n: u32) -> Self {
        self.n = Some(n);
        self
    }

    pub fn size(mut self, size: String) -> Self {
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
