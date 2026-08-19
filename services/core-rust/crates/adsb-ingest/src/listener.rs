// use bytes::BytesMut;
// use std::net::SocketAddr;
// use tokio::net::UdpSocket;

// use crate::{error::IngestError, frame::AdsbFrame};

// pub struct AdsbListener {
//     socket: UdpSocket,
//     address: SocketAddr,
// }

// impl AdsbListener {
//     pub async fn bind(address: SocketAddr) -> Result<Self, IngestError> {
//         let socket = UdpSocket::bind(address).await?;
//         let address = socket.local_addr()?;

//         Ok(Self { socket, address })
//     }

//     pub fn address(&self) -> SocketAddr {
//         self.address
//     }

//     pub async fn receive(&self) -> Result<AdsbFrame, IngestError> {
//         let mut buffer = BytesMut::zeroed(1024);

//         let (size, _) = self.socket.recv_from(&mut buffer).await?;

//         if size == 0 {
//             return Err(IngestError::EmptyPacket);
//         }

//         buffer.truncate(size);

//         Ok(AdsbFrame::new(buffer.freeze()))
//     }
// }

use std::net::SocketAddr;

use bytes::{Bytes, BytesMut};
use tokio::net::UdpSocket;

use crate::{error::IngestError, frame::AdsbFrame};

pub struct AdsbListener {
    socket: UdpSocket,
    address: SocketAddr,
}

impl AdsbListener {
    pub async fn bind(address: SocketAddr) -> Result<Self, IngestError> {
        let socket = UdpSocket::bind(address).await?;
        let address = socket.local_addr()?;

        Ok(Self { socket, address })
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    /// Waits for the next UDP packet.
    pub async fn receive(&self) -> Result<(AdsbFrame, SocketAddr), IngestError> {
        let mut buffer = BytesMut::zeroed(1024);

        let (size, sender) = self.socket.recv_from(&mut buffer).await?;

        if size == 0 {
            return Err(IngestError::EmptyPacket);
        }

        buffer.truncate(size);

        let frame = AdsbFrame::new(Bytes::from(buffer));

        Ok((frame, sender))
    }
}
