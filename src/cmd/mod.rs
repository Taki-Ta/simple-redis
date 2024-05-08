mod echo;
mod hmap;
mod map;
mod member;

use crate::{backend::Backend, BulkString, RespArray, RespError, RespFrame, SimpleString};
use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use thiserror::Error;

use self::{
    echo::Echo,
    hmap::{HGet, HGetAll, HMGet, HSet},
    map::{Get, Set},
    member::{SISMember, Sadd},
};

lazy_static! {
    static ref REST_OK: RespFrame = SimpleString::new("OK").into();
    static ref REST_NIL: RespFrame = BulkString::new("").into();
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CommandError {
    #[error("Invalid command:{0}")]
    InvalidCommand(String),
    #[error("Invalid argument:{0}")]
    InvalidArgument(String),
    #[error("{0}")]
    RespError(#[from] RespError),
}

#[enum_dispatch]
pub trait CommandExecutor {
    fn execute(&self, backend: &Backend) -> RespFrame;
}

#[derive(Debug)]
#[enum_dispatch(CommandExecutor)]
pub enum Command {
    Get(Get),
    Set(Set),
    HGet(HGet),
    HSet(HSet),
    HGetAll(HGetAll),
    UnRecognized(UnRecognized),
    Echo(Echo),
    HMGet(HMGet),
    SADD(Sadd),
    SISMEMBER(SISMember),
}

#[derive(Debug)]
pub struct UnRecognized;

impl CommandExecutor for UnRecognized {
    fn execute(&self, _backend: &Backend) -> RespFrame {
        REST_OK.clone()
    }
}

impl TryFrom<RespFrame> for Command {
    type Error = CommandError;

    fn try_from(value: RespFrame) -> Result<Self, Self::Error> {
        match value {
            RespFrame::Array(value) => Command::try_from(value),
            _ => Err(CommandError::InvalidCommand(
                "Command must be an array".to_string(),
            )),
        }
    }
}

impl TryFrom<RespArray> for Command {
    type Error = CommandError;

    fn try_from(value: RespArray) -> Result<Self, Self::Error> {
        println!("receive : {:?}", value);
        match value.first() {
            Some(RespFrame::BulkString(ref cmd)) => match cmd.to_ascii_lowercase().as_slice() {
                b"get" => Get::try_from(value).map(Command::Get),
                b"set" => Set::try_from(value).map(Command::Set),
                b"hget" => HGet::try_from(value).map(Command::HGet),
                b"hset" => HSet::try_from(value).map(Command::HSet),
                b"hgetall" => HGetAll::try_from(value).map(Command::HGetAll),
                b"echo" => Echo::try_from(value).map(Command::Echo),
                b"hmget" => HMGet::try_from(value).map(Command::HMGet),
                b"sadd" => Sadd::try_from(value).map(Command::SADD),
                b"sismember" => SISMember::try_from(value).map(Command::SISMEMBER),
                _ => Ok(UnRecognized.into()),
            },
            _ => Err(CommandError::InvalidCommand(
                "Command must have a BulkString as the first argument".to_string(),
            )),
        }
    }
}

pub fn validate_command(
    value: &RespArray,
    names: &[&'static str],
    expect_len: usize,
    comparator: impl Fn(usize, usize) -> bool,
) -> Result<(), CommandError> {
    if !comparator(value.len(), expect_len + 1) {
        return Err(CommandError::InvalidArgument(format!(
            "command {} expected {} arguments",
            names.join(" "),
            expect_len
        )));
    }
    for (i, name) in names.iter().enumerate() {
        match value[i] {
            RespFrame::BulkString(ref s) => {
                let d = s.as_ref();
                if s.as_ref().to_ascii_lowercase() != name.as_bytes() {
                    return Err(CommandError::InvalidCommand(format!(
                        "expected {} got {}",
                        name,
                        String::from_utf8_lossy(d)
                    )));
                }
            }
            _ => {
                return Err(CommandError::InvalidCommand(
                    "Command must have a BulkString as the first argument".to_string(),
                ))
            }
        }
    }
    Ok(())
}

pub fn validate_command_exact_length(
    value: &RespArray,
    names: &[&'static str],
    expect_len: usize,
) -> Result<(), CommandError> {
    validate_command(value, names, expect_len, |v, e| v == e)
}

pub fn validate_command_minimum_length(
    value: &RespArray,
    names: &[&'static str],
    expect_len: usize,
) -> Result<(), CommandError> {
    validate_command(value, names, expect_len, |v, e| v >= e)
}

pub fn extract_args(value: RespArray) -> Result<Vec<RespFrame>, CommandError> {
    Ok(value.0.into_iter().skip(1).collect::<Vec<RespFrame>>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BulkString;

    #[test]
    fn test_validate_command() {
        let frames = RespArray::new(vec![
            BulkString::new("get".as_bytes().to_vec()).into(),
            BulkString::new("hello".as_bytes().to_vec()).into(),
        ]);
        assert!(validate_command_exact_length(&frames, &["get"], 1).is_ok());

        let frames = RespArray::new(vec![
            BulkString::new("set".as_bytes().to_vec()).into(),
            BulkString::new("hello".as_bytes().to_vec()).into(),
            BulkString::new("world".as_bytes().to_vec()).into(),
        ]);
        assert_eq!(
            validate_command_exact_length(&frames, &["set"], 1).unwrap_err(),
            CommandError::InvalidArgument("command set expected 1 arguments".to_string())
        );
    }

    #[test]
    fn test_extract_args() {
        let frames = RespArray::new(vec![
            BulkString::new("set".as_bytes().to_vec()).into(),
            BulkString::new("hello".as_bytes().to_vec()).into(),
            BulkString::new("world".as_bytes().to_vec()).into(),
        ]);
        let args = extract_args(frames).unwrap();
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], BulkString::new("hello".as_bytes().to_vec()).into());
        assert_eq!(args[1], BulkString::new("world".as_bytes().to_vec()).into());
    }
}
