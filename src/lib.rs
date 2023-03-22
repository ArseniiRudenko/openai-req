pub mod chat;
pub mod completion;
pub mod edit;
pub mod image;
pub mod files;
pub mod embeddings;
pub mod fine_tunes;
pub mod moderations;
pub mod audio;
pub mod model;
mod conversions;

use anyhow::Result;
use std::io;
use std::path::PathBuf;
use std::pin::Pin;
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::TryFutureExt;
use reqwest::{Body, Client, multipart, RequestBuilder, Response};
use reqwest::multipart::Part;
use serde::de::DeserializeOwned;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::try_join;
use tokio_stream::{Stream, StreamExt};
use tokio_util::codec::{BytesCodec, FramedRead};
use with_id::WithRefId;
use std::fmt::{Debug, Display, Formatter};
use serde::{Serialize, Deserialize};
use crate::conversions::AsyncTryInto;


/// This is main client structure required for all requests.
/// It is passed as a reference parameter into all API operations.
/// It is also holds actual `reqwest::Client` http client, that performs requests.
/// # Usage example
/// ```
/// use openai_req::OpenAiClient;
///
/// let client = OpenAiClient::new("{YOUR_API_KEY}");
/// ```
#[derive(Debug, Clone)]
pub struct OpenAiClient {
    url:String,
    key:String,
    client:Client
}

impl OpenAiClient {

    const URL: &'static str = "https://api.openai.com/v1";

    ///simplest constructor, uses default https://api.openai.com/v1 url,
    /// and creates new default client with connection pool for connections
    pub fn new(key: &str)->Self{
        let client = Client::new();
        OpenAiClient::with_client(key,&client)
    }

    /// reqwest library recommends re-using single client,
    /// so if you run access to multiple api-s, pass client into constructor.
    /// Also use this constructor if you want to customize your client
    /// (for example set different timeout, or use proxy)
    pub fn with_client(key: &str, client: &Client)->Self{
        OpenAiClient::with_url_and_client(key,OpenAiClient::URL,client)
    }


    ///if you want to change base url from https://api.openai.com/v1 to something else - you can
    pub fn with_url(key: &str, url: &str) -> Self {
        let client = Client::new();
        OpenAiClient::with_url_and_client(key,url,&client)
    }


    /// this constructor allows you to customise everything:  client,
    /// key and base url for all requests
    pub fn with_url_and_client(key: &str, url: &str, client: &Client)->Self{
        OpenAiClient {
            url: url.to_string(),
            key: key.to_string(),
            client: client.clone()
        }
    }
}

///common error type used by api client traits, wraps underlying reqwest::Error,
///but also tries to provide response body, so error is easier to debug
#[derive(Debug)]
pub struct Error{
    pub(crate) response:ErrorResponse,
    pub(crate) inner:reqwest::Error
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.response)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}

/// catch-all for error responses from API, tries to deserialize API error,
/// falls back to string if unable to
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ErrorResponse{
    ApiError(ApiError),
    OtherError(String)
}


impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorResponse::ApiError(a) => write!(f,"{}",a),
            ErrorResponse::OtherError(s) => write!(f,"{}",s)
        }
    }
}

///structure returned by OpenAI for errors
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiError {
    pub error: ApiErrorDetails
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename(serialize = "error"))]
#[serde(rename(deserialize = "error"))]
pub struct ApiErrorDetails {
    pub message: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub param: Option<String>,
    pub code: Option<String>
}

impl Display for ApiError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.error.param {
            None => match &self.error.code {
                None => write!(f,"{}",self.error.message),
                Some(code) => write!(f,"{}, code:{}",self.error.message,code)
            }
            Some(param) => match &self.error.code {
                None => write!(f,"{}, param:{}",self.error.message,param),
                Some(code) => write!(f,"{}, param:{}, code: {}",self.error.message,param,code)
            }
        }
    }
}

///enum used by different requests,
/// it is common for apis ot take either single string or array of tokens
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Input {
    String(String),
    StringArray(Vec<String>)
}

impl From<String> for Input{
    fn from(value:String) -> Self {
        Input::String(value)
    }
}

impl From<&str> for Input{
    fn from(value:&str) -> Self {
        Input::String(value.to_string())
    }
}

impl From<Vec<String>> for Input{
    fn from(value: Vec<String>) -> Self {
        Input::StringArray(value)
    }
}

///common response used by multiple delete API-s
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteResponse {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

///common struct that comes up in responses
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage{
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64
}

#[async_trait]
pub trait JsonRequest<TRes: DeserializeOwned>: Serialize + Sized + Sync{

    const ENDPOINT: &'static str;

    async fn run(&self, client:&OpenAiClient) -> Result<TRes>{
        let final_url = client.url.to_owned()+Self::ENDPOINT;
        let res = client.client.post(final_url)
            .bearer_auth(client.key.clone())
            .json(self)
            .send()
            .await?;
        process_response::<TRes>(res).await
    }
}

#[async_trait]
pub trait ByUrlRequest<TRes: DeserializeOwned>:WithRefId<str>+Sync{

    const ENDPOINT: &'static str;
    const SUFFIX: &'static str;

    fn builder(client:&OpenAiClient,final_url:String)->RequestBuilder{
        client.client.get(final_url)
    }

    async fn run(&self, client:&OpenAiClient)-> Result<TRes>{
        let final_url = client.url.to_owned()+Self::ENDPOINT+self.id()+Self::SUFFIX;
        let res = Self::builder(client,final_url)
            .bearer_auth(client.key.clone())
            .send()
            .await?;
        process_response::<TRes>(res).await
    }
}


#[async_trait]
pub trait GetRequest:DeserializeOwned {

    const ENDPOINT: &'static str;

    async fn get(client:&OpenAiClient)-> Result<Self>{
        let final_url = client.url.to_owned()+Self::ENDPOINT;
        let res = client.client.get(final_url)
            .bearer_auth(client.key.clone())
            .send()
            .await?;
        process_response::<Self>(res).await
    }
}

#[async_trait]
pub trait FormRequest<TRes: DeserializeOwned> : AsyncTryInto<multipart::Form>+Clone+Sync+Send {

    const ENDPOINT: &'static str;

    async fn run(&self, client:&OpenAiClient)-> Result<TRes>{
        let final_url =  client.url.to_owned()+Self::ENDPOINT;
        let res = client.client.post(final_url)
            .bearer_auth(client.key.clone())
            .multipart(AsyncTryInto::try_into(self.clone()).await?)
            .send()
            .await?;
        process_response::<TRes>(res).await
    }
}

#[async_trait(?Send)]
pub trait DownloadRequest: WithRefId<str>{

    const ENDPOINT: &'static str;
    const SUFFIX: &'static str = "";

    async fn download(&self, client:&OpenAiClient) -> Result<Pin<Box<dyn Stream<Item=Result<Bytes, reqwest::Error>>>>>{
        let final_url = client.url.to_owned()+Self::ENDPOINT+self.id()+Self::SUFFIX;
        let res = client.client.get(final_url)
            .bearer_auth(client.key.clone())
            .send()
            .await?;
        let code = res.error_for_status_ref();
        return match code {
            Ok(_) => Ok(Box::pin(res.bytes_stream())),
            Err(err) =>
                Err(Error {
                    response: res.json::<ErrorResponse>().await?,
                    inner: err
                })?
        }
    }

    async fn download_to_file(&self, client:&OpenAiClient, target_path:&str) -> Result<()>{
        let file = File::create(target_path).map_err(anyhow::Error::new);
        let stream = self.download(client);
        let (mut file, mut stream) = try_join!(file, stream)?;
        while let Some(chunk) = stream.next().await {
            file.write_all(&chunk?).await?;
        }
        Ok(())
    }

}

async fn process_response<T:DeserializeOwned>(response: Response) ->Result<T>{
    let code = response.error_for_status_ref();
    match code {
        Ok(_) =>{
            let full = response.text().await?;
            serde_json::from_str(&full)
                .map_err(|err| anyhow::Error::new(err).context(full))
        }
        Err(err) =>
            Err(Error {
                response: response.json::<ErrorResponse>().await?,
                inner: err
            })?
    }
}

pub(crate) async fn file_to_part(path: &PathBuf) -> io::Result<Part> {
    let name = path.file_name()
        .ok_or(io::Error::new(io::ErrorKind::InvalidInput,"filename is not full"))?
        .to_str()
        .ok_or(io::Error::new(io::ErrorKind::InvalidData,"non unicode filename"))?
        .to_owned();
    let file = File::open(path).await?;
    let size = file.metadata().await?.len();
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    Ok(Part::stream_with_length(body,size).file_name(name))
}