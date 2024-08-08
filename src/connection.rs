use bytes::BytesMut;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use bytes::Buf;
use mini_redis::Result;
use mini_redis::Frame;

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

      if 0 == self.stream.read_buf(&mut self.buffer).await? {

      }
    }
  }

  fn parse_from() -> Option<Frame> {

  }
}

