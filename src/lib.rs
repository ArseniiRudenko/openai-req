pub mod chat;
pub mod completion;
pub mod edit;
pub mod structs;
pub mod image;
pub mod files;
pub mod embeddings;
pub mod fine_tunes;
pub mod moderations;

use anyhow::Result;
use std::io;
use std::io::{Error, ErrorKind};
use std::path::Path;
use async_trait::async_trait;
use reqwest::{Body, Client};
use reqwest::multipart::Part;
use serde::de::DeserializeOwned;
use serde::ser::StdError;
use serde::Serialize;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use crate::structs::{ApiResponse, ModelsResponse};

#[derive(Debug)]
pub struct OpenAiClient {
    url:String,
    key:String,
    client:Client
}

impl OpenAiClient {

    const URL: &'static str = "https://api.openai.com/v1";

    pub fn new(key: &str)->Self{
        let client = Client::new();
        OpenAiClient::with_client(key,&client)
    }

    /// reqwest library recommends reusing single client,
    /// so if you run access to multiple api-s, pass client into constructor
    pub fn with_client(key: &str, client: &Client)->Self{
        OpenAiClient::with_url_and_client(key,OpenAiClient::URL,client)
    }

    pub fn with_url(key: &str, url: &str) -> Self {
        let client = Client::new();
        OpenAiClient::with_url_and_client(key,url,&client)
    }


    pub fn with_url_and_client(key: &str, url: &str, client: &Client)->Self{
        OpenAiClient {
            url: url.to_string(),
            key: key.to_string(),
            client: client.clone()
        }
    }
}

pub trait ApiClient{
    fn client(&self) ->Client;
    fn key(&self) ->&str;
    fn url(&self) ->&str;
}

impl ApiClient for OpenAiClient {
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


#[async_trait(?Send)]
pub trait PostClient<TReq: Serialize + Sized,TRes: DeserializeOwned>: ApiClient{

    const ENDPOINT: &'static str;

    async fn run(&self, req: &TReq)-> Result<ApiResponse<TRes>>{
        self.run_with_endpoint(Self::ENDPOINT,req).await
    }

    async fn run_with_endpoint(&self, endpoint:&str, req: &TReq) -> Result<ApiResponse<TRes>>{
        let final_url = self.url().to_owned()+endpoint;
        let res = self.client().post(final_url)
            .bearer_auth(self.key())
            .json(req)
            .send()
            .await?
            .json::<ApiResponse<TRes>>()
            .await?;
        Ok(res)
    }
}

#[async_trait]
pub trait GetClient<TRes: DeserializeOwned>: ApiClient{

    const ENDPOINT: &'static str;

    async fn get(&self)-> Result<ApiResponse<TRes>>{
        return self.get_from(Self::ENDPOINT).await
    }

    async fn get_from(&self,endpoint:&str)-> Result<ApiResponse<TRes>>{
        let final_url = self.url().to_owned()+endpoint;
        let res = self.client().get(final_url)
            .bearer_auth(self.key())
            .send()
            .await?
            .json::<ApiResponse<TRes>>()
            .await?;
        Ok(res)
    }
}

#[async_trait(?Send)]
pub trait FormClient<'a, TReq:AsyncTryInto<reqwest::multipart::Form> +Clone+'a,TRes: DeserializeOwned> : ApiClient{

    const ENDPOINT: &'static str;

    async fn run(&'a self, req: &TReq)-> Result<ApiResponse<TRes>>{
        self.run_with_endpoint(Self::ENDPOINT,req).await
    }

    async fn run_with_endpoint(&'a self, endpoint:&str,req: &TReq)-> Result<ApiResponse<TRes>>{
        let final_url = self.url().to_owned()+endpoint;
        let res = self.client().post(final_url)
            .bearer_auth(self.key())
            .multipart(AsyncTryInto::try_into(req.clone()).await?)
            .send()
            .await?
            .json::<ApiResponse<TRes>>()
            .await?;
        Ok(res)
    }
}

#[async_trait(?Send)]
impl GetClient<ModelsResponse> for OpenAiClient {
    const ENDPOINT: &'static str = "/models";
}


#[async_trait]
pub trait AsyncTryFrom<T>: Sized {

    type Error: 'static+StdError+Send+Sync;

    async fn try_from(value: T) -> Result<Self, Self::Error>;
}

#[async_trait]
pub trait AsyncTryInto<T>: Sized {

    type Error: 'static+StdError+Send+Sync;

    async fn try_into(self) -> Result<T, Self::Error>;
}

#[async_trait]
impl<T, U> AsyncTryInto<U> for T
    where
        U: AsyncTryFrom<T>,
        T: Send
{
    type Error = U::Error;

    async fn try_into(self) -> Result<U, Self::Error>{
        U::try_from(self).await
    }
}


pub(crate) async fn file_to_part(path: &Path) -> io::Result<Part> {
    let name = path.file_name()
        .ok_or(Error::new(ErrorKind::InvalidInput,"filename is not full"))?
        .to_str()
        .ok_or(Error::new(ErrorKind::InvalidData,"non unicode filename"))?
        .to_owned();
    let file = File::open(path).await?;
    let size = file.metadata().await?.len();
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    Ok(Part::stream_with_length(body,size).file_name(name))
}