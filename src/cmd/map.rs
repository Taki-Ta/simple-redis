use crate::{
    backend::Backend, extract_args, validate_command, CommandError, CommandExecutor, RespArray,
    RespFrame,
};

use super::{REST_NIL, REST_OK};

#[derive(Debug)]
pub struct Get {
    key: String,
}

#[derive(Debug)]
pub struct Set {
    key: String,
    value: RespFrame,
}

impl CommandExecutor for Get {
    fn execute(&self, backend: &Backend) -> RespFrame {
        match backend.get(&self.key) {
            Some(value) => value,
            None => REST_NIL.clone(),
        }
    }
}

impl CommandExecutor for Set {
    fn execute(&self, backend: &Backend) -> RespFrame {
        backend.set(self.key.clone(), self.value.clone());
        REST_OK.clone()
    }
}

//get :"*2\r\n$3\r\nget\r\n$5\r\nhello\r\n"
impl TryFrom<RespArray> for Get {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["get"], 1)?;
        let args = extract_args(value)?;
        match &args[0] {
            RespFrame::BulkString(key) => Ok(Get {
                key: String::from_utf8_lossy(key).to_string(),
            }),
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

//set :"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
impl TryFrom<RespArray> for Set {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["set"], 2)?;
        let mut args = extract_args(value)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(value)) => Ok(Set {
                key: String::from_utf8_lossy(&key).to_string(),
                value,
            }),
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::RespDecode;
    use crate::BulkString;
    use anyhow::{Ok, Result};
    use bytes::BytesMut;

    #[test]
    fn test_get_command_tryfrom() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nget\r\n$5\r\nhello\r\n");
        let frames = RespArray::decode(&mut buf)?;
        let get = Get::try_from(frames).unwrap();
        assert_eq!(get.key, "hello");
        Ok(())
    }

    #[test]
    fn test_set_command_tryfrom() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n");
        let frames = RespArray::decode(&mut buf)?;
        let set = Set::try_from(frames).unwrap();
        assert_eq!(set.key, "hello");
        assert_eq!(set.value, BulkString::new("world").into());
        Ok(())
    }

    #[test]
    fn test_get_set_execute() {
        let backend = Backend::new();
        let set = Set {
            key: "hello".to_string(),
            value: BulkString::new("world").into(),
        };
        let resp = set.execute(&backend);
        assert_eq!(resp, REST_OK.clone());
        let get = Get {
            key: "hello".to_string(),
        };
        let resp = get.execute(&backend);
        assert_eq!(resp, BulkString::new("world").into());
    }
}
