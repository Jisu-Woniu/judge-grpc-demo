use std::net::{Ipv4Addr, SocketAddrV4};

pub mod judge {

    tonic::include_proto!("judge.v1");
}

pub const SERVER_ADDRESS: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 50051);
