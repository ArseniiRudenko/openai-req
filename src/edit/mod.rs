pub mod structs;

use crate::{OpenAiClient, PostClient};
use async_trait::async_trait;
use self::structs::{EditRequest,EditResponse};

#[async_trait(?Send)]
impl PostClient<EditRequest, EditResponse> for OpenAiClient {
    const ENDPOINT: &'static str = "/edits";
}