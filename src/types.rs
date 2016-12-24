//! OpenAL type definitions

pub type ALboolean = ::std::os::raw::c_char;
pub type ALchar = ::std::os::raw::c_char;
pub type ALbyte = ::std::os::raw::c_char;
pub type ALubyte = ::std::os::raw::c_uchar;
pub type ALshort = ::std::os::raw::c_short;
pub type ALushort = ::std::os::raw::c_ushort;
pub type ALint = ::std::os::raw::c_int;
pub type ALuint = ::std::os::raw::c_uint;
pub type ALsizei = ::std::os::raw::c_int;
pub type ALenum = ::std::os::raw::c_int;
pub type ALfloat = f32;
pub type ALdouble = f64;
pub type ALvoid = ::std::os::raw::c_void;

pub type ALCboolean = ::std::os::raw::c_char;
pub type ALCchar = ::std::os::raw::c_char;
pub type ALCbyte = ::std::os::raw::c_char;
pub type ALCubyte = ::std::os::raw::c_uchar;
pub type ALCshort = ::std::os::raw::c_short;
pub type ALCushort = ::std::os::raw::c_ushort;
pub type ALCint = ::std::os::raw::c_int;
pub type ALCuint = ::std::os::raw::c_uint;
pub type ALCsizei = ::std::os::raw::c_int;
pub type ALCenum = ::std::os::raw::c_int;
pub type ALCfloat = f32;
pub type ALCdouble = f64;
pub type ALCvoid = ::std::os::raw::c_void;

pub type LPALENABLE = ::std::option::Option<extern "C" fn(capability: ALenum)>;
pub type LPALDISABLE = ::std::option::Option<extern "C" fn(capability: ALenum)>;
pub type LPALISENABLED = ::std::option::Option<extern "C" fn(capability: ALenum) -> ALboolean>;
pub type LPALGETSTRING = ::std::option::Option<extern "C" fn(param: ALenum) -> *const ALchar>;
pub type LPALGETBOOLEANV = ::std::option::Option<unsafe extern "C" fn(param: ALenum, values: *mut ALboolean)>;
pub type LPALGETINTEGERV = ::std::option::Option<unsafe extern "C" fn(param: ALenum, values: *mut ALint)>;
pub type LPALGETFLOATV = ::std::option::Option<unsafe extern "C" fn(param: ALenum, values: *mut ALfloat)>;
pub type LPALGETDOUBLEV = ::std::option::Option<unsafe extern "C" fn(param: ALenum, values: *mut ALdouble)>;
pub type LPALGETBOOLEAN = ::std::option::Option<extern "C" fn(param: ALenum) -> ALboolean>;
pub type LPALGETINTEGER = ::std::option::Option<extern "C" fn(param: ALenum) -> ALint>;
pub type LPALGETFLOAT = ::std::option::Option<extern "C" fn(param: ALenum) -> ALfloat>;
pub type LPALGETDOUBLE = ::std::option::Option<extern "C" fn(param: ALenum) -> ALdouble>;
pub type LPALGETERROR = ::std::option::Option<extern "C" fn() -> ALenum>;
pub type LPALISEXTENSIONPRESENT = ::std::option::Option<unsafe extern "C" fn(extname: *const ALchar) -> ALboolean>;
pub type LPALGETPROCADDRESS = ::std::option::Option<unsafe extern "C" fn(fname: *const ALchar) -> *mut ::std::os::raw::c_void>;
pub type LPALGETENUMVALUE = ::std::option::Option<unsafe extern "C" fn(ename: *const ALchar) -> ALenum>;
pub type LPALLISTENERF = ::std::option::Option<extern "C" fn(param: ALenum, value: ALfloat)>;
pub type LPALLISTENER3F = ::std::option::Option<extern "C" fn(param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat)>;
pub type LPALLISTENERFV = ::std::option::Option<unsafe extern "C" fn(param: ALenum, values: *const ALfloat)>;
pub type LPALLISTENERI = ::std::option::Option<extern "C" fn(param: ALenum, value: ALint)>;
pub type LPALLISTENER3I = ::std::option::Option<extern "C" fn(param: ALenum, value1: ALint, value2: ALint, value3: ALint)>;
pub type LPALLISTENERIV = ::std::option::Option<unsafe extern "C" fn(param: ALenum, values: *const ALint)>;
pub type LPALGETLISTENERF = ::std::option::Option<unsafe extern "C" fn(param: ALenum, value: *mut ALfloat)>;
pub type LPALGETLISTENER3F = ::std::option::Option<unsafe extern "C" fn(param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat)>;
pub type LPALGETLISTENERFV = ::std::option::Option<unsafe extern "C" fn(param: ALenum, values: *mut ALfloat)>;
pub type LPALGETLISTENERI = ::std::option::Option<unsafe extern "C" fn(param: ALenum, value: *mut ALint)>;
pub type LPALGETLISTENER3I = ::std::option::Option<unsafe extern "C" fn(param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint)>;
pub type LPALGETLISTENERIV = ::std::option::Option<unsafe extern "C" fn(param: ALenum, values: *mut ALint)>;
pub type LPALGENSOURCES = ::std::option::Option<unsafe extern "C" fn(n: ALsizei, sources: *mut ALuint)>;
pub type LPALDELETESOURCES = ::std::option::Option<unsafe extern "C" fn(n: ALsizei, sources: *const ALuint)>;
pub type LPALISSOURCE = ::std::option::Option<extern "C" fn(source: ALuint) -> ALboolean>;
pub type LPALSOURCEF = ::std::option::Option<extern "C" fn(source: ALuint, param: ALenum, value: ALfloat)>;
pub type LPALSOURCE3F = ::std::option::Option<extern "C" fn(source: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat)>;
pub type LPALSOURCEFV = ::std::option::Option<unsafe extern "C" fn(source: ALuint, param: ALenum, values: *const ALfloat)>;
pub type LPALSOURCEI = ::std::option::Option<extern "C" fn(source: ALuint, param: ALenum, value: ALint)>;
pub type LPALSOURCE3I = ::std::option::Option<extern "C" fn(source: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint)>;
pub type LPALSOURCEIV = ::std::option::Option<unsafe extern "C" fn(source: ALuint, param: ALenum, values: *const ALint)>;
pub type LPALGETSOURCEF = ::std::option::Option<unsafe extern "C" fn(source: ALuint, param: ALenum, value: *mut ALfloat)>;
pub type LPALGETSOURCE3F = ::std::option::Option<unsafe extern "C" fn(source: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat)>;
pub type LPALGETSOURCEFV = ::std::option::Option<unsafe extern "C" fn(source: ALuint, param: ALenum, values: *mut ALfloat)>;
pub type LPALGETSOURCEI = ::std::option::Option<unsafe extern "C" fn(source: ALuint, param: ALenum, value: *mut ALint)>;
pub type LPALGETSOURCE3I = ::std::option::Option<unsafe extern "C" fn(source: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint)>;
pub type LPALGETSOURCEIV = ::std::option::Option<unsafe extern "C" fn(source: ALuint, param: ALenum, values: *mut ALint)>;
pub type LPALSOURCEPLAYV = ::std::option::Option<unsafe extern "C" fn(n: ALsizei, sources: *const ALuint)>;
pub type LPALSOURCESTOPV = ::std::option::Option<unsafe extern "C" fn(n: ALsizei, sources: *const ALuint)>;
pub type LPALSOURCEREWINDV = ::std::option::Option<unsafe extern "C" fn(n: ALsizei, sources: *const ALuint)>;
pub type LPALSOURCEPAUSEV = ::std::option::Option<unsafe extern "C" fn(n: ALsizei, sources: *const ALuint)>;
pub type LPALSOURCEPLAY = ::std::option::Option<extern "C" fn(source: ALuint)>;
pub type LPALSOURCESTOP = ::std::option::Option<extern "C" fn(source: ALuint)>;
pub type LPALSOURCEREWIND = ::std::option::Option<extern "C" fn(source: ALuint)>;
pub type LPALSOURCEPAUSE = ::std::option::Option<extern "C" fn(source: ALuint)>;
pub type LPALSOURCEQUEUEBUFFERS = ::std::option::Option<unsafe extern "C" fn(source: ALuint, nb: ALsizei, buffers: *const ALuint)>;
pub type LPALSOURCEUNQUEUEBUFFERS = ::std::option::Option<unsafe extern "C" fn(source: ALuint, nb: ALsizei, buffers: *mut ALuint)>;
pub type LPALGENBUFFERS = ::std::option::Option<unsafe extern "C" fn(n: ALsizei, buffers: *mut ALuint)>;
pub type LPALDELETEBUFFERS = ::std::option::Option<unsafe extern "C" fn(n: ALsizei, buffers: *const ALuint)>;
pub type LPALISBUFFER = ::std::option::Option<extern "C" fn(buffer: ALuint) -> ALboolean>;
pub type LPALBUFFERDATA = ::std::option::Option<unsafe extern "C" fn(buffer: ALuint, format: ALenum, data: *const ALvoid, size: ALsizei, freq: ALsizei)>;
pub type LPALBUFFERF = ::std::option::Option<extern "C" fn(buffer: ALuint, param: ALenum, value: ALfloat)>;
pub type LPALBUFFER3F = ::std::option::Option<extern "C" fn(buffer: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat)>;
pub type LPALBUFFERFV = ::std::option::Option<unsafe extern "C" fn(buffer: ALuint, param: ALenum, values: *const ALfloat)>;
pub type LPALBUFFERI = ::std::option::Option<extern "C" fn(buffer: ALuint, param: ALenum, value: ALint)>;
pub type LPALBUFFER3I = ::std::option::Option<extern "C" fn(buffer: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint)>;
pub type LPALBUFFERIV = ::std::option::Option<unsafe extern "C" fn(buffer: ALuint, param: ALenum, values: *const ALint)>;
pub type LPALGETBUFFERF = ::std::option::Option<unsafe extern "C" fn(buffer: ALuint, param: ALenum, value: *mut ALfloat)>;
pub type LPALGETBUFFER3F = ::std::option::Option<unsafe extern "C" fn(buffer: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat)>;
pub type LPALGETBUFFERFV = ::std::option::Option<unsafe extern "C" fn(buffer: ALuint, param: ALenum, values: *mut ALfloat)>;
pub type LPALGETBUFFERI = ::std::option::Option<unsafe extern "C" fn(buffer: ALuint, param: ALenum, value: *mut ALint)>;
pub type LPALGETBUFFER3I = ::std::option::Option<unsafe extern "C" fn(buffer: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint)>;
pub type LPALGETBUFFERIV = ::std::option::Option<unsafe extern "C" fn(buffer: ALuint, param: ALenum, values: *mut ALint)>;
pub type LPALDOPPLERFACTOR = ::std::option::Option<extern "C" fn(value: ALfloat)>;
pub type LPALDOPPLERVELOCITY = ::std::option::Option<extern "C" fn(value: ALfloat)>;
pub type LPALSPEEDOFSOUND = ::std::option::Option<extern "C" fn(value: ALfloat)>;
pub type LPALDISTANCEMODEL = ::std::option::Option<extern "C" fn(distanceModel: ALenum)>;

pub type ALCdevice = ::ALCdevice_struct;
pub type ALCcontext = ::ALCcontext_struct;

pub type LPALCCREATECONTEXT = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice, attrlist: *const ALCint) -> *mut ALCcontext>;
pub type LPALCMAKECONTEXTCURRENT = ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext) -> ALCboolean>;
pub type LPALCPROCESSCONTEXT = ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext)>;
pub type LPALCSUSPENDCONTEXT = ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext)>;
pub type LPALCDESTROYCONTEXT = ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext)>;
pub type LPALCGETCURRENTCONTEXT = ::std::option::Option<extern "C" fn() -> *mut ALCcontext>;
pub type LPALCGETCONTEXTSDEVICE = ::std::option::Option<unsafe extern "C" fn(context: *mut ALCcontext) -> *mut ALCdevice>;
pub type LPALCOPENDEVICE = ::std::option::Option<unsafe extern "C" fn(devicename: *const ALCchar) -> *mut ALCdevice>;
pub type LPALCCLOSEDEVICE = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice) -> ALCboolean>;
pub type LPALCGETERROR = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice) -> ALCenum>;
pub type LPALCISEXTENSIONPRESENT = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice, extname: *const ALCchar) -> ALCboolean>;
pub type LPALCGETPROCADDRESS = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice, funcname: *const ALCchar) -> *mut ::std::os::raw::c_void>;
pub type LPALCGETENUMVALUE = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice, enumname: *const ALCchar) -> ALCenum>;
pub type LPALCGETSTRING = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice, param: ALCenum) -> *const ALCchar>;
pub type LPALCGETINTEGERV = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice, param: ALCenum, size: ALCsizei, values: *mut ALCint)>;
pub type LPALCCAPTUREOPENDEVICE = ::std::option::Option<unsafe extern "C" fn(devicename: *const ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *mut ALCdevice>;
pub type LPALCCAPTURECLOSEDEVICE = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice) -> ALCboolean>;
pub type LPALCCAPTURESTART = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice)>;
pub type LPALCCAPTURESTOP = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice)>;
pub type LPALCCAPTURESAMPLES = ::std::option::Option<unsafe extern "C" fn(device: *mut ALCdevice, buffer: *mut ALCvoid, samples: ALCsizei)>;
