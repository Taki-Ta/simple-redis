mod array;
mod bool;
mod bulk_string;
mod double;
mod frame;
mod integer;
mod map;
mod null;
mod set;
mod simple_error;
mod simple_string;
mod util;

pub use self::{
    array::{RespArray, RespNullArray},
    bulk_string::{BulkString, RespNullBulkString},
    frame::RespFrame,
    map::RespMap,
    null::RespNull,
    set::RespSet,
    simple_error::SimpleError,
    simple_string::SimpleString,
    util::{extract_fixed_data, extract_simple_frame_data, find_crlf, parse_length},
};
use bytes::BytesMut;
use core::str;
use enum_dispatch::enum_dispatch;
use thiserror::Error;

const CRLF_LEN: usize = 2;
const BUF_CAP: usize = 4096;

#[enum_dispatch]
pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode: Sized {
    const PREFIX: &'static str;
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;
    fn expect_length(buf: &[u8]) -> Result<usize, RespError>;
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RespError {
    #[error("Invalid Frame:{0}")]
    InvalidFrame(String),
    #[error("Invalid Frame Type:{0}")]
    InvalidFrameType(String),
    #[error("Frame is not complete")]
    NotComplete,
    #[error("Parse int error")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Parse float error")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}
