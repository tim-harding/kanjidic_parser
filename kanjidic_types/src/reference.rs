use crate::{BusyPeople, Moro, Oneill};
use serde::{Deserialize, Serialize};

/// An index number into a particular kanji dictionary or reference book.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "tag", content = "content")]
pub enum Reference {
    /// Modern Reader's Japanese-English Dictionary by Andrew Nelson
    NelsonClassic(u16),
    /// The New Nelson Japanese-English Dictionary by John Haig
    NelsonNew(u16),
    /// New Japanese-English Character Dictionary by Jack Halpern
    Njecd(u16),
    /// Kodansha's Japanese-English Dictionary by Jack Halpern
    Kkd(u16),
    /// Kanji Learners Dictionary by Jack Halpern
    Kkld(u16),
    /// Kanji Learners Dictionary Second Edition by Jack Halpern
    Kkld2ed(u16),
    /// Remembering the Kanji by James Heisig
    Heisig(u16),
    /// Remembering the Kanji Sixth Edition by James Heisig
    Heisig6(u16),
    /// A New Dictionary of Kanji Usage
    Gakken(u16),
    /// Japanese Names by P.G. O'Neill
    OneillNames(Oneill),
    /// Essential Kanji by P.G. O'Neill
    OneillKk(u16),
    /// Daikanwajiten by Morohashi
    Moro(Moro),
    /// A Guide to Remembering Japanese Characters by Kenneth G. Henshall
    Henshall(u16),
    /// Kanji and Kana by Spahn and Hadamitzky
    ShKk(u16),
    /// Kanji and Kana 2011 edition by Spahn and Hadamitzky
    ShKk2(u16),
    /// A Guide to Reading and Writing Japanese by Florence Sakade
    Sakade(u16),
    /// Japanese Kanji Flashcards by Tomoko Okazaki
    Jfcards(u16),
    /// A Guide to Reading and Writing Japanese by Henshall
    Henshall3(u16),
    /// Tuttle Kanji Cards by Alexander Kask
    TuttleCards(u16),
    /// The Kanji Way to Japanese Language Power by Dale Crowley
    Crowley(u16),
    /// Kanji in Context by Nishiguchi and Kono
    KanjiInContext(u16),
    /// Japanese for Busy People
    BusyPeople(BusyPeople),
    /// The Kodansha Compact Study Guide
    KodanshaCompact(u16),
    /// Les Kanjis dans la tete by Yves Maniette
    Maniette(u16),
}
