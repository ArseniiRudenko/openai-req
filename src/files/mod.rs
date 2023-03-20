use reqwest::RequestBuilder;
use structs::FilesResponse;
use super::OpenAiClient;
use async_trait::async_trait;
use crate::files::structs::{FileDeleteRequest, FileDownloadRequest, FileInfo, FileInfoRequest, FileUploadRequest};
use crate::structs::DeleteResponse;
use crate::{ByUrlRequest, DownloadRequest, FormRequest, GetRequest};

pub mod structs;

#[async_trait]
impl GetRequest for FilesResponse {
    const ENDPOINT: &'static str = "/files";
}

impl ByUrlRequest<DeleteResponse> for FileDeleteRequest{
    const ENDPOINT: &'static str = "/files/";
    const SUFFIX: &'static str = "";

    fn builder(client: &OpenAiClient, final_url: String) -> RequestBuilder {
        client.client.delete(final_url)
    }
}

impl ByUrlRequest<FileInfo> for FileInfoRequest{
    const ENDPOINT: &'static str = "/files/";
    const SUFFIX: &'static str = "";
}

impl DownloadRequest for FileDownloadRequest{
    const ENDPOINT: &'static str = "/files/";
    const SUFFIX: &'static str = "/content";
}

impl DownloadRequest for FileInfo{
    const ENDPOINT: &'static str = "/files/";
    const SUFFIX: &'static str = "/content";
}

impl FormRequest<FileInfo> for FileUploadRequest{
    const ENDPOINT: &'static str = "/files";
}