use devices::*;


/// State of a Key or Button
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum State {
    Pressed,
    Released,
}

/// Key Identifier
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum KeyId {
    Esc, // 0x01

    F1, // 0x3b
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11, // 0x57
    F12,

    Backtick, // 0x29
    One, // 0x02
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Minus,
    Equals,
    Yen, // 0x7d
    Backspace, // 0x0e

    Tab, // 0x0f
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LeftBracket,
    RightBracket,
    Enter,

    CapsLock, // 0x3a
    A, // 0x1e
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    Quote,
    Backslash, // 0x2b

    LeftShift, // 0x2a
    Z, // 0x2c
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Period,
    Slash,
    SpecialNearShift, // 0x73
    RightShift, // 0x36

    LeftCtrl, // 0x1d
    LeftWin, // 0x5b
    LeftAlt, // 0x38
    Muhenkan, // 0x7b
    Space, // 0x39
    Henkan, // 0x79
    HiraganaKatakana, // 0x70
    RightAlt, // (0xe0) 0x38
    RightWin, // 0x5c
    Menu, // 0x5d
    RightCtrl, // (0xe0) 0x1d

    PrintScreen, // special
    ScrollLock, // special
    Pause, // special

    Insert, // 0x52
    Delete, // 0x53
    Home, // 0x47
    End, // 0x4f
    PageUp, // 0x49
    PageDown, // 0x51

    Up, // 0x48
    Down, // 0x50
    Left, // 0x4b
    Right, // 0x4d

    // TODO numpad
    NumLock, // 0x45
}

/// Mouse Buttons
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
    RX,
    RY,
    RZ,
}


/// Event types
///
/// The usize entry acts as a device ID unique to each DeviceType (Mouse, Keyboard, Hid).
/// Keyboard press events repeat when a key is held down.
#[derive(Clone, Debug)]
pub enum RawEvent {
    MouseButtonEvent(usize,MouseButton,State),
    MouseMoveEvent(usize,i32,i32),
    MouseWheelEvent(usize,f32),
    KeyboardEvent(usize,KeyId,State),
    JoystickButtonEvent(usize,usize,State),
    JoystickAxisEvent(usize,Axis,f64),
    JoystickHatSwitchEvent(usize,HatSwitch),
}


impl JoystickState {
    pub fn compare_states(&self, other_state: JoystickState, id: usize) -> Vec<RawEvent> {
        let mut output: Vec<RawEvent> = Vec::new();
        for (index, (&press_state, _)) in self.button_states.iter()
            .zip(other_state.button_states.iter()).enumerate()
            .filter(|&(_, (&a, &b))| a != b) {
            output.push(RawEvent::JoystickButtonEvent(
                id,
                index,
                if press_state { State::Released } else { State::Pressed }));
        }
        // for index in 0..self.button_states.len() {
        //     if self.button_states[index] == true
        //         && other_state.button_states[index] == false {
        //         output.push(RawEvent::JoystickButtonEvent(id,index,State::Released));
        //     }
        //     if self.button_states[index] == false
        //         && other_state.button_states[index] == true {
        //         output.push(RawEvent::JoystickButtonEvent(id,index,State::Pressed));
        //     }
        // }
        if self.raw_axis_states.x != other_state.raw_axis_states.x {
            if let Some(value) = other_state.axis_states.x {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::X, value));
            }
        }
        if self.raw_axis_states.y != other_state.raw_axis_states.y {
            if let Some(value) = other_state.axis_states.y {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::Y, value));
            }
        }
        if self.raw_axis_states.z != other_state.raw_axis_states.z {
            if let Some(value) = other_state.axis_states.z {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::Z, value));
            }
        }
        if self.raw_axis_states.rx != other_state.raw_axis_states.rx {
            if let Some(value) = other_state.axis_states.rx {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::RX, value));
            }
        }
        if self.raw_axis_states.ry != other_state.raw_axis_states.ry {
            if let Some(value) = other_state.axis_states.ry {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::RY, value));
            }
        }
        if self.raw_axis_states.rz != other_state.raw_axis_states.rz {
            if let Some(value) = other_state.axis_states.rz {
                output.push(RawEvent::JoystickAxisEvent(id, Axis::RZ, value));
            }
        }
        if let Some(value_other) = other_state.hatswitch {
            if let Some(value_self) = self.hatswitch.clone() {
                if value_self != value_other {
                    output.push(RawEvent::JoystickHatSwitchEvent(id,value_other));
                }
            }
        }
        output
    }
}
