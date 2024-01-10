use std::time::Duration;

use shared::judge::{SelfTestRequest, SelfTestResponse, SubmitRequest, SubmitResponse};
use tokio::{
    spawn,
    sync::mpsc::{channel, error::SendError},
    time::sleep,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{async_trait, transport::Server, Request, Response, Status};

use crate::shared::{
    judge::{
        judge_service_server::{JudgeService, JudgeServiceServer},
        self_test_response::SelfTestResponseType,
        submit_response::SubmitResponseType,
        CaseInfo, CasesSummary, CompileInfo, JudgeResult, SelfTestSummary,
    },
    SERVER_ADDRESS,
};

#[path = "../shared.rs"]
pub mod shared;

#[derive(Debug, Default)]
struct MyJudgeServiceServer;

#[async_trait]
impl JudgeService for MyJudgeServiceServer {
    type SelfTestStream = ReceiverStream<Result<SelfTestResponse, Status>>;

    async fn self_test(
        &self,
        request: Request<SelfTestRequest>,
    ) -> Result<Response<Self::SelfTestStream>, Status> {
        println!("Got a request: {:#?}", request);
        let data = request.into_inner();
        println!("Request data: {:#?}", data);

        let (tx, rx) = channel(5);

        spawn(async move {
            // Mocking compilation process.
            sleep(Duration::from_secs(2)).await;
            tx.send(Ok(SelfTestResponseType::CompileInfo(CompileInfo {
                exit_status: 0,
                stdout: String::new(),
                stderr: String::new(),
            })
            .into()))
                .await?;

            // Mocking running.
            sleep(Duration::from_secs(2)).await;
            tx.send(Ok(SelfTestResponseType::Summary(SelfTestSummary {
                exit_status: 0,
                stdout: String::new(),
                stderr: String::new(),
            })
            .into()))
                .await?;

            Ok::<(), SendError<Result<SelfTestResponse, Status>>>(())
        });
        Ok(Response::new(Self::SelfTestStream::new(rx)))
    }

    type SubmitStream = ReceiverStream<Result<SubmitResponse, Status>>;

    async fn submit(
        &self,
        request: Request<SubmitRequest>,
    ) -> Result<Response<Self::SubmitStream>, Status> {
        println!("Got a request: {:#?}", request);
        let data = request.into_inner();
        println!("Request data: {:#?}", data);

        let (tx, rx) = channel(5);

        spawn(async move {
            // Mocking compilation process.
            sleep(Duration::from_secs(2)).await;
            tx.send(Ok(SubmitResponseType::CompileInfo(CompileInfo {
                exit_status: 0,
                stdout: String::new(),
                stderr: String::new(),
            })
            .into()))
                .await?;

            for i in 1..4 {
                // Mocking running.
                sleep(Duration::from_secs(2)).await;
                tx.send(Ok(SubmitResponseType::CaseInfo(CaseInfo {
                    case_id: i,
                    exit_status: 0,
                    score: 100,
                    result: JudgeResult::Accepted.into(),
                })
                .into()))
                    .await?;
            }

            tx.send(Ok(SubmitResponseType::CasesSummary(CasesSummary {
                result: JudgeResult::Accepted.into(),
                score: 100,
            })
            .into()))
                .await?;
            Ok::<(), SendError<Result<SubmitResponse, Status>>>(())
        });

        Ok(Response::new(Self::SubmitStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = SERVER_ADDRESS.into();
    let server = MyJudgeServiceServer::default();
    println!("Listening on {}", addr);
    Server::builder()
        .add_service(JudgeServiceServer::new(server))
        .serve(addr)
        .await?;
    println!("Hello, world!");
    Ok(())
}
