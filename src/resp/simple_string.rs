use super::CRLF_LEN;
use crate::{extract_simple_frame_data, RespDecode, RespEncode, RespError};
use bytes::BytesMut;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct SimpleString(pub(crate) String);

impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

impl RespDecode for SimpleString {
    const PREFIX: &'static str = "+";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        let data = buf.split_to(end + CRLF_LEN);
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);
        Ok(SimpleString::new(s.to_string()))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN)
    }
}

impl Deref for SimpleString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleString(s.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespFrame;

    #[test]
    fn test_simple_string_decode() {
        let mut buf = BytesMut::from("+OK\r\n");
        let s = SimpleString::decode(&mut buf).unwrap();
        assert_eq!(s, SimpleString::new("OK"));
    }

    #[test]
    fn test_simple_string_encode() {
        let s: RespFrame = SimpleString::new("OK").into();
        assert_eq!(s.encode(), b"+OK\r\n");
    }
}
