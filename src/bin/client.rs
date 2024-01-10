use anyhow::anyhow;
use tokio_stream::StreamExt;

use crate::shared::{
    judge::{
        judge_service_client::JudgeServiceClient, self_test_response::SelfTestResponseType,
        submit_response::SubmitResponseType, SelfTestRequest, SubmitRequest,
    },
    SERVER_ADDRESS,
};

#[path = "../shared.rs"]
pub mod shared;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = JudgeServiceClient::connect(format!("http://{}", SERVER_ADDRESS)).await?;
    println!("Connected to server: {:#?}", client);

    println!("---- Start self test ----");
    let mut self_test_response = client
        .self_test(SelfTestRequest {
            language: "c".into(),
            code: "int main() { return 0; }".into(),
            stdin: String::new(),
        })
        .await?
        .into_inner();

    while let Some(message) = self_test_response.next().await {
        match message?
            .self_test_response_type
            .ok_or(anyhow!("Expect `response_type` not empty"))?
        {
            SelfTestResponseType::CompileInfo(compile_info) => {
                println!("Got compile info: {:#?}", compile_info)
            }
            SelfTestResponseType::Summary(summary) => println!("Got summary: {:#?}", summary),
        }
    }

    println!("---- Start submission ----");

    let mut submit_response = client
        .submit(SubmitRequest {
            language: "c".into(),
            code: "int main() { return 0; }".into(),
            test_case_id: 114514,
        })
        .await?
        .into_inner();

    while let Some(message) = submit_response.next().await {
        match message?
            .submit_response_type
            .ok_or(anyhow!("Expect `response_type` not empty"))?
        {
            SubmitResponseType::CompileInfo(compile_info) => {
                println!("Got compile info: {:#?}", compile_info)
            }
            SubmitResponseType::CaseInfo(case_info) => {
                println!("Got case info: {:#?}", case_info)
            }
            SubmitResponseType::CasesSummary(cases_summary) => {
                println!("Got cases summary: {:#?}", cases_summary)
            }
        }
    }

    Ok(())
}
