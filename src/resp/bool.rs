use crate::{extract_fixed_data, RespDecode, RespEncode, RespError};
use bytes::BytesMut;

impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        let c = if self { 't' } else { 'f' };
        format!("#{}\r\n", c).into_bytes()
    }
}

impl RespDecode for bool {
    const PREFIX: &'static str = "#";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        match extract_fixed_data(buf, "#t\r\n", "Bool") {
            Ok(_) => Ok(true),
            Err(RespError::NotComplete) => Err(RespError::NotComplete),
            Err(_) => match extract_fixed_data(buf, "#f\r\n", "Bool") {
                Ok(_) => Ok(false),
                Err(e) => Err(e),
            },
        }
    }

    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        Ok(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespFrame;

    #[test]
    fn test_boolean_decode() {
        let mut buf = BytesMut::from("#t\r\n");
        let s = bool::decode(&mut buf).unwrap();
        assert!(s);
        let mut buf = BytesMut::from("#f\r\n");
        let s = bool::decode(&mut buf).unwrap();
        assert!(!s);
    }

    #[test]
    fn test_boolean_encode() {
        let s: RespFrame = true.into();
        assert_eq!(s.encode(), b"#t\r\n");
        let s: RespFrame = false.into();
        assert_eq!(s.encode(), b"#f\r\n");
    }
}
