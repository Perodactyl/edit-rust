use std::io::Read;

use crate::{send, terminil::{prelude::*, input::Input}};

mod rope;
mod buffer;
pub mod style;

use buffer::Buffer;
use rope::Rope;
use style::Stylesheet;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
	Normal,
	Insert,
	Select,
} impl Default for Mode {
	fn default() -> Self {
		Mode::Normal
	}
}

#[derive(Debug, Clone)]
pub struct File {
	is_readonly: bool,
	path: String,
} impl File {
	pub fn open(path: &str, readonly: bool) -> Self {
		File {
			path: path.to_string(),
			is_readonly: readonly,
		}
	}
	
	pub fn read(&self) -> Result<Vec<u8>, std::io::Error> {
		let mut f = std::fs::File::open(&self.path)?;
		let mut out = Vec::new();
		f.read_to_end(&mut out)?;

		Ok(out)
	}
	pub fn read_to_rope(&self) -> Result<Rope, std::io::Error> {
		let data = self.read()?;
		let mut rope = Rope::new();
		rope.insert_bytes(&data, 0);

		Ok(rope)
	}
}

#[derive(Debug, Clone)]
pub enum Source {
	String(String),
	File(File),
}

pub struct Editor {
	input: Input,
	stylesheet: Stylesheet,
	buffers: Vec<Buffer>,
	current_buffer: usize,
	mode: Mode,
} impl Editor {
	pub fn new() -> Self {
		Editor {
			input: Input::new(),
			buffers: Vec::new(),
			current_buffer: 0,
			mode: Mode::default(),
			stylesheet: Stylesheet::default(),
		}
	}
	pub fn open(&mut self, source: Source) -> Result<(), std::io::Error> {
		self.buffers.push(Buffer::new(source)?);
		Ok(())
	}
	pub fn set_style(&mut self, stylesheet: Stylesheet) {
		self.stylesheet = stylesheet;
	}
	fn render(&self) {
		print!("{}{}",Action::CursorVisible(false).to_ansi(),Motion::ScreenStart.to_ansi());
		print!("{}{}",self.stylesheet.background.as_bg(), self.stylesheet.text.as_fg());
		let (width, height) = self.input.dimensions();
		for byte in &self.buffers[self.current_buffer] {
			if byte == b'\n' {
				print!("{}\n{}",Action::EraseToLineEnd.to_ansi(), Motion::LineStart.to_ansi());
			} else {
				print!("{}", char::from_u32(byte as u32).unwrap_or('@'));
			}
		}
		print!("{}{}", Motion::ToPosition(0, height-2).to_ansi(), self.stylesheet.statusbar.render(self));
		send!();
	}
	///Starts the main loop of the editor.
	pub fn run(&mut self) {
		self.render();
		'main: loop {
			if let Some(ev) = self.input.get_event() {
				use crate::terminil::input::{Event, SpecialKey};
				match ev {
					Event::SpecialKey(SpecialKey::Escape) => break 'main,
					e => self.buffers[self.current_buffer].trigger_event(e, &self.mode),
				}
			}
			self.render();
		}
	}
} impl Drop for Editor {
	fn drop(&mut self) {
		print!("{}",Action::CursorVisible(true).to_ansi());
	}
}