use bytes::{BytesMut, Bytes};
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use mini_redis::Result;

enum Frame {
  Simple(String),
  Error(String),
  Integer(u64),
  Bulk(Bytes),
  Null,
  Array(Vec<Frame>),
}

pub struct Connection {
  stream: TcpStream,
  buffer: BytesMut,
}

impl Connection {
  pub fn new(stream: TcpStream) -> Self {
    Self {
      stream,
      buffer: BytesMut::with_capacity(4096),
    }
  }

  pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
    loop {
      if let Some(frame) = self.parse_from()? {
        return Ok(Some(frame));
      }

      // 0 indicates that no more data will be received from the peer.
      if 0 == self.stream.read_buf(&mut self.buffer).await? {
        if self.buffer.is_empty() {
          return Ok(None);
        } else {
          return Err("connection reset by peer".into());
        }
      }
    }
  }

  fn parse_from(&self) -> Option<Frame> {
    None
  }
}

fn main() {

}
