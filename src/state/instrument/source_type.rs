use serde::{Serialize, Deserialize};

use crate::{CustomSynthDefId, VstPluginId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceType {
    Saw,
    Sin,
    Sqr,
    Tri,
    Noise,
    Pulse,
    SuperSaw,
    Sync,
    Ring,
    FBSin,
    FM,
    PhaseMod,
    Pluck,
    Formant,
    Gendy,
    Chaos,
    Additive,
    Wavetable,
    Granular,
    Bowed,
    Blown,
    Membrane,
    AudioIn,
    BusIn,
    PitchedSampler,
    Kit,
    Custom(CustomSynthDefId),
    Vst(VstPluginId),
}

impl SourceType {
    pub fn name(&self) -> &'static str {
        match self {
            SourceType::Saw => "Saw",
            SourceType::Sin => "Sine",
            SourceType::Sqr => "Square",
            SourceType::Tri => "Triangle",
            SourceType::Noise => "Noise",
            SourceType::Pulse => "Pulse",
            SourceType::SuperSaw => "SuperSaw",
            SourceType::Sync => "Sync",
            SourceType::Ring => "Ring Mod",
            SourceType::FBSin => "FB Sine",
            SourceType::FM => "FM",
            SourceType::PhaseMod => "Phase Mod",
            SourceType::Pluck => "Pluck",
            SourceType::Formant => "Formant",
            SourceType::Gendy => "Gendy",
            SourceType::Chaos => "Chaos",
            SourceType::Additive => "Additive",
            SourceType::Wavetable => "Wavetable",
            SourceType::Granular => "Granular",
            SourceType::Bowed => "Bowed",
            SourceType::Blown => "Blown",
            SourceType::Membrane => "Membrane",
            SourceType::AudioIn => "Audio In",
            SourceType::BusIn => "Bus In",
            SourceType::PitchedSampler => "Pitched Sampler",
            SourceType::Kit => "Kit",
            SourceType::Custom(_) => "Custom",
            SourceType::Vst(_) => "VST",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            SourceType::Saw => "saw",
            SourceType::Sin => "sin",
            SourceType::Sqr => "sqr",
            SourceType::Tri => "tri",
            SourceType::Noise => "noise",
            SourceType::Pulse => "pulse",
            SourceType::SuperSaw => "supersaw",
            SourceType::Sync => "sync",
            SourceType::Ring => "ring",
            SourceType::FBSin => "fbsin",
            SourceType::FM => "fm",
            SourceType::PhaseMod => "phasemod",
            SourceType::Pluck => "pluck",
            SourceType::Formant => "formant",
            SourceType::Gendy => "gendy",
            SourceType::Chaos => "chaos",
            SourceType::Additive => "additive",
            SourceType::Wavetable => "wavetable",
            SourceType::Granular => "granular",
            SourceType::Bowed => "bowed",
            SourceType::Blown => "blown",
            SourceType::Membrane => "membrane",
            SourceType::AudioIn => "audio_in",
            SourceType::BusIn => "bus_in",
            SourceType::PitchedSampler => "sample",
            SourceType::Kit => "kit",
            SourceType::Custom(_) => "custom",
            SourceType::Vst(_) => "vst",
        }
    }

    /// Get the SuperCollider synthdef name (static for built-ins)
    pub fn synth_def_name(&self) -> &'static str {
        match self {
            SourceType::Saw => "imbolc_saw",
            SourceType::Sin => "imbolc_sin",
            SourceType::Sqr => "imbolc_sqr",
            SourceType::Tri => "imbolc_tri",
            SourceType::Noise => "imbolc_noise",
            SourceType::Pulse => "imbolc_pulse",
            SourceType::SuperSaw => "imbolc_supersaw",
            SourceType::Sync => "imbolc_sync",
            SourceType::Ring => "imbolc_ring",
            SourceType::FBSin => "imbolc_fbsin",
            SourceType::FM => "imbolc_fm",
            SourceType::PhaseMod => "imbolc_phasemod",
            SourceType::Pluck => "imbolc_pluck",
            SourceType::Formant => "imbolc_formant",
            SourceType::Gendy => "imbolc_gendy",
            SourceType::Chaos => "imbolc_chaos",
            SourceType::Additive => "imbolc_additive",
            SourceType::Wavetable => "imbolc_wavetable",
            SourceType::Granular => "imbolc_granular",
            SourceType::Bowed => "imbolc_bowed",
            SourceType::Blown => "imbolc_blown",
            SourceType::Membrane => "imbolc_membrane",
            SourceType::AudioIn => "imbolc_audio_in",
            SourceType::BusIn => "imbolc_bus_in",
            SourceType::PitchedSampler => "imbolc_sampler",
            SourceType::Kit => "imbolc_sampler_oneshot",
            SourceType::Custom(_) => "imbolc_saw", // Fallback, use synth_def_name_with_registry instead
            SourceType::Vst(_) => "imbolc_vst_instrument",
        }
    }

    pub fn is_audio_input(&self) -> bool {
        matches!(self, SourceType::AudioIn)
    }

    pub fn is_sample(&self) -> bool {
        matches!(self, SourceType::PitchedSampler)
    }

    pub fn is_kit(&self) -> bool {
        matches!(self, SourceType::Kit)
    }

    pub fn is_bus_in(&self) -> bool {
        matches!(self, SourceType::BusIn)
    }

    #[allow(dead_code)]
    pub fn is_custom(&self) -> bool {
        matches!(self, SourceType::Custom(_))
    }

    #[allow(dead_code)]
    pub fn custom_id(&self) -> Option<CustomSynthDefId> {
        match self {
            SourceType::Custom(id) => Some(*id),
            _ => None,
        }
    }

    pub fn is_vst(&self) -> bool {
        matches!(self, SourceType::Vst(_))
    }

    #[allow(dead_code)]
    pub fn vst_id(&self) -> Option<VstPluginId> {
        match self {
            SourceType::Vst(id) => Some(*id),
            _ => None,
        }
    }

    /// Built-in source types (excluding custom)
    pub fn all() -> Vec<SourceType> {
        vec![
            SourceType::Saw, SourceType::Sin, SourceType::Sqr, SourceType::Tri,
            SourceType::Noise, SourceType::Pulse, SourceType::SuperSaw, SourceType::Sync,
            SourceType::Ring, SourceType::FBSin, SourceType::FM, SourceType::PhaseMod,
            SourceType::Pluck, SourceType::Formant, SourceType::Gendy, SourceType::Chaos,
            SourceType::Additive, SourceType::Wavetable, SourceType::Granular,
            SourceType::Bowed, SourceType::Blown, SourceType::Membrane,
            SourceType::AudioIn, SourceType::BusIn, SourceType::PitchedSampler, SourceType::Kit,
        ]
    }
}

// Note: The following methods stay in imbolc-core because they require registry lookups
// or construct Param values:
// - display_name(&self, registry: &CustomSynthDefRegistry)
// - display_name_vst(&self, custom_registry, vst_registry)
// - short_name_with_registry(&self, registry)
// - short_name_vst(&self, custom_registry, vst_registry)
// - synth_def_name_with_registry(&self, registry)
// - default_params(&self)
// - default_params_with_registry(&self, registry)
// - all_with_custom(registry)
