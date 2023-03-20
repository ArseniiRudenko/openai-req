use serde::{Serialize,Deserialize};
use with_id::WithRefId;

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


