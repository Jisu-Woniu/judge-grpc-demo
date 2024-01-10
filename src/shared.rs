use std::net::{Ipv4Addr, SocketAddrV4};

pub mod judge {
    use self::{self_test_response::SelfTestResponseType, submit_response::SubmitResponseType};

    tonic::include_proto!("judge.v1");

    impl From<SubmitResponseType> for SubmitResponse {
        fn from(value: SubmitResponseType) -> Self {
            Self {
                submit_response_type: Some(value),
            }
        }
    }

    impl From<SelfTestResponseType> for SelfTestResponse {
        fn from(value: SelfTestResponseType) -> Self {
            Self {
                self_test_response_type: Some(value),
            }
        }
    }
}

pub const SERVER_ADDRESS: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 50051);
