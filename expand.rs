#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod resp {
    mod decode {}
    mod encode {
        use crate::{
            BulkString, RespArray, RespEncode, RespMap, RespNull, RespNullArray,
            RespNullBulkString, RespSet, SimpleError, SimpleString,
        };
        const BUF_CAP: usize = 4096;
        impl RespEncode for SimpleString {
            fn encode(self) -> Vec<u8> {
                {
                    let res = ::alloc::fmt::format(format_args!("+{0}\r\n", self.0));
                    res
                }
                    .into_bytes()
            }
        }
        impl RespEncode for SimpleError {
            fn encode(self) -> Vec<u8> {
                {
                    let res = ::alloc::fmt::format(format_args!("-{0}\r\n", self.0));
                    res
                }
                    .into_bytes()
            }
        }
        impl RespEncode for i64 {
            fn encode(self) -> Vec<u8> {
                let sign = if self < 0 { "" } else { "+" };
                {
                    let res = ::alloc::fmt::format(
                        format_args!(":{0}{1}\r\n", sign, self),
                    );
                    res
                }
                    .into_bytes()
            }
        }
        impl RespEncode for BulkString {
            fn encode(self) -> Vec<u8> {
                let mut buf = Vec::with_capacity(self.len() + 16);
                buf.extend_from_slice(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!("${0}\r\n", self.len()),
                        );
                        res
                    }
                        .into_bytes(),
                );
                buf.extend_from_slice(&self);
                buf.extend_from_slice(b"\r\n");
                buf
            }
        }
        impl RespEncode for RespNullBulkString {
            fn encode(self) -> Vec<u8> {
                b"$-1\r\n".to_vec()
            }
        }
        impl RespEncode for RespArray {
            fn encode(self) -> Vec<u8> {
                let mut buf = Vec::with_capacity(BUF_CAP);
                buf.extend_from_slice(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!("*{0}\r\n", self.len()),
                        );
                        res
                    }
                        .into_bytes(),
                );
                for frame in self.0 {
                    buf.extend_from_slice(&frame.encode());
                }
                buf
            }
        }
        impl RespEncode for RespNullArray {
            fn encode(self) -> Vec<u8> {
                let mut buf = Vec::with_capacity(16);
                buf.extend_from_slice(b"*-1\r\n");
                buf
            }
        }
        impl RespEncode for RespNull {
            fn encode(self) -> Vec<u8> {
                b"_\r\n".to_vec()
            }
        }
        impl RespEncode for bool {
            fn encode(self) -> Vec<u8> {
                let c = if self { 't' } else { 'f' };
                {
                    let res = ::alloc::fmt::format(format_args!("#{0}\r\n", c));
                    res
                }
                    .into_bytes()
            }
        }
        impl RespEncode for f64 {
            fn encode(self) -> Vec<u8> {
                let mut buf = Vec::with_capacity(32);
                let ret = if self.abs() > 1e+8 {
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(",{0:+e}\r\n", self),
                        );
                        res
                    }
                } else {
                    let sign = if self < 0.0 { "" } else { "+" };
                    {
                        let res = ::alloc::fmt::format(
                            format_args!(",{0}{1}\r\n", sign, self),
                        );
                        res
                    }
                };
                buf.extend_from_slice(&ret.into_bytes());
                buf
            }
        }
        impl RespEncode for RespMap {
            fn encode(self) -> Vec<u8> {
                let mut buf = Vec::with_capacity(BUF_CAP);
                buf.extend_from_slice(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!("%{0}\r\n", self.len()),
                        );
                        res
                    }
                        .into_bytes(),
                );
                for (k, v) in self.0 {
                    buf.extend_from_slice(&SimpleString::new(k).encode());
                    buf.extend_from_slice(&v.encode());
                }
                buf
            }
        }
        impl RespEncode for RespSet {
            fn encode(self) -> Vec<u8> {
                let mut buf = Vec::with_capacity(BUF_CAP);
                buf.extend_from_slice(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!("~{0}\r\n", self.len()),
                        );
                        res
                    }
                        .into_bytes(),
                );
                for frame in self.0 {
                    buf.extend_from_slice(&frame.encode());
                }
                buf
            }
        }
    }
    use core::str;
    use enum_dispatch::enum_dispatch;
    use std::{collections::BTreeMap, ops::Deref};
    pub trait RespEncode {
        fn encode(self) -> Vec<u8>;
    }
    pub trait RespDecode {
        fn decode(buf: Self) -> Result<RespFrame, String>;
    }
    pub enum RespFrame {
        SimpleString(SimpleString),
        Error(SimpleError),
        Integer(i64),
        BulkString(BulkString),
        NullBulkString(RespNullBulkString),
        Array(RespArray),
        NullArray(RespNullArray),
        Null(RespNull),
        Boolean(bool),
        Double(f64),
        Map(RespMap),
        Set(RespSet),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RespFrame {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                RespFrame::SimpleString(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "SimpleString",
                        &__self_0,
                    )
                }
                RespFrame::Error(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Error",
                        &__self_0,
                    )
                }
                RespFrame::Integer(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Integer",
                        &__self_0,
                    )
                }
                RespFrame::BulkString(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BulkString",
                        &__self_0,
                    )
                }
                RespFrame::NullBulkString(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "NullBulkString",
                        &__self_0,
                    )
                }
                RespFrame::Array(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Array",
                        &__self_0,
                    )
                }
                RespFrame::NullArray(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "NullArray",
                        &__self_0,
                    )
                }
                RespFrame::Null(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Null",
                        &__self_0,
                    )
                }
                RespFrame::Boolean(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Boolean",
                        &__self_0,
                    )
                }
                RespFrame::Double(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Double",
                        &__self_0,
                    )
                }
                RespFrame::Map(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Map",
                        &__self_0,
                    )
                }
                RespFrame::Set(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Set",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RespFrame {
        #[inline]
        fn clone(&self) -> RespFrame {
            match self {
                RespFrame::SimpleString(__self_0) => {
                    RespFrame::SimpleString(::core::clone::Clone::clone(__self_0))
                }
                RespFrame::Error(__self_0) => {
                    RespFrame::Error(::core::clone::Clone::clone(__self_0))
                }
                RespFrame::Integer(__self_0) => {
                    RespFrame::Integer(::core::clone::Clone::clone(__self_0))
                }
                RespFrame::BulkString(__self_0) => {
                    RespFrame::BulkString(::core::clone::Clone::clone(__self_0))
                }
                RespFrame::NullBulkString(__self_0) => {
                    RespFrame::NullBulkString(::core::clone::Clone::clone(__self_0))
                }
                RespFrame::Array(__self_0) => {
                    RespFrame::Array(::core::clone::Clone::clone(__self_0))
                }
                RespFrame::NullArray(__self_0) => {
                    RespFrame::NullArray(::core::clone::Clone::clone(__self_0))
                }
                RespFrame::Null(__self_0) => {
                    RespFrame::Null(::core::clone::Clone::clone(__self_0))
                }
                RespFrame::Boolean(__self_0) => {
                    RespFrame::Boolean(::core::clone::Clone::clone(__self_0))
                }
                RespFrame::Double(__self_0) => {
                    RespFrame::Double(::core::clone::Clone::clone(__self_0))
                }
                RespFrame::Map(__self_0) => {
                    RespFrame::Map(::core::clone::Clone::clone(__self_0))
                }
                RespFrame::Set(__self_0) => {
                    RespFrame::Set(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RespFrame {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RespFrame {
        #[inline]
        fn eq(&self, other: &RespFrame) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (
                        RespFrame::SimpleString(__self_0),
                        RespFrame::SimpleString(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (RespFrame::Error(__self_0), RespFrame::Error(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (RespFrame::Integer(__self_0), RespFrame::Integer(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (
                        RespFrame::BulkString(__self_0),
                        RespFrame::BulkString(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        RespFrame::NullBulkString(__self_0),
                        RespFrame::NullBulkString(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (RespFrame::Array(__self_0), RespFrame::Array(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (RespFrame::NullArray(__self_0), RespFrame::NullArray(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (RespFrame::Null(__self_0), RespFrame::Null(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (RespFrame::Boolean(__self_0), RespFrame::Boolean(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (RespFrame::Double(__self_0), RespFrame::Double(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (RespFrame::Map(__self_0), RespFrame::Map(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    (RespFrame::Set(__self_0), RespFrame::Set(__arg1_0)) => {
                        *__self_0 == *__arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for RespFrame {
        #[inline]
        fn partial_cmp(
            &self,
            other: &RespFrame,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            match (self, other) {
                (
                    RespFrame::SimpleString(__self_0),
                    RespFrame::SimpleString(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                (RespFrame::Error(__self_0), RespFrame::Error(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (RespFrame::Integer(__self_0), RespFrame::Integer(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (RespFrame::BulkString(__self_0), RespFrame::BulkString(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (
                    RespFrame::NullBulkString(__self_0),
                    RespFrame::NullBulkString(__arg1_0),
                ) => ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0),
                (RespFrame::Array(__self_0), RespFrame::Array(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (RespFrame::NullArray(__self_0), RespFrame::NullArray(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (RespFrame::Null(__self_0), RespFrame::Null(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (RespFrame::Boolean(__self_0), RespFrame::Boolean(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (RespFrame::Double(__self_0), RespFrame::Double(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (RespFrame::Map(__self_0), RespFrame::Map(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                (RespFrame::Set(__self_0), RespFrame::Set(__arg1_0)) => {
                    ::core::cmp::PartialOrd::partial_cmp(__self_0, __arg1_0)
                }
                _ => ::core::cmp::PartialOrd::partial_cmp(&__self_tag, &__arg1_tag),
            }
        }
    }
    impl ::core::convert::From<SimpleString> for RespFrame {
        fn from(v: SimpleString) -> RespFrame {
            RespFrame::SimpleString(v)
        }
    }
    impl ::core::convert::From<SimpleError> for RespFrame {
        fn from(v: SimpleError) -> RespFrame {
            RespFrame::Error(v)
        }
    }
    impl ::core::convert::From<i64> for RespFrame {
        fn from(v: i64) -> RespFrame {
            RespFrame::Integer(v)
        }
    }
    impl ::core::convert::From<BulkString> for RespFrame {
        fn from(v: BulkString) -> RespFrame {
            RespFrame::BulkString(v)
        }
    }
    impl ::core::convert::From<RespNullBulkString> for RespFrame {
        fn from(v: RespNullBulkString) -> RespFrame {
            RespFrame::NullBulkString(v)
        }
    }
    impl ::core::convert::From<RespArray> for RespFrame {
        fn from(v: RespArray) -> RespFrame {
            RespFrame::Array(v)
        }
    }
    impl ::core::convert::From<RespNullArray> for RespFrame {
        fn from(v: RespNullArray) -> RespFrame {
            RespFrame::NullArray(v)
        }
    }
    impl ::core::convert::From<RespNull> for RespFrame {
        fn from(v: RespNull) -> RespFrame {
            RespFrame::Null(v)
        }
    }
    impl ::core::convert::From<bool> for RespFrame {
        fn from(v: bool) -> RespFrame {
            RespFrame::Boolean(v)
        }
    }
    impl ::core::convert::From<f64> for RespFrame {
        fn from(v: f64) -> RespFrame {
            RespFrame::Double(v)
        }
    }
    impl ::core::convert::From<RespMap> for RespFrame {
        fn from(v: RespMap) -> RespFrame {
            RespFrame::Map(v)
        }
    }
    impl ::core::convert::From<RespSet> for RespFrame {
        fn from(v: RespSet) -> RespFrame {
            RespFrame::Set(v)
        }
    }
    impl ::core::convert::TryInto<SimpleString> for RespFrame {
        type Error = &'static str;
        fn try_into(
            self,
        ) -> ::core::result::Result<
            SimpleString,
            <Self as ::core::convert::TryInto<SimpleString>>::Error,
        > {
            match self {
                RespFrame::SimpleString(v) => Ok(v),
                RespFrame::Error(v) => {
                    Err("Tried to convert variant Error to SimpleString")
                }
                RespFrame::Integer(v) => {
                    Err("Tried to convert variant Integer to SimpleString")
                }
                RespFrame::BulkString(v) => {
                    Err("Tried to convert variant BulkString to SimpleString")
                }
                RespFrame::NullBulkString(v) => {
                    Err("Tried to convert variant NullBulkString to SimpleString")
                }
                RespFrame::Array(v) => {
                    Err("Tried to convert variant Array to SimpleString")
                }
                RespFrame::NullArray(v) => {
                    Err("Tried to convert variant NullArray to SimpleString")
                }
                RespFrame::Null(v) => {
                    Err("Tried to convert variant Null to SimpleString")
                }
                RespFrame::Boolean(v) => {
                    Err("Tried to convert variant Boolean to SimpleString")
                }
                RespFrame::Double(v) => {
                    Err("Tried to convert variant Double to SimpleString")
                }
                RespFrame::Map(v) => Err("Tried to convert variant Map to SimpleString"),
                RespFrame::Set(v) => Err("Tried to convert variant Set to SimpleString"),
            }
        }
    }
    impl ::core::convert::TryInto<SimpleError> for RespFrame {
        type Error = &'static str;
        fn try_into(
            self,
        ) -> ::core::result::Result<
            SimpleError,
            <Self as ::core::convert::TryInto<SimpleError>>::Error,
        > {
            match self {
                RespFrame::Error(v) => Ok(v),
                RespFrame::SimpleString(v) => {
                    Err("Tried to convert variant SimpleString to Error")
                }
                RespFrame::Integer(v) => Err("Tried to convert variant Integer to Error"),
                RespFrame::BulkString(v) => {
                    Err("Tried to convert variant BulkString to Error")
                }
                RespFrame::NullBulkString(v) => {
                    Err("Tried to convert variant NullBulkString to Error")
                }
                RespFrame::Array(v) => Err("Tried to convert variant Array to Error"),
                RespFrame::NullArray(v) => {
                    Err("Tried to convert variant NullArray to Error")
                }
                RespFrame::Null(v) => Err("Tried to convert variant Null to Error"),
                RespFrame::Boolean(v) => Err("Tried to convert variant Boolean to Error"),
                RespFrame::Double(v) => Err("Tried to convert variant Double to Error"),
                RespFrame::Map(v) => Err("Tried to convert variant Map to Error"),
                RespFrame::Set(v) => Err("Tried to convert variant Set to Error"),
            }
        }
    }
    impl ::core::convert::TryInto<i64> for RespFrame {
        type Error = &'static str;
        fn try_into(
            self,
        ) -> ::core::result::Result<
            i64,
            <Self as ::core::convert::TryInto<i64>>::Error,
        > {
            match self {
                RespFrame::Integer(v) => Ok(v),
                RespFrame::SimpleString(v) => {
                    Err("Tried to convert variant SimpleString to Integer")
                }
                RespFrame::Error(v) => Err("Tried to convert variant Error to Integer"),
                RespFrame::BulkString(v) => {
                    Err("Tried to convert variant BulkString to Integer")
                }
                RespFrame::NullBulkString(v) => {
                    Err("Tried to convert variant NullBulkString to Integer")
                }
                RespFrame::Array(v) => Err("Tried to convert variant Array to Integer"),
                RespFrame::NullArray(v) => {
                    Err("Tried to convert variant NullArray to Integer")
                }
                RespFrame::Null(v) => Err("Tried to convert variant Null to Integer"),
                RespFrame::Boolean(v) => {
                    Err("Tried to convert variant Boolean to Integer")
                }
                RespFrame::Double(v) => Err("Tried to convert variant Double to Integer"),
                RespFrame::Map(v) => Err("Tried to convert variant Map to Integer"),
                RespFrame::Set(v) => Err("Tried to convert variant Set to Integer"),
            }
        }
    }
    impl ::core::convert::TryInto<BulkString> for RespFrame {
        type Error = &'static str;
        fn try_into(
            self,
        ) -> ::core::result::Result<
            BulkString,
            <Self as ::core::convert::TryInto<BulkString>>::Error,
        > {
            match self {
                RespFrame::BulkString(v) => Ok(v),
                RespFrame::SimpleString(v) => {
                    Err("Tried to convert variant SimpleString to BulkString")
                }
                RespFrame::Error(v) => {
                    Err("Tried to convert variant Error to BulkString")
                }
                RespFrame::Integer(v) => {
                    Err("Tried to convert variant Integer to BulkString")
                }
                RespFrame::NullBulkString(v) => {
                    Err("Tried to convert variant NullBulkString to BulkString")
                }
                RespFrame::Array(v) => {
                    Err("Tried to convert variant Array to BulkString")
                }
                RespFrame::NullArray(v) => {
                    Err("Tried to convert variant NullArray to BulkString")
                }
                RespFrame::Null(v) => Err("Tried to convert variant Null to BulkString"),
                RespFrame::Boolean(v) => {
                    Err("Tried to convert variant Boolean to BulkString")
                }
                RespFrame::Double(v) => {
                    Err("Tried to convert variant Double to BulkString")
                }
                RespFrame::Map(v) => Err("Tried to convert variant Map to BulkString"),
                RespFrame::Set(v) => Err("Tried to convert variant Set to BulkString"),
            }
        }
    }
    impl ::core::convert::TryInto<RespNullBulkString> for RespFrame {
        type Error = &'static str;
        fn try_into(
            self,
        ) -> ::core::result::Result<
            RespNullBulkString,
            <Self as ::core::convert::TryInto<RespNullBulkString>>::Error,
        > {
            match self {
                RespFrame::NullBulkString(v) => Ok(v),
                RespFrame::SimpleString(v) => {
                    Err("Tried to convert variant SimpleString to NullBulkString")
                }
                RespFrame::Error(v) => {
                    Err("Tried to convert variant Error to NullBulkString")
                }
                RespFrame::Integer(v) => {
                    Err("Tried to convert variant Integer to NullBulkString")
                }
                RespFrame::BulkString(v) => {
                    Err("Tried to convert variant BulkString to NullBulkString")
                }
                RespFrame::Array(v) => {
                    Err("Tried to convert variant Array to NullBulkString")
                }
                RespFrame::NullArray(v) => {
                    Err("Tried to convert variant NullArray to NullBulkString")
                }
                RespFrame::Null(v) => {
                    Err("Tried to convert variant Null to NullBulkString")
                }
                RespFrame::Boolean(v) => {
                    Err("Tried to convert variant Boolean to NullBulkString")
                }
                RespFrame::Double(v) => {
                    Err("Tried to convert variant Double to NullBulkString")
                }
                RespFrame::Map(v) => {
                    Err("Tried to convert variant Map to NullBulkString")
                }
                RespFrame::Set(v) => {
                    Err("Tried to convert variant Set to NullBulkString")
                }
            }
        }
    }
    impl ::core::convert::TryInto<RespArray> for RespFrame {
        type Error = &'static str;
        fn try_into(
            self,
        ) -> ::core::result::Result<
            RespArray,
            <Self as ::core::convert::TryInto<RespArray>>::Error,
        > {
            match self {
                RespFrame::Array(v) => Ok(v),
                RespFrame::SimpleString(v) => {
                    Err("Tried to convert variant SimpleString to Array")
                }
                RespFrame::Error(v) => Err("Tried to convert variant Error to Array"),
                RespFrame::Integer(v) => Err("Tried to convert variant Integer to Array"),
                RespFrame::BulkString(v) => {
                    Err("Tried to convert variant BulkString to Array")
                }
                RespFrame::NullBulkString(v) => {
                    Err("Tried to convert variant NullBulkString to Array")
                }
                RespFrame::NullArray(v) => {
                    Err("Tried to convert variant NullArray to Array")
                }
                RespFrame::Null(v) => Err("Tried to convert variant Null to Array"),
                RespFrame::Boolean(v) => Err("Tried to convert variant Boolean to Array"),
                RespFrame::Double(v) => Err("Tried to convert variant Double to Array"),
                RespFrame::Map(v) => Err("Tried to convert variant Map to Array"),
                RespFrame::Set(v) => Err("Tried to convert variant Set to Array"),
            }
        }
    }
    impl ::core::convert::TryInto<RespNullArray> for RespFrame {
        type Error = &'static str;
        fn try_into(
            self,
        ) -> ::core::result::Result<
            RespNullArray,
            <Self as ::core::convert::TryInto<RespNullArray>>::Error,
        > {
            match self {
                RespFrame::NullArray(v) => Ok(v),
                RespFrame::SimpleString(v) => {
                    Err("Tried to convert variant SimpleString to NullArray")
                }
                RespFrame::Error(v) => Err("Tried to convert variant Error to NullArray"),
                RespFrame::Integer(v) => {
                    Err("Tried to convert variant Integer to NullArray")
                }
                RespFrame::BulkString(v) => {
                    Err("Tried to convert variant BulkString to NullArray")
                }
                RespFrame::NullBulkString(v) => {
                    Err("Tried to convert variant NullBulkString to NullArray")
                }
                RespFrame::Array(v) => Err("Tried to convert variant Array to NullArray"),
                RespFrame::Null(v) => Err("Tried to convert variant Null to NullArray"),
                RespFrame::Boolean(v) => {
                    Err("Tried to convert variant Boolean to NullArray")
                }
                RespFrame::Double(v) => {
                    Err("Tried to convert variant Double to NullArray")
                }
                RespFrame::Map(v) => Err("Tried to convert variant Map to NullArray"),
                RespFrame::Set(v) => Err("Tried to convert variant Set to NullArray"),
            }
        }
    }
    impl ::core::convert::TryInto<RespNull> for RespFrame {
        type Error = &'static str;
        fn try_into(
            self,
        ) -> ::core::result::Result<
            RespNull,
            <Self as ::core::convert::TryInto<RespNull>>::Error,
        > {
            match self {
                RespFrame::Null(v) => Ok(v),
                RespFrame::SimpleString(v) => {
                    Err("Tried to convert variant SimpleString to Null")
                }
                RespFrame::Error(v) => Err("Tried to convert variant Error to Null"),
                RespFrame::Integer(v) => Err("Tried to convert variant Integer to Null"),
                RespFrame::BulkString(v) => {
                    Err("Tried to convert variant BulkString to Null")
                }
                RespFrame::NullBulkString(v) => {
                    Err("Tried to convert variant NullBulkString to Null")
                }
                RespFrame::Array(v) => Err("Tried to convert variant Array to Null"),
                RespFrame::NullArray(v) => {
                    Err("Tried to convert variant NullArray to Null")
                }
                RespFrame::Boolean(v) => Err("Tried to convert variant Boolean to Null"),
                RespFrame::Double(v) => Err("Tried to convert variant Double to Null"),
                RespFrame::Map(v) => Err("Tried to convert variant Map to Null"),
                RespFrame::Set(v) => Err("Tried to convert variant Set to Null"),
            }
        }
    }
    impl ::core::convert::TryInto<bool> for RespFrame {
        type Error = &'static str;
        fn try_into(
            self,
        ) -> ::core::result::Result<
            bool,
            <Self as ::core::convert::TryInto<bool>>::Error,
        > {
            match self {
                RespFrame::Boolean(v) => Ok(v),
                RespFrame::SimpleString(v) => {
                    Err("Tried to convert variant SimpleString to Boolean")
                }
                RespFrame::Error(v) => Err("Tried to convert variant Error to Boolean"),
                RespFrame::Integer(v) => {
                    Err("Tried to convert variant Integer to Boolean")
                }
                RespFrame::BulkString(v) => {
                    Err("Tried to convert variant BulkString to Boolean")
                }
                RespFrame::NullBulkString(v) => {
                    Err("Tried to convert variant NullBulkString to Boolean")
                }
                RespFrame::Array(v) => Err("Tried to convert variant Array to Boolean"),
                RespFrame::NullArray(v) => {
                    Err("Tried to convert variant NullArray to Boolean")
                }
                RespFrame::Null(v) => Err("Tried to convert variant Null to Boolean"),
                RespFrame::Double(v) => Err("Tried to convert variant Double to Boolean"),
                RespFrame::Map(v) => Err("Tried to convert variant Map to Boolean"),
                RespFrame::Set(v) => Err("Tried to convert variant Set to Boolean"),
            }
        }
    }
    impl ::core::convert::TryInto<f64> for RespFrame {
        type Error = &'static str;
        fn try_into(
            self,
        ) -> ::core::result::Result<
            f64,
            <Self as ::core::convert::TryInto<f64>>::Error,
        > {
            match self {
                RespFrame::Double(v) => Ok(v),
                RespFrame::SimpleString(v) => {
                    Err("Tried to convert variant SimpleString to Double")
                }
                RespFrame::Error(v) => Err("Tried to convert variant Error to Double"),
                RespFrame::Integer(v) => {
                    Err("Tried to convert variant Integer to Double")
                }
                RespFrame::BulkString(v) => {
                    Err("Tried to convert variant BulkString to Double")
                }
                RespFrame::NullBulkString(v) => {
                    Err("Tried to convert variant NullBulkString to Double")
                }
                RespFrame::Array(v) => Err("Tried to convert variant Array to Double"),
                RespFrame::NullArray(v) => {
                    Err("Tried to convert variant NullArray to Double")
                }
                RespFrame::Null(v) => Err("Tried to convert variant Null to Double"),
                RespFrame::Boolean(v) => {
                    Err("Tried to convert variant Boolean to Double")
                }
                RespFrame::Map(v) => Err("Tried to convert variant Map to Double"),
                RespFrame::Set(v) => Err("Tried to convert variant Set to Double"),
            }
        }
    }
    impl ::core::convert::TryInto<RespMap> for RespFrame {
        type Error = &'static str;
        fn try_into(
            self,
        ) -> ::core::result::Result<
            RespMap,
            <Self as ::core::convert::TryInto<RespMap>>::Error,
        > {
            match self {
                RespFrame::Map(v) => Ok(v),
                RespFrame::SimpleString(v) => {
                    Err("Tried to convert variant SimpleString to Map")
                }
                RespFrame::Error(v) => Err("Tried to convert variant Error to Map"),
                RespFrame::Integer(v) => Err("Tried to convert variant Integer to Map"),
                RespFrame::BulkString(v) => {
                    Err("Tried to convert variant BulkString to Map")
                }
                RespFrame::NullBulkString(v) => {
                    Err("Tried to convert variant NullBulkString to Map")
                }
                RespFrame::Array(v) => Err("Tried to convert variant Array to Map"),
                RespFrame::NullArray(v) => {
                    Err("Tried to convert variant NullArray to Map")
                }
                RespFrame::Null(v) => Err("Tried to convert variant Null to Map"),
                RespFrame::Boolean(v) => Err("Tried to convert variant Boolean to Map"),
                RespFrame::Double(v) => Err("Tried to convert variant Double to Map"),
                RespFrame::Set(v) => Err("Tried to convert variant Set to Map"),
            }
        }
    }
    impl ::core::convert::TryInto<RespSet> for RespFrame {
        type Error = &'static str;
        fn try_into(
            self,
        ) -> ::core::result::Result<
            RespSet,
            <Self as ::core::convert::TryInto<RespSet>>::Error,
        > {
            match self {
                RespFrame::Set(v) => Ok(v),
                RespFrame::SimpleString(v) => {
                    Err("Tried to convert variant SimpleString to Set")
                }
                RespFrame::Error(v) => Err("Tried to convert variant Error to Set"),
                RespFrame::Integer(v) => Err("Tried to convert variant Integer to Set"),
                RespFrame::BulkString(v) => {
                    Err("Tried to convert variant BulkString to Set")
                }
                RespFrame::NullBulkString(v) => {
                    Err("Tried to convert variant NullBulkString to Set")
                }
                RespFrame::Array(v) => Err("Tried to convert variant Array to Set"),
                RespFrame::NullArray(v) => {
                    Err("Tried to convert variant NullArray to Set")
                }
                RespFrame::Null(v) => Err("Tried to convert variant Null to Set"),
                RespFrame::Boolean(v) => Err("Tried to convert variant Boolean to Set"),
                RespFrame::Double(v) => Err("Tried to convert variant Double to Set"),
                RespFrame::Map(v) => Err("Tried to convert variant Map to Set"),
            }
        }
    }
    impl RespEncode for RespFrame {
        #[inline]
        fn encode(self) -> Vec<u8> {
            match self {
                RespFrame::SimpleString(inner) => RespEncode::encode(inner),
                RespFrame::Error(inner) => RespEncode::encode(inner),
                RespFrame::Integer(inner) => RespEncode::encode(inner),
                RespFrame::BulkString(inner) => RespEncode::encode(inner),
                RespFrame::NullBulkString(inner) => RespEncode::encode(inner),
                RespFrame::Array(inner) => RespEncode::encode(inner),
                RespFrame::NullArray(inner) => RespEncode::encode(inner),
                RespFrame::Null(inner) => RespEncode::encode(inner),
                RespFrame::Boolean(inner) => RespEncode::encode(inner),
                RespFrame::Double(inner) => RespEncode::encode(inner),
                RespFrame::Map(inner) => RespEncode::encode(inner),
                RespFrame::Set(inner) => RespEncode::encode(inner),
            }
        }
    }
    pub struct SimpleString(String);
    #[automatically_derived]
    impl ::core::fmt::Debug for SimpleString {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "SimpleString",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SimpleString {
        #[inline]
        fn clone(&self) -> SimpleString {
            SimpleString(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SimpleString {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SimpleString {
        #[inline]
        fn eq(&self, other: &SimpleString) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for SimpleString {
        #[inline]
        fn partial_cmp(
            &self,
            other: &SimpleString,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    pub struct SimpleError(String);
    #[automatically_derived]
    impl ::core::fmt::Debug for SimpleError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "SimpleError", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for SimpleError {
        #[inline]
        fn clone(&self) -> SimpleError {
            SimpleError(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for SimpleError {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for SimpleError {
        #[inline]
        fn eq(&self, other: &SimpleError) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for SimpleError {
        #[inline]
        fn partial_cmp(
            &self,
            other: &SimpleError,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    pub struct BulkString(Vec<u8>);
    #[automatically_derived]
    impl ::core::fmt::Debug for BulkString {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "BulkString", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for BulkString {
        #[inline]
        fn clone(&self) -> BulkString {
            BulkString(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for BulkString {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for BulkString {
        #[inline]
        fn eq(&self, other: &BulkString) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for BulkString {
        #[inline]
        fn partial_cmp(
            &self,
            other: &BulkString,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    pub struct RespNullBulkString;
    #[automatically_derived]
    impl ::core::fmt::Debug for RespNullBulkString {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "RespNullBulkString")
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RespNullBulkString {
        #[inline]
        fn clone(&self) -> RespNullBulkString {
            RespNullBulkString
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RespNullBulkString {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RespNullBulkString {
        #[inline]
        fn eq(&self, other: &RespNullBulkString) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for RespNullBulkString {
        #[inline]
        fn partial_cmp(
            &self,
            other: &RespNullBulkString,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        }
    }
    pub struct RespArray(Vec<RespFrame>);
    #[automatically_derived]
    impl ::core::fmt::Debug for RespArray {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "RespArray", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RespArray {
        #[inline]
        fn clone(&self) -> RespArray {
            RespArray(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RespArray {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RespArray {
        #[inline]
        fn eq(&self, other: &RespArray) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for RespArray {
        #[inline]
        fn partial_cmp(
            &self,
            other: &RespArray,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    pub struct RespNullArray;
    #[automatically_derived]
    impl ::core::fmt::Debug for RespNullArray {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "RespNullArray")
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RespNullArray {
        #[inline]
        fn clone(&self) -> RespNullArray {
            RespNullArray
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RespNullArray {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RespNullArray {
        #[inline]
        fn eq(&self, other: &RespNullArray) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for RespNullArray {
        #[inline]
        fn partial_cmp(
            &self,
            other: &RespNullArray,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        }
    }
    pub struct RespNull;
    #[automatically_derived]
    impl ::core::fmt::Debug for RespNull {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "RespNull")
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RespNull {
        #[inline]
        fn clone(&self) -> RespNull {
            RespNull
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RespNull {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RespNull {
        #[inline]
        fn eq(&self, other: &RespNull) -> bool {
            true
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for RespNull {
        #[inline]
        fn partial_cmp(
            &self,
            other: &RespNull,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::option::Option::Some(::core::cmp::Ordering::Equal)
        }
    }
    pub struct RespMap(BTreeMap<String, RespFrame>);
    #[automatically_derived]
    impl ::core::fmt::Debug for RespMap {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "RespMap", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RespMap {
        #[inline]
        fn clone(&self) -> RespMap {
            RespMap(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RespMap {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RespMap {
        #[inline]
        fn eq(&self, other: &RespMap) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for RespMap {
        #[inline]
        fn partial_cmp(
            &self,
            other: &RespMap,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    pub struct RespSet(Vec<RespFrame>);
    #[automatically_derived]
    impl ::core::fmt::Debug for RespSet {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "RespSet", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for RespSet {
        #[inline]
        fn clone(&self) -> RespSet {
            RespSet(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RespSet {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RespSet {
        #[inline]
        fn eq(&self, other: &RespSet) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for RespSet {
        #[inline]
        fn partial_cmp(
            &self,
            other: &RespSet,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            ::core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
        }
    }
    impl Deref for SimpleString {
        type Target = String;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl Deref for SimpleError {
        type Target = String;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl Deref for BulkString {
        type Target = Vec<u8>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl Deref for RespArray {
        type Target = Vec<RespFrame>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl Deref for RespMap {
        type Target = BTreeMap<String, RespFrame>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl Deref for RespSet {
        type Target = Vec<RespFrame>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl SimpleString {
        pub fn new(s: impl Into<String>) -> Self {
            SimpleString(s.into())
        }
    }
    impl SimpleError {
        pub fn new(s: impl Into<String>) -> Self {
            SimpleError(s.into())
        }
    }
    impl BulkString {
        pub fn new(s: impl Into<Vec<u8>>) -> Self {
            BulkString(s.into())
        }
    }
    impl RespArray {
        pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
            RespArray(s.into())
        }
    }
    impl RespMap {
        pub fn new(s: impl Into<BTreeMap<String, RespFrame>>) -> Self {
            RespMap(s.into())
        }
    }
    impl RespSet {
        pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
            RespSet(s.into())
        }
    }
}
pub use resp::*;
