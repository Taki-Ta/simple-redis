use crate::{
    extract_args, validate_command_exact_length, validate_command_minimum_length, Backend,
    CommandError, CommandExecutor, RespArray, RespFrame,
};

#[derive(Debug)]
pub struct Sadd {
    key: String,
    members: Vec<String>,
}

#[derive(Debug)]
pub struct SISMember {
    key: String,
    field: String,
}

impl CommandExecutor for Sadd {
    fn execute(&self, backend: &Backend) -> RespFrame {
        let res = backend.sadd(self.key.clone(), self.members.clone());
        RespFrame::Integer(res)
    }
}

impl CommandExecutor for SISMember {
    fn execute(&self, backend: &Backend) -> RespFrame {
        match backend.sismember(&self.key, &self.field) {
            Some(i) => RespFrame::Integer(i),
            None => RespFrame::Integer(0),
        }
    }
}

impl TryFrom<RespArray> for Sadd {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command_minimum_length(&value, &["sadd"], 3)?;
        let mut args = extract_args(value)?.into_iter();
        let key = args.next();
        let members: Vec<String> = args
            .filter_map(|v| {
                if let RespFrame::BulkString(key) = v {
                    Some(String::from_utf8_lossy(&key).to_string())
                } else {
                    None
                }
            })
            .collect();
        match key {
            Some(RespFrame::BulkString(key)) => Ok(Sadd {
                key: String::from_utf8_lossy(&key).to_string(),
                members,
            }),
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

impl TryFrom<RespArray> for SISMember {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command_exact_length(&value, &["sismember"], 2)?;
        let mut args = extract_args(value)?.into_iter();
        match (args.next(), args.next()) {
            (Some(RespFrame::BulkString(key)), Some(RespFrame::BulkString(field))) => {
                Ok(SISMember {
                    key: String::from_utf8_lossy(&key).to_string(),
                    field: String::from_utf8_lossy(&field).to_string(),
                })
            }
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;
    use bytes::BytesMut;

    use super::*;
    use crate::RespDecode;
    #[test]
    fn test_sadd_tryfrom() -> anyhow::Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*4\r\n$4\r\nsadd\r\n$3\r\nkey\r\n$3\r\n123\r\n$3\r\n345\r\n");
        let frames = RespArray::decode(&mut buf)?;
        let cmd = Sadd::try_from(frames).unwrap();
        assert_eq!(cmd.key, "key");
        assert_eq!(cmd.members, vec!["123".to_string(), "345".to_string()]);
        Ok(())
    }

    #[test]
    fn test_sismember_tryfrom() -> anyhow::Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*3\r\n$9\r\nsismember\r\n$3\r\nkey\r\n$3\r\n123\r\n");
        let frames = RespArray::decode(&mut buf)?;
        let cmd = SISMember::try_from(frames).unwrap();
        assert_eq!(cmd.key, "key");
        assert_eq!(cmd.field, "123");
        Ok(())
    }

    #[test]
    fn test_sadd_execute() -> anyhow::Result<()> {
        let backend = Backend::new();
        let cmd = Sadd {
            key: "key".to_string(),
            members: vec!["123".to_string(), "345".to_string()],
        };
        let res = cmd.execute(&backend);
        assert_eq!(res, RespFrame::Integer(2));
        Ok(())
    }

    #[test]
    fn test_sismember_execute() -> anyhow::Result<()> {
        let backend = Backend::new();
        let cmd = SISMember {
            key: "key".to_string(),
            field: "123".to_string(),
        };
        let res = cmd.execute(&backend);
        assert_eq!(res, RespFrame::Integer(0));

        let cmd_sadd = Sadd {
            key: "key".to_string(),
            members: vec!["123".to_string(), "345".to_string()],
        };

        let _ = cmd_sadd.execute(&backend);
        let res = cmd.execute(&backend);
        assert_eq!(res, RespFrame::Integer(1));
        Ok(())
    }
}
