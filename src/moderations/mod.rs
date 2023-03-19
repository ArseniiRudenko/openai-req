use crate::moderations::structs::{ModerationRequest, ModerationResponse};
use crate::{OpenAiClient, PostClient};

pub mod structs;


impl PostClient<ModerationRequest,ModerationResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/moderations";
}