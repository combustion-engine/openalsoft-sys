pub mod types;
pub mod consts;

pub mod ffi {
    use super::types::*;

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
    }
}