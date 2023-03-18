use std::pin::Pin;
use bytes::Bytes;
use reqwest::Error;
use structs::FilesResponse;
use super::{GetClient, OpenAiClient};
use async_trait::async_trait;
use crate::files::structs::{FileDeleteResponse, FileInfo};
use crate::structs::ApiResponse;
use anyhow::Result;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_stream::{Stream,StreamExt};
pub mod structs;

#[async_trait]
impl GetClient<FilesResponse> for OpenAiClient {
    const ENDPOINT: &'static str = "/files";
}

#[async_trait(?Send)]
trait FileClient{

    async fn delete_file(&self, id: &str) -> Result<ApiResponse<FileDeleteResponse>>;

    async fn retrieve_file(&self,id:&str) -> Result<ApiResponse<FileInfo>>;

    async fn retrieve_file_content(&self,id:&str) -> Result<Pin<Box<dyn Stream<Item=Result<Bytes, Error>>>>>;

    async fn retrieve_file_content_to_file(&self, id: &str, target_path:&str) -> Result<()>;
}

#[async_trait(?Send)]
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

    async fn retrieve_file_content(&self, id: &str) -> Result<Pin<Box<dyn Stream<Item=Result<Bytes, Error>>>>> {
        let final_url = self.url.to_owned()+"/files/"+id+"/content";
        let res = self.client.get(final_url)
            .bearer_auth(self.key.to_owned())
            .send()
            .await?
            .error_for_status()?
            .bytes_stream();
        Ok(Box::pin(res))
    }

    async fn retrieve_file_content_to_file(&self, id: &str, target_path:&str) -> Result<()> {
        let mut stream = self.retrieve_file_content(id).await?;
        let mut file = File::create(target_path).await?;
        while let Some(chunk) = stream.next().await {
            file.write_all(&chunk?).await?;
        }
        Ok(())
    }

}