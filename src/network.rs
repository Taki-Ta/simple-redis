use crate::{backend::Backend, CommandExecutor, RespDecode, RespEncode, RespError, RespFrame};
use anyhow::Result;
use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder, Framed};
use tracing::info;

pub async fn stream_handler(stream: TcpStream, backend: Backend) -> Result<()> {
    let mut framed = Framed::new(stream, RespFrameCodec);
    loop {
        match framed.next().await {
            Some(Ok(frame)) => {
                println!("Received frame: {:?}", frame);
                let backend = backend.clone();
                let request = RedisRequest { frame, backend };
                let response = handle_request(request).await?;
                println!("Send response: {:?}", response);
                println!("msg is {:?}", &response.clone().frame);
                framed.send(response.frame).await?;
            }
            Some(Err(e)) => {
                return Err(e);
            }
            None => return Ok(()),
        }
    }
}

pub async fn handle_request(request: RedisRequest) -> Result<RedisResponse> {
    let (frame, backend) = (request.frame, request.backend);
    let cmd = crate::Command::try_from(frame)?;
    info!("Executing command: {:?}", cmd);
    let frame = cmd.execute(&backend);
    Ok(RedisResponse { frame })
}

#[derive(Debug)]
pub struct RespFrameCodec;

#[derive(Debug, Clone)]
pub struct RedisResponse {
    frame: RespFrame,
}

#[derive(Debug)]
pub struct RedisRequest {
    frame: RespFrame,
    backend: Backend,
}

impl Encoder<RespFrame> for RespFrameCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: RespFrame, dst: &mut bytes::BytesMut) -> Result<()> {
        let data = item.encode();
        dst.extend_from_slice(&data);
        Ok(())
    }
}

impl Decoder for RespFrameCodec {
    type Item = RespFrame;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>> {
        match RespFrame::decode(src) {
            Ok(frame) => Ok(Some(frame)),
            //if the frame is not complete, return Ok(None) to wait for more data
            Err(RespError::NotComplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;
    use bytes::BytesMut;

    use super::*;
    use crate::{BulkString, RespArray, SimpleString};

    #[test]
    fn test_codec_decode() -> Result<()> {
        let mut buf = BytesMut::from("*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n");
        let mut codec = RespFrameCodec;
        let frame = codec.decode(&mut buf.clone())?;
        let resp_frame: RespFrame = RespArray::decode(&mut buf)?.into();
        assert_eq!(frame, Some(resp_frame));

        let mut buf = bytes::BytesMut::new();
        buf.extend_from_slice(b"");
        let frame = codec.decode(&mut buf)?;
        assert!(frame.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_stream_handler() -> Result<()> {
        let backend = Backend::new();

        let mut buf = BytesMut::from("*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n");
        let frame: RespFrame = RespArray::decode(&mut buf)?.into();

        let response = handle_request(RedisRequest {
            frame: frame.clone(),
            backend: backend.clone(),
        })
        .await?;
        let new_frame: RespFrame = SimpleString::new("OK".to_string()).into();
        assert_eq!(response.frame, new_frame);

        let mut buf = BytesMut::from("*2\r\n$3\r\nget\r\n$5\r\nhello\r\n");
        let frame: RespFrame = RespArray::decode(&mut buf)?.into();
        let response = handle_request(RedisRequest {
            frame: frame.clone(),
            backend: backend.clone(),
        })
        .await?;
        let new_frame: RespFrame = BulkString::new("world".to_string()).into();
        assert_eq!(response.frame, new_frame);
        Ok(())
    }
}
