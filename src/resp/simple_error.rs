use super::CRLF_LEN;
use crate::{extract_simple_frame_data, RespDecode, RespEncode, RespError};
use bytes::BytesMut;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SimpleError(pub(crate) String);

impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}

impl RespDecode for SimpleError {
    const PREFIX: &'static str = "-";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        let data = buf.split_to(end + CRLF_LEN);
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);
        Ok(SimpleError::new(s.to_string()))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN)
    }
}
impl Deref for SimpleError {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SimpleError {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleError(s.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespFrame;

    #[test]
    fn test_simple_error_decode() {
        let mut buf = BytesMut::from("-Error message\r\n");
        let s = SimpleError::decode(&mut buf).unwrap();
        assert_eq!(s, SimpleError::new("Error message"));
    }

    #[test]
    fn test_simple_error_encode() -> anyhow::Result<()> {
        let s: RespFrame = SimpleError::new("Error Message").into();
        assert_eq!(s.encode(), b"-Error Message\r\n");
        Ok(())
    }
}
