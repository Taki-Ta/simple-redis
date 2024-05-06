use super::CRLF_LEN;
use crate::{extract_simple_frame_data, RespDecode, RespEncode, RespError};
use bytes::BytesMut;

impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(32);
        let ret = if self.abs() > 1e+8 || self.abs() < 1e-8 {
            format!(",{:+e}\r\n", self)
        } else {
            let sign = if self < 0.0 { "" } else { "+" };
            format!(",{}{}\r\n", sign, self)
        };
        buf.extend_from_slice(&ret.into_bytes());
        buf
    }
}

impl RespDecode for f64 {
    const PREFIX: &'static str = ",";

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
    fn test_double_decode() {
        let mut buf = BytesMut::from(",+100.123\r\n");
        let s = f64::decode(&mut buf).unwrap();
        assert_eq!(s, 100.123);
        let mut buf = BytesMut::from(",+1.234e+9\r\n");
        let s = f64::decode(&mut buf).unwrap();
        assert_eq!(s, 1.234e+9);
        let mut buf = BytesMut::from(",-1.234e-9\r\n");
        let s = f64::decode(&mut buf).unwrap();
        assert_eq!(s, -1.234e-9);
    }

    #[test]
    fn test_double_encode() {
        let s: RespFrame = 123.456.into();
        assert_eq!(s.encode(), b",+123.456\r\n");
        let s: RespFrame = (-123.456).into();
        assert_eq!(s.encode(), b",-123.456\r\n");
        let s: RespFrame = 1.23456e+8.into();
        assert_eq!(s.encode(), b",+1.23456e8\r\n");
        let s: RespFrame = (-1.23456e-9).into();
        assert_eq!(s.encode(), b",-1.23456e-9\r\n");
    }
}
