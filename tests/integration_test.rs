extern crate openai_api_rust;
use tokio::runtime::Runtime;
use std::fs;
use openai_api_rust::{OpenAiClient, PostClient};
use openai_api_rust::chat::structs::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config{
    key: String
}


fn get_client() -> OpenAiClient{
    let key_config= fs::read_to_string("key.toml").expect("failed reading config file");
    let openai:Config = toml::from_str(&key_config).expect("can't parse config file");
    return  OpenAiClient::new(&openai.key);
}


#[test]
fn chat() {
   let client = get_client();
   let messages  = vec!(Message{
     role: Role::User,
     content: "hello!".to_string(),
   });
   let chat_request = ChatRequest::new(messages);
   let response = Runtime::new().unwrap().block_on(client.run(chat_request)).expect("failed contacting api");
   dbg!(response);
}


#[test]

fn edit() {

}

#[test]
fn completion() {

}
