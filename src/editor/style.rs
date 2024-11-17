use std::fmt::Display;

use serde::Deserialize;

use crate::terminil::prelude::*;

use super::Editor;

#[derive(Deserialize, Debug)]
pub struct Statusbar {
	background: Color,
	foreground: Color,
	modules: Vec<StatusbarModule>,
} impl Statusbar {
	pub fn render(&self, context: &Editor) -> String {
		let mut sections = Vec::new();
		let mut section = String::new();

		for module in &self.modules {
			match module {
				StatusbarModule::Flex => {
					sections.push(section);
					section = String::new();
				},
				StatusbarModule::Text { content, style } => {
					section += content;
				},
				StatusbarModule::Mode => {
					use super::Mode;
					section += match context.mode {
						Mode::Normal => "Normal",
						Mode::Insert => "Insert",
						Mode::Select => "Select",
					}
				},
				StatusbarModule::Whitespace { length } => {
					section += &(" ").repeat(*length as usize);
				},
				_ => {
					section += "@";
				}
			}
		}
		sections.push(section);
		if sections.len() == 1 {
			sections.push(String::new());
		}

		use crate::terminil::ansi::strip;

		let width = context.input.dimensions().0;

		let mut out = self.background.as_bg() + &self.foreground.as_fg() + &Action::EraseToLineEnd.to_ansi();

		let spacing = width / (sections.len()-1) as u16;
		let section_count = sections.len();

		for (i, section) in sections.into_iter().enumerate() {
			if i == 0 {
				out += &format!("{}{section}", Motion::LineStart.to_ansi());
			} else if i == section_count - 1 {
				out += &format!("{}{section}", Motion::LineAbsolute(width-strip(&section).len() as u16).to_ansi());
			} else {
				let start = (i as u16) * spacing - (strip(&section).len() as u16)/2;
				out += &format!("{}{section}", Motion::LineAbsolute(start).to_ansi());
			}
		}

		out
	}
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum StatusbarModule {
	Whitespace { length: u16 },
	Flex,
	Text { content: String, style: Option<Style> },
	Row,
	Column,
	Filename,
	RowPercent,
	Mode,
}

#[derive(Deserialize, Debug)]
pub struct Stylesheet {
	pub background: Color,
	pub text: Color,

	pub statusbar: Statusbar
} impl Default for Stylesheet {
	fn default() -> Self {
		Stylesheet {
			background: Color::Uncolored,
			text: Color::Uncolored,
			statusbar: Statusbar {
				background: Color::White,
				foreground: Color::Black,
				modules: vec![
					StatusbarModule::Whitespace { length: 1 },
					StatusbarModule::Mode,
					StatusbarModule::Whitespace { length: 1 },
					StatusbarModule::Filename,
				]
			}
		}
	}
}