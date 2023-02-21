use crate::internal::Memory::*;
use glam::Vec3;
use libc;
pub type uchar = libc::c_uchar;
pub type Key = uchar;
pub const SDL_SCANCODE_A: C2RustUnnamed = 4;
pub const SDL_SCANCODE_B: C2RustUnnamed = 5;
pub const SDL_SCANCODE_C: C2RustUnnamed = 6;
pub const SDL_SCANCODE_D: C2RustUnnamed = 7;
pub const SDL_SCANCODE_E: C2RustUnnamed = 8;
pub const SDL_SCANCODE_F: C2RustUnnamed = 9;
pub const SDL_SCANCODE_G: C2RustUnnamed = 10;
pub const SDL_SCANCODE_H: C2RustUnnamed = 11;
pub const SDL_SCANCODE_I: C2RustUnnamed = 12;
pub const SDL_SCANCODE_J: C2RustUnnamed = 13;
pub const SDL_SCANCODE_K: C2RustUnnamed = 14;
pub const SDL_SCANCODE_L: C2RustUnnamed = 15;
pub const SDL_SCANCODE_M: C2RustUnnamed = 16;
pub const SDL_SCANCODE_N: C2RustUnnamed = 17;
pub const SDL_SCANCODE_O: C2RustUnnamed = 18;
pub const SDL_SCANCODE_P: C2RustUnnamed = 19;
pub const SDL_SCANCODE_Q: C2RustUnnamed = 20;
pub const SDL_SCANCODE_R: C2RustUnnamed = 21;
pub const SDL_SCANCODE_S: C2RustUnnamed = 22;
pub const SDL_SCANCODE_T: C2RustUnnamed = 23;
pub const SDL_SCANCODE_U: C2RustUnnamed = 24;
pub const SDL_SCANCODE_V: C2RustUnnamed = 25;
pub const SDL_SCANCODE_W: C2RustUnnamed = 26;
pub const SDL_SCANCODE_X: C2RustUnnamed = 27;
pub const SDL_SCANCODE_Y: C2RustUnnamed = 28;
pub const SDL_SCANCODE_Z: C2RustUnnamed = 29;
pub const SDL_SCANCODE_0: C2RustUnnamed = 39;
pub const SDL_SCANCODE_1: C2RustUnnamed = 30;
pub const SDL_SCANCODE_2: C2RustUnnamed = 31;
pub const SDL_SCANCODE_3: C2RustUnnamed = 32;
pub const SDL_SCANCODE_4: C2RustUnnamed = 33;
pub const SDL_SCANCODE_5: C2RustUnnamed = 34;
pub const SDL_SCANCODE_6: C2RustUnnamed = 35;
pub const SDL_SCANCODE_7: C2RustUnnamed = 36;
pub const SDL_SCANCODE_8: C2RustUnnamed = 37;
pub const SDL_SCANCODE_9: C2RustUnnamed = 38;
pub const SDL_SCANCODE_F1: C2RustUnnamed = 58;
pub const SDL_SCANCODE_F2: C2RustUnnamed = 59;
pub const SDL_SCANCODE_F3: C2RustUnnamed = 60;
pub const SDL_SCANCODE_F4: C2RustUnnamed = 61;
pub const SDL_SCANCODE_F5: C2RustUnnamed = 62;
pub const SDL_SCANCODE_F6: C2RustUnnamed = 63;
pub const SDL_SCANCODE_F7: C2RustUnnamed = 64;
pub const SDL_SCANCODE_F8: C2RustUnnamed = 65;
pub const SDL_SCANCODE_F9: C2RustUnnamed = 66;
pub const SDL_SCANCODE_F10: C2RustUnnamed = 67;
pub const SDL_SCANCODE_F11: C2RustUnnamed = 68;
pub const SDL_SCANCODE_F12: C2RustUnnamed = 69;
pub const SDL_SCANCODE_F13: C2RustUnnamed = 104;
pub const SDL_SCANCODE_F14: C2RustUnnamed = 105;
pub const SDL_SCANCODE_F15: C2RustUnnamed = 106;
pub const SDL_SCANCODE_F16: C2RustUnnamed = 107;
pub const SDL_SCANCODE_F17: C2RustUnnamed = 108;
pub const SDL_SCANCODE_F18: C2RustUnnamed = 109;
pub const SDL_SCANCODE_F19: C2RustUnnamed = 110;
pub const SDL_SCANCODE_F20: C2RustUnnamed = 111;
pub const SDL_SCANCODE_F21: C2RustUnnamed = 112;
pub const SDL_SCANCODE_F22: C2RustUnnamed = 113;
pub const SDL_SCANCODE_F23: C2RustUnnamed = 114;
pub const SDL_SCANCODE_F24: C2RustUnnamed = 115;
pub const SDL_SCANCODE_KP_0: C2RustUnnamed = 98;
pub const SDL_SCANCODE_KP_1: C2RustUnnamed = 89;
pub const SDL_SCANCODE_KP_2: C2RustUnnamed = 90;
pub const SDL_SCANCODE_KP_3: C2RustUnnamed = 91;
pub const SDL_SCANCODE_KP_4: C2RustUnnamed = 92;
pub const SDL_SCANCODE_KP_5: C2RustUnnamed = 93;
pub const SDL_SCANCODE_KP_6: C2RustUnnamed = 94;
pub const SDL_SCANCODE_KP_7: C2RustUnnamed = 95;
pub const SDL_SCANCODE_KP_8: C2RustUnnamed = 96;
pub const SDL_SCANCODE_KP_9: C2RustUnnamed = 97;
pub const SDL_SCANCODE_NUMLOCKCLEAR: C2RustUnnamed = 83;
pub const SDL_SCANCODE_KP_DIVIDE: C2RustUnnamed = 84;
pub const SDL_SCANCODE_KP_MULTIPLY: C2RustUnnamed = 85;
pub const SDL_SCANCODE_KP_MINUS: C2RustUnnamed = 86;
pub const SDL_SCANCODE_KP_PLUS: C2RustUnnamed = 87;
pub const SDL_SCANCODE_KP_ENTER: C2RustUnnamed = 88;
pub const SDL_SCANCODE_KP_DECIMAL: C2RustUnnamed = 220;
pub const SDL_SCANCODE_BACKSPACE: C2RustUnnamed = 42;
pub const SDL_SCANCODE_ESCAPE: C2RustUnnamed = 41;
pub const SDL_SCANCODE_RETURN: C2RustUnnamed = 40;
pub const SDL_SCANCODE_SPACE: C2RustUnnamed = 44;
pub const SDL_SCANCODE_TAB: C2RustUnnamed = 43;
pub const SDL_SCANCODE_GRAVE: C2RustUnnamed = 53;
pub const SDL_SCANCODE_CAPSLOCK: C2RustUnnamed = 57;
pub const SDL_SCANCODE_MINUS: C2RustUnnamed = 45;
pub const SDL_SCANCODE_EQUALS: C2RustUnnamed = 46;
pub const SDL_SCANCODE_LEFTBRACKET: C2RustUnnamed = 47;
pub const SDL_SCANCODE_RIGHTBRACKET: C2RustUnnamed = 48;
pub const SDL_SCANCODE_BACKSLASH: C2RustUnnamed = 49;
pub const SDL_SCANCODE_SEMICOLON: C2RustUnnamed = 51;
pub const SDL_SCANCODE_APOSTROPHE: C2RustUnnamed = 52;
pub const SDL_SCANCODE_COMMA: C2RustUnnamed = 54;
pub const SDL_SCANCODE_PERIOD: C2RustUnnamed = 55;
pub const SDL_SCANCODE_SLASH: C2RustUnnamed = 56;
pub const SDL_SCANCODE_PRINTSCREEN: C2RustUnnamed = 70;
pub const SDL_SCANCODE_SCROLLLOCK: C2RustUnnamed = 71;
pub const SDL_SCANCODE_PAUSE: C2RustUnnamed = 72;
pub const SDL_SCANCODE_INSERT: C2RustUnnamed = 73;
pub const SDL_SCANCODE_HOME: C2RustUnnamed = 74;
pub const SDL_SCANCODE_PAGEUP: C2RustUnnamed = 75;
pub const SDL_SCANCODE_PAGEDOWN: C2RustUnnamed = 78;
pub const SDL_SCANCODE_DELETE: C2RustUnnamed = 76;
pub const SDL_SCANCODE_RIGHT: C2RustUnnamed = 79;
pub const SDL_SCANCODE_LEFT: C2RustUnnamed = 80;
pub const SDL_SCANCODE_DOWN: C2RustUnnamed = 81;
pub const SDL_SCANCODE_UP: C2RustUnnamed = 82;
pub const SDL_SCANCODE_LCTRL: C2RustUnnamed = 224;
pub const SDL_SCANCODE_LSHIFT: C2RustUnnamed = 225;
pub const SDL_SCANCODE_LALT: C2RustUnnamed = 226;
pub const SDL_SCANCODE_LGUI: C2RustUnnamed = 227;
pub const SDL_SCANCODE_RCTRL: C2RustUnnamed = 228;
pub const SDL_SCANCODE_RSHIFT: C2RustUnnamed = 229;
pub const SDL_SCANCODE_RALT: C2RustUnnamed = 230;
pub const SDL_SCANCODE_RGUI: C2RustUnnamed = 231;
pub type C2RustUnnamed = u32;
pub const SDL_NUM_SCANCODES: C2RustUnnamed = 512;
pub const SDL_SCANCODE_ENDCALL: C2RustUnnamed = 290;
pub const SDL_SCANCODE_CALL: C2RustUnnamed = 289;
pub const SDL_SCANCODE_SOFTRIGHT: C2RustUnnamed = 288;
pub const SDL_SCANCODE_SOFTLEFT: C2RustUnnamed = 287;
pub const SDL_SCANCODE_AUDIOFASTFORWARD: C2RustUnnamed = 286;
pub const SDL_SCANCODE_AUDIOREWIND: C2RustUnnamed = 285;
pub const SDL_SCANCODE_APP2: C2RustUnnamed = 284;
pub const SDL_SCANCODE_APP1: C2RustUnnamed = 283;
pub const SDL_SCANCODE_SLEEP: C2RustUnnamed = 282;
pub const SDL_SCANCODE_EJECT: C2RustUnnamed = 281;
pub const SDL_SCANCODE_KBDILLUMUP: C2RustUnnamed = 280;
pub const SDL_SCANCODE_KBDILLUMDOWN: C2RustUnnamed = 279;
pub const SDL_SCANCODE_KBDILLUMTOGGLE: C2RustUnnamed = 278;
pub const SDL_SCANCODE_DISPLAYSWITCH: C2RustUnnamed = 277;
pub const SDL_SCANCODE_BRIGHTNESSUP: C2RustUnnamed = 276;
pub const SDL_SCANCODE_BRIGHTNESSDOWN: C2RustUnnamed = 275;
pub const SDL_SCANCODE_AC_BOOKMARKS: C2RustUnnamed = 274;
pub const SDL_SCANCODE_AC_REFRESH: C2RustUnnamed = 273;
pub const SDL_SCANCODE_AC_STOP: C2RustUnnamed = 272;
pub const SDL_SCANCODE_AC_FORWARD: C2RustUnnamed = 271;
pub const SDL_SCANCODE_AC_BACK: C2RustUnnamed = 270;
pub const SDL_SCANCODE_AC_HOME: C2RustUnnamed = 269;
pub const SDL_SCANCODE_AC_SEARCH: C2RustUnnamed = 268;
pub const SDL_SCANCODE_COMPUTER: C2RustUnnamed = 267;
pub const SDL_SCANCODE_CALCULATOR: C2RustUnnamed = 266;
pub const SDL_SCANCODE_MAIL: C2RustUnnamed = 265;
pub const SDL_SCANCODE_WWW: C2RustUnnamed = 264;
pub const SDL_SCANCODE_MEDIASELECT: C2RustUnnamed = 263;
pub const SDL_SCANCODE_AUDIOMUTE: C2RustUnnamed = 262;
pub const SDL_SCANCODE_AUDIOPLAY: C2RustUnnamed = 261;
pub const SDL_SCANCODE_AUDIOSTOP: C2RustUnnamed = 260;
pub const SDL_SCANCODE_AUDIOPREV: C2RustUnnamed = 259;
pub const SDL_SCANCODE_AUDIONEXT: C2RustUnnamed = 258;
pub const SDL_SCANCODE_MODE: C2RustUnnamed = 257;
pub const SDL_SCANCODE_KP_HEXADECIMAL: C2RustUnnamed = 221;
pub const SDL_SCANCODE_KP_OCTAL: C2RustUnnamed = 219;
pub const SDL_SCANCODE_KP_BINARY: C2RustUnnamed = 218;
pub const SDL_SCANCODE_KP_CLEARENTRY: C2RustUnnamed = 217;
pub const SDL_SCANCODE_KP_CLEAR: C2RustUnnamed = 216;
pub const SDL_SCANCODE_KP_PLUSMINUS: C2RustUnnamed = 215;
pub const SDL_SCANCODE_KP_MEMDIVIDE: C2RustUnnamed = 214;
pub const SDL_SCANCODE_KP_MEMMULTIPLY: C2RustUnnamed = 213;
pub const SDL_SCANCODE_KP_MEMSUBTRACT: C2RustUnnamed = 212;
pub const SDL_SCANCODE_KP_MEMADD: C2RustUnnamed = 211;
pub const SDL_SCANCODE_KP_MEMCLEAR: C2RustUnnamed = 210;
pub const SDL_SCANCODE_KP_MEMRECALL: C2RustUnnamed = 209;
pub const SDL_SCANCODE_KP_MEMSTORE: C2RustUnnamed = 208;
pub const SDL_SCANCODE_KP_EXCLAM: C2RustUnnamed = 207;
pub const SDL_SCANCODE_KP_AT: C2RustUnnamed = 206;
pub const SDL_SCANCODE_KP_SPACE: C2RustUnnamed = 205;
pub const SDL_SCANCODE_KP_HASH: C2RustUnnamed = 204;
pub const SDL_SCANCODE_KP_COLON: C2RustUnnamed = 203;
pub const SDL_SCANCODE_KP_DBLVERTICALBAR: C2RustUnnamed = 202;
pub const SDL_SCANCODE_KP_VERTICALBAR: C2RustUnnamed = 201;
pub const SDL_SCANCODE_KP_DBLAMPERSAND: C2RustUnnamed = 200;
pub const SDL_SCANCODE_KP_AMPERSAND: C2RustUnnamed = 199;
pub const SDL_SCANCODE_KP_GREATER: C2RustUnnamed = 198;
pub const SDL_SCANCODE_KP_LESS: C2RustUnnamed = 197;
pub const SDL_SCANCODE_KP_PERCENT: C2RustUnnamed = 196;
pub const SDL_SCANCODE_KP_POWER: C2RustUnnamed = 195;
pub const SDL_SCANCODE_KP_XOR: C2RustUnnamed = 194;
pub const SDL_SCANCODE_KP_F: C2RustUnnamed = 193;
pub const SDL_SCANCODE_KP_E: C2RustUnnamed = 192;
pub const SDL_SCANCODE_KP_D: C2RustUnnamed = 191;
pub const SDL_SCANCODE_KP_C: C2RustUnnamed = 190;
pub const SDL_SCANCODE_KP_B: C2RustUnnamed = 189;
pub const SDL_SCANCODE_KP_A: C2RustUnnamed = 188;
pub const SDL_SCANCODE_KP_BACKSPACE: C2RustUnnamed = 187;
pub const SDL_SCANCODE_KP_TAB: C2RustUnnamed = 186;
pub const SDL_SCANCODE_KP_RIGHTBRACE: C2RustUnnamed = 185;
pub const SDL_SCANCODE_KP_LEFTBRACE: C2RustUnnamed = 184;
pub const SDL_SCANCODE_KP_RIGHTPAREN: C2RustUnnamed = 183;
pub const SDL_SCANCODE_KP_LEFTPAREN: C2RustUnnamed = 182;
pub const SDL_SCANCODE_CURRENCYSUBUNIT: C2RustUnnamed = 181;
pub const SDL_SCANCODE_CURRENCYUNIT: C2RustUnnamed = 180;
pub const SDL_SCANCODE_DECIMALSEPARATOR: C2RustUnnamed = 179;
pub const SDL_SCANCODE_THOUSANDSSEPARATOR: C2RustUnnamed = 178;
pub const SDL_SCANCODE_KP_000: C2RustUnnamed = 177;
pub const SDL_SCANCODE_KP_00: C2RustUnnamed = 176;
pub const SDL_SCANCODE_EXSEL: C2RustUnnamed = 164;
pub const SDL_SCANCODE_CRSEL: C2RustUnnamed = 163;
pub const SDL_SCANCODE_CLEARAGAIN: C2RustUnnamed = 162;
pub const SDL_SCANCODE_OPER: C2RustUnnamed = 161;
pub const SDL_SCANCODE_OUT: C2RustUnnamed = 160;
pub const SDL_SCANCODE_SEPARATOR: C2RustUnnamed = 159;
pub const SDL_SCANCODE_RETURN2: C2RustUnnamed = 158;
pub const SDL_SCANCODE_PRIOR: C2RustUnnamed = 157;
pub const SDL_SCANCODE_CLEAR: C2RustUnnamed = 156;
pub const SDL_SCANCODE_CANCEL: C2RustUnnamed = 155;
pub const SDL_SCANCODE_SYSREQ: C2RustUnnamed = 154;
pub const SDL_SCANCODE_ALTERASE: C2RustUnnamed = 153;
pub const SDL_SCANCODE_LANG9: C2RustUnnamed = 152;
pub const SDL_SCANCODE_LANG8: C2RustUnnamed = 151;
pub const SDL_SCANCODE_LANG7: C2RustUnnamed = 150;
pub const SDL_SCANCODE_LANG6: C2RustUnnamed = 149;
pub const SDL_SCANCODE_LANG5: C2RustUnnamed = 148;
pub const SDL_SCANCODE_LANG4: C2RustUnnamed = 147;
pub const SDL_SCANCODE_LANG3: C2RustUnnamed = 146;
pub const SDL_SCANCODE_LANG2: C2RustUnnamed = 145;
pub const SDL_SCANCODE_LANG1: C2RustUnnamed = 144;
pub const SDL_SCANCODE_INTERNATIONAL9: C2RustUnnamed = 143;
pub const SDL_SCANCODE_INTERNATIONAL8: C2RustUnnamed = 142;
pub const SDL_SCANCODE_INTERNATIONAL7: C2RustUnnamed = 141;
pub const SDL_SCANCODE_INTERNATIONAL6: C2RustUnnamed = 140;
pub const SDL_SCANCODE_INTERNATIONAL5: C2RustUnnamed = 139;
pub const SDL_SCANCODE_INTERNATIONAL4: C2RustUnnamed = 138;
pub const SDL_SCANCODE_INTERNATIONAL3: C2RustUnnamed = 137;
pub const SDL_SCANCODE_INTERNATIONAL2: C2RustUnnamed = 136;
pub const SDL_SCANCODE_INTERNATIONAL1: C2RustUnnamed = 135;
pub const SDL_SCANCODE_KP_EQUALSAS400: C2RustUnnamed = 134;
pub const SDL_SCANCODE_KP_COMMA: C2RustUnnamed = 133;
pub const SDL_SCANCODE_VOLUMEDOWN: C2RustUnnamed = 129;
pub const SDL_SCANCODE_VOLUMEUP: C2RustUnnamed = 128;
pub const SDL_SCANCODE_MUTE: C2RustUnnamed = 127;
pub const SDL_SCANCODE_FIND: C2RustUnnamed = 126;
pub const SDL_SCANCODE_PASTE: C2RustUnnamed = 125;
pub const SDL_SCANCODE_COPY: C2RustUnnamed = 124;
pub const SDL_SCANCODE_CUT: C2RustUnnamed = 123;
pub const SDL_SCANCODE_UNDO: C2RustUnnamed = 122;
pub const SDL_SCANCODE_AGAIN: C2RustUnnamed = 121;
pub const SDL_SCANCODE_STOP: C2RustUnnamed = 120;
pub const SDL_SCANCODE_SELECT: C2RustUnnamed = 119;
pub const SDL_SCANCODE_MENU: C2RustUnnamed = 118;
pub const SDL_SCANCODE_HELP: C2RustUnnamed = 117;
pub const SDL_SCANCODE_EXECUTE: C2RustUnnamed = 116;
pub const SDL_SCANCODE_KP_EQUALS: C2RustUnnamed = 103;
pub const SDL_SCANCODE_POWER: C2RustUnnamed = 102;
pub const SDL_SCANCODE_APPLICATION: C2RustUnnamed = 101;
pub const SDL_SCANCODE_NONUSBACKSLASH: C2RustUnnamed = 100;
pub const SDL_SCANCODE_KP_PERIOD: C2RustUnnamed = 99;
pub const SDL_SCANCODE_END: C2RustUnnamed = 77;
pub const SDL_SCANCODE_NONUSHASH: C2RustUnnamed = 50;
pub const SDL_SCANCODE_UNKNOWN: C2RustUnnamed = 0;
#[no_mangle]
pub static mut Key_A: Key = SDL_SCANCODE_A as i32 as Key;
#[no_mangle]
pub static mut Key_B: Key = SDL_SCANCODE_B as i32 as Key;
#[no_mangle]
pub static mut Key_C: Key = SDL_SCANCODE_C as i32 as Key;
#[no_mangle]
pub static mut Key_D: Key = SDL_SCANCODE_D as i32 as Key;
#[no_mangle]
pub static mut Key_E: Key = SDL_SCANCODE_E as i32 as Key;
#[no_mangle]
pub static mut Key_F: Key = SDL_SCANCODE_F as i32 as Key;
#[no_mangle]
pub static mut Key_G: Key = SDL_SCANCODE_G as i32 as Key;
#[no_mangle]
pub static mut Key_H: Key = SDL_SCANCODE_H as i32 as Key;
#[no_mangle]
pub static mut Key_I: Key = SDL_SCANCODE_I as i32 as Key;
#[no_mangle]
pub static mut Key_J: Key = SDL_SCANCODE_J as i32 as Key;
#[no_mangle]
pub static mut Key_K: Key = SDL_SCANCODE_K as i32 as Key;
#[no_mangle]
pub static mut Key_L: Key = SDL_SCANCODE_L as i32 as Key;
#[no_mangle]
pub static mut Key_M: Key = SDL_SCANCODE_M as i32 as Key;
#[no_mangle]
pub static mut Key_N: Key = SDL_SCANCODE_N as i32 as Key;
#[no_mangle]
pub static mut Key_O: Key = SDL_SCANCODE_O as i32 as Key;
#[no_mangle]
pub static mut Key_P: Key = SDL_SCANCODE_P as i32 as Key;
#[no_mangle]
pub static mut Key_Q: Key = SDL_SCANCODE_Q as i32 as Key;
#[no_mangle]
pub static mut Key_R: Key = SDL_SCANCODE_R as i32 as Key;
#[no_mangle]
pub static mut Key_S: Key = SDL_SCANCODE_S as i32 as Key;
#[no_mangle]
pub static mut Key_T: Key = SDL_SCANCODE_T as i32 as Key;
#[no_mangle]
pub static mut Key_U: Key = SDL_SCANCODE_U as i32 as Key;
#[no_mangle]
pub static mut Key_V: Key = SDL_SCANCODE_V as i32 as Key;
#[no_mangle]
pub static mut Key_W: Key = SDL_SCANCODE_W as i32 as Key;
#[no_mangle]
pub static mut Key_X: Key = SDL_SCANCODE_X as i32 as Key;
#[no_mangle]
pub static mut Key_Y: Key = SDL_SCANCODE_Y as i32 as Key;
#[no_mangle]
pub static mut Key_Z: Key = SDL_SCANCODE_Z as i32 as Key;
#[no_mangle]
pub static mut Key_N0: Key = SDL_SCANCODE_0 as i32 as Key;
#[no_mangle]
pub static mut Key_N1: Key = SDL_SCANCODE_1 as i32 as Key;
#[no_mangle]
pub static mut Key_N2: Key = SDL_SCANCODE_2 as i32 as Key;
#[no_mangle]
pub static mut Key_N3: Key = SDL_SCANCODE_3 as i32 as Key;
#[no_mangle]
pub static mut Key_N4: Key = SDL_SCANCODE_4 as i32 as Key;
#[no_mangle]
pub static mut Key_N5: Key = SDL_SCANCODE_5 as i32 as Key;
#[no_mangle]
pub static mut Key_N6: Key = SDL_SCANCODE_6 as i32 as Key;
#[no_mangle]
pub static mut Key_N7: Key = SDL_SCANCODE_7 as i32 as Key;
#[no_mangle]
pub static mut Key_N8: Key = SDL_SCANCODE_8 as i32 as Key;
#[no_mangle]
pub static mut Key_N9: Key = SDL_SCANCODE_9 as i32 as Key;
#[no_mangle]
pub static mut Key_F1: Key = SDL_SCANCODE_F1 as i32 as Key;
#[no_mangle]
pub static mut Key_F2: Key = SDL_SCANCODE_F2 as i32 as Key;
#[no_mangle]
pub static mut Key_F3: Key = SDL_SCANCODE_F3 as i32 as Key;
#[no_mangle]
pub static mut Key_F4: Key = SDL_SCANCODE_F4 as i32 as Key;
#[no_mangle]
pub static mut Key_F5: Key = SDL_SCANCODE_F5 as i32 as Key;
#[no_mangle]
pub static mut Key_F6: Key = SDL_SCANCODE_F6 as i32 as Key;
#[no_mangle]
pub static mut Key_F7: Key = SDL_SCANCODE_F7 as i32 as Key;
#[no_mangle]
pub static mut Key_F8: Key = SDL_SCANCODE_F8 as i32 as Key;
#[no_mangle]
pub static mut Key_F9: Key = SDL_SCANCODE_F9 as i32 as Key;
#[no_mangle]
pub static mut Key_F10: Key = SDL_SCANCODE_F10 as i32 as Key;
#[no_mangle]
pub static mut Key_F11: Key = SDL_SCANCODE_F11 as i32 as Key;
#[no_mangle]
pub static mut Key_F12: Key = SDL_SCANCODE_F12 as i32 as Key;
#[no_mangle]
pub static mut Key_F13: Key = SDL_SCANCODE_F13 as i32 as Key;
#[no_mangle]
pub static mut Key_F14: Key = SDL_SCANCODE_F14 as i32 as Key;
#[no_mangle]
pub static mut Key_F15: Key = SDL_SCANCODE_F15 as i32 as Key;
#[no_mangle]
pub static mut Key_F16: Key = SDL_SCANCODE_F16 as i32 as Key;
#[no_mangle]
pub static mut Key_F17: Key = SDL_SCANCODE_F17 as i32 as Key;
#[no_mangle]
pub static mut Key_F18: Key = SDL_SCANCODE_F18 as i32 as Key;
#[no_mangle]
pub static mut Key_F19: Key = SDL_SCANCODE_F19 as i32 as Key;
#[no_mangle]
pub static mut Key_F20: Key = SDL_SCANCODE_F20 as i32 as Key;
#[no_mangle]
pub static mut Key_F21: Key = SDL_SCANCODE_F21 as i32 as Key;
#[no_mangle]
pub static mut Key_F22: Key = SDL_SCANCODE_F22 as i32 as Key;
#[no_mangle]
pub static mut Key_F23: Key = SDL_SCANCODE_F23 as i32 as Key;
#[no_mangle]
pub static mut Key_F24: Key = SDL_SCANCODE_F24 as i32 as Key;
#[no_mangle]
pub static mut Key_KP0: Key = SDL_SCANCODE_KP_0 as i32 as Key;
#[no_mangle]
pub static mut Key_KP1: Key = SDL_SCANCODE_KP_1 as i32 as Key;
#[no_mangle]
pub static mut Key_KP2: Key = SDL_SCANCODE_KP_2 as i32 as Key;
#[no_mangle]
pub static mut Key_KP3: Key = SDL_SCANCODE_KP_3 as i32 as Key;
#[no_mangle]
pub static mut Key_KP4: Key = SDL_SCANCODE_KP_4 as i32 as Key;
#[no_mangle]
pub static mut Key_KP5: Key = SDL_SCANCODE_KP_5 as i32 as Key;
#[no_mangle]
pub static mut Key_KP6: Key = SDL_SCANCODE_KP_6 as i32 as Key;
#[no_mangle]
pub static mut Key_KP7: Key = SDL_SCANCODE_KP_7 as i32 as Key;
#[no_mangle]
pub static mut Key_KP8: Key = SDL_SCANCODE_KP_8 as i32 as Key;
#[no_mangle]
pub static mut Key_KP9: Key = SDL_SCANCODE_KP_9 as i32 as Key;
#[no_mangle]
pub static mut Key_KPNumLock: Key = SDL_SCANCODE_NUMLOCKCLEAR as i32 as Key;
#[no_mangle]
pub static mut Key_KPDivide: Key = SDL_SCANCODE_KP_DIVIDE as i32 as Key;
#[no_mangle]
pub static mut Key_KPMultiply: Key = SDL_SCANCODE_KP_MULTIPLY as i32 as Key;
#[no_mangle]
pub static mut Key_KPSubtract: Key = SDL_SCANCODE_KP_MINUS as i32 as Key;
#[no_mangle]
pub static mut Key_KPAdd: Key = SDL_SCANCODE_KP_PLUS as i32 as Key;
#[no_mangle]
pub static mut Key_KPEnter: Key = SDL_SCANCODE_KP_ENTER as i32 as Key;
#[no_mangle]
pub static mut Key_KPDecimal: Key = SDL_SCANCODE_KP_DECIMAL as i32 as Key;
#[no_mangle]
pub static mut Key_Backspace: Key = SDL_SCANCODE_BACKSPACE as i32 as Key;
#[no_mangle]
pub static mut Key_Escape: Key = SDL_SCANCODE_ESCAPE as i32 as Key;
#[no_mangle]
pub static mut Key_Return: Key = SDL_SCANCODE_RETURN as i32 as Key;
#[no_mangle]
pub static mut Key_Space: Key = SDL_SCANCODE_SPACE as i32 as Key;
#[no_mangle]
pub static mut Key_Tab: Key = SDL_SCANCODE_TAB as i32 as Key;
#[no_mangle]
pub static mut Key_Backtick: Key = SDL_SCANCODE_GRAVE as i32 as Key;
#[no_mangle]
pub static mut Key_CapsLock: Key = SDL_SCANCODE_CAPSLOCK as i32 as Key;
#[no_mangle]
pub static mut Key_Minus: Key = SDL_SCANCODE_MINUS as i32 as Key;
#[no_mangle]
pub static mut Key_Equals: Key = SDL_SCANCODE_EQUALS as i32 as Key;
#[no_mangle]
pub static mut Key_LBracket: Key = SDL_SCANCODE_LEFTBRACKET as i32 as Key;
#[no_mangle]
pub static mut Key_RBracket: Key = SDL_SCANCODE_RIGHTBRACKET as i32 as Key;
#[no_mangle]
pub static mut Key_Backslash: Key = SDL_SCANCODE_BACKSLASH as i32 as Key;
#[no_mangle]
pub static mut Key_Semicolon: Key = SDL_SCANCODE_SEMICOLON as i32 as Key;
#[no_mangle]
pub static mut Key_Apostrophe: Key = SDL_SCANCODE_APOSTROPHE as i32 as Key;
#[no_mangle]
pub static mut Key_Comma: Key = SDL_SCANCODE_COMMA as i32 as Key;
#[no_mangle]
pub static mut Key_Period: Key = SDL_SCANCODE_PERIOD as i32 as Key;
#[no_mangle]
pub static mut Key_Slash: Key = SDL_SCANCODE_SLASH as i32 as Key;
#[no_mangle]
pub static mut Key_PrintScreen: Key = SDL_SCANCODE_PRINTSCREEN as i32 as Key;
#[no_mangle]
pub static mut Key_ScrollLock: Key = SDL_SCANCODE_SCROLLLOCK as i32 as Key;
#[no_mangle]
pub static mut Key_Pause: Key = SDL_SCANCODE_PAUSE as i32 as Key;
#[no_mangle]
pub static mut Key_Insert: Key = SDL_SCANCODE_INSERT as i32 as Key;
#[no_mangle]
pub static mut Key_Home: Key = SDL_SCANCODE_HOME as i32 as Key;
#[no_mangle]
pub static mut Key_PageUp: Key = SDL_SCANCODE_PAGEUP as i32 as Key;
#[no_mangle]
pub static mut Key_PageDown: Key = SDL_SCANCODE_PAGEDOWN as i32 as Key;
#[no_mangle]
pub static mut Key_Delete: Key = SDL_SCANCODE_DELETE as i32 as Key;
#[no_mangle]
pub static mut Key_Right: Key = SDL_SCANCODE_RIGHT as i32 as Key;
#[no_mangle]
pub static mut Key_Left: Key = SDL_SCANCODE_LEFT as i32 as Key;
#[no_mangle]
pub static mut Key_Down: Key = SDL_SCANCODE_DOWN as i32 as Key;
#[no_mangle]
pub static mut Key_Up: Key = SDL_SCANCODE_UP as i32 as Key;
#[no_mangle]
pub static mut Key_LCtrl: Key = SDL_SCANCODE_LCTRL as i32 as Key;
#[no_mangle]
pub static mut Key_LShift: Key = SDL_SCANCODE_LSHIFT as i32 as Key;
#[no_mangle]
pub static mut Key_LAlt: Key = SDL_SCANCODE_LALT as i32 as Key;
#[no_mangle]
pub static mut Key_LMeta: Key = SDL_SCANCODE_LGUI as i32 as Key;
#[no_mangle]
pub static mut Key_RCtrl: Key = SDL_SCANCODE_RCTRL as i32 as Key;
#[no_mangle]
pub static mut Key_RShift: Key = SDL_SCANCODE_RSHIFT as i32 as Key;
#[no_mangle]
pub static mut Key_RAlt: Key = SDL_SCANCODE_RALT as i32 as Key;
#[no_mangle]
pub static mut Key_RMeta: Key = SDL_SCANCODE_RGUI as i32 as Key;
