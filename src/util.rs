use ::libc;

#[derive(Clone, Copy, Debug)]
pub enum Key {
    /// Space
    Space,
    /// Enter
    Enter,
    ///Tabulation
    Tab,
    /// Backspace
    Backspace,
    /// Home key
    Home,
    /// End key
    End,
    /// Page Up key
    PageUp,
    /// Page Down key
    PageDown,
    /// Delete key
    Delete,
    /// Insert key
    Insert,
    /// Function keys between 1-16
    F(libc::c_uchar),
    /// Shit and Function keys between 1-16
    ShiftF(libc::c_uchar),
    /// Simple character
    Char(libc::c_uchar),
    /// Character used with Alt
    Alt(libc::c_uchar),
    /// Character used with Ctrl
    /// Note that certain keys may not be modifiable with `ctrl`,
    /// due to limitations of terminals.
    Ctrl(libc::c_uchar),
    /// Esc key
    Esc,
    ///The Bell signal when something wrong happened
    Bell,
    ///The carriage return lead the cursor to the beginning of the current line
    Carriage,
    /// Left arrow
    Left,
    /// Right arrow
    Right,
    /// Up arrow
    Up,
    /// Down arrow
    Down,
    /// Shift Left arrow
    ShiftLeft,
    /// Shift Right arrow
    ShiftRight,
    /// Shift Up arrow
    ShiftUp,
    /// Shift Down arrow
    ShiftDown,
    /// Shift End key
    ShiftEnd,
    /// Alt Left arrow
    AltLeft,
    /// Alt Right arrow
    AltRight,
    /// Alt Up arrow
    AltUp,
    /// Alt Down arrow
    AltDown,
    /// Alt End key
    AltEnd,
    /// Control Left arrow
    CtrlLeft,
    /// Control Right arrow
    CtrlRight,
    /// Control Up arrow
    CtrlUp,
    /// Control Down arrow
    CtrlDown,
    /// Ctrl End key
    CtrlEnd,
    /// Alt Shift Left arrow
    AltShiftLeft,
    /// Alt Shift Right arrow
    AltShiftRight,
    /// Alt Shift Up arrow
    AltShiftUp,
    /// Alt Shift Down arrow
    AltShiftDown,
    /// Alt Shift End key
    AltShiftEnd,
    /// Control Shift Left arrow
    CtrlShiftLeft,
    /// Control Shift Right arrow
    CtrlShiftRight,
    /// Control Shift Up arrow
    CtrlShiftUp,
    /// Control Shift Down arrow
    CtrlShiftDown,
    /// Ctrl Shift End key
    CtrlShiftEnd,
    /// Ctrl Alt End key
    CtrlAltEnd,
    /// Ctrl Alt Shift End key
    CtrlAltShiftEnd,
}

impl Key
{ pub fn new(s: u8) -> Self
  { Key::Char(s) }}


#[derive(Clone, Copy, Debug)]
/// Note that the button "Command" is like the function button
/// between Ctrl and Alt on Azerty keyboards
pub enum Mouse {
       /// The left mouse button is pressed.
       Left = 0isize,
       /// The right mouse button is pressed.
       Right = 1isize,
       /// The mouse wheel button is pressed.
       Wheel = 2isize,
       /// The mouse wheel is going up.
       WheelUp = 64isize,
       /// The mouse wheel is going down.
       WheelDown = 65isize,
       /// The left mouse button is held while moving pointer.
       LeftDrag = 32isize,
       /// The wheel mouse button is held while moving pointer.
       WheelDrag = 33isize,
       /// The right mouse button is held while moving pointer.
       RightDrag = 34isize,
       /// The left mouse button is pressed while helding Shift.
       ShiftLeft = 4isize,
       /// The wheel mouse button is pressed while helding Shift.
       ShiftWheel = 5isize,
       /// The right mouse button is pressed while helding Shift.
       ShiftRight = 6isize,
       /// The left mouse button and Shift are held while moving pointer.
       ShiftLeftDrag = 36isize,
       /// The wheel mouse button and Shift are held while moving pointer.
       ShiftWheelDrag = 37isize,
       /// The right mouse button and Shift are held while moving pointer.
       ShiftRightDrag = 38isize,
       /// The left mouse button is pressed while helding Ctrl
       CtrlLeft = 16isize,
       /// The wheel mouse button is pressed while helding Ctrl
       CtrlWheel = 17isize,
       /// The right mouse button is pressed while helding Ctrl
       CtrlRight = 18isize,
       /// The mouse wheel is going up while helding Ctrl
       CtrlWheelUp = 80isize,
       /// The mouse wheel is going down while helding Ctrl
       CtrlWheelDown = 81isize,
       /// The left mouse button and Ctrl are held while moving pointer
       CtrlLeftDrag = 48isize,
       /// The wheel mouse button and Ctrl are held while moving pointer
       CtrlWheelDrag = 49isize,
       /// The right mouse button and Ctrl are held while moving pointer
       CtrlRightDrag = 50isize,
       /// The left mouse button is pressed while Ctrl and Shift are held
       ShiftCtrlLeft = 20isize,
       /// The wheel mouse button is pressed while Ctrl and Shift are held
       ShiftCtrlWheel = 21isize,
       /// The right mouse button is pressed while Ctrl and Shift are held
       ShiftCtrlRight = 22isize,
       /// The left mouse button, Ctrl and Shift are held while moving pointer
       ShiftCtrlLeftDrag = 52isize,
       /// The wheel mouse button, Ctrl and Shift are held while moving pointer
       ShiftCtrlWheelDrag = 53isize,
       /// The right mouse button, Ctrl and Shift are held while moving pointer
       ShiftCtrlRightDrag = 54isize,
       /// The left mouse button is pressed while helding Command
       CmdLeft = 8isize,
       /// The wheel mouse button is pressed while helding Command
       CmdWheel = 9isize,
       /// The right mouse button is pressed while helding Command
       CmdRight = 10isize,
       /// The mouse wheel is going up while helding Command
       CmdWheelUp = 72isize,
       /// The mouse wheel is going down while helding Command
       CmdWheelDown = 73isize,
       /// The left mouse button and Command are held while moving pointer
       CmdLeftDrag = 40isize,
       /// The wheel mouse button and Command are held while moving pointer
       CmdWheelDrag = 41isize,
       /// The right mouse button and Command are held while moving pointer
       CmdRightDrag = 42isize,
       /// The left mouse button is pressed while helding Command and Shift
       CmdShiftLeft = 12isize,
       /// The wheel mouse button is pressed while helding Command and Shift
       CmdShiftWheel = 13isize,
       /// The right mouse button is pressed while helding Command and Shift
       CmdShiftRight = 14isize,
       /// The left mouse button, Shift and Command are held while moving pointer
       CmdShiftLeftDrag = 44isize,
       /// The wheel mouse button, Shift and Command are held while moving pointer
       CmdShiftWheelDrag = 45isize,
       /// The right mouse button, Shift and Command are held while moving pointer
       CmdShiftRightDrag = 46isize,
       /// The left mouse button is pressed while helding Command and Ctrl
       CmdCtrlLeft = 24isize,
       /// The wheel mouse button is pressed while helding Command and Ctrl
       CmdCtrlWheel = 25isize,
       /// The right mouse button is pressed while helding Command and Ctrl
       CmdCtrlRight = 26isize,
       /// The mouse wheel is going up while helding Command and Ctrl
       CmdCtrlWheelUp = 88isize,
       /// The mouse wheel is going down while helding Command and Ctrl
       CmdCtrlWheelDown = 89isize,
       /// The left mouse button, Ctrl and Command are held while moving pointer
       CmdCtrlLeftDrag = 56isize,
       /// The wheel mouse button, Ctrl and Command are held while moving pointer
       CmdCtrlWheelDrag = 57isize,
       /// The right mouse button, Ctrl and Command are held while moving pointer
       CmdCtrlRightDrag = 58isize,
       /// The left mouse button is pressed while helding Command, Shift and Ctrl
       CmdShiftCtrlLeft = 28isize,
       /// The wheel mouse button is pressed while helding Command, Shift and Ctrl
       CmdShiftCtrlWheel = 29isize,
       /// The right mouse button is pressed while helding Command, Shift and Ctrl
       CmdShiftCtrlRight = 30isize,
       /// The left mouse button, Ctrl, Shift and Command are held while moving pointer
       CmdShiftCtrlLeftDrag = 60isize,
       /// The wheel mouse button, Ctrl, Shift and Command are held while moving pointer
       CmdShiftCtrlWheelDrag = 61isize,
       /// The right mouse button, Ctrl, Shift and Command are held while moving pointer
       CmdShiftCtrlRightDrag = 62isize,
}

impl Mouse {
  pub fn new(s: i32) -> Self {
    match s {
        ///Click
        0 => Mouse::Left,
        1 => Mouse::Wheel,
        2 => Mouse::Right,
        64 => Mouse::WheelUp,
        65 => Mouse::WheelDown,

        ///Drag
        32 => Mouse::LeftDrag,
        33 => Mouse::WheelDrag,
        34 => Mouse::RightDrag,

        ///Shift Click
        4 => Mouse::ShiftLeft,
        5 => Mouse::ShiftWheel,
        6 => Mouse::ShiftRight,

        ///Shift Drag
        36 => Mouse::ShiftLeftDrag,
        37 => Mouse::ShiftWheelDrag,
        38 => Mouse::ShiftRightDrag,

        ///Control Click
        16 => Mouse::CtrlLeft,
        17 => Mouse::CtrlWheel,
        18 => Mouse::CtrlRight,
        80 => Mouse::CtrlWheelUp,
        81 => Mouse::CtrlWheelDown,

        ///Control Drag
        48 => Mouse::CtrlLeftDrag,
        49 => Mouse::CtrlWheelDrag,
        50 => Mouse::CtrlRightDrag,

        ///Control Shift Click
        20 => Mouse::ShiftCtrlLeft,
        21 => Mouse::ShiftCtrlWheel,
        22 => Mouse::ShiftCtrlRight,

        ///Control Shift Drag
        52 => Mouse::ShiftCtrlLeftDrag,
        53 => Mouse::ShiftCtrlWheelDrag,
        54 => Mouse::ShiftCtrlRightDrag,

        ///Command Click
        8 => Mouse::CmdLeft,
        9 => Mouse::CmdWheel,
        10 => Mouse::CmdRight,
        72 => Mouse::CmdWheelUp,
        73 => Mouse::CmdWheelDown,

        ///Command Drag
        40 => Mouse::CmdLeftDrag,
        41 => Mouse::CmdWheelDrag,
        42 => Mouse::CmdRightDrag,

        ///Command Shift Click
        12 => Mouse::CmdShiftLeft,
        13 => Mouse::CmdShiftWheel,
        14 => Mouse::CmdShiftRight,

        ///Command Shift Drag
        44 => Mouse::CmdShiftLeftDrag,
        45 => Mouse::CmdShiftWheelDrag,
        46 => Mouse::CmdShiftRightDrag,

        ///Command Control Click
        24 => Mouse::CmdCtrlLeft,
        25 => Mouse::CmdCtrlWheel,
        26 => Mouse::CmdCtrlRight,
        88 => Mouse::CmdCtrlWheelUp,
        89 => Mouse::CmdCtrlWheelDown,

        ///Command Control Drag
        56 => Mouse::CmdCtrlLeftDrag,
        57 => Mouse::CmdCtrlWheelDrag,
        58 => Mouse::CmdCtrlRightDrag,

        ///Command Shift Control Click
        28 => Mouse::CmdShiftCtrlLeft,
        29 => Mouse::CmdShiftCtrlWheel,
        30 => Mouse::CmdShiftCtrlRight,

        ///Command Shift Control Drag
        60 => Mouse::CmdShiftCtrlLeftDrag,
        61 => Mouse::CmdShiftCtrlWheelDrag,
        62 => Mouse::CmdShiftCtrlRightDrag,

        _ => unimplemented!(),
    }
  }
}
