use ::al::types::*;
use ::alc::types::*;

use libc::{int64_t, uint64_t};

pub type ALint64SOFT = int64_t;
pub type ALuint64SOFT = uint64_t;

pub type PFNALBUFFERDATASTATICPROC = ::std::option::Option<unsafe extern "C" fn(arg1: ALint, arg2: ALenum, arg3: *mut ALvoid, arg4: ALsizei, arg5: ALsizei) -> ALvoid>;
pub type PFNALCGETTHREADCONTEXTPROC = ::std::option::Option<extern "C" fn() -> *mut ALCcontext>;
pub type PFNALBUFFERSUBDATASOFTPROC = ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum, arg3: *const ALvoid, arg4: ALsizei, arg5: ALsizei) -> ALvoid>;
pub type LPALFOLDBACKCALLBACK = ::std::option::Option<extern "C" fn(arg1: ALenum, arg2: ALsizei)>;
pub type LPALREQUESTFOLDBACKSTART = ::std::option::Option<unsafe extern "C" fn(arg1: ALenum, arg2: ALsizei, arg3: ALsizei, arg4: *mut ALfloat, arg5: LPALFOLDBACKCALLBACK)>;
pub type LPALREQUESTFOLDBACKSTOP = ::std::option::Option<extern "C" fn()>;
pub type LPALBUFFERSAMPLESSOFT = ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALuint, arg3: ALenum, arg4: ALsizei, arg5: ALenum, arg6: ALenum, arg7: *const ALvoid)>;
pub type LPALBUFFERSUBSAMPLESSOFT = ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALsizei, arg3: ALsizei, arg4: ALenum, arg5: ALenum, arg6: *const ALvoid)>;
pub type LPALGETBUFFERSAMPLESSOFT = ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALsizei, arg3: ALsizei, arg4: ALenum, arg5: ALenum, arg6: *mut ALvoid)>;
pub type LPALISBUFFERFORMATSUPPORTEDSOFT = ::std::option::Option<extern "C" fn(arg1: ALenum) -> ALboolean>;
pub type LPALCLOOPBACKOPENDEVICESOFT = ::std::option::Option<unsafe extern "C" fn(arg1: *const ALCchar) -> *mut ALCdevice>;
pub type LPALCISRENDERFORMATSUPPORTEDSOFT = ::std::option::Option<unsafe extern "C" fn(arg1: *mut ALCdevice, arg2: ALCsizei, arg3: ALCenum, arg4: ALCenum) -> ALCboolean>;
pub type LPALCRENDERSAMPLESSOFT = ::std::option::Option<unsafe extern "C" fn(arg1: *mut ALCdevice, arg2: *mut ALCvoid, arg3: ALCsizei)>;

pub type LPALSOURCEDSOFT = ::std::option::Option<extern "C" fn(arg1: ALuint, arg2: ALenum, arg3: ALdouble)>;
pub type LPALSOURCE3DSOFT = ::std::option::Option<extern "C" fn(arg1: ALuint, arg2: ALenum, arg3: ALdouble, arg4: ALdouble, arg5: ALdouble)>;
pub type LPALSOURCEDVSOFT = ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum, arg3: *const ALdouble)>;
pub type LPALGETSOURCEDSOFT = ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum, arg3: *mut ALdouble)>;
pub type LPALGETSOURCE3DSOFT = ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum, arg3: *mut ALdouble, arg4: *mut ALdouble, arg5: *mut ALdouble)>;
pub type LPALGETSOURCEDVSOFT = ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum, arg3: *mut ALdouble)>;
pub type LPALSOURCEI64SOFT = ::std::option::Option<extern "C" fn(arg1: ALuint, arg2: ALenum, arg3: ALint64SOFT)>;
pub type LPALSOURCE3I64SOFT = ::std::option::Option<extern "C" fn(arg1: ALuint, arg2: ALenum, arg3: ALint64SOFT, arg4: ALint64SOFT, arg5: ALint64SOFT)>;
pub type LPALSOURCEI64VSOFT = ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum, arg3: *const ALint64SOFT)>;
pub type LPALGETSOURCEI64SOFT = ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum, arg3: *mut ALint64SOFT)>;
pub type LPALGETSOURCE3I64SOFT = ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum, arg3: *mut ALint64SOFT, arg4: *mut ALint64SOFT, arg5: *mut ALint64SOFT)>;
pub type LPALGETSOURCEI64VSOFT = ::std::option::Option<unsafe extern "C" fn(arg1: ALuint, arg2: ALenum, arg3: *mut ALint64SOFT)>;
pub type LPALDEFERUPDATESSOFT = ::std::option::Option<extern "C" fn() -> ALvoid>;
pub type LPALPROCESSUPDATESSOFT = ::std::option::Option<extern "C" fn() -> ALvoid>;
pub type LPALCDEVICEPAUSESOFT = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice)>;
pub type LPALCDEVICERESUMESOFT = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice)>;
pub type LPALCGETSTRINGISOFT = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice, paramName: ALCenum, index: ALCsizei) -> *const ALCchar>;
pub type LPALCRESETDEVICESOFT = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice, attribs: *const ALCint) -> ALCboolean>;