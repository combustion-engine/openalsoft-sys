//! OpenALSoft Extensions

use types::*;

pub mod types;
pub mod consts;

pub const AL_EXT_FOLDBACK_NAME: &'static str = "AL_EXT_FOLDBACK";

pub use self::types::*;

extern "C" {
    pub fn alBufferDataStatic(buffer: ALint, format: ALenum, data: *mut ALvoid, len: ALsizei, freq: ALsizei) -> ALvoid;
    pub fn alcSetThreadContext(context: *mut ALCcontext) -> ALCboolean;
    pub fn alcGetThreadContext() -> *mut ALCcontext;
    pub fn alBufferSubDataSOFT(buffer: ALuint, format: ALenum, data: *const ALvoid, offset: ALsizei, length: ALsizei) -> ALvoid;
    pub fn alRequestFoldbackStart(mode: ALenum, count: ALsizei, length: ALsizei, mem: *mut ALfloat, callback: LPALFOLDBACKCALLBACK);
    pub fn alRequestFoldbackStop();
    pub fn alBufferSamplesSOFT(buffer: ALuint, samplerate: ALuint, internalformat: ALenum, samples: ALsizei, channels: ALenum, type_: ALenum, data: *const ALvoid);
    pub fn alBufferSubSamplesSOFT(buffer: ALuint, offset: ALsizei, samples: ALsizei, channels: ALenum, type_: ALenum, data: *const ALvoid);
    pub fn alGetBufferSamplesSOFT(buffer: ALuint, offset: ALsizei, samples: ALsizei, channels: ALenum, type_: ALenum, data: *mut ALvoid);
    pub fn alIsBufferFormatSupportedSOFT(format: ALenum) -> ALboolean;
    pub fn alcLoopbackOpenDeviceSOFT(deviceName: *const ALCchar) -> *mut ALCdevice;
    pub fn alcIsRenderFormatSupportedSOFT(device: *mut ALCdevice, freq: ALCsizei, channels: ALCenum, type_: ALCenum) -> ALCboolean;
    pub fn alcRenderSamplesSOFT(device: *mut ALCdevice, buffer: *mut ALCvoid, samples: ALCsizei);
    pub fn alSourcedSOFT(source: ALuint, param: ALenum, value: ALdouble);
    pub fn alSource3dSOFT(source: ALuint, param: ALenum, value1: ALdouble, value2: ALdouble, value3: ALdouble);
    pub fn alSourcedvSOFT(source: ALuint, param: ALenum, values: *const ALdouble);
    pub fn alGetSourcedSOFT(source: ALuint, param: ALenum, value: *mut ALdouble);
    pub fn alGetSource3dSOFT(source: ALuint, param: ALenum, value1: *mut ALdouble, value2: *mut ALdouble, value3: *mut ALdouble);
    pub fn alGetSourcedvSOFT(source: ALuint, param: ALenum, values: *mut ALdouble);
    pub fn alSourcei64SOFT(source: ALuint, param: ALenum, value: ALint64SOFT);
    pub fn alSource3i64SOFT(source: ALuint, param: ALenum, value1: ALint64SOFT, value2: ALint64SOFT, value3: ALint64SOFT);
    pub fn alSourcei64vSOFT(source: ALuint, param: ALenum, values: *const ALint64SOFT);
    pub fn alGetSourcei64SOFT(source: ALuint, param: ALenum, value: *mut ALint64SOFT);
    pub fn alGetSource3i64SOFT(source: ALuint, param: ALenum, value1: *mut ALint64SOFT, value2: *mut ALint64SOFT, value3: *mut ALint64SOFT);
    pub fn alGetSourcei64vSOFT(source: ALuint, param: ALenum, values: *mut ALint64SOFT);
    pub fn alDeferUpdatesSOFT() -> ALvoid;
    pub fn alProcessUpdatesSOFT() -> ALvoid;
    pub fn alcDevicePauseSOFT(device: *mut ALCdevice);
    pub fn alcDeviceResumeSOFT(device: *mut ALCdevice);
    pub fn alcGetStringiSOFT(device: *mut ALCdevice, paramName: ALCenum, index: ALCsizei) -> *const ALCchar;
    pub fn alcResetDeviceSOFT(device: *mut ALCdevice, attribs: *const ALCint) -> ALCboolean;
}