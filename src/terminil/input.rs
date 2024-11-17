use std::{io::{stdin, ErrorKind, Read}, mem};

use terminal_utils::RawModeGuard;

use crate::{printnl, send, terminil::{ansi::ToAnsi, control::Action}};

#[derive(Debug, Clone, Copy)]
pub enum SpecialKey {
	Escape,
	Backspace,
	Insert,
	Delete,
	Up,
	Down,
	Left,
	Right,
	Home,
	End,
	PgUp,
	PgDn,
	Fn(u8),
}

#[derive(Debug, Clone)]
pub enum Event {
	///A single character of valid unicode (e.g. a UTF-8 sequence). Should only contain printable characters.
	TextChar(char),
	///A single character that isn't valid unicode (e.g. ASCII past 127)
	Byte(u8),
	Paste(Vec<u8>),
	Focus(bool),
	SpecialKey(SpecialKey),

	Unknown,
}

///Input handler which can be iterated over to receive Events.
///
///This struct may be platform-specific.
///
///Note that **upon instantiating this struct, certain terminal settings will be changed.** These changes will be reset
///when the struct is dropped. **The default settings which are applied afterward may not match the values of the settings
///before the struct was instantiated.**
pub struct Input {
	_guard: RawModeGuard,
}
impl Input {
	pub fn new() -> Self {
		println!("Hi");
		let guard = terminal_utils::enable_raw_mode().expect("to be able to enter raw mode");
		send!("{}{}{}{}", Action::AlternateBuffer(true).to_ansi(), Action::BracketPaste(true).to_ansi(), Action::FocusReport(true).to_ansi(), Action::EraseScrollback.to_ansi());

		Input {
			_guard: guard
		}
	}
	pub fn get_event(&mut self) -> Option<Event> {
		let mut data = [0;16];

		match stdin().read(&mut data) {
			Ok(len) => {
				let data = mem::take(&mut data);
				if len == 0 {
					None
				} else if data[0] == 0x1B {
					if len == 1 {
						Some(Event::SpecialKey(SpecialKey::Escape))
					} else {
						match data[1..6] {
							[b'[',b'2',b'~',..] => Some(Event::SpecialKey(SpecialKey::Insert)),
							[b'[',b'3',b'~',..] => Some(Event::SpecialKey(SpecialKey::Delete)),
							[b'[',b'H',..]      => Some(Event::SpecialKey(SpecialKey::Home)),
							[b'[',b'F',..]      => Some(Event::SpecialKey(SpecialKey::End)),
							[b'[',b'5',b'~',..] => Some(Event::SpecialKey(SpecialKey::PgUp)),
							[b'[',b'6',b'~',..] => Some(Event::SpecialKey(SpecialKey::PgDn)),
							
							[b'[',b'A',..]      => Some(Event::SpecialKey(SpecialKey::Up)),
							[b'[',b'B',..]      => Some(Event::SpecialKey(SpecialKey::Down)),
							[b'[',b'C',..]      => Some(Event::SpecialKey(SpecialKey::Right)),
							[b'[',b'D',..]      => Some(Event::SpecialKey(SpecialKey::Left)),

							[b'O',b'P',..]                => Some(Event::SpecialKey(SpecialKey::Fn(1))),
							[b'O',b'Q',..]                => Some(Event::SpecialKey(SpecialKey::Fn(2))),
							[b'O',b'R',..]                => Some(Event::SpecialKey(SpecialKey::Fn(3))),
							[b'O',b'S',..]                => Some(Event::SpecialKey(SpecialKey::Fn(4))),
							[b'[',b'1',b'5',b'~',..]      => Some(Event::SpecialKey(SpecialKey::Fn(5))),
							[b'[',b'1',b'7',b'~',..]      => Some(Event::SpecialKey(SpecialKey::Fn(6))),
							[b'[',b'1',b'8',b'~',..]      => Some(Event::SpecialKey(SpecialKey::Fn(7))),
							[b'[',b'1',b'9',b'~',..]      => Some(Event::SpecialKey(SpecialKey::Fn(8))),
							[b'[',b'2',b'0',b'~',..]      => Some(Event::SpecialKey(SpecialKey::Fn(9))),
							[b'[',b'2',b'1',b'~',..]      => Some(Event::SpecialKey(SpecialKey::Fn(10))),
							[b'[',b'2',b'3',b'~',..]      => Some(Event::SpecialKey(SpecialKey::Fn(11))),
							[b'[',b'2',b'4',b'~',..]      => Some(Event::SpecialKey(SpecialKey::Fn(12))),
							
							[b'[',b'I',..]      => Some(Event::Focus(true)),
							[b'[',b'O',..]      => Some(Event::Focus(false)),

							[b'[',b'2',b'0',b'0',b'~',..] => { //Paste
								let mut data = data[6..].to_vec();

								while !data.ends_with(b"\x1b[201~") {
									let mut next_chunk = [0;128];
									let len = stdin().read(&mut next_chunk);
									if let Ok(len) = len {
										data.extend_from_slice(&next_chunk[0..len]);
									} else {
										break;
									}
								}
								//Remove the escape
								data.truncate(data.len()-6);
								//Unlikely this will be written to after this.
								data.shrink_to_fit();

								Some(Event::Paste(data))
							},

							_ => {
								printnl!("{:?} {:?}", &data[0..len], String::from_utf8((&data[0..len]).to_vec()));
								Some(Event::Unknown)
							}
						}
					}
				} else if len == 1 {
					if data[0] == 127 {
						Some(Event::SpecialKey(SpecialKey::Backspace))
					} else {
						match data[0] {
							9 => Some(Event::TextChar('\t')),
							13 => Some(Event::TextChar('\n')),
							32..=126 => Some(Event::TextChar(char::from_u32(data[0] as u32).expect("this predefined range to only include valid UTF-8"))),
							u => Some(Event::Byte(u))
						}
					}
				} else {
					//TODO
					if let Some(c) = char::from_u32(u32::from_le_bytes([data[0],data[1],data[2],data[3]])) {
						Some(Event::TextChar(c))
					} else {
						Some(Event::Unknown)
					}
				}
			},
			Err(e) => match e.kind() {
				ErrorKind::UnexpectedEof => None,
				_ => panic!("{e}"),
			}
		}
	}

	pub fn dimensions(&self) -> (u16, u16) {
		let terminal_utils::TerminalSize { width, height, .. } = terminal_utils::size().unwrap();
		(width, height)
	}
}
impl Drop for Input {
	fn drop(&mut self) {
		send!("{}{}{}", Action::BracketPaste(false).to_ansi(), Action::AlternateBuffer(false).to_ansi(), Action::FocusReport(false).to_ansi());
	}
}
