use crate::{
    codepoint, grade, query_code, radical, reading, reference,
    shared::{child, children, text, text_uint, SharedError},
    stroke_count, translation, variant, CodepointError, GradeError, PosError, QueryCodeError,
    RadicalError, ReadingError, ReferenceError, StrokeCountError, TranslationError, VariantError,
};
use kanjidic_types::{Character, Codepoint, Grade, QueryCode, Radical, Reading, Reference, StrokeCount, Translations, Variant};
use roxmltree::Node;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CharacterError {
    #[error("(Character) Shared: {0}")]
    Shared(#[from] SharedError),
    #[error("(Character) Codepoint: {0}")]
    Codepoint(#[from] CodepointError),
    #[error("(Character) Radical: {0}")]
    Radical(#[from] RadicalError),
    #[error("(Character) Grade: {0}")]
    Grade(#[from] GradeError),
    #[error("(Character) Stroke count: {0}")]
    StrokeCount(#[from] StrokeCountError),
    #[error("(Character) Variant: {0}")]
    Variant(#[from] VariantError),
    #[error("(Character) Translation: {0}")]
    Translation(#[from] TranslationError),
    #[error("(Character) Reading: {0}")]
    Reading(#[from] ReadingError),
    #[error("(Character) Query code: {0}")]
    QueryCode(#[from] QueryCodeError),
    #[error("(Character) Dictionary reference: {0}")]
    DictionaryReference(#[from] ReferenceError),
    #[error("(Character) Nanori node missing text: {0}")]
    NanoriText(PosError),
    #[error("(Character) Expected a single char")]
    NonCharString,
}

struct CharacterBuilder {
    /// The character itself.
    pub literal: Option<char>,
    /// Alternate encodings for the character.
    pub codepoints: Option<Vec<Codepoint>>,
    /// Alternate classifications for the character by radical.
    pub radicals: Option<Vec<Radical>>,
    /// The kanji grade level.
    pub grade: Option<Grade>,
    /// The stroke count of the character.
    pub stroke_counts: Option<StrokeCount>,
    /// Cross-references to other characters or alternative indexings.
    pub variants: Option<Vec<Variant>>,
    /// A ranking of how often the character appears in newspapers.
    pub frequency: Option<u16>,
    /// The kanji's name as a radical if it is one.
    pub radical_names: Option<Vec<String>>,
    /// Old JLPT level of the kanji. Based on pre-2010 test levels
    /// that go up to four, not five.
    pub jlpt: Option<u8>,
    /// Indexes into dictionaries and other instructional books
    pub references: Option<Vec<Reference>>,
    /// Codes used to identify the kanji
    pub query_codes: Option<Vec<QueryCode>>,
    /// Different ways the kanji can be read.
    pub readings: Option<Vec<Reading>>,
    /// Translations of the kanji into different languages.
    pub translations: Option<Translations>,
    /// Japanese readings associated with names.
    pub nanori: Option<Vec<String>>,
    /// The constituent radicals in the kanji
    pub decomposition: Option<Vec<char>>,
}

impl CharacterBuilder {
    pub fn new() -> Self {
        Self {
            literal: None,
            codepoints: None,
            radicals: None,
            grade: None,
            stroke_counts: None,
            variants: None,
            frequency: None,
            radical_names: None,
            jlpt: None,
            references: None,
            query_codes: None,
            readings: None,
            translations: None,
            nanori: None,
            decomposition: None,
        }
    }

    pub fn literal(mut self, literal: char) -> Self {
        self.literal = Some(literal);
        self
    }

    pub fn codepoints(mut self, codepoints: Vec<Codepoint>) -> Self {
        self.codepoints = Some(codepoints);
        self
    }

    pub fn radicals(mut self, radicals: Vec<Radical>) -> Self {
        self.radicals = Some(radicals);
        self
    }

    pub fn grade(mut self, grade: Grade) -> Self {
        self.grade = Some(grade);
        self
    }

    pub fn stroke_counts(mut self, stroke_counts: StrokeCount) -> Self {
        self.stroke_counts = Some(stroke_counts);
        self
    }

    pub fn variants(mut self, variants: Vec<Variant>) -> Self {
        self.variants = Some(variants);
        self
    }

    pub fn frequency(mut self, frequency: u16) -> Self {
        self.frequency = Some(frequency);
        self
    }

    pub fn radical_names(mut self, radical_names: Vec<String>) -> Self {
        self.radical_names = Some(radical_names);
        self
    }

    pub fn jlpt(mut self, jlpt: u8) -> Self {
        self.jlpt = Some(jlpt);
        self
    }

    pub fn references(mut self, references: Vec<Reference>) -> Self {
        self.references = Some(references);
        self
    }

    pub fn query_codes(mut self, query_codes: Vec<QueryCode>) -> Self {
        self.query_codes = Some(query_codes);
        self
    }

    pub fn readings(mut self, readings: Vec<Reading>) -> Self {
        self.readings = Some(readings);
        self
    }

    pub fn translations(mut self, translations: Translations) -> Self {
        self.translations = Some(translations);
        self
    }

    pub fn nanori(mut self, nanori: Vec<String>) -> Self {
        self.nanori = Some(nanori);
        self
    }

    pub fn decomposition(mut self, decomposition: Vec<char>) -> Self {
        self.decomposition = Some(decomposition);
        self
    }
}

pub fn string_to_char(s: &str) -> Result<char, CharacterError> {
    let mut chars = s.chars();
    let radical = chars.next().ok_or(CharacterError::NonCharString);
    match chars.next() {
        Some(_) => Err(CharacterError::NonCharString),
        None => radical,
    }
}

pub fn from(node: Node) -> Result<Character, CharacterError> {
    let literal = string_to_char(text(child(node, "literal")?)?)?.to_owned();
    let codepoints = children(child(node, "codepoint")?, "cp_value", codepoint::from)?;
    let radicals = children(child(node, "radical")?, "rad_value", radical::from)?;
    let misc = child(node, "misc")?;
    let grade = coalesce(child(misc, "grade").ok().map(grade::from))?;
    let stroke_counts = stroke_count::from(misc)?;
    let variants = children(misc, "variant", variant::from)?;
    let frequency = coalesce(child(misc, "freq").ok().map(text_uint::<u16>))?;
    let radical_names =
        children::<_, SharedError, _>(misc, "rad_name", |child| Ok(text(child)?.to_owned()))?;
    let jlpt = coalesce(child(misc, "jlpt").ok().map(text_uint::<u8>))?;
    let references = match child(node, "dic_number") {
        Ok(dic_number) => Ok(children(dic_number, "dic_ref", reference::from)?),
        Err(SharedError::MissingChild(_, _)) => Ok(vec![]),
        Err(other) => Err(other),
    }?;
    let query_codes = children(child(node, "query_code")?, "q_code", query_code::from)?;
    let (readings, nanori, translations) = match child(node, "reading_meaning") {
        Ok(reading_meaning) => {
            let rmgroup = child(reading_meaning, "rmgroup")?;
            let readings = children(rmgroup, "reading", reading::from)?;
            let translations = translation::from(rmgroup)?;
            let nanori = children(reading_meaning, "nanori", |child| {
                text(child)
                    .map(|s: &str| s.to_owned())
                    .map_err(|_| CharacterError::NanoriText(PosError::from(reading_meaning)))
            })?;
            (readings, nanori, translations)
        }
        Err(_) => (vec![], vec![], Translations::default()),
    };
    let decomposition = decomposition(literal);
    Ok(Character {
        literal,
        codepoints,
        radicals,
        grade,
        stroke_counts,
        variants,
        frequency,
        radical_names,
        jlpt,
        references,
        query_codes,
        nanori,
        readings,
        translations,
        decomposition,
    })
}

fn decomposition(literal: char) -> Vec<char> {
    for decomposition in kradical_static::DECOMPOSITIONS {
        if decomposition.kanji == literal {
            let out: Vec<char> = decomposition
                .radicals
                .iter()
                .map(|&c| c)
                .collect();
            return out;
        }
    }
    vec![]
}

fn coalesce<T, E: std::error::Error>(opt: Option<Result<T, E>>) -> Result<Option<T>, E> {
    Ok(match opt {
        Some(v) => Some(v?),
        None => None,
    })
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, iter::FromIterator};

    use kanjidic_types::{
        Character, Codepoint, DeRoo, ExtremeBottom, ExtremeTop, FourCorner, Grade, KangXi, Kunyomi,
        KunyomiKind, Kuten, Moro, MoroSuffix, Oneill, OneillSuffix, PinYin, QueryCode, Radical,
        RadicalKind, Reading, Reference, ShDesc, Skip, SkipSolid, SolidSubpattern, Stroke,
        StrokeCount, Tone, Variant,
    };

    use super::from;
    use crate::test_shared::DOC;

    #[test]
    fn character() {
        let node = DOC
            .descendants()
            .find(|node| node.has_tag_name("character"))
            .unwrap();
        let character = from(node);
        assert_eq!(
            character,
            Ok(Character {
                literal: "亜".into(),
                decomposition: Some(vec!["｜".into(), "一".into(), "口".into()]),
                codepoints: vec![
                    Codepoint::Unicode(20124),
                    Codepoint::Jis208(Kuten {
                        plane: 1,
                        ku: 16,
                        ten: 1,
                    })
                ],
                radicals: vec![
                    Radical {
                        kind: RadicalKind::Classical,
                        radical: KangXi::Two,
                    },
                    Radical {
                        kind: RadicalKind::Nelson,
                        radical: KangXi::One,
                    },
                ],
                grade: Some(Grade::Jouyou),
                stroke_counts: StrokeCount {
                    accepted: 7,
                    miscounts: vec![]
                },
                variants: vec![Variant::Jis208(Kuten {
                    plane: 1,
                    ku: 48,
                    ten: 19,
                })],
                frequency: Some(1509),
                jlpt: Some(1),
                references: vec![
                    Reference::NelsonClassic(43),
                    Reference::NelsonNew(81),
                    Reference::Njecd(3540),
                    Reference::Kkd(4354),
                    Reference::Kkld(2204),
                    Reference::Kkld2ed(2966),
                    Reference::Heisig(1809),
                    Reference::Heisig6(1950),
                    Reference::Gakken(1331),
                    Reference::OneillNames(Oneill {
                        number: 525,
                        suffix: OneillSuffix::None,
                    }),
                    Reference::OneillKk(1788),
                    Reference::Moro(Moro {
                        volume: Some(1),
                        page: Some(525),
                        index: 272,
                        suffix: MoroSuffix::None,
                    }),
                    Reference::Henshall(997),
                    Reference::ShKk(1616),
                    Reference::ShKk2(1724),
                    Reference::Jfcards(1032),
                    Reference::TuttleCards(1092),
                    Reference::KanjiInContext(1818),
                    Reference::KodanshaCompact(35),
                    Reference::Maniette(1827),
                ],
                query_codes: vec![
                    QueryCode::Skip(Skip::Solid(SkipSolid {
                        total_stroke_count: 7,
                        solid_subpattern: SolidSubpattern::TopLine,
                    })),
                    QueryCode::SpahnHadamitzky(ShDesc {
                        radical_strokes: 0,
                        radical: 'a',
                        other_strokes: 7,
                        sequence: 14,
                    }),
                    QueryCode::FourCorner(FourCorner {
                        top_left: Stroke::LineHorizontal,
                        top_right: Stroke::Lid,
                        bottom_left: Stroke::LineHorizontal,
                        bottom_right: Stroke::Lid,
                        fifth_corner: Some(Stroke::Box),
                    }),
                    QueryCode::DeRoo(DeRoo {
                        top: ExtremeTop::Bald,
                        bottom: ExtremeBottom::StandingBottom,
                    }),
                ],
                radical_names: vec![],
                nanori: vec!["や".into(), "つぎ".into(), "つぐ".into(),],
                readings: vec![
                    Reading::PinYin(PinYin {
                        romanization: "ya".into(),
                        tone: Tone::Falling,
                    }),
                    Reading::KoreanRomanized("a".into()),
                    Reading::KoreanHangul("아".into()),
                    Reading::Vietnam("A".into()),
                    Reading::Vietnam("Á".into()),
                    Reading::Onyomi("ア".into()),
                    Reading::Kunyomi(Kunyomi {
                        kind: KunyomiKind::Normal,
                        okurigana: vec!["つ".into(), "ぐ".into(),]
                    })
                ],
                translations: HashMap::from_iter([
                    (
                        "en".to_owned(),
                        vec![
                            "Asia".to_owned(),
                            "rank next".to_owned(),
                            "come after".to_owned(),
                            "-ous".to_owned(),
                        ]
                    ),
                    (
                        "fr".to_owned(),
                        vec![
                            "Asie".to_owned(),
                            "suivant".to_owned(),
                            "sub-".to_owned(),
                            "sous-".to_owned(),
                        ]
                    ),
                    (
                        "pt".to_owned(),
                        vec![
                            "Ásia".to_owned(),
                            "próxima".to_owned(),
                            "o que vem depois".to_owned(),
                            "-ous".to_owned(),
                        ]
                    ),
                    (
                        "es".to_owned(),
                        vec![
                            "pref. para indicar".to_owned(),
                            "venir después de".to_owned(),
                            "Asia".to_owned(),
                        ]
                    )
                ])
            })
        )
    }
}
