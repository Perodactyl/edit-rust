use std::{io::{stdin, ErrorKind, Read}, mem};

use terminal_utils::RawModeGuard;

use crate::terminil::{ansi::ToAnsi, control::Action};

pub enum Event {
	Char(u8),
	Paste(Vec<u8>),
	Focus(bool),
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
		println!("{}{}{}", Action::AlternateBuffer(true).to_ansi(), Action::BracketPaste(true).to_ansi(), Action::EraseScrollback.to_ansi());

		Input {
			guard
		}
	}
	pub fn get_event(&mut self) -> Option<Event> {
		let mut data = [0;16];

		match stdin().read(&mut data) {
			Ok(len) => {
				let data = mem::take(&mut data);
				println!("READ");
				if len == 0 {
					None
				} else {
					// Some(Event::Paste(data.into()))
					Some(Event::Char(data[0]))
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
		// print!("{}", Action::BracketPaste(false).to_ansi());
		print!("{}", Action::AlternateBuffer(false).to_ansi());
	}
}
