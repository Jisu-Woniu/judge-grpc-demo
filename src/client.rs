use anyhow::anyhow;
use tokio_stream::StreamExt;

use crate::shared::{
    judge::{
        judge_response::JudgeResponseType, judge_service_client::JudgeServiceClient,
        self_test_response::SelfTestResponseType, JudgeRequest, SelfTestRequest,
    },
    SERVER_ADDRESS,
};

pub mod shared;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello from client");
    let mut client = JudgeServiceClient::connect(format!("http://{}", SERVER_ADDRESS)).await?;

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

    let mut judge_response = client
        .judge(JudgeRequest {
            language: "c".into(),
            code: "int main() { return 0; }".into(),
            test_case_id: 114514,
        })
        .await?
        .into_inner();

    while let Some(message) = judge_response.next().await {
        match message?
            .judge_response_type
            .ok_or(anyhow!("Expect `response_type` not empty"))?
        {
            JudgeResponseType::CompileInfo(compile_info) => {
                println!("Got compile info: {:#?}", compile_info)
            }
            JudgeResponseType::CaseInfo(case_info) => {
                println!("Got case info: {:#?}", case_info)
            }
            JudgeResponseType::CasesSummary(cases_summary) => {
                println!("Got cases summary: {:#?}", cases_summary)
            }
        }
    }

    Ok(())
}
