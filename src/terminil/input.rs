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
	guard: RawModeGuard,
}
impl Input {
	pub fn new() -> Self {
		println!("Hi");
		let guard = terminal_utils::enable_raw_mode().expect("to be able to enter raw mode");
		send!("{}{}{}", Action::AlternateBuffer(true).to_ansi(), Action::BracketPaste(true).to_ansi(), Action::EraseScrollback.to_ansi());

		Input {
			guard
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
					} else if data[1] == b'[' {
						match data[2..4] {
							[50,126,_,_] => Some(Event::SpecialKey(SpecialKey::Insert)),
							[51,126,_,_] => Some(Event::SpecialKey(SpecialKey::Delete)),
							
							_ => {
								printnl!("{:?} {:?}", &data[0..len], String::from_utf8((&data[2..len]).to_vec()));
								Some(Event::Unknown)
							}
						}
					} else {
						Some(Event::Unknown)
					}
				} else {
					// Some(Event::Paste(data.into()))
					Some(Event::Unknown)
				}
			},
			Err(e) => match e.kind() {
				ErrorKind::UnexpectedEof => None,
				_ => panic!("{e}"),
			}
		}

	}
}
impl Drop for Input {
	fn drop(&mut self) {
		send!("{}{}", Action::BracketPaste(false).to_ansi(), Action::AlternateBuffer(false).to_ansi());
	}
}
