use reqwest::RequestBuilder;
use super::OpenAiClient;
use async_trait::async_trait;
use crate::{ByUrlRequest, DeleteResponse, DownloadRequest, FormRequest, GetRequest};
use std::io;
use std::path::PathBuf;
use reqwest::multipart::{Form, Part};
use serde::{Serialize, Deserialize};
use with_id::WithRefId;
use crate::conversions::AsyncTryFrom;
use crate::file_to_part;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileListResponse {
    pub data: Vec<FileInfo>,
    pub object: String,
}

#[async_trait]
impl GetRequest for FileListResponse {
    const ENDPOINT: &'static str = "/files";
}


#[derive(Serialize, Deserialize, Debug, Clone, WithRefId)]
pub struct FileInfo {
    pub id: String,
    pub object: String,
    pub bytes: i64,
    pub created_at: i64,
    pub filename: String,
    pub purpose: String,
}


#[derive(Serialize, Deserialize, Debug, Clone, WithRefId)]
pub struct FileDeleteRequest{
    #[id]
    pub file_id:String
}

impl From<FileInfo> for FileDeleteRequest {
    fn from(value: FileInfo) -> Self {
        FileDeleteRequest{
            file_id:value.id
        }
    }
}

impl ByUrlRequest<DeleteResponse> for FileDeleteRequest{
    const ENDPOINT: &'static str = "/files/";
    const SUFFIX: &'static str = "";

    fn builder(client: &OpenAiClient, final_url: String) -> RequestBuilder {
        client.client.delete(final_url)
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, WithRefId)]
pub struct FileInfoRequest{
    #[id]
    pub file_id:String
}

impl ByUrlRequest<FileInfo> for FileInfoRequest{
    const ENDPOINT: &'static str = "/files/";
    const SUFFIX: &'static str = "";
}

#[derive(Serialize, Deserialize, Debug, Clone, WithRefId)]
pub struct FileDownloadRequest{
    #[id]
    pub file_id:String
}

impl DownloadRequest for FileDownloadRequest{
    const ENDPOINT: &'static str = "/files/";
    const SUFFIX: &'static str = "/content";
}

impl From<FileInfo> for FileDownloadRequest {
    fn from(value: FileInfo) -> Self {
        FileDownloadRequest{
            file_id:value.id
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileUploadRequest{
    pub file:PathBuf,
    pub purpose:String
}


impl FormRequest<FileInfo> for FileUploadRequest{
    const ENDPOINT: &'static str = "/files";
}

impl FileUploadRequest {
    pub fn new(file:PathBuf, purpose:String) ->FileUploadRequest{
        FileUploadRequest{
            file,
            purpose
        }
    }

    pub fn with_str(file:&str,purpose:&str)->FileUploadRequest{
        FileUploadRequest{
            file: PathBuf::from(file),
            purpose:purpose.to_string()
        }
    }
}

#[async_trait]
impl AsyncTryFrom<FileUploadRequest> for Form{
    type Error = io::Error;

    async fn try_from(value: FileUploadRequest) -> anyhow::Result<Self, Self::Error> {
        let form =
            Form::new()
                .part("purpose",Part::text(value.purpose))
                .part("file",file_to_part(&value.file).await?);
        Ok(form)
    }
}


