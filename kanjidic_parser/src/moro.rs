use crate::{
    pos_error::PosError,
    shared::{attr_uint, text, SharedError},
};
use kanjidic_types::{moro::MoroSuffix, take_uint, IResult, Moro, NomErrorReason};
use nom::{bytes::complete::take_while, combinator::map_res, sequence::tuple};
use roxmltree::Node;

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum Error {
    #[error("(Moro) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Moro) Unknown index suffix")]
    IndexSuffix,
    #[error("(Moro) Format: {0}, {1}")]
    Format(PosError, NomErrorReason),
}

pub fn from(node: Node) -> Result<Moro, Error> {
    let (_i, (index, suffix)) = parse_index(text(&node)?)
        .map_err(|err| Error::Format(PosError::from(&node), err.into()))?;
    let volume = attr_uint::<u8>(&node, "m_vol")?;
    let page = attr_uint::<u16>(&node, "m_page")?;
    Ok(Moro {
        volume,
        page,
        index,
        suffix,
    })
}

fn parse_index(s: &str) -> IResult<(u16, MoroSuffix)> {
    tuple((take_uint, index_suffix))(s)
}

fn index_suffix(s: &str) -> IResult<MoroSuffix> {
    map_res(
        take_while(|c: char| c.is_ascii_alphabetic()),
        |text| match text {
            "X" => Ok(MoroSuffix::X),
            "P" => Ok(MoroSuffix::P),
            "PX" => Ok(MoroSuffix::PX),
            "" => Ok(MoroSuffix::None),
            _ => Err(Error::IndexSuffix),
        },
    )(s)
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{moro::MoroSuffix, Moro};

    #[test]
    fn pin_yin() {
        let node = DOC
            .descendants()
            .find(|node| {
                node.has_tag_name("dic_ref")
                    && node
                        .attribute("dr_type")
                        .map(|value| value.eq("moro"))
                        .unwrap_or(false)
            })
            .unwrap();
        let moro = from(node);
        assert_eq!(
            moro,
            Ok(Moro {
                volume: Some(1),
                page: Some(525),
                index: 272,
                suffix: MoroSuffix::None,
            })
        )
    }
}
