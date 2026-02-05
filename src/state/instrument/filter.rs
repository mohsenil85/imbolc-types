use serde::{Serialize, Deserialize};

use super::ModulatedParam;
use crate::Param;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterType {
    Lpf,
    Hpf,
    Bpf,
    Notch,
    Comb,
    Allpass,
    Vowel,
    ResDrive,
}

impl FilterType {
    pub fn name(&self) -> &'static str {
        match self {
            FilterType::Lpf => "Low-Pass",
            FilterType::Hpf => "High-Pass",
            FilterType::Bpf => "Band-Pass",
            FilterType::Notch => "Notch",
            FilterType::Comb => "Comb",
            FilterType::Allpass => "Allpass",
            FilterType::Vowel => "Vowel",
            FilterType::ResDrive => "ResDrive",
        }
    }

    pub fn synth_def_name(&self) -> &'static str {
        match self {
            FilterType::Lpf => "imbolc_lpf",
            FilterType::Hpf => "imbolc_hpf",
            FilterType::Bpf => "imbolc_bpf",
            FilterType::Notch => "imbolc_notch",
            FilterType::Comb => "imbolc_comb",
            FilterType::Allpass => "imbolc_allpass",
            FilterType::Vowel => "imbolc_vowel",
            FilterType::ResDrive => "imbolc_resdrive",
        }
    }

    #[allow(dead_code)]
    pub fn all() -> Vec<FilterType> {
        vec![
            FilterType::Lpf, FilterType::Hpf, FilterType::Bpf,
            FilterType::Notch, FilterType::Comb, FilterType::Allpass,
            FilterType::Vowel, FilterType::ResDrive,
        ]
    }
}

// Note: default_extra_params() stays in imbolc-core because it constructs Param values
// with specific defaults that are part of business logic.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterConfig {
    pub filter_type: FilterType,
    pub cutoff: ModulatedParam,
    pub resonance: ModulatedParam,
    pub extra_params: Vec<Param>,
}

// Note: FilterConfig::new() stays in imbolc-core because it calls default_extra_params()

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EqBandType {
    LowShelf,
    Peaking,
    HighShelf,
}

impl EqBandType {
    pub fn name(&self) -> &'static str {
        match self {
            EqBandType::LowShelf => "LS",
            EqBandType::Peaking => "PK",
            EqBandType::HighShelf => "HS",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqBand {
    pub band_type: EqBandType,
    pub freq: f32,
    pub gain: f32,
    pub q: f32,
    pub enabled: bool,
}

pub const EQ_BAND_COUNT: usize = 12;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqConfig {
    pub bands: [EqBand; EQ_BAND_COUNT],
    pub enabled: bool,
}

impl Default for EqConfig {
    fn default() -> Self {
        Self {
            bands: [
                EqBand { band_type: EqBandType::LowShelf, freq: 40.0,    gain: 0.0, q: 0.7, enabled: true },
                EqBand { band_type: EqBandType::Peaking,  freq: 80.0,    gain: 0.0, q: 1.0, enabled: true },
                EqBand { band_type: EqBandType::Peaking,  freq: 160.0,   gain: 0.0, q: 1.0, enabled: true },
                EqBand { band_type: EqBandType::Peaking,  freq: 320.0,   gain: 0.0, q: 1.0, enabled: true },
                EqBand { band_type: EqBandType::Peaking,  freq: 640.0,   gain: 0.0, q: 1.0, enabled: true },
                EqBand { band_type: EqBandType::Peaking,  freq: 1200.0,  gain: 0.0, q: 1.0, enabled: true },
                EqBand { band_type: EqBandType::Peaking,  freq: 2500.0,  gain: 0.0, q: 1.0, enabled: true },
                EqBand { band_type: EqBandType::Peaking,  freq: 5000.0,  gain: 0.0, q: 1.0, enabled: true },
                EqBand { band_type: EqBandType::Peaking,  freq: 8000.0,  gain: 0.0, q: 1.0, enabled: true },
                EqBand { band_type: EqBandType::Peaking,  freq: 12000.0, gain: 0.0, q: 1.0, enabled: true },
                EqBand { band_type: EqBandType::Peaking,  freq: 16000.0, gain: 0.0, q: 1.0, enabled: true },
                EqBand { band_type: EqBandType::HighShelf, freq: 18000.0, gain: 0.0, q: 0.7, enabled: true },
            ],
            enabled: true,
        }
    }
}
