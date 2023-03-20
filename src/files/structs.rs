use std::io;
use std::path::PathBuf;
use async_trait::async_trait;
use reqwest::multipart::{Form, Part};
use serde::{Serialize, Deserialize};
use with_id::WithRefId;
use crate::{AsyncTryFrom, file_to_part};

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


#[derive(Serialize, Deserialize, Debug, Clone, WithRefId)]
pub struct FileInfoRequest{
    #[id]
    pub file_id:String
}


#[derive(Serialize, Deserialize, Debug, Clone, WithRefId)]
pub struct FileDownloadRequest{
    #[id]
    pub file_id:String
}


impl From<FileInfo> for FileDownloadRequest {
    fn from(value: FileInfo) -> Self {
        FileDownloadRequest{
            file_id:value.id
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilesResponse {
    pub data: Vec<FileInfo>,
    pub object: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileUploadRequest{
    pub file:PathBuf,
    pub purpose:String
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






