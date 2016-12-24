//! OpenAL Effects

use ::types::*;

pub mod types;
#[cfg(feature = "presets")]
pub mod presets;
pub mod consts;

pub use self::types::*;

pub const ALC_EXT_EFX_NAME: &'static str = "ALC_EXT_EFX";

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
