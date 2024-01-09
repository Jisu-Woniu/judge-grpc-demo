use std::time::Duration;

use shared::judge::{SelfTestRequest, SelfTestResponse};
use tokio::{
    spawn,
    sync::mpsc::{channel, error::SendError},
    time::sleep,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{async_trait, transport::Server, Request, Response, Status};

use crate::shared::{
    judge::{
        judge_response::ResponseType,
        judge_service_server::{JudgeService, JudgeServiceServer},
        CaseInfo, CasesSummary, CompileInfo, JudgeRequest, JudgeResponse, JudgeResult,
    },
    SERVER_ADDRESS,
};

pub mod shared;

#[derive(Debug, Default)]
struct MyJudgeServiceServer;

#[async_trait]
impl JudgeService for MyJudgeServiceServer {
    type SelfTestStream = ReceiverStream<Result<SelfTestResponse, Status>>;

    async fn self_test(
        &self,
        _request: Request<SelfTestRequest>,
    ) -> Result<Response<Self::SelfTestStream>, Status> {
        Err(Status::unimplemented("unimplemented"))
    }

    type JudgeStream = ReceiverStream<Result<JudgeResponse, Status>>;

    async fn judge(
        &self,
        request: Request<JudgeRequest>,
    ) -> Result<Response<Self::JudgeStream>, Status> {
        println!("Got a request: {:#?}", request);
        let request = request.into_inner();
        println!("Request data: {:#?}", request);

        let (tx, rx) = channel(5);

        spawn(async move {
            sleep(Duration::from_secs(2)).await;
            tx.send(Ok(JudgeResponse {
                response_type: Some(ResponseType::CompileInfo(CompileInfo {
                    exit_status: 0,
                    stdout: String::new(),
                    stderr: String::new(),
                })),
            }))
            .await?;

            for i in 1..4 {
                sleep(Duration::from_secs(2)).await;
                tx.send(Ok(JudgeResponse {
                    response_type: Some(ResponseType::CaseInfo(CaseInfo {
                        case_id: i,
                        exit_status: 0,
                        score: 100,
                        result: JudgeResult::Accepted.into(),
                    })),
                }))
                .await?;
            }

            tx.send(Ok(JudgeResponse {
                response_type: Some(ResponseType::CasesSummary(CasesSummary {
                    result: JudgeResult::Accepted.into(),
                    score: 100,
                })),
            }))
            .await?;
            Ok::<(), SendError<Result<JudgeResponse, Status>>>(())
        });

        Ok(Response::new(Self::JudgeStream::new(rx)))
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
