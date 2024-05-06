use super::CRLF_LEN;
use crate::{extract_simple_frame_data, RespDecode, RespEncode, RespError};
use bytes::BytesMut;

impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "" } else { "+" };
        format!(":{}{}\r\n", sign, self).into_bytes()
    }
}

impl RespDecode for i64 {
    const PREFIX: &'static str = ":";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        let data = buf.split_to(end + CRLF_LEN);
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end]);
        Ok(s.parse()?)
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end = extract_simple_frame_data(buf, Self::PREFIX)?;
        Ok(end + CRLF_LEN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespFrame;

    #[test]
    fn test_integer_decode() {
        let mut buf = BytesMut::from(":100\r\n");
        let s = i64::decode(&mut buf).unwrap();
        assert_eq!(s, 100);
    }

    #[test]
    fn test_integer_encode() {
        let s: RespFrame = 100.into();
        assert_eq!(s.encode(), b":+100\r\n");
        let s: RespFrame = (-100).into();
        assert_eq!(s.encode(), b":-100\r\n");
    }
}
