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
  buffer: Vec<u8>,
  cursor: usize,
}

impl Connection {
  pub fn new(stream: TcpStream) -> Self {
    Self {
      stream,
      buffer: vec![0; 4096],
      cursor: 0,
    }
  }

  pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
    loop {
      if let Some(frame) = self.parse_from()? {
        return Ok(Some(frame));
      }

      // 0 indicates that no more data will be received from the peer.
      let n = self.stream.read(&mut self.buffer[self.cursor..]).await?;

      if 0 == n {
        if self.buffer.is_empty() {
          return Ok(None);
        } else {
          return Err("connection reset by peer".into());
        }
      } else {
        self.cursor += n;
      }
    }
  }

  fn parse_from(&self) -> Option<Frame> {
    todo!()
  }
}

fn main() {

}
