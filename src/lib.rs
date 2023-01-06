use rsa::{RsaPrivateKey, pkcs8::DecodePrivateKey, PaddingScheme};
use std::fs::OpenOptions;


use std::{net::{TcpListener, TcpStream}, io::Read};

fn handle_client(mut stream: TcpStream, private_key: &RsaPrivateKey) {
    let mut read_buffer = [0u8;256];
    stream.read_exact(&mut read_buffer).ok();
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let dec_data = private_key.decrypt(padding, &read_buffer).expect("failed to decrypt");
    println!("{:?}",dec_data);
}

pub fn open_tcp_listener(port: i32) -> std::io::Result<()> {
    println!("{}", port.to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port.to_string()))?; //0.0.0.0:
    let private_key = decode_private_key();
    for stream in listener.incoming() {
        handle_client(stream?, &private_key);
    }
    Ok(())
}

fn decode_private_key() -> RsaPrivateKey {

    let mut file_options = OpenOptions::new();
            file_options.read(true);
            file_options.write(false);
    let mut pem_file = file_options.open("./private_key.pem").expect("You fucked up fool");
    let mut pem:String = String::from("");
    pem_file.read_to_string(&mut pem).ok();
    println!("{:?}", pem);
    RsaPrivateKey::from_pkcs8_pem(&pem).unwrap()
}





/* 
const KEY_RESERVED:u8 =0;
const KEY_ESC:u8 =1;
const KEY_1:u8 =2;
const KEY_2:u8 =3;
const KEY_3:u8 =4;
const KEY_4:u8 =5;
const KEY_5:u8 =6;
const KEY_6:u8 =7;
const KEY_7:u8 =8;
const KEY_8:u8 =9;
const KEY_9:u8 =10;
const KEY_0:u8 =11;
const KEY_MINUS:u8 =12;
const KEY_EQUAL:u8 =13;
const KEY_BACKSPACE:u8 =14;
const KEY_TAB:u8 =15;
const KEY_Q:u8 =16;
const KEY_W:u8 =17;
const KEY_E:u8 =18;
const KEY_R:u8 =19;
const KEY_T:u8 =20;
const KEY_Y:u8 =21;
const KEY_U:u8 =22;
const KEY_I:u8 =23;
const KEY_O:u8 =24;
const KEY_P:u8 =25;
const KEY_LEFTBRACE:u8 =26;
const KEY_RIGHTBRACE:u8 =27;
const KEY_ENTER:u8 =28;
const KEY_LEFTCTRL:u8 =29;
const KEY_A:u8 =30;
const KEY_S:u8 =31;
const KEY_D:u8 =32;
const KEY_F:u8 =33;
const KEY_G:u8 =34;
const KEY_H:u8 =35;
const KEY_J:u8 =36;
const KEY_K:u8 =37;
const KEY_L:u8 =38;
const KEY_SEMICOLON:u8 =39;
const KEY_APOSTROPHE:u8 =40;
const KEY_GRAVE:u8 =41;
const KEY_LEFTSHIFT:u8 =42;
const KEY_BACKSLASH:u8 =43;
const KEY_Z:u8 =44;
const KEY_X:u8 =45;
const KEY_C:u8 =46;
const KEY_V:u8 =47;
const KEY_B:u8 =48;
const KEY_N:u8 =49;
const KEY_M:u8 =50;
const KEY_COMMA:u8 =51;
const KEY_DOT:u8 =52;
const KEY_SLASH:u8 =53;
const KEY_RIGHTSHIFT:u8 =54;
const KEY_KPASTERISK:u8 =55;
const KEY_LEFTALT:u8 =56;
const KEY_SPACE:u8 =57;
const KEY_CAPSLOCK:u8 =58;
const KEY_F1:u8 =59;
const KEY_F2:u8 =60;
const KEY_F3:u8 =61;
const KEY_F4:u8 =62;
const KEY_F5:u8 =63;
const KEY_F6:u8 =64;
const KEY_F7:u8 =65;
const KEY_F8:u8 =66;
const KEY_F9:u8 =67;
const KEY_F10:u8 =68;
const KEY_NUMLOCK:u8 =69;
const KEY_SCROLLLOCK:u8 =70;
const KEY_KP7:u8 =71;
const KEY_KP8:u8 =72;
const KEY_KP9:u8 =73;
const KEY_KPMINUS:u8 =74;
const KEY_KP4:u8 =75;
const KEY_KP5:u8 =76;
const KEY_KP6:u8 =77;
const KEY_KPPLUS:u8 =78;
const KEY_KP1:u8 =79;
const KEY_KP2:u8 =80;
const KEY_KP3:u8 =81;
const KEY_KP0:u8 =82;
const KEY_KPDOT:u8 =83;
const KEY_ZENKAKUHANKAKU:u8 =85;
const KEY_102ND:u8 =86;
const KEY_F11:u8 =87;
const KEY_F12:u8 =88;
const KEY_RO:u8 =89;
const KEY_KATAKANA:u8 =90;
const KEY_HIRAGANA:u8 =91;
const KEY_HENKAN:u8 =92;
const KEY_KATAKANAHIRAGANA:u8 =93;
const KEY_MUHENKAN:u8 =94;
const KEY_KPJPCOMMA:u8 =95;
const KEY_KPENTER:u8 =96;
const KEY_RIGHTCTRL:u8 =97;
const KEY_KPSLASH:u8 =98;
const KEY_SYSRQ:u8 =99;
const KEY_RIGHTALT:u8 =100;
const KEY_LINEFEED:u8 =101;
const KEY_HOME:u8 =102;
const KEY_UP:u8 =103;
const KEY_PAGEUP:u8 =104;
const KEY_LEFT:u8 =105;
const KEY_RIGHT:u8 =106;
const KEY_END:u8 =107;
const KEY_DOWN:u8 =108;
const KEY_PAGEDOWN:u8 =109;
const KEY_INSERT:u8 =110;
const KEY_DELETE:u8 =111;
const KEY_MACRO:u8 =112;
const KEY_MUTE:u8 =113;
const KEY_VOLUMEDOWN:u8 =114;
const KEY_VOLUMEUP:u8 =115;
const KEY_POWER:u8 =116;
const KEY_KPEQUAL:u8 =117;
const KEY_KPPLUSMINUS:u8 =118;
const KEY_PAUSE:u8 =119;
const KEY_SCALE:u8 =120;
const KEY_KPCOMMA:u8 =121;
const KEY_HANGEUL:u8 =122;
const KEY_HANGUEL:u8 =KEY_HANGEUL;
const KEY_HANJA:u8 =123;
const KEY_YEN:u8 =124;
const KEY_LEFTMETA:u8 =125;
const KEY_RIGHTMETA:u8 =126;
const KEY_COMPOSE:u8 =127;
const KEY_STOP:u8 =128;
const KEY_AGAIN:u8 =129;
const KEY_PROPS:u8 =130;
const KEY_UNDO:u8 =131;
const KEY_FRONT:u8 =132;
const KEY_COPY:u8 =133;
const KEY_OPEN:u8 =134;
const KEY_PASTE:u8 =135;
const KEY_FIND:u8 =136;
const KEY_CUT:u8 =137;
const KEY_HELP:u8 =138;
const KEY_MENU:u8 =139;
const KEY_CALC:u8 =140;
const KEY_SETUP:u8 =141;
const KEY_SLEEP:u8 =142;
const KEY_WAKEUP:u8 =143;
const KEY_FILE:u8 =144;
const KEY_SENDFILE:u8 =145;
const KEY_DELETEFILE:u8 =146;
const KEY_XFER:u8 =147;
const KEY_PROG1:u8 =148;
const KEY_PROG2:u8 =149;
const KEY_WWW:u8 =150;
const KEY_MSDOS:u8 =151;
const KEY_COFFEE:u8 =152;
const KEY_SCREENLOCK:u8 =KEY_COFFEE;
const KEY_ROTATE_DISPLAY:u8 =153;
const KEY_DIRECTION:u8 =KEY_ROTATE_DISPLAY;
const KEY_CYCLEWINDOWS:u8 =154;
const KEY_MAIL:u8 =155;
const KEY_BOOKMARKS:u8 =156;
const KEY_COMPUTER:u8 =157;
const KEY_BACK:u8 =158;
const KEY_FORWARD:u8 =159;
const KEY_CLOSECD:u8 =160;
const KEY_EJECTCD:u8 =161;
const KEY_EJECTCLOSECD:u8 =162;
const KEY_NEXTSONG:u8 =163;
const KEY_PLAYPAUSE:u8 =164;
const KEY_PREVIOUSSONG:u8 =165;
const KEY_STOPCD:u8 =166;
const KEY_RECORD:u8 =167;
const KEY_REWIND:u8 =168;
const KEY_PHONE:u8 =169;
const KEY_ISO:u8 =170;
const KEY_CONFIG:u8 =171;
const KEY_HOMEPAGE:u8 =172;
const KEY_REFRESH:u8 =173;
const KEY_EXIT:u8 =174;
const KEY_MOVE:u8 =175;
const KEY_EDIT:u8 =176;
const KEY_SCROLLUP:u8 =177;
const KEY_SCROLLDOWN:u8 =178;
const KEY_KPLEFTPAREN:u8 =179;
const KEY_KPRIGHTPAREN:u8 =180;
const KEY_NEW:u8 =181;
const KEY_REDO:u8 =182;
const KEY_F13:u8 =183;
const KEY_F14:u8 =184;
const KEY_F15:u8 =185;
const KEY_F16:u8 =186;
const KEY_F17:u8 =187;
const KEY_F18:u8 =188;
const KEY_F19:u8 =189;
const KEY_F20:u8 =190;
const KEY_F21:u8 =191;
const KEY_F22:u8 =192;
const KEY_F23:u8 =193;
const KEY_F24:u8 =194;
const KEY_PLAYCD:u8 =200;
const KEY_PAUSECD:u8 =201;
const KEY_PROG3:u8 =202;
const KEY_PROG4:u8 =203;
const KEY_ALL_APPLICATIONS:u8 =204;
const KEY_DASHBOARD:u8 =KEY_ALL_APPLICATIONS;
const KEY_SUSPEND:u8 =205;
const KEY_CLOSE:u8 =206;
const KEY_PLAY:u8 =207;
const KEY_FASTFORWARD:u8 =208;
const KEY_BASSBOOST:u8 =209;
const KEY_PRINT:u8 =210;
const KEY_HP:u8 =211;
const KEY_CAMERA:u8 =212;
const KEY_SOUND:u8 =213;
const KEY_QUESTION:u8 =214;
const KEY_EMAIL:u8 =215;
const KEY_CHAT:u8 =216;
const KEY_SEARCH:u8 =217;
const KEY_CONNECT:u8 =218;
const KEY_FINANCE:u8 =219;
const KEY_SPORT:u8 =220;
const KEY_SHOP:u8 =221;
const KEY_ALTERASE:u8 =222;
const KEY_CANCEL:u8 =223;
const KEY_BRIGHTNESSDOWN:u8 =224;
const KEY_BRIGHTNESSUP:u8 =225;
const KEY_MEDIA:u8 =226;
const KEY_SWITCHVIDEOMODE:u8 =227;
const KEY_KBDILLUMTOGGLE:u8 =228;
const KEY_KBDILLUMDOWN:u8 =229;
const KEY_KBDILLUMUP:u8 =230;
const KEY_SEND:u8 =231;
const KEY_REPLY:u8 =232;
const KEY_FORWARDMAIL:u8 =233;
const KEY_SAVE:u8 =234;
const KEY_DOCUMENTS:u8 =235;
const KEY_BATTERY:u8 =236;
const KEY_BLUETOOTH:u8 =237;
const KEY_WLAN:u8 =238;
const KEY_UWB:u8 =239;
const KEY_UNKNOWN:u8 =240;
const KEY_VIDEO_NEXT:u8 =241;
const KEY_VIDEO_PREV:u8 =242;
const KEY_BRIGHTNESS_CYCLE:u8 =243;
const KEY_BRIGHTNESS_AUTO:u8 =244;
const KEY_BRIGHTNESS_ZERO:u8 =KEY_BRIGHTNESS_AUTO;
const KEY_DISPLAY_OFF:u8 =245;
const KEY_WWAN:u8 =246;
const KEY_WIMAX:u8 =KEY_WWAN;
const KEY_RFKILL:u8 =247;
const KEY_MICMUTE:u8 =248;
*/

//ToDo: ReMap the Above into a HashMap so it can be used to decode the incoming event codes
