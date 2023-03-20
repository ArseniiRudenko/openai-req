pub mod structs;

use crate::JsonRequest;
use async_trait::async_trait;
use self::structs::{EditRequest,EditResponse};

#[async_trait(?Send)]
impl JsonRequest<EditResponse> for EditRequest {
    const ENDPOINT: &'static str = "/edits";
}