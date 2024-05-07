use crate::{
    extract_args, validate_command, Backend, BulkString, CommandError, CommandExecutor, RespArray,
    RespFrame,
};

#[derive(Debug)]
pub struct Echo {
    value: String,
}

impl TryFrom<RespArray> for Echo {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        validate_command(&value, &["echo"], 1)?;
        let args = extract_args(value)?;
        match &args[0] {
            RespFrame::BulkString(key) => Ok(Echo {
                value: String::from_utf8_lossy(key).to_string(),
            }),
            _ => Err(CommandError::InvalidArgument("Invalid key".to_string())),
        }
    }
}

impl CommandExecutor for Echo {
    fn execute(&self, _backend: &Backend) -> RespFrame {
        BulkString::new(self.value.to_owned()).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{backend, resp::RespDecode};

    #[test]
    fn test_echo() {
        let backend = backend::Backend::new();
        let input = "$5\r\nhello\r\n".as_bytes();
        let frame = RespFrame::decode(&mut input.into()).unwrap();
        let cmd = Echo {
            value: "hello".to_string(),
        };
        let resp = cmd.execute(&backend);
        assert_eq!(frame, resp);
    }
}
