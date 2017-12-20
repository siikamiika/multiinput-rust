use winapi::um::winuser::*;
use event::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum KeyPos {
    Left,
    Right,
}

pub fn process_keyboard_data(raw_data: &RAWKEYBOARD, id: usize) -> Vec<RawEvent> {
    let mut output: Vec<RawEvent> = Vec::new();
    let flags = raw_data.Flags as u32;
    let key = raw_data.MakeCode as i32;
    let mut key_opt: Option<KeyId> = None;
    let key_state: State;
    let key_pos: KeyPos;

    if flags & RI_KEY_BREAK != 0 {
        key_state = State::Released;
    }
    else {
        key_state = State::Pressed;
    }

    if flags & RI_KEY_E0 == 0 {
        key_pos = KeyPos::Left;
    }
    else {
        key_pos = KeyPos::Right;
    }

    ////
    if key == 0x01 {
        key_opt = Some(KeyId::Esc);
    }
    if key == 0x3b {
        key_opt = Some(KeyId::F1);
    }
    if key == 0x3c {
        key_opt = Some(KeyId::F2);
    }
    if key == 0x3d {
        key_opt = Some(KeyId::F3);
    }
    if key == 0x3e {
        key_opt = Some(KeyId::F4);
    }
    if key == 0x3f {
        key_opt = Some(KeyId::F5);
    }
    if key == 0x40 {
        key_opt = Some(KeyId::F6);
    }
    if key == 0x41 {
        key_opt = Some(KeyId::F7);
    }
    if key == 0x42 {
        key_opt = Some(KeyId::F8);
    }
    if key == 0x43 {
        key_opt = Some(KeyId::F9);
    }
    if key == 0x44 {
        key_opt = Some(KeyId::F10);
    }
    if key == 0x57 {
        key_opt = Some(KeyId::F11);
    }
    if key == 0x58 {
        key_opt = Some(KeyId::F12);
    }
    ////
    if key == 0x29 {
        key_opt = Some(KeyId::Backtick);
    }
    if key == 0x02 {
        key_opt = Some(KeyId::One);
    }
    if key == 0x03 {
        key_opt = Some(KeyId::Two);
    }
    if key == 0x04 {
        key_opt = Some(KeyId::Three);
    }
    if key == 0x05 {
        key_opt = Some(KeyId::Four);
    }
    if key == 0x06 {
        key_opt = Some(KeyId::Five);
    }
    if key == 0x07 {
        key_opt = Some(KeyId::Six);
    }
    if key == 0x08 {
        key_opt = Some(KeyId::Seven);
    }
    if key == 0x09 {
        key_opt = Some(KeyId::Eight);
    }
    if key == 0x0a {
        key_opt = Some(KeyId::Nine);
    }
    if key == 0x0b {
        key_opt = Some(KeyId::Zero);
    }
    if key == 0x0c {
        key_opt = Some(KeyId::Minus);
    }
    if key == 0x0d {
        key_opt = Some(KeyId::Equals);
    }
    if key == 0x7d {
        key_opt = Some(KeyId::Yen);
    }
    if key == 0x0e {
        key_opt = Some(KeyId::Backspace);
    }
    ////
    if key == 0x0f {
        key_opt = Some(KeyId::Tab);
    }
    if key == 0x10 {
        key_opt = Some(KeyId::Q);
    }
    if key == 0x11 {
        key_opt = Some(KeyId::W);
    }
    if key == 0x12 {
        key_opt = Some(KeyId::E);
    }
    if key == 0x13 {
        key_opt = Some(KeyId::R);
    }
    if key == 0x14 {
        key_opt = Some(KeyId::T);
    }
    if key == 0x15 {
        key_opt = Some(KeyId::Y);
    }
    if key == 0x16 {
        key_opt = Some(KeyId::U);
    }
    if key == 0x17 {
        key_opt = Some(KeyId::I);
    }
    if key == 0x18 {
        key_opt = Some(KeyId::O);
    }
    if key == 0x19 {
        key_opt = Some(KeyId::P);
    }
    if key == 0x1a {
        key_opt = Some(KeyId::LeftBracket);
    }
    if key == 0x1b {
        key_opt = Some(KeyId::RightBracket);
    }
    if key == 0x1c {
        key_opt = Some(KeyId::Enter);
    }
    ////
    if key == 0x3a {
        key_opt = Some(KeyId::CapsLock);
    }
    if key == 0x1e {
        key_opt = Some(KeyId::A);
    }
    if key == 0x1f {
        key_opt = Some(KeyId::S);
    }
    if key == 0x20 {
        key_opt = Some(KeyId::D);
    }
    if key == 0x21 {
        key_opt = Some(KeyId::F);
    }
    if key == 0x22 {
        key_opt = Some(KeyId::G);
    }
    if key == 0x23 {
        key_opt = Some(KeyId::H);
    }
    if key == 0x24 {
        key_opt = Some(KeyId::J);
    }
    if key == 0x25 {
        key_opt = Some(KeyId::K);
    }
    if key == 0x26 {
        key_opt = Some(KeyId::L);
    }
    if key == 0x27 {
        key_opt = Some(KeyId::Semicolon);
    }
    if key == 0x28 {
        key_opt = Some(KeyId::Quote);
    }
    if key == 0x2b {
        key_opt = Some(KeyId::Backslash);
    }
    ////
    if key == 0x2a {
        key_opt = Some(KeyId::LeftShift);
    }
    if key == 0x2c {
        key_opt = Some(KeyId::Z);
    }
    if key == 0x2d {
        key_opt = Some(KeyId::X);
    }
    if key == 0x2e {
        key_opt = Some(KeyId::C);
    }
    if key == 0x2f {
        key_opt = Some(KeyId::V);
    }
    if key == 0x30 {
        key_opt = Some(KeyId::B);
    }
    if key == 0x31 {
        key_opt = Some(KeyId::N);
    }
    if key == 0x32 {
        key_opt = Some(KeyId::M);
    }
    if key == 0x33 {
        key_opt = Some(KeyId::Comma);
    }
    if key == 0x34 {
        key_opt = Some(KeyId::Period);
    }
    if key == 0x35 {
        key_opt = Some(KeyId::Slash);
    }
    if key == 0x73 {
        key_opt = Some(KeyId::SpecialNearShift);
    }
    if key == 0x36 {
        key_opt = Some(KeyId::RightShift);
    }
    ////
    if key == 0x1d && key_pos == KeyPos::Left {
        key_opt = Some(KeyId::LeftCtrl);
    }
    if key == 0x5b {
        key_opt = Some(KeyId::LeftWin);
    }
    if key == 0x38 && key_pos == KeyPos::Left {
        key_opt = Some(KeyId::LeftAlt);
    }
    if key == 0x7b {
        key_opt = Some(KeyId::Muhenkan);
    }
    if key == 0x39 {
        key_opt = Some(KeyId::Space);
    }
    if key == 0x79 {
        key_opt = Some(KeyId::Henkan);
    }
    if key == 0x70 {
        key_opt = Some(KeyId::HiraganaKatakana);
    }
    if key == 0x38 && key_pos == KeyPos::Right {
        key_opt = Some(KeyId::RightAlt);
    }
    if key == 0x5c {
        key_opt = Some(KeyId::RightWin);
    }
    if key == 0x5d {
        key_opt = Some(KeyId::Menu);
    }
    if key == 0x1d && key_pos == KeyPos::Right {
        key_opt = Some(KeyId::RightCtrl);
    }
    ////
    // PrintScreen, // special
    // ScrollLock, // special
    // Pause, // special
    ////
    if key == 0x52 {
        key_opt = Some(KeyId::Insert);
    }
    if key == 0x53 {
        key_opt = Some(KeyId::Delete);
    }
    if key == 0x47 {
        key_opt = Some(KeyId::Home);
    }
    if key == 0x4f {
        key_opt = Some(KeyId::End);
    }
    if key == 0x49 {
        key_opt = Some(KeyId::PageUp);
    }
    if key == 0x51 {
        key_opt = Some(KeyId::PageDown);
    }
    ////
    if key == 0x48 {
        key_opt = Some(KeyId::Up);
    }
    if key == 0x50 {
        key_opt = Some(KeyId::Down);
    }
    if key == 0x4b {
        key_opt = Some(KeyId::Left);
    }
    if key == 0x4d {
        key_opt = Some(KeyId::Right);
    }

    if let Some(key_id) = key_opt {
        output.push(RawEvent::KeyboardEvent(id, key_id, key_state));
        }
    output
}
