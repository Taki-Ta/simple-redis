use super::CRLF_LEN;
use crate::{RespDecode, RespError, RespFrame, SimpleString};
use bytes::{Buf, BytesMut};

pub fn extract_simple_frame_data(buf: &[u8], prefix: &str) -> Result<usize, RespError> {
    //prevalidate the buffer
    if buf.len() < 3 {
        return Err(RespError::NotComplete);
    }
    if !buf.starts_with(prefix.as_bytes()) {
        return Err(RespError::InvalidFrameType(format!(
            "Invalid Frame Type: expected{}, fount:{:?}",
            prefix, buf
        )));
    }
    let end = find_crlf(buf, 1).ok_or(RespError::NotComplete)?;
    Ok(end)
}

//find the index of nth CRLF in the buffer
pub fn find_crlf(buf: &[u8], offset: usize) -> Option<usize> {
    let mut count = 0;
    for idx in 0..buf.len() - 1 {
        if buf[idx] == b'\r' && buf[idx + 1] == b'\n' {
            count += 1;
            if count == offset {
                return Some(idx);
            }
        }
    }
    None
}

//get the length identifier of the signature part of frame
pub fn parse_length(buf: &[u8], prefix: &str) -> Result<(usize, usize), RespError> {
    let end = extract_simple_frame_data(buf, prefix)?;
    let s = String::from_utf8_lossy(&buf[prefix.len()..end]);
    Ok((end, s.parse()?))
}

//handle null frame with expect value
pub fn extract_fixed_data(
    buf: &mut BytesMut,
    expect: &str,
    expect_type: &str,
) -> Result<(), RespError> {
    if buf.len() < expect.len() {
        return Err(RespError::NotComplete);
    }

    if !buf.starts_with(expect.as_bytes()) {
        return Err(RespError::InvalidFrameType(format!(
            "expect: {}, got: {:?}",
            expect_type, buf
        )));
    }

    buf.advance(expect.len());
    Ok(())
}

//calculate the expect length of the frame
pub fn calc_total_length(
    buf: &[u8],
    end: usize,
    len: usize,
    prefix: &str,
) -> Result<usize, RespError> {
    let mut total = end + CRLF_LEN;
    let mut data = &buf[total..];
    match prefix {
        "*" | "~" => {
            // find nth CRLF in the buffer, for array and set, we need to find 1 CRLF for each element
            for _ in 0..len {
                let len = RespFrame::expect_length(data)?;
                data = &data[len..];
                total += len;
            }
            Ok(total)
        }
        "%" => {
            // find nth CRLF in the buffer. For map, we need to find 2 CRLF for each key-value pair
            for _ in 0..len {
                let len = SimpleString::expect_length(data)?;

                data = &data[len..];
                total += len;

                let len = RespFrame::expect_length(data)?;
                data = &data[len..];
                total += len;
            }
            Ok(total)
        }
        _ => Ok(len + CRLF_LEN),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_fixed_data() {
        let mut buf = BytesMut::from("+OK\r\n");
        let res = extract_fixed_data(&mut buf, "+OK\r\n", "SimpleString");
        assert!(res.is_ok());
        assert_eq!(buf.len(), 0);

        let mut buf = BytesMut::from("-Error\r\n");
        let res = extract_fixed_data(&mut buf, "-Error\r\n", "SimpleError");
        assert!(res.is_ok());
        assert_eq!(buf.len(), 0);

        let mut buf = BytesMut::from("_\r\n");
        let res = extract_fixed_data(&mut buf, "_\r\n\r\n", "Null");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), RespError::NotComplete);
    }

    #[test]
    fn test_parse_length() {
        let buf = BytesMut::from("$6\r\nhello1\r\n");
        let (end, length) = parse_length(&buf, "$").unwrap();
        assert_eq!(end, 2);
        assert_eq!(length, 6);

        let buf = BytesMut::from("+OK\r\n");
        let may_error = parse_length(&buf, "+");
        assert!(may_error.is_err());
    }

    #[test]
    fn test_calc_total_length() -> anyhow::Result<()> {
        let buf = b"*2\r\n$3\r\nget\r\n$5\r\nhello\r\n";
        let (end, length) = parse_length(buf, "*").unwrap();
        let total = calc_total_length(buf, end, length, "*").unwrap();
        assert_eq!(total, buf.len());

        let buf = b"*2\r\n$3\r\nget\r\n";
        let (end, length) = parse_length(buf, "*").unwrap();
        let res = calc_total_length(buf, end, length, "*");
        assert_eq!(res.unwrap_err(), RespError::NotComplete);
        Ok(())
    }
}
