use bytes::Bytes;
use reqwest::Client;
use structs::FilesResponse;
use super::{GetClient, OpenAiClient};
use async_trait::async_trait;
use crate::files::structs::{FileDeleteResponse, FileInfo};
use crate::structs::ApiResponse;
use anyhow::Result;
use tokio_stream::Stream;
pub mod structs;


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

#[async_trait]
trait FileClient{

    async fn delete_file(&self, id: &str) -> Result<ApiResponse<FileDeleteResponse>>;

    async fn retrieve_file(&self,id:&str) -> Result<ApiResponse<FileInfo>>;

    async fn retrieve_file_content(&self,id:&str) -> Result<Box<dyn Stream<Item=Result<Bytes, reqwest::Error>>>>;
}

#[async_trait]
impl FileClient for OpenAiClient {

    async fn delete_file(&self, id: &str) -> Result<ApiResponse<FileDeleteResponse>> {
        let final_url = self.url.to_owned()+"/files/"+id;
        let res = self.client.delete(final_url)
            .bearer_auth(self.key.to_owned())
            .send()
            .await?
            .json::<ApiResponse<FileDeleteResponse>>()
            .await?;
        Ok(res)
    }

    async fn retrieve_file(&self, id: &str) -> Result<ApiResponse<FileInfo>> {
        let final_url = self.url.to_owned()+"/files/"+id;
        let res = self.client.get(final_url)
            .bearer_auth(self.key.to_owned())
            .send()
            .await?
            .json::<ApiResponse<FileInfo>>()
            .await?;
        Ok(res)
    }

    async fn retrieve_file_content(&self, id: &str) -> Result<Box<dyn Stream<Item=Result<Bytes, reqwest::Error>>>> {
        let final_url = self.url.to_owned()+"/files/"+id+"/content";
        let res = self.client.get(final_url)
            .bearer_auth(self.key.to_owned())
            .send()
            .await?
            .error_for_status()?
            .bytes_stream();
        Ok(Box::new(res))
    }

}