use crate::{extract_fixed_data, RespDecode, RespEncode, RespError};
use bytes::BytesMut;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespNull;

impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

impl RespDecode for RespNull {
    const PREFIX: &'static str = "_";

    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        extract_fixed_data(buf, "_\r\n", "Null")?;
        Ok(RespNull)
    }

    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        Ok(3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RespFrame;

    #[test]
    fn test_null_decode() {
        let mut buf = BytesMut::from("_\r\n");
        let s = RespNull::decode(&mut buf).unwrap();
        assert_eq!(s, RespNull);
    }

    #[test]
    fn test_null_encode() {
        let s: RespFrame = RespNull.into();
        assert_eq!(s.encode(), b"_\r\n");
    }
}
