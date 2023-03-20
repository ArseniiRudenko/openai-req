pub mod structs;

use crate::{OpenAiClient, JsonRequestClient};
use async_trait::async_trait;
use self::structs::{EditRequest,EditResponse};

#[async_trait(?Send)]
impl JsonRequestClient<EditRequest, EditResponse> for OpenAiClient {
    const ENDPOINT: &'static str = "/edits";
}