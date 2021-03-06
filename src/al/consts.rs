use super::types::*;

pub const AL_INVALID: ALenum = -1;
pub const AL_NONE: ALenum = 0;
pub const AL_FALSE: ALboolean = 0;
pub const AL_TRUE: ALboolean = 1;
pub const AL_SOURCE_RELATIVE: ALenum = 514;
pub const AL_CONE_INNER_ANGLE: ALenum = 4097;
pub const AL_CONE_OUTER_ANGLE: ALenum = 4098;
pub const AL_PITCH: ALenum = 4099;
pub const AL_POSITION: ALenum = 4100;
pub const AL_DIRECTION: ALenum = 4101;
pub const AL_VELOCITY: ALenum = 4102;
pub const AL_LOOPING: ALenum = 4103;
pub const AL_BUFFER: ALenum = 4105;
pub const AL_GAIN: ALenum = 4106;
pub const AL_MIN_GAIN: ALenum = 4109;
pub const AL_MAX_GAIN: ALenum = 4110;
pub const AL_ORIENTATION: ALenum = 4111;
pub const AL_SOURCE_STATE: ALenum = 4112;
pub const AL_INITIAL: ALenum = 4113;
pub const AL_PLAYING: ALenum = 4114;
pub const AL_PAUSED: ALenum = 4115;
pub const AL_STOPPED: ALenum = 4116;
pub const AL_BUFFERS_QUEUED: ALenum = 4117;
pub const AL_BUFFERS_PROCESSED: ALenum = 4118;
pub const AL_REFERENCE_DISTANCE: ALenum = 4128;
pub const AL_ROLLOFF_FACTOR: ALenum = 4129;
pub const AL_CONE_OUTER_GAIN: ALenum = 4130;
pub const AL_MAX_DISTANCE: ALenum = 4131;
pub const AL_SEC_OFFSET: ALenum = 4132;
pub const AL_SAMPLE_OFFSET: ALenum = 4133;
pub const AL_BYTE_OFFSET: ALenum = 4134;
pub const AL_SOURCE_TYPE: ALenum = 4135;
pub const AL_STATIC: ALenum = 4136;
pub const AL_STREAMING: ALenum = 4137;
pub const AL_UNDETERMINED: ALenum = 4144;
pub const AL_FORMAT_MONO8: ALenum = 4352;
pub const AL_FORMAT_MONO16: ALenum = 4353;
pub const AL_FORMAT_STEREO8: ALenum = 4354;
pub const AL_FORMAT_STEREO16: ALenum = 4355;
pub const AL_FREQUENCY: ALenum = 8193;
pub const AL_BITS: ALenum = 8194;
pub const AL_CHANNELS: ALenum = 8195;
pub const AL_SIZE: ALenum = 8196;
pub const AL_UNUSED: ALenum = 8208;
pub const AL_PENDING: ALenum = 8209;
pub const AL_PROCESSED: ALenum = 8210;
pub const AL_NO_ERROR: ALenum = 0;
pub const AL_INVALID_NAME: ALenum = 40961;
pub const AL_INVALID_ENUM: ALenum = 40962;
pub const AL_INVALID_VALUE: ALenum = 40963;
pub const AL_INVALID_OPERATION: ALenum = 40964;
pub const AL_OUT_OF_MEMORY: ALenum = 40965;
pub const AL_VENDOR: ALenum = 45057;
pub const AL_VERSION: ALenum = 45058;
pub const AL_RENDERER: ALenum = 45059;
pub const AL_EXTENSIONS: ALenum = 45060;
pub const AL_DOPPLER_FACTOR: ALenum = 49152;
pub const AL_DOPPLER_VELOCITY: ALenum = 49153;
pub const AL_SPEED_OF_SOUND: ALenum = 49155;
pub const AL_DISTANCE_MODEL: ALenum = 53248;
pub const AL_INVERSE_DISTANCE: ALenum = 53249;
pub const AL_INVERSE_DISTANCE_CLAMPED: ALenum = 53250;
pub const AL_LINEAR_DISTANCE: ALenum = 53251;
pub const AL_LINEAR_DISTANCE_CLAMPED: ALenum = 53252;
pub const AL_EXPONENT_DISTANCE: ALenum = 53253;
pub const AL_EXPONENT_DISTANCE_CLAMPED: ALenum = 53254;
pub const AL_METERS_PER_UNIT: ALenum = 131076;
