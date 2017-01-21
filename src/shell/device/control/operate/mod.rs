mod err;
pub mod mouse;
pub mod key;

use std::str;

use ::libc;

pub use super::In;
pub use self::mouse::Mouse;
pub use self::key::Key;
pub use self::err::{OperateError, Result};

#[derive(Clone, Copy, Debug)]
pub enum Operate {
    /// The mouse operate.
    Mouse(Mouse, bool, libc::c_ushort, libc::c_ushort),
    /// The key operate.
    Key(Key),
}

impl Operate {
    /// The constructor method `new` returns evaluated Operate.
    pub fn new(buf: In, len: libc::size_t) -> Self {
        if let Ok(opt) = Operate::from_mouse(&buf, len) {
            opt
        } else {
            Operate::Key(Key::new(buf, len))
        }
    }

    /// The constructor method `from_mouse` returns evaluated a mouse input.
    pub fn from_mouse(buf: &In, len: usize) -> Result<Self> {
        match buf {
            &In([b'\x1B', b'[', b'<', action, b';', ref coordinate.., m @ b'M'...b'm']) => {
                match Mouse::new(action) {
                    Ok(cmd) => {
                        coordinate.iter().position(|&sep| sep.eq(&b';')).map(|index: usize| unsafe {
                            let (term_x, term_y): (&[u8], &[u8]) = coordinate.split_at(index);
                            let term_y: &[u8] = &term_y[1..len];
                            if let (Ok(x), Ok(y)) = (
                                u16::from_str_radix(str::from_utf8_unchecked(term_x), 10),
                                u16::from_str_radix(str::from_utf8_unchecked(term_y), 10)
                            ) {
                                Ok(Operate::Mouse(cmd, m.eq(&b'M'), x, y))
                            } else {
                                Err(OperateError::FromStrFail)
                            }
                        }).unwrap_or(Err(OperateError::PositionNotFound))
                    },
                    Err(why) => Err(OperateError::MouseFail(why)),
                }
            },
            _ => Err(OperateError::NotMouse),
        }
    }

    /// The accessor method `is_mouse` returns a Option for the Mouse Operate.
    pub fn is_mouse(&self) -> Option<(Mouse, bool, libc::c_ushort, libc::c_ushort)> {
        match *self {
            Operate::Mouse(mouse, release, x, y) => Some((mouse, release, x, y)),
            Operate::Key(_) => None,
        }
    }

    /// The accessor method `is_key` returns a Option for the Key Operate.
    pub fn is_key(&self) -> Option<Key> {
        match *self {
            Operate::Key(key) => Some(key),
            _ => None,
        }
    }
}
