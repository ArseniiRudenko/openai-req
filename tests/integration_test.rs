extern crate openai_api;
use std::fs;
use file_diff::{diff, diff_files};
use openai_api::{ByUrlRequest, DownloadRequest, FormRequest, GetRequest, JsonRequest, OpenAiClient};
use openai_api::chat::structs::*;
use serde::Deserialize;
use openai_api::completion::structs::{CompletionRequest};
use openai_api::edit::structs::EditRequest;
use openai_api::embeddings::structs::EmbeddingRequest;
use openai_api::files::structs::{FileDeleteRequest, FileDownloadRequest, FileInfoRequest, FileListResponse, FileUploadRequest};
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



#[tokio::test]
async fn file_add() -> Result<(),anyhow::Error> {
    let client = get_client();
    //upload file
    let file = FileUploadRequest::with_str("fine-tune.json","fine-tune");
    let response = file.run(&client).await?;
    dbg!(&response);
    //list uploaded files
    let files = FileListResponse::get(&client).await?;
    dbg!(files);
    //get info about single file
    let info_request=FileInfoRequest{
        file_id: response.id
    };
    let info= info_request.run(&client).await?;
    dbg!(&info);

    //download file
    // IMPORTANT! downloading files are disabled for free accounts, so this wont work on free account

    /*let download_request:FileDownloadRequest = info.clone().into();
    download_request.download_to_file(&client,"fine-tune2.json").await?;
    if !diff("fine-tune.json","fine-tune2.json"){
        panic!("downloaded file are not the same as uploaded file")
    }
    fs::remove_file("fine-tune2.json")?;
    */

    //delete file
    //also will not work immediately, because
    let delete_request:FileDeleteRequest = info.clone().into();
    let delete_result = delete_request.run(&client).await?;
    dbg!(delete_result);


    Ok(())
}

#[tokio::test]
async fn file_delete() -> Result<(),anyhow::Error> {
    let client = get_client();
    //list uploaded files
    let files = FileListResponse::get(&client).await?;
    dbg!(&files);

    //delete all uploaded  files
    // IMPORTANT! will not work immediately after uploading, because openai does some processing on uploaded files
    for file in files.data{
        let delete_request:FileDeleteRequest = file.clone().into();
        let delete_result = delete_request.run(&client).await?;
        dbg!(delete_result);
    }
    Ok(())
}




#[tokio::test]
async fn fine_tunes() -> Result<(),anyhow::Error> {
    let client = get_client();
    //upload fine-tune file
    let file = FileUploadRequest::with_str("fine-tune.json","fine-tune");
    let response = file.run(&client).await?;
    dbg!(&response);



    //delete file
    let delete_request:FileDeleteRequest = response.clone().into();
    let delete_result = delete_request.run(&client).await?;
    dbg!(&delete_result);
    Ok(())
}