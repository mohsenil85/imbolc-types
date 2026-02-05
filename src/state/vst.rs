//! VST plugin types.

use serde::{Deserialize, Serialize};

/// Whether a VST plugin is an instrument or effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VstPluginKind {
    Instrument,
    Effect,
}
