pub mod chat;
pub mod completion;
pub mod edit;
pub mod structs;

use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use structs::FilesResponse;
use crate::structs::ApiResponse;

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
        return OpenAiClient::with_client(key,&client);

    }

    /// reqwest library recommends reusing single client,
    /// so if you run access to multiple api-s, pass client into constructor
    pub fn with_client(key: &str, client: &Client)->Self{
        return  OpenAiClient::with_url_and_client(key,OpenAiClient::URL,client);
    }

    pub fn with_url(key: &str, url: &str) -> Self {
        let client = Client::new();
        return  OpenAiClient::with_url_and_client(key,url,&client)
    }


    pub fn with_url_and_client(key: &str, url: &str, client: &Client)->Self{
        OpenAiClient {
            url: url.to_string(),
            key: key.to_string(),
            client: client.clone()
        }
    }
}

#[derive(Debug)]
pub enum Error{
    LibErr(reqwest::Error),
    ApiErr(StatusCode,String)
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::LibErr(value)
    }
}


#[async_trait(?Send)]
pub trait PostClient<'a, TReq: Serialize + Sized +'a,TRes: DeserializeOwned>{

    const ENDPOINT: &'static str;

    fn get_client(&self)->Client;
    fn get_key(&self)->&str;
    fn get_url(&self)->&str;

    async fn run(&'a self, req:TReq)-> Result<ApiResponse<TRes>,Error>{
        let final_url = self.get_url().to_owned()+Self::ENDPOINT;
        self.get_client().post(final_url)
            .bearer_auth(self.get_key())
            .json(&req)
            .send()
            .await?.json::<ApiResponse<TRes>>().await.map_err(|e|Error::from(e))
    }
}

#[async_trait(?Send)]
trait GetClient<TRes: DeserializeOwned>{

    const ENDPOINT: &'static str;

    fn get_client(&self)->reqwest::Client;
    fn get_key(&self)->&str;
    fn get_url(&self)->&str;

    async fn get(&self)-> Result<TRes,reqwest::Error>{
        let final_url = self.get_url().to_owned()+Self::ENDPOINT;
        return self.get_client().get(final_url)
            .bearer_auth(self.get_key())
            .send()
            .await?
            .json::<TRes>()
            .await;
    }
}

#[async_trait(?Send)]
impl GetClient<FilesResponse> for OpenAiClient {
    const ENDPOINT: &'static str = "/files";

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