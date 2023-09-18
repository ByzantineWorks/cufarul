use std::fmt::Display;

use super::{Error, Result};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum ByzantineGenus {
    Diatonic,
    Enharmonic,
    SoftChromatic,
    HardChromatic,
}

#[derive(Clone, Debug, Deserialize)]
#[serde[try_from = "u8"]]
pub enum ByzantineMode {
    A,
    B,
    C,
    D,
    PlagalA,
    PLagalB,
    PlagalC,
    PlagalD,
}

#[derive(Clone, Debug, Deserialize)]
pub enum ModalSystem {
    Diapason,
    Tethrachord,
    Pentachord,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Phthong {
    NH,
    PA,
    VOU,
    GA,
    DI,
    KE,
    ZW,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModeProperty {
    // pub genus: ByzantineGenus,
    // pub system: ModalSystem,
    pub mode: ByzantineMode,
    pub base: Phthong,
    pub modulations: Vec<ByzantineMode>,
}

impl TryFrom<u8> for ByzantineMode {
    type Error = Error;
    fn try_from(value: u8) -> Result<Self> {
        match value {
            1 => Ok(ByzantineMode::A),
            2 => Ok(ByzantineMode::B),
            3 => Ok(ByzantineMode::C),
            4 => Ok(ByzantineMode::D),
            5 => Ok(ByzantineMode::PlagalA),
            6 => Ok(ByzantineMode::PLagalB),
            7 => Ok(ByzantineMode::PlagalC),
            8 => Ok(ByzantineMode::PlagalD),
            other => Err(Error::InvalidMode(other)),
        }
    }
}

impl From<ByzantineMode> for u8 {
    fn from(value: ByzantineMode) -> Self {
        match value {
            ByzantineMode::A => 1,
            ByzantineMode::B => 2,
            ByzantineMode::C => 3,
            ByzantineMode::D => 4,
            ByzantineMode::PlagalA => 5,
            ByzantineMode::PLagalB => 6,
            ByzantineMode::PlagalC => 7,
            ByzantineMode::PlagalD => 8,
        }
    }
}

impl Display for ByzantineMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::A => "First Mode",
            Self::B => "Second Mode",
            Self::C => "Third Mode",
            Self::D => "Fourth Mode",
            Self::PlagalA => "Plagal First Mode",
            Self::PLagalB => "Plagal Second Mode",
            Self::PlagalC => "Plagal Third Mode",
            Self::PlagalD => "Plagal Fourth Mode",
        })
    }
}

impl Display for Phthong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::NH => "Nh",
            Self::PA => "Pa",
            Self::VOU => "Vou",
            Self::GA => "Ga",
            Self::DI => "Di",
            Self::KE => "Ke",
            Self::ZW => "Zw",
        })
    }
}
