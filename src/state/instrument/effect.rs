use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::{Param, VstPluginId, EffectId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectType {
    Delay,
    Reverb,
    Gate,
    TapeComp,
    SidechainComp,
    // Modulation
    Chorus,
    Flanger,
    Phaser,
    Tremolo,
    // Distortion
    Distortion,
    Bitcrusher,
    Wavefolder,
    Saturator,
    // EQ
    TiltEq,
    // Stereo
    StereoWidener,
    FreqShifter,
    // Utility
    Limiter,
    PitchShifter,
    // Lo-fi
    Vinyl,
    Cabinet,
    // Granular
    GranularDelay,
    GranularFreeze,
    // Convolution
    ConvolutionReverb,
    // New effects
    Vocoder,
    RingMod,
    Autopan,
    Resonator,
    MultibandComp,
    ParaEq,
    SpectralFreeze,
    Glitch,
    Leslie,
    SpringReverb,
    EnvFollower,
    MidSide,
    Crossfader,
    Denoise,
    Vst(VstPluginId),
}

impl EffectType {
    pub fn name(&self) -> &'static str {
        match self {
            EffectType::Delay => "Delay",
            EffectType::Reverb => "Reverb",
            EffectType::Gate => "Gate",
            EffectType::TapeComp => "Tape Comp",
            EffectType::SidechainComp => "SC Comp",
            EffectType::Chorus => "Chorus",
            EffectType::Flanger => "Flanger",
            EffectType::Phaser => "Phaser",
            EffectType::Tremolo => "Tremolo",
            EffectType::Distortion => "Distortion",
            EffectType::Bitcrusher => "Bitcrusher",
            EffectType::Wavefolder => "Wavefolder",
            EffectType::Saturator => "Saturator",
            EffectType::TiltEq => "Tilt EQ",
            EffectType::StereoWidener => "Stereo Widener",
            EffectType::FreqShifter => "Freq Shifter",
            EffectType::Limiter => "Limiter",
            EffectType::PitchShifter => "Pitch Shifter",
            EffectType::Vinyl => "Vinyl",
            EffectType::Cabinet => "Cabinet",
            EffectType::GranularDelay => "Granular Delay",
            EffectType::GranularFreeze => "Granular Freeze",
            EffectType::ConvolutionReverb => "Conv Reverb",
            EffectType::Vocoder => "Vocoder",
            EffectType::RingMod => "Ring Mod",
            EffectType::Autopan => "Autopan",
            EffectType::Resonator => "Resonator",
            EffectType::MultibandComp => "MB Comp",
            EffectType::ParaEq => "Para EQ",
            EffectType::SpectralFreeze => "Spectral Freeze",
            EffectType::Glitch => "Glitch",
            EffectType::Leslie => "Leslie",
            EffectType::SpringReverb => "Spring Reverb",
            EffectType::EnvFollower => "Env Follower",
            EffectType::MidSide => "Mid/Side",
            EffectType::Crossfader => "Crossfader",
            EffectType::Denoise => "Denoise",
            EffectType::Vst(_) => "VST",
        }
    }

    pub fn synth_def_name(&self) -> &'static str {
        match self {
            EffectType::Delay => "imbolc_delay",
            EffectType::Reverb => "imbolc_reverb",
            EffectType::Gate => "imbolc_gate",
            EffectType::TapeComp => "imbolc_tape_comp",
            EffectType::SidechainComp => "imbolc_sc_comp",
            EffectType::Chorus => "imbolc_chorus",
            EffectType::Flanger => "imbolc_flanger",
            EffectType::Phaser => "imbolc_phaser",
            EffectType::Tremolo => "imbolc_tremolo",
            EffectType::Distortion => "imbolc_distortion",
            EffectType::Bitcrusher => "imbolc_bitcrusher",
            EffectType::Wavefolder => "imbolc_wavefolder",
            EffectType::Saturator => "imbolc_saturator",
            EffectType::TiltEq => "imbolc_tilt_eq",
            EffectType::StereoWidener => "imbolc_stereo_widener",
            EffectType::FreqShifter => "imbolc_freq_shifter",
            EffectType::Limiter => "imbolc_limiter",
            EffectType::PitchShifter => "imbolc_pitch_shifter",
            EffectType::Vinyl => "imbolc_vinyl",
            EffectType::Cabinet => "imbolc_cabinet",
            EffectType::GranularDelay => "imbolc_granular_delay",
            EffectType::GranularFreeze => "imbolc_granular_freeze",
            EffectType::ConvolutionReverb => "imbolc_convolution_reverb",
            EffectType::Vocoder => "imbolc_vocoder",
            EffectType::RingMod => "imbolc_ringmod",
            EffectType::Autopan => "imbolc_autopan",
            EffectType::Resonator => "imbolc_resonator",
            EffectType::MultibandComp => "imbolc_multiband_comp",
            EffectType::ParaEq => "imbolc_para_eq",
            EffectType::SpectralFreeze => "imbolc_spectral_freeze",
            EffectType::Glitch => "imbolc_glitch",
            EffectType::Leslie => "imbolc_leslie",
            EffectType::SpringReverb => "imbolc_spring_reverb",
            EffectType::EnvFollower => "imbolc_env_follower",
            EffectType::MidSide => "imbolc_midside",
            EffectType::Crossfader => "imbolc_crossfader",
            EffectType::Denoise => "imbolc_denoise",
            EffectType::Vst(_) => "imbolc_vst_effect",
        }
    }

    pub fn is_vst(&self) -> bool {
        matches!(self, EffectType::Vst(_))
    }

    #[allow(dead_code)]
    pub fn vst_id(&self) -> Option<VstPluginId> {
        match self {
            EffectType::Vst(id) => Some(*id),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn all() -> Vec<EffectType> {
        vec![
            EffectType::Delay, EffectType::Reverb, EffectType::Gate,
            EffectType::TapeComp, EffectType::SidechainComp,
            EffectType::Chorus, EffectType::Flanger, EffectType::Phaser, EffectType::Tremolo,
            EffectType::Distortion, EffectType::Bitcrusher, EffectType::Wavefolder, EffectType::Saturator,
            EffectType::TiltEq,
            EffectType::StereoWidener, EffectType::FreqShifter,
            EffectType::Limiter, EffectType::PitchShifter,
            EffectType::Vinyl, EffectType::Cabinet,
            EffectType::GranularDelay, EffectType::GranularFreeze,
            EffectType::ConvolutionReverb,
            EffectType::Vocoder, EffectType::RingMod, EffectType::Autopan,
            EffectType::Resonator, EffectType::MultibandComp, EffectType::ParaEq,
            EffectType::SpectralFreeze, EffectType::Glitch, EffectType::Leslie,
            EffectType::SpringReverb, EffectType::EnvFollower, EffectType::MidSide,
            EffectType::Crossfader, EffectType::Denoise,
        ]
    }
}

// Note: The following methods stay in imbolc-core:
// - display_name(&self, vst_registry) - requires VstPluginRegistry
// - default_params(&self) - constructs Param values

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectSlot {
    pub id: EffectId,
    pub effect_type: EffectType,
    pub params: Vec<Param>,
    pub enabled: bool,
    pub vst_param_values: Vec<(u32, f32)>,
    pub vst_state_path: Option<PathBuf>,
}

// Note: EffectSlot::new() stays in imbolc-core because it calls EffectType::default_params()
