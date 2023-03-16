pub mod chat;
pub mod completion;
pub mod edit;
pub mod structs;

use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use structs::FilesResponse;
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



#[async_trait(?Send)]
pub trait PostClient<TReq: Serialize + Sized,TRes: DeserializeOwned>{

    const ENDPOINT: &'static str;

    fn client(&self) ->Client;
    fn key(&self) ->&str;
    fn url(&self) ->&str;

    async fn run(&self, req: &TReq)-> Result<ApiResponse<TRes>,reqwest::Error>{
        let final_url = self.url().to_owned()+Self::ENDPOINT;
        self.client().post(final_url)
            .bearer_auth(self.key())
            .json(req)
            .send()
            .await?
            .json::<ApiResponse<TRes>>()
            .await
    }
}

#[async_trait(?Send)]
pub trait GetClient<TRes: DeserializeOwned>{

    const ENDPOINT: &'static str;

    fn client(&self) ->Client;
    fn key(&self) ->&str;
    fn url(&self) ->&str;

    async fn get(&self)-> Result<ApiResponse<TRes>,reqwest::Error>{
        let final_url = self.url().to_owned()+Self::ENDPOINT;
        return self.client().get(final_url)
            .bearer_auth(self.key())
            .send()
            .await?
            .json::<ApiResponse<TRes>>()
            .await;
    }
}

#[async_trait(?Send)]
pub trait FormClient<'a, TReq:'a,TRes: DeserializeOwned>
    where reqwest::multipart::Form: From<TReq>{

    const ENDPOINT: &'static str;

    fn client(&self) ->Client;
    fn key(&self) ->&str;
    fn url(&self) ->&str;

    async fn run(&'a self, req:TReq)-> Result<ApiResponse<TRes>,reqwest::Error>{
        let final_url = self.url().to_owned()+Self::ENDPOINT;
        self.client().post(final_url)
            .bearer_auth(self.key())
            .multipart(reqwest::multipart::Form::from(req))
            .send()
            .await?
            .json::<ApiResponse<TRes>>()
            .await
    }
}



#[async_trait(?Send)]
impl GetClient<FilesResponse> for OpenAiClient {
    const ENDPOINT: &'static str = "/files";

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
impl GetClient<ModelsResponse> for OpenAiClient {
    const ENDPOINT: &'static str = "/models";

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