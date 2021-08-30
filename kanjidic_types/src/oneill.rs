use crate::{shared::NomErrorReason, take_uint, IResult, NomErr};
use nom::{
    bytes::complete::take_while,
    combinator::{map, map_res},
    sequence::tuple,
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use thiserror::Error;

/// An index into the Japanese Names reference book
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Oneill {
    /// The reference number
    pub number: u16,
    /// A reference's suffix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<OneillSuffix>,
}

/// The suffix for a Japanese Names reference
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum OneillSuffix {
    /// 'A' suffix
    A,
}

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum OneillParseError {
    #[error("(Oneill) Unknown reference suffix")]
    UnknownSuffix,
    #[error("(Oneill) Format: {0}")]
    Format(NomErrorReason),
}

impl<'a> From<NomErr<'a>> for OneillParseError {
    fn from(err: NomErr<'a>) -> Self {
        OneillParseError::Format(err.into())
    }
}

impl TryFrom<&str> for Oneill {
    type Error = OneillParseError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let (_i, index) = parse(text)?;
        Ok(index)
    }
}

fn parse(s: &str) -> IResult<Oneill> {
    map(parts, |parts| {
        let (number, suffix) = parts;
        Oneill { number, suffix }
    })(s)
}

fn parts(s: &str) -> IResult<(u16, Option<OneillSuffix>)> {
    tuple((take_uint, suffix))(s)
}

fn suffix(s: &str) -> IResult<Option<OneillSuffix>> {
    map_res(take_while(|c: char| c.is_ascii_alphabetic()), |v| match v {
        "A" => Ok(Some(OneillSuffix::A)),
        "" => Ok(None),
        _ => Err(OneillParseError::UnknownSuffix),
    })(s)
}
