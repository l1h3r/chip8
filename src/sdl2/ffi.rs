use core::ffi::c_void;

pub const SDL_INIT_TIMER: u32 = 1;
pub const SDL_INIT_AUDIO: u32 = 16;
pub const SDL_INIT_VIDEO: u32 = 32;
pub const SDL_INIT_JOYSTICK: u32 = 512;
pub const SDL_INIT_HAPTIC: u32 = 4096;
pub const SDL_INIT_GAMECONTROLLER: u32 = 8192;
pub const SDL_INIT_EVENTS: u32 = 16384;
pub const SDL_INIT_SENSOR: u32 = 32768;
pub const SDL_INIT_NOPARACHUTE: u32 = 1048576;
pub const SDL_INIT_EVERYTHING: u32 = 62001;

pub const SDL_WINDOWPOS_UNDEFINED_MASK: u32 = 536805376;
pub const SDL_WINDOWPOS_CENTERED_MASK: u32 = 805240832;

pub const SDL_BUTTON_LEFT: u32 = 1;
pub const SDL_BUTTON_MIDDLE: u32 = 2;
pub const SDL_BUTTON_RIGHT: u32 = 3;
pub const SDL_BUTTON_X1: u32 = 4;
pub const SDL_BUTTON_X2: u32 = 5;

pub const AUDIO_U8: u32 = 8;
pub const AUDIO_S8: u32 = 32776;
pub const AUDIO_U16LSB: u32 = 16;
pub const AUDIO_S16LSB: u32 = 32784;
pub const AUDIO_U16MSB: u32 = 4112;
pub const AUDIO_S16MSB: u32 = 36880;
pub const AUDIO_U16: u32 = 16;
pub const AUDIO_S16: u32 = 32784;
pub const AUDIO_S32LSB: u32 = 32800;
pub const AUDIO_S32MSB: u32 = 36896;
pub const AUDIO_S32: u32 = 32800;
pub const AUDIO_F32LSB: u32 = 33056;
pub const AUDIO_F32MSB: u32 = 37152;
pub const AUDIO_F32: u32 = 33056;
pub const AUDIO_U16SYS: u32 = 16;
pub const AUDIO_S16SYS: u32 = 32784;
pub const AUDIO_S32SYS: u32 = 32800;
pub const AUDIO_F32SYS: u32 = 33056;

pub type SDL_GLContext = *mut c_void;

pub type SDL_TouchID = i64;
pub type SDL_FingerID = i64;
pub type SDL_GestureID = i64;
pub type SDL_JoystickID = i32;

pub type SDL_Keycode = i32;

pub type SDL_AudioFormat = u16;
pub type SDL_AudioDeviceID = u32;
pub type SDL_AudioCallback =
  Option<unsafe extern "C" fn(userdata: *mut c_void, stream: *mut u8, len: i32)>;

bitflags! {
  pub struct WindowFlags: u32 {
    const FULLSCREEN = 1;
    const OPENGL = 2;
    const SHOWN = 4;
    const HIDDEN = 8;
    const BORDERLESS = 16;
    const RESIZABLE = 32;
    const MINIMIZED = 64;
    const MAXIMIZED = 128;
    const INPUT_GRABBED = 256;
    const INPUT_FOCUS = 512;
    const MOUSE_FOCUS = 1024;
    const FULLSCREEN_DESKTOP = 4097;
    const FOREIGN = 2048;
    const ALLOW_HIGHDPI = 8192;
    const MOUSE_CAPTURE = 16384;
    const ALWAYS_ON_TOP = 32768;
    const SKIP_TASKBAR = 65536;
    const UTILITY = 131072;
    const TOOLTIP = 262144;
    const POPUP_MENU = 524288;
    const VULKAN = 268435456;
  }
}

bitflags! {
  pub struct RendererFlags: u32 {
    const SOFTWARE = 1;
    const ACCELERATED = 2;
    const PRESENTVSYNC = 4;
    const TARGETTEXTURE = 8;
  }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum SDL_Button {
  LEFT = SDL_BUTTON_LEFT,
  MIDDLE = SDL_BUTTON_MIDDLE,
  RIGHT = SDL_BUTTON_RIGHT,
  X1 = SDL_BUTTON_X1,
  X2 = SDL_BUTTON_X2,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum GLattr {
  RED_SIZE = 0,
  GREEN_SIZE = 1,
  BLUE_SIZE = 2,
  ALPHA_SIZE = 3,
  BUFFER_SIZE = 4,
  DOUBLEBUFFER = 5,
  DEPTH_SIZE = 6,
  STENCIL_SIZE = 7,
  ACCUM_RED_SIZE = 8,
  ACCUM_GREEN_SIZE = 9,
  ACCUM_BLUE_SIZE = 10,
  ACCUM_ALPHA_SIZE = 11,
  STEREO = 12,
  MULTISAMPLEBUFFERS = 13,
  MULTISAMPLESAMPLES = 14,
  ACCELERATED_VISUAL = 15,
  RETAINED_BACKING = 16,
  CONTEXT_MAJOR_VERSION = 17,
  CONTEXT_MINOR_VERSION = 18,
  CONTEXT_EGL = 19,
  CONTEXT_FLAGS = 20,
  CONTEXT_PROFILE_MASK = 21,
  SHARE_WITH_CURRENT_CONTEXT = 22,
  FRAMEBUFFER_SRGB_CAPABLE = 23,
  CONTEXT_RELEASE_BEHAVIOR = 24,
  CONTEXT_RESET_NOTIFICATION = 25,
  CONTEXT_NO_ERROR = 26,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum GLprofile {
  CORE = 1,
  COMPATIBILITY = 2,
  ES = 4,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SDL_Window {
  _unused: [u8; 0],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SDL_Renderer {
  _unused: [u8; 0],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SDL_Texture {
  _unused: [u8; 0],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SDL_BlitMap {
  pub _address: u8,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_Rect {
  pub x: i32,
  pub y: i32,
  pub w: i32,
  pub h: i32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_Surface {
  pub flags: u32,
  pub format: *mut SDL_PixelFormat,
  pub w: i32,
  pub h: i32,
  pub pitch: i32,
  pub pixels: *mut c_void,
  pub userdata: *mut c_void,
  pub locked: i32,
  pub lock_data: *mut c_void,
  pub clip_rect: SDL_Rect,
  pub map: *mut SDL_BlitMap,
  pub refcount: i32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_PixelFormat {
  pub format: u32,
  pub palette: *mut SDL_Palette,
  pub BitsPerPixel: u8,
  pub BytesPerPixel: u8,
  pub padding: [u8; 2],
  pub Rmask: u32,
  pub Gmask: u32,
  pub Bmask: u32,
  pub Amask: u32,
  pub Rloss: u8,
  pub Gloss: u8,
  pub Bloss: u8,
  pub Aloss: u8,
  pub Rshift: u8,
  pub Gshift: u8,
  pub Bshift: u8,
  pub Ashift: u8,
  pub refcount: i32,
  pub next: *mut SDL_PixelFormat,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_Palette {
  pub ncolors: i32,
  pub colors: *mut SDL_Color,
  pub version: u32,
  pub refcount: i32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum PixelFormat {
  UNKNOWN = 0,
  INDEX1LSB = 286261504,
  INDEX1MSB = 287310080,
  INDEX4LSB = 303039488,
  INDEX4MSB = 304088064,
  INDEX8 = 318769153,
  RGB332 = 336660481,
  RGB444 = 353504258,
  RGB555 = 353570562,
  BGR555 = 357764866,
  ARGB4444 = 355602434,
  RGBA4444 = 356651010,
  ABGR4444 = 359796738,
  BGRA4444 = 360845314,
  ARGB1555 = 355667970,
  RGBA5551 = 356782082,
  ABGR1555 = 359862274,
  BGRA5551 = 360976386,
  RGB565 = 353701890,
  BGR565 = 357896194,
  RGB24 = 386930691,
  BGR24 = 390076419,
  RGB888 = 370546692,
  RGBX8888 = 371595268,
  BGR888 = 374740996,
  BGRX8888 = 375789572,
  ARGB8888 = 372645892,
  RGBA8888 = 373694468,
  ABGR8888 = 376840196,
  BGRA8888 = 377888772,
  ARGB2101010 = 372711428,
  YV12 = 842094169,
  IYUV = 1448433993,
  YUY2 = 844715353,
  UYVY = 1498831189,
  YVYU = 1431918169,
  NV12 = 842094158,
  NV21 = 825382478,
  EXTERNAL_OES = 542328143,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum BlendMode {
  NONE = 0,
  BLEND = 1,
  ADD = 2,
  MOD = 4,
  INVALID = 2147483647,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum TextureAccess {
  STATIC = 0,
  STREAMING = 1,
  TARGET = 2,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum SDL_Scancode {
  SDL_SCANCODE_UNKNOWN = 0,
  SDL_SCANCODE_A = 4,
  SDL_SCANCODE_B = 5,
  SDL_SCANCODE_C = 6,
  SDL_SCANCODE_D = 7,
  SDL_SCANCODE_E = 8,
  SDL_SCANCODE_F = 9,
  SDL_SCANCODE_G = 10,
  SDL_SCANCODE_H = 11,
  SDL_SCANCODE_I = 12,
  SDL_SCANCODE_J = 13,
  SDL_SCANCODE_K = 14,
  SDL_SCANCODE_L = 15,
  SDL_SCANCODE_M = 16,
  SDL_SCANCODE_N = 17,
  SDL_SCANCODE_O = 18,
  SDL_SCANCODE_P = 19,
  SDL_SCANCODE_Q = 20,
  SDL_SCANCODE_R = 21,
  SDL_SCANCODE_S = 22,
  SDL_SCANCODE_T = 23,
  SDL_SCANCODE_U = 24,
  SDL_SCANCODE_V = 25,
  SDL_SCANCODE_W = 26,
  SDL_SCANCODE_X = 27,
  SDL_SCANCODE_Y = 28,
  SDL_SCANCODE_Z = 29,
  SDL_SCANCODE_1 = 30,
  SDL_SCANCODE_2 = 31,
  SDL_SCANCODE_3 = 32,
  SDL_SCANCODE_4 = 33,
  SDL_SCANCODE_5 = 34,
  SDL_SCANCODE_6 = 35,
  SDL_SCANCODE_7 = 36,
  SDL_SCANCODE_8 = 37,
  SDL_SCANCODE_9 = 38,
  SDL_SCANCODE_0 = 39,
  SDL_SCANCODE_RETURN = 40,
  SDL_SCANCODE_ESCAPE = 41,
  SDL_SCANCODE_BACKSPACE = 42,
  SDL_SCANCODE_TAB = 43,
  SDL_SCANCODE_SPACE = 44,
  SDL_SCANCODE_MINUS = 45,
  SDL_SCANCODE_EQUALS = 46,
  SDL_SCANCODE_LEFTBRACKET = 47,
  SDL_SCANCODE_RIGHTBRACKET = 48,
  SDL_SCANCODE_BACKSLASH = 49,
  SDL_SCANCODE_NONUSHASH = 50,
  SDL_SCANCODE_SEMICOLON = 51,
  SDL_SCANCODE_APOSTROPHE = 52,
  SDL_SCANCODE_GRAVE = 53,
  SDL_SCANCODE_COMMA = 54,
  SDL_SCANCODE_PERIOD = 55,
  SDL_SCANCODE_SLASH = 56,
  SDL_SCANCODE_CAPSLOCK = 57,
  SDL_SCANCODE_F1 = 58,
  SDL_SCANCODE_F2 = 59,
  SDL_SCANCODE_F3 = 60,
  SDL_SCANCODE_F4 = 61,
  SDL_SCANCODE_F5 = 62,
  SDL_SCANCODE_F6 = 63,
  SDL_SCANCODE_F7 = 64,
  SDL_SCANCODE_F8 = 65,
  SDL_SCANCODE_F9 = 66,
  SDL_SCANCODE_F10 = 67,
  SDL_SCANCODE_F11 = 68,
  SDL_SCANCODE_F12 = 69,
  SDL_SCANCODE_PRINTSCREEN = 70,
  SDL_SCANCODE_SCROLLLOCK = 71,
  SDL_SCANCODE_PAUSE = 72,
  SDL_SCANCODE_INSERT = 73,
  SDL_SCANCODE_HOME = 74,
  SDL_SCANCODE_PAGEUP = 75,
  SDL_SCANCODE_DELETE = 76,
  SDL_SCANCODE_END = 77,
  SDL_SCANCODE_PAGEDOWN = 78,
  SDL_SCANCODE_RIGHT = 79,
  SDL_SCANCODE_LEFT = 80,
  SDL_SCANCODE_DOWN = 81,
  SDL_SCANCODE_UP = 82,
  SDL_SCANCODE_NUMLOCKCLEAR = 83,
  SDL_SCANCODE_KP_DIVIDE = 84,
  SDL_SCANCODE_KP_MULTIPLY = 85,
  SDL_SCANCODE_KP_MINUS = 86,
  SDL_SCANCODE_KP_PLUS = 87,
  SDL_SCANCODE_KP_ENTER = 88,
  SDL_SCANCODE_KP_1 = 89,
  SDL_SCANCODE_KP_2 = 90,
  SDL_SCANCODE_KP_3 = 91,
  SDL_SCANCODE_KP_4 = 92,
  SDL_SCANCODE_KP_5 = 93,
  SDL_SCANCODE_KP_6 = 94,
  SDL_SCANCODE_KP_7 = 95,
  SDL_SCANCODE_KP_8 = 96,
  SDL_SCANCODE_KP_9 = 97,
  SDL_SCANCODE_KP_0 = 98,
  SDL_SCANCODE_KP_PERIOD = 99,
  SDL_SCANCODE_NONUSBACKSLASH = 100,
  SDL_SCANCODE_APPLICATION = 101,
  SDL_SCANCODE_POWER = 102,
  SDL_SCANCODE_KP_EQUALS = 103,
  SDL_SCANCODE_F13 = 104,
  SDL_SCANCODE_F14 = 105,
  SDL_SCANCODE_F15 = 106,
  SDL_SCANCODE_F16 = 107,
  SDL_SCANCODE_F17 = 108,
  SDL_SCANCODE_F18 = 109,
  SDL_SCANCODE_F19 = 110,
  SDL_SCANCODE_F20 = 111,
  SDL_SCANCODE_F21 = 112,
  SDL_SCANCODE_F22 = 113,
  SDL_SCANCODE_F23 = 114,
  SDL_SCANCODE_F24 = 115,
  SDL_SCANCODE_EXECUTE = 116,
  SDL_SCANCODE_HELP = 117,
  SDL_SCANCODE_MENU = 118,
  SDL_SCANCODE_SELECT = 119,
  SDL_SCANCODE_STOP = 120,
  SDL_SCANCODE_AGAIN = 121,
  SDL_SCANCODE_UNDO = 122,
  SDL_SCANCODE_CUT = 123,
  SDL_SCANCODE_COPY = 124,
  SDL_SCANCODE_PASTE = 125,
  SDL_SCANCODE_FIND = 126,
  SDL_SCANCODE_MUTE = 127,
  SDL_SCANCODE_VOLUMEUP = 128,
  SDL_SCANCODE_VOLUMEDOWN = 129,
  SDL_SCANCODE_KP_COMMA = 133,
  SDL_SCANCODE_KP_EQUALSAS400 = 134,
  SDL_SCANCODE_INTERNATIONAL1 = 135,
  SDL_SCANCODE_INTERNATIONAL2 = 136,
  SDL_SCANCODE_INTERNATIONAL3 = 137,
  SDL_SCANCODE_INTERNATIONAL4 = 138,
  SDL_SCANCODE_INTERNATIONAL5 = 139,
  SDL_SCANCODE_INTERNATIONAL6 = 140,
  SDL_SCANCODE_INTERNATIONAL7 = 141,
  SDL_SCANCODE_INTERNATIONAL8 = 142,
  SDL_SCANCODE_INTERNATIONAL9 = 143,
  SDL_SCANCODE_LANG1 = 144,
  SDL_SCANCODE_LANG2 = 145,
  SDL_SCANCODE_LANG3 = 146,
  SDL_SCANCODE_LANG4 = 147,
  SDL_SCANCODE_LANG5 = 148,
  SDL_SCANCODE_LANG6 = 149,
  SDL_SCANCODE_LANG7 = 150,
  SDL_SCANCODE_LANG8 = 151,
  SDL_SCANCODE_LANG9 = 152,
  SDL_SCANCODE_ALTERASE = 153,
  SDL_SCANCODE_SYSREQ = 154,
  SDL_SCANCODE_CANCEL = 155,
  SDL_SCANCODE_CLEAR = 156,
  SDL_SCANCODE_PRIOR = 157,
  SDL_SCANCODE_RETURN2 = 158,
  SDL_SCANCODE_SEPARATOR = 159,
  SDL_SCANCODE_OUT = 160,
  SDL_SCANCODE_OPER = 161,
  SDL_SCANCODE_CLEARAGAIN = 162,
  SDL_SCANCODE_CRSEL = 163,
  SDL_SCANCODE_EXSEL = 164,
  SDL_SCANCODE_KP_00 = 176,
  SDL_SCANCODE_KP_000 = 177,
  SDL_SCANCODE_THOUSANDSSEPARATOR = 178,
  SDL_SCANCODE_DECIMALSEPARATOR = 179,
  SDL_SCANCODE_CURRENCYUNIT = 180,
  SDL_SCANCODE_CURRENCYSUBUNIT = 181,
  SDL_SCANCODE_KP_LEFTPAREN = 182,
  SDL_SCANCODE_KP_RIGHTPAREN = 183,
  SDL_SCANCODE_KP_LEFTBRACE = 184,
  SDL_SCANCODE_KP_RIGHTBRACE = 185,
  SDL_SCANCODE_KP_TAB = 186,
  SDL_SCANCODE_KP_BACKSPACE = 187,
  SDL_SCANCODE_KP_A = 188,
  SDL_SCANCODE_KP_B = 189,
  SDL_SCANCODE_KP_C = 190,
  SDL_SCANCODE_KP_D = 191,
  SDL_SCANCODE_KP_E = 192,
  SDL_SCANCODE_KP_F = 193,
  SDL_SCANCODE_KP_XOR = 194,
  SDL_SCANCODE_KP_POWER = 195,
  SDL_SCANCODE_KP_PERCENT = 196,
  SDL_SCANCODE_KP_LESS = 197,
  SDL_SCANCODE_KP_GREATER = 198,
  SDL_SCANCODE_KP_AMPERSAND = 199,
  SDL_SCANCODE_KP_DBLAMPERSAND = 200,
  SDL_SCANCODE_KP_VERTICALBAR = 201,
  SDL_SCANCODE_KP_DBLVERTICALBAR = 202,
  SDL_SCANCODE_KP_COLON = 203,
  SDL_SCANCODE_KP_HASH = 204,
  SDL_SCANCODE_KP_SPACE = 205,
  SDL_SCANCODE_KP_AT = 206,
  SDL_SCANCODE_KP_EXCLAM = 207,
  SDL_SCANCODE_KP_MEMSTORE = 208,
  SDL_SCANCODE_KP_MEMRECALL = 209,
  SDL_SCANCODE_KP_MEMCLEAR = 210,
  SDL_SCANCODE_KP_MEMADD = 211,
  SDL_SCANCODE_KP_MEMSUBTRACT = 212,
  SDL_SCANCODE_KP_MEMMULTIPLY = 213,
  SDL_SCANCODE_KP_MEMDIVIDE = 214,
  SDL_SCANCODE_KP_PLUSMINUS = 215,
  SDL_SCANCODE_KP_CLEAR = 216,
  SDL_SCANCODE_KP_CLEARENTRY = 217,
  SDL_SCANCODE_KP_BINARY = 218,
  SDL_SCANCODE_KP_OCTAL = 219,
  SDL_SCANCODE_KP_DECIMAL = 220,
  SDL_SCANCODE_KP_HEXADECIMAL = 221,
  SDL_SCANCODE_LCTRL = 224,
  SDL_SCANCODE_LSHIFT = 225,
  SDL_SCANCODE_LALT = 226,
  SDL_SCANCODE_LGUI = 227,
  SDL_SCANCODE_RCTRL = 228,
  SDL_SCANCODE_RSHIFT = 229,
  SDL_SCANCODE_RALT = 230,
  SDL_SCANCODE_RGUI = 231,
  SDL_SCANCODE_MODE = 257,
  SDL_SCANCODE_AUDIONEXT = 258,
  SDL_SCANCODE_AUDIOPREV = 259,
  SDL_SCANCODE_AUDIOSTOP = 260,
  SDL_SCANCODE_AUDIOPLAY = 261,
  SDL_SCANCODE_AUDIOMUTE = 262,
  SDL_SCANCODE_MEDIASELECT = 263,
  SDL_SCANCODE_WWW = 264,
  SDL_SCANCODE_MAIL = 265,
  SDL_SCANCODE_CALCULATOR = 266,
  SDL_SCANCODE_COMPUTER = 267,
  SDL_SCANCODE_AC_SEARCH = 268,
  SDL_SCANCODE_AC_HOME = 269,
  SDL_SCANCODE_AC_BACK = 270,
  SDL_SCANCODE_AC_FORWARD = 271,
  SDL_SCANCODE_AC_STOP = 272,
  SDL_SCANCODE_AC_REFRESH = 273,
  SDL_SCANCODE_AC_BOOKMARKS = 274,
  SDL_SCANCODE_BRIGHTNESSDOWN = 275,
  SDL_SCANCODE_BRIGHTNESSUP = 276,
  SDL_SCANCODE_DISPLAYSWITCH = 277,
  SDL_SCANCODE_KBDILLUMTOGGLE = 278,
  SDL_SCANCODE_KBDILLUMDOWN = 279,
  SDL_SCANCODE_KBDILLUMUP = 280,
  SDL_SCANCODE_EJECT = 281,
  SDL_SCANCODE_SLEEP = 282,
  SDL_SCANCODE_APP1 = 283,
  SDL_SCANCODE_APP2 = 284,
  SDL_SCANCODE_AUDIOREWIND = 285,
  SDL_SCANCODE_AUDIOFASTFORWARD = 286,
  SDL_NUM_SCANCODES = 512,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SDL_EventType {
  FIRSTEVENT = 0,
  QUIT = 256,
  APP_TERMINATING = 257,
  APP_LOWMEMORY = 258,
  APP_WILLENTERBACKGROUND = 259,
  APP_DIDENTERBACKGROUND = 260,
  APP_WILLENTERFOREGROUND = 261,
  APP_DIDENTERFOREGROUND = 262,
  DISPLAYEVENT = 336,
  WINDOWEVENT = 512,
  SYSWMEVENT = 513,
  KEYDOWN = 768,
  KEYUP = 769,
  TEXTEDITING = 770,
  TEXTINPUT = 771,
  KEYMAPCHANGED = 772,
  MOUSEMOTION = 1024,
  MOUSEBUTTONDOWN = 1025,
  MOUSEBUTTONUP = 1026,
  MOUSEWHEEL = 1027,
  JOYAXISMOTION = 1536,
  JOYBALLMOTION = 1537,
  JOYHATMOTION = 1538,
  JOYBUTTONDOWN = 1539,
  JOYBUTTONUP = 1540,
  JOYDEVICEADDED = 1541,
  JOYDEVICEREMOVED = 1542,
  CONTROLLERAXISMOTION = 1616,
  CONTROLLERBUTTONDOWN = 1617,
  CONTROLLERBUTTONUP = 1618,
  CONTROLLERDEVICEADDED = 1619,
  CONTROLLERDEVICEREMOVED = 1620,
  CONTROLLERDEVICEREMAPPED = 1621,
  FINGERDOWN = 1792,
  FINGERUP = 1793,
  FINGERMOTION = 1794,
  DOLLARGESTURE = 2048,
  DOLLARRECORD = 2049,
  MULTIGESTURE = 2050,
  CLIPBOARDUPDATE = 2304,
  DROPFILE = 4096,
  DROPTEXT = 4097,
  DROPBEGIN = 4098,
  DROPCOMPLETE = 4099,
  AUDIODEVICEADDED = 4352,
  AUDIODEVICEREMOVED = 4353,
  SENSORUPDATE = 4608,
  RENDER_TARGETS_RESET = 8192,
  RENDER_DEVICE_RESET = 8193,
  USEREVENT = 32768,
  LASTEVENT = 65535,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[repr(u32)]
pub enum SDLK_Keycode {
  SDLK_UNKNOWN = 0,
  SDLK_RETURN = 13,
  SDLK_ESCAPE = 27,
  SDLK_BACKSPACE = 8,
  SDLK_TAB = 9,
  SDLK_SPACE = 32,
  SDLK_EXCLAIM = 33,
  SDLK_QUOTEDBL = 34,
  SDLK_HASH = 35,
  SDLK_PERCENT = 37,
  SDLK_DOLLAR = 36,
  SDLK_AMPERSAND = 38,
  SDLK_QUOTE = 39,
  SDLK_LEFTPAREN = 40,
  SDLK_RIGHTPAREN = 41,
  SDLK_ASTERISK = 42,
  SDLK_PLUS = 43,
  SDLK_COMMA = 44,
  SDLK_MINUS = 45,
  SDLK_PERIOD = 46,
  SDLK_SLASH = 47,
  SDLK_0 = 48,
  SDLK_1 = 49,
  SDLK_2 = 50,
  SDLK_3 = 51,
  SDLK_4 = 52,
  SDLK_5 = 53,
  SDLK_6 = 54,
  SDLK_7 = 55,
  SDLK_8 = 56,
  SDLK_9 = 57,
  SDLK_COLON = 58,
  SDLK_SEMICOLON = 59,
  SDLK_LESS = 60,
  SDLK_EQUALS = 61,
  SDLK_GREATER = 62,
  SDLK_QUESTION = 63,
  SDLK_AT = 64,
  SDLK_LEFTBRACKET = 91,
  SDLK_BACKSLASH = 92,
  SDLK_RIGHTBRACKET = 93,
  SDLK_CARET = 94,
  SDLK_UNDERSCORE = 95,
  SDLK_BACKQUOTE = 96,
  SDLK_a = 97,
  SDLK_b = 98,
  SDLK_c = 99,
  SDLK_d = 100,
  SDLK_e = 101,
  SDLK_f = 102,
  SDLK_g = 103,
  SDLK_h = 104,
  SDLK_i = 105,
  SDLK_j = 106,
  SDLK_k = 107,
  SDLK_l = 108,
  SDLK_m = 109,
  SDLK_n = 110,
  SDLK_o = 111,
  SDLK_p = 112,
  SDLK_q = 113,
  SDLK_r = 114,
  SDLK_s = 115,
  SDLK_t = 116,
  SDLK_u = 117,
  SDLK_v = 118,
  SDLK_w = 119,
  SDLK_x = 120,
  SDLK_y = 121,
  SDLK_z = 122,
  SDLK_CAPSLOCK = 1073741881,
  SDLK_F1 = 1073741882,
  SDLK_F2 = 1073741883,
  SDLK_F3 = 1073741884,
  SDLK_F4 = 1073741885,
  SDLK_F5 = 1073741886,
  SDLK_F6 = 1073741887,
  SDLK_F7 = 1073741888,
  SDLK_F8 = 1073741889,
  SDLK_F9 = 1073741890,
  SDLK_F10 = 1073741891,
  SDLK_F11 = 1073741892,
  SDLK_F12 = 1073741893,
  SDLK_PRINTSCREEN = 1073741894,
  SDLK_SCROLLLOCK = 1073741895,
  SDLK_PAUSE = 1073741896,
  SDLK_INSERT = 1073741897,
  SDLK_HOME = 1073741898,
  SDLK_PAGEUP = 1073741899,
  SDLK_DELETE = 127,
  SDLK_END = 1073741901,
  SDLK_PAGEDOWN = 1073741902,
  SDLK_RIGHT = 1073741903,
  SDLK_LEFT = 1073741904,
  SDLK_DOWN = 1073741905,
  SDLK_UP = 1073741906,
  SDLK_NUMLOCKCLEAR = 1073741907,
  SDLK_KP_DIVIDE = 1073741908,
  SDLK_KP_MULTIPLY = 1073741909,
  SDLK_KP_MINUS = 1073741910,
  SDLK_KP_PLUS = 1073741911,
  SDLK_KP_ENTER = 1073741912,
  SDLK_KP_1 = 1073741913,
  SDLK_KP_2 = 1073741914,
  SDLK_KP_3 = 1073741915,
  SDLK_KP_4 = 1073741916,
  SDLK_KP_5 = 1073741917,
  SDLK_KP_6 = 1073741918,
  SDLK_KP_7 = 1073741919,
  SDLK_KP_8 = 1073741920,
  SDLK_KP_9 = 1073741921,
  SDLK_KP_0 = 1073741922,
  SDLK_KP_PERIOD = 1073741923,
  SDLK_APPLICATION = 1073741925,
  SDLK_POWER = 1073741926,
  SDLK_KP_EQUALS = 1073741927,
  SDLK_F13 = 1073741928,
  SDLK_F14 = 1073741929,
  SDLK_F15 = 1073741930,
  SDLK_F16 = 1073741931,
  SDLK_F17 = 1073741932,
  SDLK_F18 = 1073741933,
  SDLK_F19 = 1073741934,
  SDLK_F20 = 1073741935,
  SDLK_F21 = 1073741936,
  SDLK_F22 = 1073741937,
  SDLK_F23 = 1073741938,
  SDLK_F24 = 1073741939,
  SDLK_EXECUTE = 1073741940,
  SDLK_HELP = 1073741941,
  SDLK_MENU = 1073741942,
  SDLK_SELECT = 1073741943,
  SDLK_STOP = 1073741944,
  SDLK_AGAIN = 1073741945,
  SDLK_UNDO = 1073741946,
  SDLK_CUT = 1073741947,
  SDLK_COPY = 1073741948,
  SDLK_PASTE = 1073741949,
  SDLK_FIND = 1073741950,
  SDLK_MUTE = 1073741951,
  SDLK_VOLUMEUP = 1073741952,
  SDLK_VOLUMEDOWN = 1073741953,
  SDLK_KP_COMMA = 1073741957,
  SDLK_KP_EQUALSAS400 = 1073741958,
  SDLK_ALTERASE = 1073741977,
  SDLK_SYSREQ = 1073741978,
  SDLK_CANCEL = 1073741979,
  SDLK_CLEAR = 1073741980,
  SDLK_PRIOR = 1073741981,
  SDLK_RETURN2 = 1073741982,
  SDLK_SEPARATOR = 1073741983,
  SDLK_OUT = 1073741984,
  SDLK_OPER = 1073741985,
  SDLK_CLEARAGAIN = 1073741986,
  SDLK_CRSEL = 1073741987,
  SDLK_EXSEL = 1073741988,
  SDLK_KP_00 = 1073742000,
  SDLK_KP_000 = 1073742001,
  SDLK_THOUSANDSSEPARATOR = 1073742002,
  SDLK_DECIMALSEPARATOR = 1073742003,
  SDLK_CURRENCYUNIT = 1073742004,
  SDLK_CURRENCYSUBUNIT = 1073742005,
  SDLK_KP_LEFTPAREN = 1073742006,
  SDLK_KP_RIGHTPAREN = 1073742007,
  SDLK_KP_LEFTBRACE = 1073742008,
  SDLK_KP_RIGHTBRACE = 1073742009,
  SDLK_KP_TAB = 1073742010,
  SDLK_KP_BACKSPACE = 1073742011,
  SDLK_KP_A = 1073742012,
  SDLK_KP_B = 1073742013,
  SDLK_KP_C = 1073742014,
  SDLK_KP_D = 1073742015,
  SDLK_KP_E = 1073742016,
  SDLK_KP_F = 1073742017,
  SDLK_KP_XOR = 1073742018,
  SDLK_KP_POWER = 1073742019,
  SDLK_KP_PERCENT = 1073742020,
  SDLK_KP_LESS = 1073742021,
  SDLK_KP_GREATER = 1073742022,
  SDLK_KP_AMPERSAND = 1073742023,
  SDLK_KP_DBLAMPERSAND = 1073742024,
  SDLK_KP_VERTICALBAR = 1073742025,
  SDLK_KP_DBLVERTICALBAR = 1073742026,
  SDLK_KP_COLON = 1073742027,
  SDLK_KP_HASH = 1073742028,
  SDLK_KP_SPACE = 1073742029,
  SDLK_KP_AT = 1073742030,
  SDLK_KP_EXCLAM = 1073742031,
  SDLK_KP_MEMSTORE = 1073742032,
  SDLK_KP_MEMRECALL = 1073742033,
  SDLK_KP_MEMCLEAR = 1073742034,
  SDLK_KP_MEMADD = 1073742035,
  SDLK_KP_MEMSUBTRACT = 1073742036,
  SDLK_KP_MEMMULTIPLY = 1073742037,
  SDLK_KP_MEMDIVIDE = 1073742038,
  SDLK_KP_PLUSMINUS = 1073742039,
  SDLK_KP_CLEAR = 1073742040,
  SDLK_KP_CLEARENTRY = 1073742041,
  SDLK_KP_BINARY = 1073742042,
  SDLK_KP_OCTAL = 1073742043,
  SDLK_KP_DECIMAL = 1073742044,
  SDLK_KP_HEXADECIMAL = 1073742045,
  SDLK_LCTRL = 1073742048,
  SDLK_LSHIFT = 1073742049,
  SDLK_LALT = 1073742050,
  SDLK_LGUI = 1073742051,
  SDLK_RCTRL = 1073742052,
  SDLK_RSHIFT = 1073742053,
  SDLK_RALT = 1073742054,
  SDLK_RGUI = 1073742055,
  SDLK_MODE = 1073742081,
  SDLK_AUDIONEXT = 1073742082,
  SDLK_AUDIOPREV = 1073742083,
  SDLK_AUDIOSTOP = 1073742084,
  SDLK_AUDIOPLAY = 1073742085,
  SDLK_AUDIOMUTE = 1073742086,
  SDLK_MEDIASELECT = 1073742087,
  SDLK_WWW = 1073742088,
  SDLK_MAIL = 1073742089,
  SDLK_CALCULATOR = 1073742090,
  SDLK_COMPUTER = 1073742091,
  SDLK_AC_SEARCH = 1073742092,
  SDLK_AC_HOME = 1073742093,
  SDLK_AC_BACK = 1073742094,
  SDLK_AC_FORWARD = 1073742095,
  SDLK_AC_STOP = 1073742096,
  SDLK_AC_REFRESH = 1073742097,
  SDLK_AC_BOOKMARKS = 1073742098,
  SDLK_BRIGHTNESSDOWN = 1073742099,
  SDLK_BRIGHTNESSUP = 1073742100,
  SDLK_DISPLAYSWITCH = 1073742101,
  SDLK_KBDILLUMTOGGLE = 1073742102,
  SDLK_KBDILLUMDOWN = 1073742103,
  SDLK_KBDILLUMUP = 1073742104,
  SDLK_EJECT = 1073742105,
  SDLK_SLEEP = 1073742106,
  SDLK_APP1 = 1073742107,
  SDLK_APP2 = 1073742108,
  SDLK_AUDIOREWIND = 1073742109,
  SDLK_AUDIOFASTFORWARD = 1073742110,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SDL_Keysym {
  pub scancode: SDL_Scancode,
  pub sym: SDL_Keycode,
  pub mod_: u16,
  pub unused: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SDL_version {
  pub major: u8,
  pub minor: u8,
  pub patch: u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union SDL_Event {
  pub type_: u32,
  pub common: SDL_CommonEvent,
  pub display: SDL_DisplayEvent,
  pub window: SDL_WindowEvent,
  pub key: SDL_KeyboardEvent,
  pub edit: SDL_TextEditingEvent,
  pub text: SDL_TextInputEvent,
  pub motion: SDL_MouseMotionEvent,
  pub button: SDL_MouseButtonEvent,
  pub wheel: SDL_MouseWheelEvent,
  pub jaxis: SDL_JoyAxisEvent,
  pub jball: SDL_JoyBallEvent,
  pub jhat: SDL_JoyHatEvent,
  pub jbutton: SDL_JoyButtonEvent,
  pub jdevice: SDL_JoyDeviceEvent,
  pub caxis: SDL_ControllerAxisEvent,
  pub cbutton: SDL_ControllerButtonEvent,
  pub cdevice: SDL_ControllerDeviceEvent,
  pub adevice: SDL_AudioDeviceEvent,
  pub sensor: SDL_SensorEvent,
  pub quit: SDL_QuitEvent,
  pub user: SDL_UserEvent,
  // pub syswm: SDL_SysWMEvent,
  pub tfinger: SDL_TouchFingerEvent,
  pub mgesture: SDL_MultiGestureEvent,
  pub dgesture: SDL_DollarGestureEvent,
  pub drop: SDL_DropEvent,
  pub padding: [u8; 56],
  _union_align: [u64; 7],
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_CommonEvent {
  pub type_: u32,
  pub timestamp: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_DisplayEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub display: u32,
  pub event: u8,
  pub padding1: u8,
  pub padding2: u8,
  pub padding3: u8,
  pub data1: i32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_WindowEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub windowID: u32,
  pub event: u8,
  pub padding1: u8,
  pub padding2: u8,
  pub padding3: u8,
  pub data1: i32,
  pub data2: i32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_KeyboardEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub windowID: u32,
  pub state: u8,
  pub repeat: u8,
  pub padding2: u8,
  pub padding3: u8,
  pub keysym: SDL_Keysym,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_TextEditingEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub windowID: u32,
  pub text: [i8; 32],
  pub start: i32,
  pub length: i32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_TextInputEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub windowID: u32,
  pub text: [i8; 32],
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_MouseMotionEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub windowID: u32,
  pub which: u32,
  pub state: u32,
  pub x: i32,
  pub y: i32,
  pub xrel: i32,
  pub yrel: i32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_MouseButtonEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub windowID: u32,
  pub which: u32,
  pub button: u8,
  pub state: u8,
  pub clicks: u8,
  pub padding1: u8,
  pub x: i32,
  pub y: i32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_MouseWheelEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub windowID: u32,
  pub which: u32,
  pub x: i32,
  pub y: i32,
  pub direction: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_JoyAxisEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub which: SDL_JoystickID,
  pub axis: u8,
  pub padding1: u8,
  pub padding2: u8,
  pub padding3: u8,
  pub value: i16,
  pub padding4: u16,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_JoyBallEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub which: SDL_JoystickID,
  pub ball: u8,
  pub padding1: u8,
  pub padding2: u8,
  pub padding3: u8,
  pub xrel: i16,
  pub yrel: i16,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_JoyHatEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub which: SDL_JoystickID,
  pub hat: u8,
  pub value: u8,
  pub padding1: u8,
  pub padding2: u8,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_JoyButtonEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub which: SDL_JoystickID,
  pub button: u8,
  pub state: u8,
  pub padding1: u8,
  pub padding2: u8,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_JoyDeviceEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub which: i32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_ControllerAxisEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub which: SDL_JoystickID,
  pub axis: u8,
  pub padding1: u8,
  pub padding2: u8,
  pub padding3: u8,
  pub value: i16,
  pub padding4: u16,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_ControllerButtonEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub which: SDL_JoystickID,
  pub button: u8,
  pub state: u8,
  pub padding1: u8,
  pub padding2: u8,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_ControllerDeviceEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub which: i32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_AudioDeviceEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub which: u32,
  pub iscapture: u8,
  pub padding1: u8,
  pub padding2: u8,
  pub padding3: u8,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_SensorEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub which: i32,
  pub data: [f32; 6],
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_QuitEvent {
  pub type_: u32,
  pub timestamp: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_UserEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub windowID: u32,
  pub code: i32,
  pub data1: *mut c_void,
  pub data2: *mut c_void,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_MultiGestureEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub touchId: SDL_TouchID,
  pub dTheta: f32,
  pub dDist: f32,
  pub x: f32,
  pub y: f32,
  pub numFingers: u16,
  pub padding: u16,
}

// #[derive(Clone, Copy, Debug)]
// #[repr(C)]
// pub struct SDL_SysWMEvent {
//   pub type_: u32,
//   pub timestamp: u32,
//   pub msg: *mut SDL_SysWMmsg,
// }

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_TouchFingerEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub touchId: SDL_TouchID,
  pub fingerId: SDL_FingerID,
  pub x: f32,
  pub y: f32,
  pub dx: f32,
  pub dy: f32,
  pub pressure: f32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_DollarGestureEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub touchId: SDL_TouchID,
  pub gestureId: SDL_GestureID,
  pub numFingers: u32,
  pub error: f32,
  pub x: f32,
  pub y: f32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_DropEvent {
  pub type_: u32,
  pub timestamp: u32,
  pub file: *mut i8,
  pub windowID: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SDL_AudioSpec {
  pub freq: i32,
  pub format: SDL_AudioFormat,
  pub channels: u8,
  pub silence: u8,
  pub samples: u16,
  pub padding: u16,
  pub size: u32,
  pub callback: SDL_AudioCallback,
  pub userdata: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SDL_RWops {
  pub size: Option<unsafe extern "C" fn(context: *mut SDL_RWops) -> i64>,
  pub seek: Option<unsafe extern "C" fn(context: *mut SDL_RWops, offset: i64, whence: i32) -> i64>,
  pub read: Option<
    unsafe extern "C" fn(
      context: *mut SDL_RWops,
      ptr: *mut c_void,
      size: usize,
      maxnum: usize,
    ) -> usize,
  >,
  pub write: Option<
    unsafe extern "C" fn(
      context: *mut SDL_RWops,
      ptr: *const c_void,
      size: usize,
      num: usize,
    ) -> usize,
  >,
  pub close: Option<unsafe extern "C" fn(context: *mut SDL_RWops) -> i32>,
  pub type_: u32,
  pub hidden: __SDL_RWops,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union __SDL_RWops {
  pub mem: __SDL_RWops_mem,
  pub unknown: __SDL_RWops_unknown,
  _union_align: [u64; 3],
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct __SDL_RWops_mem {
  pub base: *mut u8,
  pub here: *mut u8,
  pub stop: *mut u8,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct __SDL_RWops_unknown {
  pub data1: *mut c_void,
  pub data2: *mut c_void,
}

#[link(name = "SDL2")]
extern "C" {
  // ===========================================================================
  // Init
  // ===========================================================================

  pub fn SDL_Init(flags: u32) -> i32;

  pub fn SDL_Quit();

  // ===========================================================================
  // Display and Window Management
  // ===========================================================================

  pub fn SDL_CreateWindow(
    title: *const i8,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    flags: u32,
  ) -> *mut SDL_Window;

  pub fn SDL_DestroyWindow(window: *mut SDL_Window);

  pub fn SDL_PollEvent(event: *mut SDL_Event) -> i32;

  // ===========================================================================
  // OpenGL
  // ===========================================================================

  pub fn SDL_GL_CreateContext(window: *mut SDL_Window) -> SDL_GLContext;

  pub fn SDL_GL_DeleteContext(context: SDL_GLContext);

  pub fn SDL_GL_GetSwapInterval() -> i32;

  pub fn SDL_GL_SetSwapInterval(interval: i32) -> i32;

  pub fn SDL_GL_GetAttribute(attr: GLattr, value: *mut i32) -> i32;

  pub fn SDL_GL_SetAttribute(attr: GLattr, value: i32) -> i32;

  pub fn SDL_GL_GetDrawableSize(window: *mut SDL_Window, w: *mut i32, h: *mut i32);

  // ===========================================================================
  // 2D Accelerated Rendering
  // ===========================================================================

  pub fn SDL_CreateRenderer(window: *mut SDL_Window, index: i32, flags: u32) -> *mut SDL_Renderer;

  pub fn SDL_RenderPresent(renderer: *mut SDL_Renderer);

  pub fn SDL_DestroyRenderer(renderer: *mut SDL_Renderer);

  pub fn SDL_SetRenderDrawColor(renderer: *mut SDL_Renderer, r: u8, g: u8, b: u8, a: u8) -> i32;

  pub fn SDL_RenderFillRect(renderer: *mut SDL_Renderer, rect: *const SDL_Rect) -> i32;

  pub fn SDL_RenderClear(renderer: *mut SDL_Renderer) -> i32;

  pub fn SDL_RenderCopy(
    renderer: *mut SDL_Renderer,
    texture: *mut SDL_Texture,
    srcrect: *const SDL_Rect,
    dstrect: *const SDL_Rect,
  ) -> i32;

  pub fn SDL_RenderDrawLine(renderer: *mut SDL_Renderer, x1: i32, y1: i32, x2: i32, y2: i32)
    -> i32;

  pub fn SDL_CreateTexture(
    renderer: *mut SDL_Renderer,
    format: u32,
    access: i32,
    w: i32,
    h: i32,
  ) -> *mut SDL_Texture;

  pub fn SDL_CreateTextureFromSurface(
    renderer: *mut SDL_Renderer,
    surface: *mut SDL_Surface,
  ) -> *mut SDL_Texture;

  pub fn SDL_DestroyTexture(texture: *mut SDL_Texture);

  pub fn SDL_FreeSurface(surface: *mut SDL_Surface);

  pub fn SDL_LoadBMP_RW(src: *mut SDL_RWops, freesrc: i32) -> *mut SDL_Surface;

  pub fn SDL_RWFromFile(file: *const i8, mode: *const i8) -> *mut SDL_RWops;

  pub fn SDL_RWFromConstMem(mem: *const c_void, size: i32) -> *mut SDL_RWops;

  pub fn SDL_MapRGB(format: *const SDL_PixelFormat, r: u8, g: u8, b: u8) -> u32;

  pub fn SDL_SetColorKey(surface: *mut SDL_Surface, flag: i32, key: u32) -> i32;

  // ===========================================================================
  // Error Handling
  // ===========================================================================

  pub fn SDL_GetError() -> *const i8;

  // ===========================================================================
  // Timer
  // ===========================================================================

  pub fn SDL_GetTicks() -> u32;

  pub fn SDL_Delay(millis: u32);

  // ===========================================================================
  // Audio
  // ===========================================================================

  pub fn SDL_OpenAudioDevice(
    device: *const i8,
    iscapture: i32,
    desired: *const SDL_AudioSpec,
    obtained: *mut SDL_AudioSpec,
    allowed_changes: i32,
  ) -> SDL_AudioDeviceID;

  pub fn SDL_CloseAudioDevice(device: SDL_AudioDeviceID);

  pub fn SDL_PauseAudioDevice(device: SDL_AudioDeviceID, pause_on: i32);

  pub fn SDL_QueueAudio(device: SDL_AudioDeviceID, data: *const c_void, len: u32) -> i32;

  pub fn SDL_GetNumAudioDevices(iscapture: i32) -> i32;

  pub fn SDL_GetQueuedAudioSize(device: SDL_AudioDeviceID) -> u32;

  // ===========================================================================
  // ???
  // ===========================================================================

  pub fn SDL_strlen(str: *const i8) -> usize;
}
