use reqwest::RequestBuilder;
use structs::FilesResponse;
use super::{GetClient, OpenAiClient};
use async_trait::async_trait;
use crate::files::structs::{FileDeleteRequest, FileDownloadRequest, FileInfo, FileInfoRequest};
use crate::structs::DeleteResponse;
use crate::{ByUrlClient, DownloadClient};

pub mod structs;

#[async_trait]
impl GetClient<FilesResponse> for OpenAiClient {
    const ENDPOINT: &'static str = "/files";
}

impl ByUrlClient<FileDeleteRequest, DeleteResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/files/";
    const SUFFIX: &'static str = "";

    fn builder(&self, final_url: String) -> RequestBuilder {
        self.client.delete(final_url)
    }
}

impl ByUrlClient<FileInfoRequest,FileInfo> for OpenAiClient{
    const ENDPOINT: &'static str = "/files/";
    const SUFFIX: &'static str = "";
}

impl DownloadClient<FileDownloadRequest> for OpenAiClient{
    const ENDPOINT: &'static str = "/files/";
    const SUFFIX: &'static str = "/content";
}

impl DownloadClient<FileInfo> for OpenAiClient{
    const ENDPOINT: &'static str = "/files/";
    const SUFFIX: &'static str = "/content";
}
