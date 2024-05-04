use crate::{
    BulkString, RespArray, RespEncode, RespMap, RespNull, RespNullArray, RespNullBulkString,
    RespSet, SimpleError, SimpleString,
};

const BUF_CAP: usize = 4096;

/*
- 如何解析 Frame
    - simple string: "+OK\r\n"
    - error: "-Error message\r\n"
    - bulk error: "!<length>\r\n<error>\r\n"
    - integer: ":[<+|->]<value>\r\n"
    - bulk string: "$<length>\r\n<data>\r\n"
    - null bulk string: "$-1\r\n"
    - array: "*<number-of-elements>\r\n<element-1>...<element-n>"
        - "*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
    - null array: "*-1\r\n"
    - null: "_\r\n"
    - boolean: "#<t|f>\r\n"
    - double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
    - map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
    - set: "~<number-of-elements>\r\n<element-1>...<element-n>"
 */

//simple string: "+OK\r\n"
impl RespEncode for SimpleString {
    fn encode(self) -> Vec<u8> {
        format!("+{}\r\n", self.0).into_bytes()
    }
}

//error: "-Error message\r\n"
impl RespEncode for SimpleError {
    fn encode(self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}
//:[<+|->]<value>\r\n
impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = if self < 0 { "" } else { "+" };
        format!(":{}{}\r\n", sign, self).into_bytes()
    }
}

//$<length>\r\n<data>\r\n
impl RespEncode for BulkString {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.len() + 16);
        buf.extend_from_slice(&format!("${}\r\n", self.len()).into_bytes());
        buf.extend_from_slice(&self);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

//$-1\r\n
impl RespEncode for RespNullBulkString {
    fn encode(self) -> Vec<u8> {
        b"$-1\r\n".to_vec()
    }
}

//*<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>
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

//- null: "_\r\n"
impl RespEncode for RespNull {
    fn encode(self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

//- boolean: "#<t|f>\r\n"
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        let c = if self { 't' } else { 'f' };
        format!("#{}\r\n", c).into_bytes()
    }
}

//- double: ",[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n"
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

//- map: "%<number-of-entries>\r\n<key-1><value-1>...<key-n><value-n>"
impl RespEncode for RespMap {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("%{}\r\n", self.len()).into_bytes());
        for (k, v) in self.0 {
            buf.extend_from_slice(&SimpleString::new(k).encode());
            buf.extend_from_slice(&v.encode());
        }
        buf
    }
}

//- set: "~<number-of-elements>\r\n<element-1>...<element-n>"
impl RespEncode for RespSet {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("~{}\r\n", self.len()).into_bytes());
        for frame in self.0 {
            buf.extend_from_slice(&frame.encode());
        }
        buf
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use crate::RespFrame;

    use super::*;

    #[test]
    fn test_simple_string() {
        let s: RespFrame = SimpleString::new("OK").into();
        assert_eq!(s.encode(), b"+OK\r\n");
    }

    #[test]
    fn test_simple_error() -> anyhow::Result<()> {
        let s: RespFrame = SimpleError::new("Error Message").into();
        assert_eq!(s.encode(), b"-Error Message\r\n");
        Ok(())
    }

    #[test]
    fn test_integer() {
        let s: RespFrame = 100.into();
        assert_eq!(s.encode(), b":+100\r\n");
        let s: RespFrame = (-100).into();
        assert_eq!(s.encode(), b":-100\r\n");
    }

    #[test]
    fn test_bulk_string() {
        let s: RespFrame = BulkString::new("hello".as_bytes().to_vec()).into();
        assert_eq!(s.encode(), b"$5\r\nhello\r\n");
    }

    #[test]
    fn test_null_bulk_string() {
        let s: RespFrame = RespNullBulkString.into();
        assert_eq!(s.encode(), b"$-1\r\n");
    }

    #[test]
    fn test_array() {
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
    fn test_null_array() {
        let s: RespFrame = RespNullArray.into();
        assert_eq!(s.encode(), b"*-1\r\n");
    }

    #[test]
    fn test_null() {
        let s: RespFrame = RespNull.into();
        assert_eq!(s.encode(), b"_\r\n");
    }

    #[test]
    fn test_boolean() {
        let s: RespFrame = true.into();
        assert_eq!(s.encode(), b"#t\r\n");
        let s: RespFrame = false.into();
        assert_eq!(s.encode(), b"#f\r\n");
    }

    #[test]
    fn test_double() {
        let s: RespFrame = 123.456.into();
        assert_eq!(s.encode(), b",+123.456\r\n");
        let s: RespFrame = (-123.456).into();
        assert_eq!(s.encode(), b",-123.456\r\n");
        let s: RespFrame = 1.23456e+8.into();
        assert_eq!(s.encode(), b",+1.23456e8\r\n");
        let s: RespFrame = (-1.23456e-9).into();
        assert_eq!(s.encode(), b",-1.23456e-9\r\n");
    }

    #[test]
    fn test_map_encode() {
        let mut map = RespMap::new();
        map.insert(
            "hello".to_string(),
            BulkString::new("world".to_string()).into(),
        );
        map.insert("foo".to_string(), (-123456.789).into());

        let frame: RespFrame = map.into();
        assert_eq!(
            &frame.encode(),
            b"%2\r\n+foo\r\n,-123456.789\r\n+hello\r\n$5\r\nworld\r\n"
        );
    }

    #[test]
    fn test_set_encode() {
        let frame: RespFrame = RespSet::new([
            RespArray::new([1234.into(), true.into()]).into(),
            BulkString::new("world".to_string()).into(),
        ])
        .into();
        assert_eq!(
            frame.encode(),
            b"~2\r\n*2\r\n:+1234\r\n#t\r\n$5\r\nworld\r\n"
        );
    }
}