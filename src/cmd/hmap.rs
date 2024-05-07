use crate::{
    backend::Backend, extract_args, validate_command, BulkString, CommandError, CommandExecutor,
    HGet, HGetAll, HSet, RespArray, RespFrame,
};

use super::REST_OK;

impl CommandExecutor for HGet {
    fn execute(&self, backend: &Backend) -> RespFrame {
        match backend.hget(&self.key, &self.field) {
            Some(value) => value,
            None => REST_OK.clone(),
        }
    }
}

impl CommandExecutor for HSet {
    fn execute(&self, backend: &Backend) -> RespFrame {
        backend.hset(self.key.clone(), self.field.clone(), self.value.clone());
        REST_OK.clone()
    }
}

impl CommandExecutor for HGetAll {
    fn execute(&self, backend: &Backend) -> RespFrame {
        match backend.hgetall(&self.key) {
            Some(value) => {
                // let mut frames = RespMap::new();
                let mut frame = Vec::with_capacity(value.len() * 2);
                for v in value.iter() {
                    frame.push(BulkString::new(v.key().to_owned()).into());
                    frame.push(v.value().to_owned());
                }
                RespArray::new(frame).into()
            }
            None => REST_OK.clone(),
        }
    }
}

//hget :"*3\r\n$4\r\nHGet\r\n$3\r\nkey\r\n$5\r\nfield\r\n"
impl TryFrom<RespArray> for HGet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hget"], 2)?;
        let mut args = extract_args(value)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field))) => Ok(HGet {
                key: String::from_utf8_lossy(&key).to_string(),
                field: String::from_utf8_lossy(&field).to_string(),
            }),
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

//hset :"*4\r\n$4\r\nHset\r\n$3\r\nkey\r\n$5\r\nfield\r\n$5\r\nvalue\r\n"
impl TryFrom<RespArray> for HSet {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hset"], 3)?;
        let mut args = extract_args(value)?.into_iter();
        match (args.next(), args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field)), Some(value)) => {
                Ok(HSet {
                    key: String::from_utf8_lossy(&key).to_string(),
                    field: String::from_utf8_lossy(&field).to_string(),
                    value,
                })
            }
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}
//hgetall :"*2\r\n$7\r\nHGETALL\r\n$3\r\nkey\r\n"
impl TryFrom<RespArray> for HGetAll {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["hgetall"], 1)?;
        let mut args = extract_args(value)?.into_iter();
        match args.next() {
            Some(RespFrame::BulkString(key)) => Ok(HGetAll {
                key: String::from_utf8_lossy(&key).to_string(),
            }),
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{resp::RespDecode, BulkString};
    use anyhow::{Ok, Result};
    use bytes::BytesMut;

    #[test]
    fn test_hget_command_tryfrom() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*3\r\n$4\r\nHGet\r\n$3\r\nkey\r\n$5\r\nfield\r\n");
        let frames = RespArray::decode(&mut buf)?;
        let hget = HGet::try_from(frames)?;
        assert_eq!(hget.key, "key");
        assert_eq!(hget.field, "field");
        Ok(())
    }

    #[test]
    fn test_hset_command_tryfrom() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*4\r\n$4\r\nHset\r\n$3\r\nkey\r\n$5\r\nfield\r\n$5\r\nvalue\r\n");
        let frames = RespArray::decode(&mut buf)?;
        let hset = HSet::try_from(frames)?;
        assert_eq!(hset.key, "key");
        assert_eq!(hset.field, "field");
        Ok(())
    }

    #[test]
    fn test_hgetall_command_tryfrom() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$7\r\nHGETALL\r\n$3\r\nkey\r\n");
        let frames = RespArray::decode(&mut buf)?;
        let hgetall = HGetAll::try_from(frames)?;
        assert_eq!(hgetall.key, "key");
        Ok(())
    }

    #[test]
    fn test_hget_hset_command_execute() -> Result<()> {
        let backend = Backend::new();
        let hset = HSet {
            key: "key".to_string(),
            field: "field".to_string(),
            value: BulkString::new("value").into(),
        };
        let result = hset.execute(&backend);
        assert_eq!(result, REST_OK.clone());
        let hget = HGet {
            key: "key".to_string(),
            field: "field".to_string(),
        };
        let result = hget.execute(&backend);
        assert_eq!(result, BulkString::new("value").into());
        Ok(())
    }
}
