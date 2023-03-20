use crate::moderations::structs::{ModerationRequest, ModerationResponse};
use crate::{OpenAiClient, JsonRequestClient};

pub mod structs;


impl JsonRequestClient<ModerationRequest,ModerationResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/moderations";
}