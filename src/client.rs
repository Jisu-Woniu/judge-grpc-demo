use anyhow::anyhow;
use tokio_stream::StreamExt;

use crate::shared::{
    judge::{judge_response::ResponseType, judge_service_client::JudgeServiceClient, JudgeRequest},
    SERVER_ADDRESS,
};

pub mod shared;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello from client");
    let mut client = JudgeServiceClient::connect(format!("http://{}", SERVER_ADDRESS)).await?;
    let mut response = client
        .judge(JudgeRequest {
            language: "c".into(),
            code: "int main() { return 0; }".into(),
            test_case_id: 1,
        })
        .await?
        .into_inner();

    while let Some(message) = response.next().await {
        match message?
            .response_type
            .ok_or(anyhow!("Expect `response_type` not empty"))?
        {
            ResponseType::CompileInfo(compile_info) => {
                println!("Got compile info: {:#?}", compile_info)
            }
            ResponseType::CaseInfo(case_info) => {
                println!("Got case info: {:#?}", case_info)
            }
            ResponseType::CasesSummary(cases_summary) => {
                println!("Got cases summary: {:#?}", cases_summary)
            }
        }
    }
    Ok(())
}
