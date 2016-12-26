pub mod types;
pub mod consts;

pub enum ALCdevice_struct {}

pub enum ALCcontext_struct {}

pub mod ffi {
    use super::types::*;

    extern "C" {
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
}