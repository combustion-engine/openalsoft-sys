#![allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]

extern crate libc;

pub mod types;
pub mod consts;

#[cfg(feature = "presets")]
pub mod ambisonic_presets;

pub mod efx;
pub mod ext;

#[cfg(feature = "hrtf")]
pub mod hrtf;

pub use types::*;

pub mod all {
    pub use super::*;
    pub use super::efx::*;
    pub use super::ext::*;

    pub use super::consts::*;
    pub use super::efx::efx_consts::*;
    pub use super::ext::ext_consts::*;
}

#[cfg_attr(all(feature = "static", target_os = "linux"), link(name = "openal", kind = "static"))]
#[cfg_attr(all(not(feature = "static"), target_os = "linux"), link(name = "openal"))]
#[cfg_attr(all(feature = "static", target_os = "macos"), link(name = "OpenAL", kind = "static"))]
#[cfg_attr(all(not(feature = "static"), target_os = "macos"), link(name = "OpenAL", kind = "framework"))]
#[cfg_attr(all(feature = "static", target_os = "windows"), link(name = "OpenAL32", kind = "static"))]
#[cfg_attr(all(not(feature = "static"), target_os = "windows"), link(name = "OpenAL32"))]
extern {}

pub enum ALCdevice_struct {}

pub enum ALCcontext_struct {}

extern "C" {
    pub fn alDopplerFactor(value: ALfloat);
    pub fn alDopplerVelocity(value: ALfloat);
    pub fn alSpeedOfSound(value: ALfloat);
    pub fn alDistanceModel(distanceModel: ALenum);
    pub fn alEnable(capability: ALenum);
    pub fn alDisable(capability: ALenum);
    pub fn alIsEnabled(capability: ALenum) -> ALboolean;
    pub fn alGetString(param: ALenum) -> *const ALchar;
    pub fn alGetBooleanv(param: ALenum, values: *mut ALboolean);
    pub fn alGetIntegerv(param: ALenum, values: *mut ALint);
    pub fn alGetFloatv(param: ALenum, values: *mut ALfloat);
    pub fn alGetDoublev(param: ALenum, values: *mut ALdouble);
    pub fn alGetBoolean(param: ALenum) -> ALboolean;
    pub fn alGetInteger(param: ALenum) -> ALint;
    pub fn alGetFloat(param: ALenum) -> ALfloat;
    pub fn alGetDouble(param: ALenum) -> ALdouble;
    pub fn alGetError() -> ALenum;
    pub fn alIsExtensionPresent(extname: *const ALchar) -> ALboolean;
    pub fn alGetProcAddress(fname: *const ALchar) -> *mut ::std::os::raw::c_void;
    pub fn alGetEnumValue(ename: *const ALchar) -> ALenum;
    pub fn alListenerf(param: ALenum, value: ALfloat);
    pub fn alListener3f(param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
    pub fn alListenerfv(param: ALenum, values: *const ALfloat);
    pub fn alListeneri(param: ALenum, value: ALint);
    pub fn alListener3i(param: ALenum, value1: ALint, value2: ALint, value3: ALint);
    pub fn alListeneriv(param: ALenum, values: *const ALint);
    pub fn alGetListenerf(param: ALenum, value: *mut ALfloat);
    pub fn alGetListener3f(param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat);
    pub fn alGetListenerfv(param: ALenum, values: *mut ALfloat);
    pub fn alGetListeneri(param: ALenum, value: *mut ALint);
    pub fn alGetListener3i(param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint);
    pub fn alGetListeneriv(param: ALenum, values: *mut ALint);
    pub fn alGenSources(n: ALsizei, sources: *mut ALuint);
    pub fn alDeleteSources(n: ALsizei, sources: *const ALuint);
    pub fn alIsSource(source: ALuint) -> ALboolean;
    pub fn alSourcef(source: ALuint, param: ALenum, value: ALfloat);
    pub fn alSource3f(source: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
    pub fn alSourcefv(source: ALuint, param: ALenum, values: *const ALfloat);
    pub fn alSourcei(source: ALuint, param: ALenum, value: ALint);
    pub fn alSource3i(source: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
    pub fn alSourceiv(source: ALuint, param: ALenum, values: *const ALint);
    pub fn alGetSourcef(source: ALuint, param: ALenum, value: *mut ALfloat);
    pub fn alGetSource3f(source: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat);
    pub fn alGetSourcefv(source: ALuint, param: ALenum, values: *mut ALfloat);
    pub fn alGetSourcei(source: ALuint, param: ALenum, value: *mut ALint);
    pub fn alGetSource3i(source: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint);
    pub fn alGetSourceiv(source: ALuint, param: ALenum, values: *mut ALint);
    pub fn alSourcePlayv(n: ALsizei, sources: *const ALuint);
    pub fn alSourceStopv(n: ALsizei, sources: *const ALuint);
    pub fn alSourceRewindv(n: ALsizei, sources: *const ALuint);
    pub fn alSourcePausev(n: ALsizei, sources: *const ALuint);
    pub fn alSourcePlay(source: ALuint);
    pub fn alSourceStop(source: ALuint);
    pub fn alSourceRewind(source: ALuint);
    pub fn alSourcePause(source: ALuint);
    pub fn alSourceQueueBuffers(source: ALuint, nb: ALsizei, buffers: *const ALuint);
    pub fn alSourceUnqueueBuffers(source: ALuint, nb: ALsizei, buffers: *mut ALuint);
    pub fn alGenBuffers(n: ALsizei, buffers: *mut ALuint);
    pub fn alDeleteBuffers(n: ALsizei, buffers: *const ALuint);
    pub fn alIsBuffer(buffer: ALuint) -> ALboolean;
    pub fn alBufferData(buffer: ALuint, format: ALenum, data: *const ALvoid, size: ALsizei, freq: ALsizei);
    pub fn alBufferf(buffer: ALuint, param: ALenum, value: ALfloat);
    pub fn alBuffer3f(buffer: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat);
    pub fn alBufferfv(buffer: ALuint, param: ALenum, values: *const ALfloat);
    pub fn alBufferi(buffer: ALuint, param: ALenum, value: ALint);
    pub fn alBuffer3i(buffer: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint);
    pub fn alBufferiv(buffer: ALuint, param: ALenum, values: *const ALint);
    pub fn alGetBufferf(buffer: ALuint, param: ALenum, value: *mut ALfloat);
    pub fn alGetBuffer3f(buffer: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat);
    pub fn alGetBufferfv(buffer: ALuint, param: ALenum, values: *mut ALfloat);
    pub fn alGetBufferi(buffer: ALuint, param: ALenum, value: *mut ALint);
    pub fn alGetBuffer3i(buffer: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint);
    pub fn alGetBufferiv(buffer: ALuint, param: ALenum, values: *mut ALint);
    pub fn alcCreateContext(device: *mut ALCdevice, attrlist: *const ALCint) -> *mut ALCcontext;
    pub fn alcMakeContextCurrent(context: *mut ALCcontext) -> ALCboolean;
    pub fn alcProcessContext(context: *mut ALCcontext);
    pub fn alcSuspendContext(context: *mut ALCcontext);
    pub fn alcDestroyContext(context: *mut ALCcontext);
    pub fn alcGetCurrentContext() -> *mut ALCcontext;
    pub fn alcGetContextsDevice(context: *mut ALCcontext) -> *mut ALCdevice;
    pub fn alcOpenDevice(devicename: *const ALCchar) -> *mut ALCdevice;
    pub fn alcCloseDevice(device: *mut ALCdevice) -> ALCboolean;
    pub fn alcGetError(device: *mut ALCdevice) -> ALCenum;
    pub fn alcIsExtensionPresent(device: *mut ALCdevice, extname: *const ALCchar) -> ALCboolean;
    pub fn alcGetProcAddress(device: *mut ALCdevice, funcname: *const ALCchar) -> *mut ::std::os::raw::c_void;
    pub fn alcGetEnumValue(device: *mut ALCdevice, enumname: *const ALCchar) -> ALCenum;
    pub fn alcGetString(device: *mut ALCdevice, param: ALCenum) -> *const ALCchar;
    pub fn alcGetIntegerv(device: *mut ALCdevice, param: ALCenum, size: ALCsizei, values: *mut ALCint);
    pub fn alcCaptureOpenDevice(devicename: *const ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *mut ALCdevice;
    pub fn alcCaptureCloseDevice(device: *mut ALCdevice) -> ALCboolean;
    pub fn alcCaptureStart(device: *mut ALCdevice);
    pub fn alcCaptureStop(device: *mut ALCdevice);
    pub fn alcCaptureSamples(device: *mut ALCdevice, buffer: *mut ALCvoid, samples: ALCsizei);
}
