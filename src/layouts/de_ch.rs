use pc_keyboard::{DecodedKey, HandleControl, KeyCode, KeyboardLayout};

use crate::print;

pub struct DeCh;

impl KeyboardLayout for DeCh {
    fn map_keycode(
        keycode: pc_keyboard::KeyCode,
        modifiers: &pc_keyboard::Modifiers,
        handle_ctrl: pc_keyboard::HandleControl,
    ) -> pc_keyboard::DecodedKey {
        match keycode {
            KeyCode::Escape => DecodedKey::Unicode(0x1B.into()),
            KeyCode::Key1 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('+')
                } else {
                    DecodedKey::Unicode('1')
                }
            }
            KeyCode::Key2 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('"')
                } else {
                    DecodedKey::Unicode('2')
                }
            }
            KeyCode::Key3 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('*')
                } else {
                    DecodedKey::Unicode('3')
                }
            }
            KeyCode::Key4 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('ç')
                } else {
                    DecodedKey::Unicode('4')
                }
            }
            KeyCode::Key5 => {
                if modifiers.alt_gr {
                    DecodedKey::Unicode('[')
                } else if modifiers.is_shifted() {
                    DecodedKey::Unicode('%')
                } else {
                    DecodedKey::Unicode('5')
                }
            }
            KeyCode::Key6 => {
                if modifiers.alt_gr {
                    DecodedKey::Unicode(']')
                } else if modifiers.is_shifted() {
                    DecodedKey::Unicode('&')
                } else {
                    DecodedKey::Unicode('6')
                }
            }
            KeyCode::Key7 => {
                if modifiers.alt_gr {
                    if modifiers.is_shifted() {
                        DecodedKey::Unicode('\\')
                    } else {
                        DecodedKey::Unicode('|')
                    }
                } else if modifiers.is_shifted() {
                    DecodedKey::Unicode('/')
                } else {
                    DecodedKey::Unicode('7')
                }
            }
            KeyCode::Key8 => {
                if modifiers.alt_gr {
                    DecodedKey::Unicode('{')
                } else if modifiers.is_shifted() {
                    DecodedKey::Unicode('(')
                } else {
                    DecodedKey::Unicode('8')
                }
            }
            KeyCode::Key9 => {
                if modifiers.alt_gr {
                    DecodedKey::Unicode('}')
                } else if modifiers.is_shifted() {
                    DecodedKey::Unicode(')')
                } else {
                    DecodedKey::Unicode('9')
                }
            }
            KeyCode::Key0 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('=')
                } else {
                    DecodedKey::Unicode('0')
                }
            }

            KeyCode::A => {
                if modifiers.alt_gr {
                    DecodedKey::Unicode('å')
                } else if modifiers.is_shifted() {
                    DecodedKey::Unicode('A')
                } else {
                    DecodedKey::Unicode('a')
                }
            }
            KeyCode::B => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('B')
                } else {
                    DecodedKey::Unicode('b')
                }
            }
            KeyCode::C => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('C')
                } else {
                    DecodedKey::Unicode('c')
                }
            }
            KeyCode::D => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('D')
                } else {
                    DecodedKey::Unicode('d')
                }
            }
            KeyCode::E => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('E')
                } else {
                    DecodedKey::Unicode('e')
                }
            }
            KeyCode::F => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('F')
                } else {
                    DecodedKey::Unicode('f')
                }
            }
            KeyCode::G => {
                if modifiers.alt_gr {
                    DecodedKey::Unicode('@')
                } else if modifiers.is_shifted() {
                    DecodedKey::Unicode('G')
                } else {
                    DecodedKey::Unicode('g')
                }
            }
            KeyCode::H => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('H')
                } else {
                    DecodedKey::Unicode('h')
                }
            }
            KeyCode::I => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('I')
                } else {
                    DecodedKey::Unicode('i')
                }
            }
            KeyCode::J => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('J')
                } else {
                    DecodedKey::Unicode('j')
                }
            }
            KeyCode::K => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('K')
                } else {
                    DecodedKey::Unicode('k')
                }
            }
            KeyCode::L => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('L')
                } else {
                    DecodedKey::Unicode('l')
                }
            }
            KeyCode::M => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('M')
                } else {
                    DecodedKey::Unicode('m')
                }
            }
            KeyCode::N => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('N')
                } else {
                    DecodedKey::Unicode('n')
                }
            }
            KeyCode::O => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('O')
                } else {
                    DecodedKey::Unicode('o')
                }
            }
            KeyCode::P => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('P')
                } else {
                    DecodedKey::Unicode('p')
                }
            }
            KeyCode::Q => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('Q')
                } else {
                    DecodedKey::Unicode('q')
                }
            }
            KeyCode::R => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('R')
                } else {
                    DecodedKey::Unicode('r')
                }
            }
            KeyCode::S => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('S')
                } else {
                    DecodedKey::Unicode('s')
                }
            }
            KeyCode::T => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('T')
                } else {
                    DecodedKey::Unicode('t')
                }
            }
            KeyCode::U => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('U')
                } else {
                    DecodedKey::Unicode('u')
                }
            }
            KeyCode::V => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('V')
                } else {
                    DecodedKey::Unicode('v')
                }
            }
            KeyCode::W => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('W')
                } else {
                    DecodedKey::Unicode('w')
                }
            }
            KeyCode::X => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('X')
                } else {
                    DecodedKey::Unicode('x')
                }
            }
            KeyCode::Z => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('Y')
                } else {
                    DecodedKey::Unicode('y')
                }
            }
            KeyCode::Y => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('Z')
                } else {
                    DecodedKey::Unicode('z')
                }
            }
            KeyCode::Spacebar => DecodedKey::Unicode(' '),
            KeyCode::Comma => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode(';')
                } else {
                    DecodedKey::Unicode(',')
                }
            }
            KeyCode::BackTick => {
                if modifiers.is_shifted() {
                    if modifiers.alt_gr {
                        DecodedKey::Unicode('\u{2265}')
                    } else {
                        DecodedKey::Unicode('>')
                    }
                } else {
                    if modifiers.alt_gr {
                        DecodedKey::Unicode('\u{2264}')
                    } else {
                        DecodedKey::Unicode('<')
                    }
                }
            }
            KeyCode::Fullstop => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode(':')
                } else {
                    DecodedKey::Unicode('.')
                }
            }
            KeyCode::Slash => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('_')
                } else {
                    DecodedKey::Unicode('-')
                }
            }
            KeyCode::SemiColon => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('é')
                } else {
                    DecodedKey::Unicode('ö')
                }
            }
            KeyCode::Quote => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('à')
                } else {
                    DecodedKey::Unicode('ä')
                }
            }
            KeyCode::BracketSquareLeft => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('è')
                } else {
                    DecodedKey::Unicode('ü')
                }
            }
            KeyCode::BracketSquareRight => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('!')
                } else {
                    DecodedKey::Unicode('¨')
                }
            }
            KeyCode::Minus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('?')
                } else {
                    DecodedKey::Unicode('\'')
                }
            }
            KeyCode::Enter => DecodedKey::RawKey(KeyCode::Enter),
            k => {
                print!("{k:?}");
                DecodedKey::RawKey(k)
            }
        }
    }
}
