use reqwest::RequestBuilder;
use super::OpenAiClient;
use async_trait::async_trait;
use crate::{ByUrlRequest, DeleteResponse, DownloadRequest, FormRequest, GetRequest};
use std::io;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use reqwest::multipart::{Form, Part};
use serde::{Serialize, Deserialize};
use with_id::WithRefId;
use crate::conversions::AsyncTryFrom;
use crate::file_to_part;
use derive_more::*;
use crate::fine_tunes::FineTuneFileInfo;

/// Gets list of available files. More details at
/// https://platform.openai.com/docs/api-reference/files/list
/// # Usage example
///```
/// use openai_req::files::FileListResponse;
/// use openai_req::GetRequest;
///
/// let files = FileListResponse::get(&client).await?;
///```
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

impl From<FineTuneFileInfo> for FileInfo{
    fn from(value: FineTuneFileInfo) -> Self {
        FileInfo{
            id: value.id,
            object: value.object,
            bytes: value.bytes,
            created_at: value.created_at,
            filename: value.filename,
            purpose: value.purpose,
        }
    }
}

/// File delete request allows to delete file in OpenAI storage
/// https://platform.openai.com/docs/api-reference/files/delete
/// # Usage example
///```
/// use openai_req::ByUrlRequest;
/// use openai_req::files::FileDeleteRequest;
///
/// let req = FileDeleteRequest::new("{file_id}".to_string());
/// let delete_result = req.run(&client).await?;
///```
#[derive(Serialize, Deserialize, Debug, Clone, WithRefId, Constructor)]
pub struct FileDeleteRequest{
    #[id]
    file_id:String
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

///Gets info about single uploaded file.
///Refer to https://platform.openai.com/docs/api-reference/files/retrieve for additional details
///# Usage example
///```
/// use openai_req::ByUrlRequest;
/// use openai_req::files::FileInfoRequest;
///
/// let info_request = FileInfoRequest::new("{file_id}".to_string());
/// let info = info_request.run(&client).await?;
///```
#[derive(Serialize, Deserialize, Debug, Clone, WithRefId, Constructor)]
pub struct FileInfoRequest{
    #[id]
    file_id:String
}

impl ByUrlRequest<FileInfo> for FileInfoRequest{
    const ENDPOINT: &'static str = "/files/";
    const SUFFIX: &'static str = "";
}
///Request to download file.
/// More details at https://platform.openai.com/docs/api-reference/files/retrieve-content
/// # Usage example
/// ```
///  use openai_req::DownloadRequest;
///  use openai_req::files::FileDownloadRequest;
///
///  let req = FileDownloadRequest::new("{file_id}".to_string());
///  req.download_to_file(&client, "C:/Downloads/fine-name.ext").await?;
///```
#[derive(Serialize, Deserialize, Debug, Clone, WithRefId, Constructor)]
pub struct FileDownloadRequest{
    #[id]
    file_id:String
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

///Upload local file to OpenAI storage(usually for the purpose of fine-tuning).
///More details at https://platform.openai.com/docs/api-reference/files/upload
/// # Usage example
///```
/// use openai_req::files::FileUploadRequest;
/// use openai_req::FormRequest;
///
/// let file = FileUploadRequest::with_str("tests/fine-tune.json","fine-tune");
/// let response = file.run(&client).await?;
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileUploadRequest{
    file:PathBuf,
    purpose:String
}


impl FormRequest<FileInfo> for FileUploadRequest{
    const ENDPOINT: &'static str = "/files";
}

impl FileUploadRequest {

    ///basic constructor, takes path to file and file purpose
    pub fn new(file:PathBuf, purpose:String) ->Result<FileUploadRequest,Error>{
        if file.exists() {
            Ok(
                FileUploadRequest {
                    file,
                    purpose
                }
            )
        }else {
            Err(Error::new(ErrorKind::NotFound, "File does not exist"))
        }
    }
    ///same arguments as in ::new, but as str refs for convenience
    pub fn with_str(file:&str,purpose:&str)->Result<FileUploadRequest,Error>{
        let path = PathBuf::from(file);
        if path.exists() {
            Ok(
                FileUploadRequest {
                    file: path,
                    purpose: purpose.to_string()
                }
            )
        }else {
            Err(io::Error::new(ErrorKind::NotFound, "File does not exist"))
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


