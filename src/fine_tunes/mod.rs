use crate::fine_tunes::structs::{FineTuneCreateRequest, FineTune, FineTunesResponse, FineTuneEventsResponse};
use crate::{ApiClient, GetClient, OpenAiClient, PostClient};
use crate::structs::{ApiResponse, DeleteResponse, Model};
use anyhow::Result;
pub mod structs;
use async_trait::async_trait;

impl GetClient<FineTunesResponse> for OpenAiClient{
    const ENDPOINT: &'static str = "/fine-tunes";
}

impl PostClient<FineTuneCreateRequest, FineTune> for OpenAiClient{
    const ENDPOINT: &'static str = "/fine-tunes";
}

#[async_trait]
pub trait FineTuneClient{

    async fn get_fine_tune_model(&self, id:&str)-> Result<ApiResponse<FineTune>>;
    async fn cancel_fine_tune(&self, id:&str)-> Result<ApiResponse<FineTune>>;
    async fn get_fine_tune_events(&self, id:&str)-> Result<ApiResponse<FineTuneEventsResponse>>;
    async fn delete_fine_tune_model(&self, model_name:&str)->Result<ApiResponse<DeleteResponse>>;
    async fn get_model(&self, model_name: &str) -> Result<ApiResponse<Model>>;
}

#[async_trait]
impl FineTuneClient for OpenAiClient{

    async fn get_fine_tune_model(&self, id: &str) -> Result<ApiResponse<FineTune>> {
        let final_url = self.url().to_owned()+"/fine-tunes/"+id;
        let res = self.client().get(final_url)
            .bearer_auth(self.key())
            .send()
            .await?
            .json::<ApiResponse<FineTune>>()
            .await?;
        Ok(res)
    }

    async fn cancel_fine_tune(&self,id: &str) -> Result<ApiResponse<FineTune>> {
        let final_url = self.url().to_owned()+"/fine-tunes/"+id+"/cancel";
        let res = self.client().post(final_url)
            .bearer_auth(self.key())
            .send()
            .await?
            .json::<ApiResponse<FineTune>>()
            .await?;
        Ok(res)
    }

    async fn get_fine_tune_events(&self,id: &str) -> Result<ApiResponse<FineTuneEventsResponse>> {
        let final_url = self.url().to_owned()+"/fine-tunes/"+id+"/events";
        let res = self.client().post(final_url)
            .bearer_auth(self.key())
            .send()
            .await?
            .json::<ApiResponse<FineTuneEventsResponse>>()
            .await?;
        Ok(res)
    }

    async fn delete_fine_tune_model(&self,model_name: &str) -> Result<ApiResponse<DeleteResponse>> {
        let final_url = self.url().to_owned()+"/models/"+model_name;
        let res = self.client().delete(final_url)
            .bearer_auth(self.key())
            .send()
            .await?
            .json::<ApiResponse<DeleteResponse>>()
            .await?;
        Ok(res)
    }

    async fn get_model(&self,model_name: &str) -> Result<ApiResponse<Model>> {
        let final_url = self.url().to_owned()+"/models/"+model_name;
        let res = self.client().delete(final_url)
            .bearer_auth(self.key())
            .send()
            .await?
            .json::<ApiResponse<Model>>()
            .await?;
        Ok(res)
    }
}