//! OpenAL Effects

pub mod types;
pub mod consts;

#[cfg(feature = "presets")]
pub mod presets;

pub const ALC_EXT_EFX_NAME: &'static str = "ALC_EXT_EFX";

pub const AL_EFX_FILTERS: &'static [&'static str] = &[
    "AL_FILTER_LOWPASS", "AL_FILTER_HIGHPASS", "AL_FILTER_BANDPASS"
];

pub const AL_EFX_FILTER_NAMES: &'static [&'static str] = &[
    "Low-pass", "High-pass", "Band-pass"
];

pub const AL_EFX_EFFECTS: &'static [&'static str] = &[
    "AL_EFFECT_EAXREVERB", "AL_EFFECT_REVERB", "AL_EFFECT_CHORUS",
    "AL_EFFECT_DISTORTION", "AL_EFFECT_ECHO", "AL_EFFECT_FLANGER",
    "AL_EFFECT_FREQUENCY_SHIFTER", "AL_EFFECT_VOCAL_MORPHER",
    "AL_EFFECT_PITCH_SHIFTER", "AL_EFFECT_RING_MODULATOR",
    "AL_EFFECT_AUTOWAH", "AL_EFFECT_COMPRESSOR", "AL_EFFECT_EQUALIZER"
];

pub const AL_EFX_EFFECT_NAMES: &'static [&'static str] = &[
    "EAX Reverb", "Reverb", "Chorus", "Distortion", "Echo", "Flanger",
    "Frequency Shifter", "Vocal Morpher", "Pitch Shifter",
    "Ring Modulator", "Autowah", "Compressor", "Equalizer"
];

pub const AL_EFX_DEDICATED_EFFECTS: &'static [&'static str] = &["AL_EFFECT_DEDICATED_DIALOGUE", "AL_EFFECT_DEDICATED_LOW_FREQUENCY_EFFECT", "AL_DEDICATED_GAIN"];

pub const AL_EFX_DEDICATED_EFFECT_NAMES: &'static [&'static str] = &["Dedicated Dialogue", "Dedicated LFE", "Dedicated Gain"];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EFXEAXREVERBPROPERTIES {
    pub flDensity: f32,
    pub flDiffusion: f32,
    pub flGain: f32,
    pub flGainHF: f32,
    pub flGainLF: f32,
    pub flDecayTime: f32,
    pub flDecayHFRatio: f32,
    pub flDecayLFRatio: f32,
    pub flReflectionsGain: f32,
    pub flReflectionsDelay: f32,
    pub flReflectionsPan: [f32; 3usize],
    pub flLateReverbGain: f32,
    pub flLateReverbDelay: f32,
    pub flLateReverbPan: [f32; 3usize],
    pub flEchoTime: f32,
    pub flEchoDepth: f32,
    pub flModulationTime: f32,
    pub flModulationDepth: f32,
    pub flAirAbsorptionGainHF: f32,
    pub flHFReference: f32,
    pub flLFReference: f32,
    pub flRoomRolloffFactor: f32,
    pub iDecayHFLimit: ::std::os::raw::c_int,
}

impl ::std::default::Default for EFXEAXREVERBPROPERTIES {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub mod ffi {
    use ::al::types::*;

    extern "C" {
        pub fn alGenEffects(n: ALsizei, effects: *mut ALuint) -> ALvoid;
        pub fn alDeleteEffects(n: ALsizei, effects: *const ALuint) -> ALvoid;
        pub fn alIsEffect(effect: ALuint) -> ALboolean;
        pub fn alEffecti(effect: ALuint, param: ALenum, iValue: ALint) -> ALvoid;
        pub fn alEffectiv(effect: ALuint, param: ALenum, piValues: *const ALint) -> ALvoid;
        pub fn alEffectf(effect: ALuint, param: ALenum, flValue: ALfloat) -> ALvoid;
        pub fn alEffectfv(effect: ALuint, param: ALenum, pflValues: *const ALfloat) -> ALvoid;
        pub fn alGetEffecti(effect: ALuint, param: ALenum, piValue: *mut ALint) -> ALvoid;
        pub fn alGetEffectiv(effect: ALuint, param: ALenum, piValues: *mut ALint) -> ALvoid;
        pub fn alGetEffectf(effect: ALuint, param: ALenum, pflValue: *mut ALfloat) -> ALvoid;
        pub fn alGetEffectfv(effect: ALuint, param: ALenum, pflValues: *mut ALfloat) -> ALvoid;
        pub fn alGenFilters(n: ALsizei, filters: *mut ALuint) -> ALvoid;
        pub fn alDeleteFilters(n: ALsizei, filters: *const ALuint) -> ALvoid;
        pub fn alIsFilter(filter: ALuint) -> ALboolean;
        pub fn alFilteri(filter: ALuint, param: ALenum, iValue: ALint) -> ALvoid;
        pub fn alFilteriv(filter: ALuint, param: ALenum, piValues: *const ALint) -> ALvoid;pub fn alFilterf(filter: ALuint, param: ALenum, flValue: ALfloat) -> ALvoid;
        pub fn alFilterfv(filter: ALuint, param: ALenum, pflValues: *const ALfloat) -> ALvoid;
        pub fn alGetFilteri(filter: ALuint, param: ALenum, piValue: *mut ALint) -> ALvoid;
        pub fn alGetFilteriv(filter: ALuint, param: ALenum, piValues: *mut ALint) -> ALvoid;
        pub fn alGetFilterf(filter: ALuint, param: ALenum, pflValue: *mut ALfloat) -> ALvoid;
        pub fn alGetFilterfv(filter: ALuint, param: ALenum, pflValues: *mut ALfloat) -> ALvoid;
        pub fn alGenAuxiliaryEffectSlots(n: ALsizei, effectslots: *mut ALuint) -> ALvoid;
        pub fn alDeleteAuxiliaryEffectSlots(n: ALsizei, effectslots: *const ALuint) -> ALvoid;
        pub fn alIsAuxiliaryEffectSlot(effectslot: ALuint) -> ALboolean;
        pub fn alAuxiliaryEffectSloti(effectslot: ALuint, param: ALenum, iValue: ALint) -> ALvoid;
        pub fn alAuxiliaryEffectSlotiv(effectslot: ALuint, param: ALenum, piValues: *const ALint) -> ALvoid;
        pub fn alAuxiliaryEffectSlotf(effectslot: ALuint, param: ALenum, flValue: ALfloat) -> ALvoid;
        pub fn alAuxiliaryEffectSlotfv(effectslot: ALuint, param: ALenum, pflValues: *const ALfloat) -> ALvoid;
        pub fn alGetAuxiliaryEffectSloti(effectslot: ALuint, param: ALenum, piValue: *mut ALint) -> ALvoid;
        pub fn alGetAuxiliaryEffectSlotiv(effectslot: ALuint, param: ALenum, piValues: *mut ALint) -> ALvoid;
        pub fn alGetAuxiliaryEffectSlotf(effectslot: ALuint, param: ALenum, pflValue: *mut ALfloat) -> ALvoid;
        pub fn alGetAuxiliaryEffectSlotfv(effectslot: ALuint, param: ALenum, pflValues: *mut ALfloat) -> ALvoid;
    }
}