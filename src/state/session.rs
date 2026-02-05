use serde::{Serialize, Deserialize};

use super::music::{Key, Scale};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MixerSelection {
    Instrument(usize), // index into instruments vec
    Bus(u8),      // 1-8
    Master,
}

impl Default for MixerSelection {
    fn default() -> Self {
        Self::Instrument(0)
    }
}

/// The subset of session fields that are cheap to clone for editing (BPM, key, scale, etc.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MusicalSettings {
    pub key: Key,
    pub scale: Scale,
    pub bpm: u16,
    pub tuning_a4: f32,
    pub snap: bool,
    pub time_signature: (u8, u8),
}

impl Default for MusicalSettings {
    fn default() -> Self {
        Self {
            key: Key::C,
            scale: Scale::Major,
            bpm: 120,
            tuning_a4: 440.0,
            snap: false,
            time_signature: (4, 4),
        }
    }
}

// Note: SessionState struct stays in imbolc-core for now because it has many dependencies
// (PianoRollState, ArrangementState, AutomationState, etc.) and complex constructor logic.
// It can be moved later once all its dependencies are in imbolc-types.
