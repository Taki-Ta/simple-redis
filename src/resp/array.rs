use super::{util::calc_total_length, BUF_CAP, CRLF_LEN};
use crate::{extract_fixed_data, parse_length, RespDecode, RespEncode, RespError, RespFrame};
use bytes::{Buf, BytesMut};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespArray(pub(crate) Vec<RespFrame>);

impl RespEncode for RespArray {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("*{}\r\n", self.0.len()).into_bytes());
        for frame in self.0 {
            buf.extend_from_slice(&frame.encode());
        }
        buf
    }
}

//null array: "*-1\r\n"
impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(16);
        buf.extend_from_slice(b"*-1\r\n");
        buf
    }
}

impl RespDecode for RespArray {
    const PREFIX: &'static str = "*";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        let total_len = calc_total_length(buf, end, len, Self::PREFIX)?;
        if buf.len() < total_len {
            return Err(RespError::NotComplete);
        }
        let mut frames = Vec::with_capacity(len);
        buf.advance(end + CRLF_LEN);
        for _ in 0..len {
            frames.push(RespFrame::decode(buf)?);
        }
        Ok(RespArray::new(frames))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        calc_total_length(buf, end, len, Self::PREFIX)
    }
}

impl RespDecode for RespNullArray {
    const PREFIX: &'static str = "*";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        extract_fixed_data(buf, "*-1\r\n", "NullArray")?;
        Ok(RespNullArray)
    }

    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        Ok(4)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespNullArray;

impl Deref for RespArray {
    type Target = Vec<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RespArray {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespArray(s.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BulkString;

    #[test]
    fn test_array_decode() {
        let mut buf = BytesMut::from("*3\r\n$3\r\nget\r\n$5\r\nhello\r\n$5\r\nworld\r\n");
        let s = RespArray::decode(&mut buf).unwrap();
        assert_eq!(
            s,
            RespArray::new(vec![
                BulkString::new("get".as_bytes().to_vec()).into(),
                BulkString::new("hello".as_bytes().to_vec()).into(),
                BulkString::new("world".as_bytes().to_vec()).into(),
            ])
        );
    }

    #[test]
    fn test_null_array_decode() {
        let mut buf = BytesMut::from("*-1\r\n");
        let s = RespNullArray::decode(&mut buf).unwrap();
        assert_eq!(s, RespNullArray);
    }

    #[test]
    fn test_array_encode() {
        let s: RespFrame = RespArray::new(vec![
            BulkString::new("get").into(),
            BulkString::new("hello").into(),
            BulkString::new("world").into(),
        ])
        .into();
        assert_eq!(
            s.encode(),
            b"*3\r\n$3\r\nget\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
        );
    }

    #[test]
    fn test_null_array_encode() {
        let s: RespFrame = RespNullArray.into();
        assert_eq!(s.encode(), b"*-1\r\n");
    }
}
