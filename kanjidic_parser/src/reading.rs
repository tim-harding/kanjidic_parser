use crate::{
    kunyomi, pin_yin,
    pos_error::PosError,
    shared::{attr, text, SharedError},
};
use kanjidic_types::Reading;
use roxmltree::Node;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("(Reading) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Reading) qc_type not recognized: {0}")]
    UnrecognizedType(PosError),
    #[error("(Reading) Pin yin: {0}")]
    PinYin(#[from] pin_yin::Error),
    #[error("(Reading) Kunyomi: {0}")]
    Kunyomi(#[from] kunyomi::Error),
}

pub fn from(node: Node) -> Result<Reading, Error> {
    let r_type = attr(&node, "r_type")?;
    match r_type {
        "pinyin" => Ok(Reading::PinYin(pin_yin::from(node)?)),
        "korean_r" => Ok(Reading::KoreanRomanized(text(&node)?.into())),
        "korean_h" => Ok(Reading::KoreanHangul(text(&node)?.into())),
        "vietnam" => Ok(Reading::Vietnam(text(&node)?.into())),
        "ja_on" => Ok(Reading::Onyomi(text(&node)?.into())),
        "ja_kun" => Ok(Reading::Kunyomi(kunyomi::from(node)?)),
        _ => Err(Error::UnrecognizedType(PosError::from(&node))),
    }
}

#[cfg(test)]
mod tests {
    use super::from;
    use crate::test_shared::DOC;
    use kanjidic_types::{pin_yin::Tone, PinYin, Reading};

    #[test]
    fn reading() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("reading"))
            .unwrap();
        let reading = from(node);
        assert_eq!(
            reading,
            Ok(Reading::PinYin(PinYin {
                romanization: "ya".into(),
                tone: Tone::Falling,
            }))
        )
    }
}
