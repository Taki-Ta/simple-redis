use super::CRLF_LEN;
use crate::{parse_length, RespDecode, RespEncode, RespError};
use bytes::{Buf, BytesMut};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BulkString(pub(crate) Vec<u8>);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespNullBulkString;

impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        if self.len() == 0 {
            return b"$-1\r\n".to_vec();
        }
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("${}\r\n", self.len()).into_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl RespDecode for BulkString {
    const PREFIX: &'static str = "$";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        if len == -1 {
            buf.advance(end + CRLF_LEN);
            return Ok(BulkString::new(Vec::new()));
        }
        let len = len as usize;
        let remained = &buf[end + CRLF_LEN..];
        if remained.len() < len + CRLF_LEN {
            return Err(RespError::NotComplete);
        }

        buf.advance(end + CRLF_LEN);

        let data = buf.split_to(len + CRLF_LEN);
        Ok(BulkString::new(data[..len].to_vec()))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        let len = len as usize;
        Ok(end + CRLF_LEN + len + CRLF_LEN)
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BulkString {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        BulkString(s.into())
    }
}

impl AsRef<[u8]> for BulkString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespFrame;

    #[test]
    fn test_bulk_string_decode() {
        let mut buf = BytesMut::from("$5\r\nhello\r\n");
        let s = BulkString::decode(&mut buf).unwrap();
        assert_eq!(s, BulkString::new("hello".as_bytes().to_vec()));
    }

    #[test]
    fn test_null_bulk_string_decode() -> anyhow::Result<()> {
        let mut buf = BytesMut::from("$-1\r\n");
        let s = BulkString::decode(&mut buf)?;
        assert_eq!(s, BulkString::new(""));
        Ok(())
    }

    #[test]
    fn test_bulk_string_encode() {
        let s: RespFrame = BulkString::new("hello".as_bytes().to_vec()).into();
        assert_eq!(s.encode(), b"$5\r\nhello\r\n");
    }

    #[test]
    fn test_null_bulk_string_encode() {
        let s: RespFrame = BulkString::new("").into();
        assert_eq!(s.encode(), b"$-1\r\n");
    }
}
