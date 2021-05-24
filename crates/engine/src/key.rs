pub type KeyCode = u16;
pub type KeyValue = i16;

pub const KEY_NONE: KeyCode = 0;

pub const KEY_A: KeyCode = 4;
pub const KEY_B: KeyCode = 5;
pub const KEY_C: KeyCode = 6;
pub const KEY_D: KeyCode = 7;
pub const KEY_E: KeyCode = 8;
pub const KEY_F: KeyCode = 9;
pub const KEY_G: KeyCode = 10;
pub const KEY_H: KeyCode = 11;
pub const KEY_I: KeyCode = 12;
pub const KEY_J: KeyCode = 13;
pub const KEY_K: KeyCode = 14;
pub const KEY_L: KeyCode = 15;
pub const KEY_M: KeyCode = 16;
pub const KEY_N: KeyCode = 17;
pub const KEY_O: KeyCode = 18;
pub const KEY_P: KeyCode = 19;
pub const KEY_Q: KeyCode = 20;
pub const KEY_R: KeyCode = 21;
pub const KEY_S: KeyCode = 22;
pub const KEY_T: KeyCode = 23;
pub const KEY_U: KeyCode = 24;
pub const KEY_V: KeyCode = 25;
pub const KEY_W: KeyCode = 26;
pub const KEY_X: KeyCode = 27;
pub const KEY_Y: KeyCode = 28;
pub const KEY_Z: KeyCode = 29;
pub const KEY_1: KeyCode = 30;
pub const KEY_2: KeyCode = 31;
pub const KEY_3: KeyCode = 32;
pub const KEY_4: KeyCode = 33;
pub const KEY_5: KeyCode = 34;
pub const KEY_6: KeyCode = 35;
pub const KEY_7: KeyCode = 36;
pub const KEY_8: KeyCode = 37;
pub const KEY_9: KeyCode = 38;
pub const KEY_0: KeyCode = 39;
pub const KEY_RETURN: KeyCode = 40;
pub const KEY_ESCAPE: KeyCode = 41;
pub const KEY_BACKSPACE: KeyCode = 42;
pub const KEY_TAB: KeyCode = 43;
pub const KEY_SPACE: KeyCode = 44;
pub const KEY_MINUS: KeyCode = 45;
pub const KEY_EQUALS: KeyCode = 46;
pub const KEY_LEFTBRACKET: KeyCode = 47;
pub const KEY_RIGHTBRACKET: KeyCode = 48;
pub const KEY_BACKSLASH: KeyCode = 49;
pub const KEY_NONUSHASH: KeyCode = 50;
pub const KEY_SEMICOLON: KeyCode = 51;
pub const KEY_APOSTROPHE: KeyCode = 52;
pub const KEY_GRAVE: KeyCode = 53;
pub const KEY_COMMA: KeyCode = 54;
pub const KEY_PERIOD: KeyCode = 55;
pub const KEY_SLASH: KeyCode = 56;
pub const KEY_CAPSLOCK: KeyCode = 57;
pub const KEY_F1: KeyCode = 58;
pub const KEY_F2: KeyCode = 59;
pub const KEY_F3: KeyCode = 60;
pub const KEY_F4: KeyCode = 61;
pub const KEY_F5: KeyCode = 62;
pub const KEY_F6: KeyCode = 63;
pub const KEY_F7: KeyCode = 64;
pub const KEY_F8: KeyCode = 65;
pub const KEY_F9: KeyCode = 66;
pub const KEY_F10: KeyCode = 67;
pub const KEY_F11: KeyCode = 68;
pub const KEY_F12: KeyCode = 69;
pub const KEY_PRINTSCREEN: KeyCode = 70;
pub const KEY_SCROLLLOCK: KeyCode = 71;
pub const KEY_PAUSE: KeyCode = 72;
pub const KEY_INSERT: KeyCode = 73;
pub const KEY_HOME: KeyCode = 74;
pub const KEY_PAGEUP: KeyCode = 75;
pub const KEY_DELETE: KeyCode = 76;
pub const KEY_END: KeyCode = 77;
pub const KEY_PAGEDOWN: KeyCode = 78;
pub const KEY_RIGHT: KeyCode = 79;
pub const KEY_LEFT: KeyCode = 80;
pub const KEY_DOWN: KeyCode = 81;
pub const KEY_UP: KeyCode = 82;
pub const KEY_NUMLOCKCLEAR: KeyCode = 83;
pub const KEY_KP_DIVIDE: KeyCode = 84;
pub const KEY_KP_MULTIPLY: KeyCode = 85;
pub const KEY_KP_MINUS: KeyCode = 86;
pub const KEY_KP_PLUS: KeyCode = 87;
pub const KEY_KP_ENTER: KeyCode = 88;
pub const KEY_KP_1: KeyCode = 89;
pub const KEY_KP_2: KeyCode = 90;
pub const KEY_KP_3: KeyCode = 91;
pub const KEY_KP_4: KeyCode = 92;
pub const KEY_KP_5: KeyCode = 93;
pub const KEY_KP_6: KeyCode = 94;
pub const KEY_KP_7: KeyCode = 95;
pub const KEY_KP_8: KeyCode = 96;
pub const KEY_KP_9: KeyCode = 97;
pub const KEY_KP_0: KeyCode = 98;
pub const KEY_KP_PERIOD: KeyCode = 99;
pub const KEY_NONUSBACKSLASH: KeyCode = 100;
pub const KEY_APPLICATION: KeyCode = 101;
pub const KEY_POWER: KeyCode = 102;
pub const KEY_KP_EQUALS: KeyCode = 103;
pub const KEY_F13: KeyCode = 104;
pub const KEY_F14: KeyCode = 105;
pub const KEY_F15: KeyCode = 106;
pub const KEY_F16: KeyCode = 107;
pub const KEY_F17: KeyCode = 108;
pub const KEY_F18: KeyCode = 109;
pub const KEY_F19: KeyCode = 110;
pub const KEY_F20: KeyCode = 111;
pub const KEY_F21: KeyCode = 112;
pub const KEY_F22: KeyCode = 113;
pub const KEY_F23: KeyCode = 114;
pub const KEY_F24: KeyCode = 115;
pub const KEY_EXECUTE: KeyCode = 116;
pub const KEY_HELP: KeyCode = 117;
pub const KEY_MENU: KeyCode = 118;
pub const KEY_SELECT: KeyCode = 119;
pub const KEY_STOP: KeyCode = 120;
pub const KEY_AGAIN: KeyCode = 121;
pub const KEY_UNDO: KeyCode = 122;
pub const KEY_CUT: KeyCode = 123;
pub const KEY_COPY: KeyCode = 124;
pub const KEY_PASTE: KeyCode = 125;
pub const KEY_FIND: KeyCode = 126;
pub const KEY_MUTE: KeyCode = 127;
pub const KEY_VOLUMEUP: KeyCode = 128;
pub const KEY_VOLUMEDOWN: KeyCode = 129;
pub const KEY_KP_COMMA: KeyCode = 133;
pub const KEY_KP_EQUALSAS400: KeyCode = 134;
pub const KEY_INTERNATIONAL1: KeyCode = 135;
pub const KEY_INTERNATIONAL2: KeyCode = 136;
pub const KEY_INTERNATIONAL3: KeyCode = 137;
pub const KEY_INTERNATIONAL4: KeyCode = 138;
pub const KEY_INTERNATIONAL5: KeyCode = 139;
pub const KEY_INTERNATIONAL6: KeyCode = 140;
pub const KEY_INTERNATIONAL7: KeyCode = 141;
pub const KEY_INTERNATIONAL8: KeyCode = 142;
pub const KEY_INTERNATIONAL9: KeyCode = 143;
pub const KEY_LANG1: KeyCode = 144;
pub const KEY_LANG2: KeyCode = 145;
pub const KEY_LANG3: KeyCode = 146;
pub const KEY_LANG4: KeyCode = 147;
pub const KEY_LANG5: KeyCode = 148;
pub const KEY_LANG6: KeyCode = 149;
pub const KEY_LANG7: KeyCode = 150;
pub const KEY_LANG8: KeyCode = 151;
pub const KEY_LANG9: KeyCode = 152;
pub const KEY_ALTERASE: KeyCode = 153;
pub const KEY_SYSREQ: KeyCode = 154;
pub const KEY_CANCEL: KeyCode = 155;
pub const KEY_CLEAR: KeyCode = 156;
pub const KEY_PRIOR: KeyCode = 157;
pub const KEY_RETURN2: KeyCode = 158;
pub const KEY_SEPARATOR: KeyCode = 159;
pub const KEY_OUT: KeyCode = 160;
pub const KEY_OPER: KeyCode = 161;
pub const KEY_CLEARAGAIN: KeyCode = 162;
pub const KEY_CRSEL: KeyCode = 163;
pub const KEY_EXSEL: KeyCode = 164;
pub const KEY_KP_00: KeyCode = 176;
pub const KEY_KP_000: KeyCode = 177;
pub const KEY_THOUSANDSSEPARATOR: KeyCode = 178;
pub const KEY_DECIMALSEPARATOR: KeyCode = 179;
pub const KEY_CURRENCYUNIT: KeyCode = 180;
pub const KEY_CURRENCYSUBUNIT: KeyCode = 181;
pub const KEY_KP_LEFTPAREN: KeyCode = 182;
pub const KEY_KP_RIGHTPAREN: KeyCode = 183;
pub const KEY_KP_LEFTBRACE: KeyCode = 184;
pub const KEY_KP_RIGHTBRACE: KeyCode = 185;
pub const KEY_KP_TAB: KeyCode = 186;
pub const KEY_KP_BACKSPACE: KeyCode = 187;
pub const KEY_KP_A: KeyCode = 188;
pub const KEY_KP_B: KeyCode = 189;
pub const KEY_KP_C: KeyCode = 190;
pub const KEY_KP_D: KeyCode = 191;
pub const KEY_KP_E: KeyCode = 192;
pub const KEY_KP_F: KeyCode = 193;
pub const KEY_KP_XOR: KeyCode = 194;
pub const KEY_KP_POWER: KeyCode = 195;
pub const KEY_KP_PERCENT: KeyCode = 196;
pub const KEY_KP_LESS: KeyCode = 197;
pub const KEY_KP_GREATER: KeyCode = 198;
pub const KEY_KP_AMPERSAND: KeyCode = 199;
pub const KEY_KP_DBLAMPERSAND: KeyCode = 200;
pub const KEY_KP_VERTICALBAR: KeyCode = 201;
pub const KEY_KP_DBLVERTICALBAR: KeyCode = 202;
pub const KEY_KP_COLON: KeyCode = 203;
pub const KEY_KP_HASH: KeyCode = 204;
pub const KEY_KP_SPACE: KeyCode = 205;
pub const KEY_KP_AT: KeyCode = 206;
pub const KEY_KP_EXCLAM: KeyCode = 207;
pub const KEY_KP_MEMSTORE: KeyCode = 208;
pub const KEY_KP_MEMRECALL: KeyCode = 209;
pub const KEY_KP_MEMCLEAR: KeyCode = 210;
pub const KEY_KP_MEMADD: KeyCode = 211;
pub const KEY_KP_MEMSUBTRACT: KeyCode = 212;
pub const KEY_KP_MEMMULTIPLY: KeyCode = 213;
pub const KEY_KP_MEMDIVIDE: KeyCode = 214;
pub const KEY_KP_PLUSMINUS: KeyCode = 215;
pub const KEY_KP_CLEAR: KeyCode = 216;
pub const KEY_KP_CLEARENTRY: KeyCode = 217;
pub const KEY_KP_BINARY: KeyCode = 218;
pub const KEY_KP_OCTAL: KeyCode = 219;
pub const KEY_KP_DECIMAL: KeyCode = 220;
pub const KEY_KP_HEXADECIMAL: KeyCode = 221;
pub const KEY_LCTRL: KeyCode = 224;
pub const KEY_LSHIFT: KeyCode = 225;
pub const KEY_LALT: KeyCode = 226;
pub const KEY_LGUI: KeyCode = 227;
pub const KEY_RCTRL: KeyCode = 228;
pub const KEY_RSHIFT: KeyCode = 229;
pub const KEY_RALT: KeyCode = 230;
pub const KEY_RGUI: KeyCode = 231;
pub const KEY_MODE: KeyCode = 257;
pub const KEY_AUDIONEXT: KeyCode = 258;
pub const KEY_AUDIOPREV: KeyCode = 259;
pub const KEY_AUDIOSTOP: KeyCode = 260;
pub const KEY_AUDIOPLAY: KeyCode = 261;
pub const KEY_AUDIOMUTE: KeyCode = 262;
pub const KEY_MEDIASELECT: KeyCode = 263;
pub const KEY_WWW: KeyCode = 264;
pub const KEY_MAIL: KeyCode = 265;
pub const KEY_CALCULATOR: KeyCode = 266;
pub const KEY_COMPUTER: KeyCode = 267;
pub const KEY_AC_SEARCH: KeyCode = 268;
pub const KEY_AC_HOME: KeyCode = 269;
pub const KEY_AC_BACK: KeyCode = 270;
pub const KEY_AC_FORWARD: KeyCode = 271;
pub const KEY_AC_STOP: KeyCode = 272;
pub const KEY_AC_REFRESH: KeyCode = 273;
pub const KEY_AC_BOOKMARKS: KeyCode = 274;
pub const KEY_BRIGHTNESSDOWN: KeyCode = 275;
pub const KEY_BRIGHTNESSUP: KeyCode = 276;
pub const KEY_DISPLAYSWITCH: KeyCode = 277;
pub const KEY_KBDILLUMTOGGLE: KeyCode = 278;
pub const KEY_KBDILLUMDOWN: KeyCode = 279;
pub const KEY_KBDILLUMUP: KeyCode = 280;
pub const KEY_EJECT: KeyCode = 281;
pub const KEY_SLEEP: KeyCode = 282;
pub const KEY_APP1: KeyCode = 283;
pub const KEY_APP2: KeyCode = 284;
pub const KEY_AUDIOREWIND: KeyCode = 285;
pub const KEY_AUDIOFASTFORWARD: KeyCode = 286;

pub const KEY_MIN_VALUE: KeyCode = KEY_A;
pub const KEY_MAX_VALUE: KeyCode = KEY_AUDIOFASTFORWARD;

pub const KEY_SHIFT: KeyCode = 300;
pub const KEY_CONTROL: KeyCode = 301;
pub const KEY_ALT: KeyCode = 302;
pub const KEY_GUI: KeyCode = 303;

pub const MOUSE_POS_X: KeyCode = 400;
pub const MOUSE_POS_Y: KeyCode = 401;
pub const MOUSE_WHEEL_X: KeyCode = 402;
pub const MOUSE_WHEEL_Y: KeyCode = 403;

pub const MOUSE_BUTTON_LEFT: KeyCode = 450;
pub const MOUSE_BUTTON_MIDDLE: KeyCode = 451;
pub const MOUSE_BUTTON_RIGHT: KeyCode = 452;
pub const MOUSE_BUTTON_X1: KeyCode = 453;
pub const MOUSE_BUTTON_X2: KeyCode = 454;
pub const MOUSE_BUTTON_UNKOWN: KeyCode = 455;

pub const CONTROLLER_1_AXIS_LEFTX: KeyCode = 500;
pub const CONTROLLER_1_AXIS_LEFTY: KeyCode = 501;
pub const CONTROLLER_1_AXIS_RIGHTX: KeyCode = 502;
pub const CONTROLLER_1_AXIS_RIGHTY: KeyCode = 503;
pub const CONTROLLER_1_AXIS_TRIGGERLEFT: KeyCode = 504;
pub const CONTROLLER_1_AXIS_TRIGGERRIGHT: KeyCode = 505;

pub const CONTROLLER1_BUTTON_A: KeyCode = 550;
pub const CONTROLLER1_BUTTON_B: KeyCode = 551;
pub const CONTROLLER1_BUTTON_X: KeyCode = 552;
pub const CONTROLLER1_BUTTON_Y: KeyCode = 553;
pub const CONTROLLER1_BUTTON_BACK: KeyCode = 554;
pub const CONTROLLER1_BUTTON_GUIDE: KeyCode = 555;
pub const CONTROLLER1_BUTTON_START: KeyCode = 556;
pub const CONTROLLER1_BUTTON_LEFTSTICK: KeyCode = 557;
pub const CONTROLLER1_BUTTON_RIGHTSTICK: KeyCode = 558;
pub const CONTROLLER1_BUTTON_LEFTSHOULDER: KeyCode = 559;
pub const CONTROLLER1_BUTTON_RIGHTSHOULDER: KeyCode = 560;
pub const CONTROLLER1_BUTTON_DPAD_UP: KeyCode = 561;
pub const CONTROLLER1_BUTTON_DPAD_DOWN: KeyCode = 562;
pub const CONTROLLER1_BUTTON_DPAD_LEFT: KeyCode = 563;
pub const CONTROLLER1_BUTTON_DPAD_RIGHT: KeyCode = 564;

pub const CONTROLLER_2_AXIS_LEFTX: KeyCode = 600;
pub const CONTROLLER_2_AXIS_LEFTY: KeyCode = 601;
pub const CONTROLLER_2_AXIS_RIGHTX: KeyCode = 602;
pub const CONTROLLER_2_AXIS_RIGHTY: KeyCode = 603;
pub const CONTROLLER_2_AXIS_TRIGGERLEFT: KeyCode = 604;
pub const CONTROLLER_2_AXIS_TRIGGERRIGHT: KeyCode = 605;

pub const CONTROLLER2_BUTTON_A: KeyCode = 650;
pub const CONTROLLER2_BUTTON_B: KeyCode = 651;
pub const CONTROLLER2_BUTTON_X: KeyCode = 652;
pub const CONTROLLER2_BUTTON_Y: KeyCode = 653;
pub const CONTROLLER2_BUTTON_BACK: KeyCode = 654;
pub const CONTROLLER2_BUTTON_GUIDE: KeyCode = 655;
pub const CONTROLLER2_BUTTON_START: KeyCode = 656;
pub const CONTROLLER2_BUTTON_LEFTSTICK: KeyCode = 657;
pub const CONTROLLER2_BUTTON_RIGHTSTICK: KeyCode = 658;
pub const CONTROLLER2_BUTTON_LEFTSHOULDER: KeyCode = 659;
pub const CONTROLLER2_BUTTON_RIGHTSHOULDER: KeyCode = 660;
pub const CONTROLLER2_BUTTON_DPAD_UP: KeyCode = 661;
pub const CONTROLLER2_BUTTON_DPAD_DOWN: KeyCode = 662;
pub const CONTROLLER2_BUTTON_DPAD_LEFT: KeyCode = 663;
pub const CONTROLLER2_BUTTON_DPAD_RIGHT: KeyCode = 664;