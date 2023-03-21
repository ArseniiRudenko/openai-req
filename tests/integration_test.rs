extern crate openai_api;
use std::fs;
use anyhow::anyhow;
use file_diff::diff;
use openai_api::{ByUrlRequest, DownloadRequest, FormRequest, GetRequest, JsonRequest, OpenAiClient};
use openai_api::chat::structs::*;
use serde::Deserialize;
use openai_api::completion::structs::{CompletionRequest};
use openai_api::edit::structs::EditRequest;
use openai_api::embeddings::structs::EmbeddingRequest;
use openai_api::files::structs::{FileDeleteRequest, FileDownloadRequest, FileInfoRequest, FileListResponse, FileUploadRequest};
use openai_api::fine_tunes::structs::{FineTuneCreateRequest, FineTuneDeleteRequest, FineTuneListResponse};
use openai_api::moderations::structs::ModerationRequest;
use openai_api::structs::{Input, ModelListResponse};

#[derive(Deserialize)]
struct Config{
    key: String
}

fn get_client() -> OpenAiClient{
    let key_config=
        fs::read_to_string("key.toml")
            .expect("failed reading config file");
    let openai:Config =
        toml::from_str(&key_config)
            .expect("can't parse config file");

    return  OpenAiClient::new(&openai.key);
}

#[tokio::test]
async fn chat() -> Result<(),anyhow::Error> {
   let client = get_client();
   let messages  = vec!(Message{
     role: Role::User,
     content: "hello!".to_string(),
   });
   let chat_request = ChatRequest::new(messages);
   let response = chat_request.run(&client).await?;
   dbg!(response);
   Ok(())
}


#[tokio::test]
async fn edit()-> Result<(),anyhow::Error> {
    let client = get_client();
    let instruction = "correct spelling";
    let text = "quick blck fox jupms over lazy dog";
    let request = EditRequest::new_text(instruction).set_input(text);
    let response = request.run(&client).await?;
    dbg!(response);
    Ok(())
}

#[tokio::test]
async fn completion()-> Result<(),anyhow::Error> {
    let client = get_client();
    let prompt = Input::String("long long time ago".to_string());
    let completion_request = CompletionRequest::new(prompt);
    let response =
        completion_request.run(&client).await?;
    dbg!(response);
    Ok(())
}

#[tokio::test]
async fn models()-> Result<(),anyhow::Error> {
    let client = get_client();
    let response = ModelListResponse::get(&client).await?;
    dbg!(response);
    Ok(())
}


#[tokio::test]
async fn embeddings()-> Result<(),anyhow::Error> {
    let client = get_client();
    let embedding_request
        = EmbeddingRequest::new("The food was delicious and the waiter...".into());
    let response =
        embedding_request.run(&client).await?;
    dbg!(response);
    Ok(())
}


///test creates files, so it is ignored by default
#[tokio::test]
#[ignore]
async fn file_upload() -> Result<(),anyhow::Error> {
    let client = get_client();
    //upload file
    let file = FileUploadRequest::with_str("fine-tune.json","fine-tune");
    let response = file.run(&client).await?;
    dbg!(&response);
    //get info about single file
    let info_request=FileInfoRequest{
        file_id: response.id
    };
    let info= info_request.run(&client).await?;
    dbg!(&info);
    Ok(())
}


#[tokio::test]
async fn file_list() -> Result<(),anyhow::Error> {
    let client = get_client();
    //list uploaded files
    let files = FileListResponse::get(&client).await?;
    dbg!(files);
    Ok(())
}


#[tokio::test]
#[ignore]
async fn file_download() -> Result<(),anyhow::Error> {
    //download file
    // IMPORTANT! downloading files are disabled for free accounts, so this wont work on free account
    let client = get_client();
    let files = FileListResponse::get(&client).await?;
    let info = files.data.first().ok_or(anyhow!("No files available"))?;
    let download_request: FileDownloadRequest = info.clone().into();
    download_request.download_to_file(&client, "fine-tune2.json").await?;
    if !diff("fine-tune.json", "fine-tune2.json") {
        panic!("downloaded file are not the same as uploaded file")
    }
    fs::remove_file("fine-tune2.json")?;
    Ok(())
}

#[tokio::test]
#[ignore]
async fn file_delete() -> Result<(),anyhow::Error> {
    let client = get_client();
    //list uploaded files
    let files = FileListResponse::get(&client).await?;
    dbg!(&files);

    //delete all uploaded  files
    // IMPORTANT! deleting file will not work immediately after uploading it,
    // because openai does some processing on uploaded files
    for file in files.data{
        let delete_request:FileDeleteRequest = file.clone().into();
        let delete_result = delete_request.run(&client).await?;
        dbg!(delete_result);
    }
    Ok(())
}


#[tokio::test]
async fn fine_tune_create() -> Result<(),anyhow::Error> {
    let client = get_client();
    let files = FileListResponse::get(&client).await?;
    let info = files.data.first().ok_or(anyhow!("No files available"))?;
    let ft_req = FineTuneCreateRequest::new(info.id.to_string());
    let ft = ft_req.run(&client).await?;
    dbg!(&ft);
    Ok(())
}


#[tokio::test]
async fn fine_tune_list() -> Result<(),anyhow::Error> {
    let client = get_client();
    let lst = FineTuneListResponse::get(&client).await?;
    dbg!(lst);
    Ok(())
}


#[tokio::test]
#[ignore]
async fn file_tune_model_delete() -> Result<(),anyhow::Error> {
    let client = get_client();
    //list fine tunes
    let fine_tunes = FineTuneListResponse::get(&client).await?;
    dbg!(&fine_tunes);

    //delete all fine tunes
    // IMPORTANT! deleting fine tune model will not work immediately after creating fine-tune,
    // you will need to wait for it to finish
    // also, after model is deleted, fine-tune for it will still be there for some reason,
    // there is nothing in documentation about deleting fine-tunes, only models
    for file in fine_tunes.data{
        let delete_request:FineTuneDeleteRequest = file.clone().try_into()?;
        let delete_result = delete_request.run(&client).await?;
        dbg!(delete_result);
    }
    Ok(())
}

#[tokio::test]
async fn moderation() -> Result<(),anyhow::Error> {
    let client = get_client();
    let req = ModerationRequest::new("I want to kill everyone".into());
    let res = req.run(&client).await?;
    dbg!(res);
    Ok(())
}


