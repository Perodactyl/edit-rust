use super::{rope::{Rope, RopeIterator}, Editor, Mode, Source};
use crate::terminil::prelude::*;

#[derive(Debug, Clone, Copy)]
struct SelectionPoint {
	line: u32,
	col: u32,
	target_col: u32,
}

#[derive(Debug, Clone, Copy)]
enum Selection {
	Point(SelectionPoint),
	Range { start_line: u32, start_col: u32, end: SelectionPoint },
} impl Selection {
	pub fn move_by(&mut self, offsetX: i32, offsetY: i32) {
		match self {
			Selection::Point(p) => {
				// ? In order to move, we need to know the length of the lines around the selection.
				// ? That way the cursor can wrap and/or set its target_col.
				// ! It seems as if my modularity is failing here.
			}
			Selection::Range { start_line, start_col, end } => {

			}
		}
	}
}

pub struct Buffer {
	selections: Vec<Selection>,
	source: Source,
	rope: Rope,
} impl Buffer {
	pub fn new(source: Source) -> Result<Self, std::io::Error> {
		let mut r;
		match &source {
			Source::File(f) => r = f.read_to_rope()?,
			Source::String(s) => {
				r = Rope::new();
				r.insert_bytes(s.as_bytes(), 0);
			}
		}
		let mut selections =  Vec::with_capacity(1);
		selections.push(Selection::Point(SelectionPoint { line: 1, col: 1, target_col: 1 }));

		Ok(Buffer {
			selections,
			source,
			rope: r,
		})
	}
	fn move_cursor(&mut self, offsetX: i32, offsetY: i32) {
		for sel in &mut self.selections {
			sel.move_by(offsetX, offsetY);
		}
	}
	pub fn trigger_event(&mut self, event: Event, mode: &Mode) {
		match (mode, event) {
			(_, Event::SpecialKey(SpecialKey::Up))    => self.move_cursor( 0, -1),
			(_, Event::SpecialKey(SpecialKey::Down))  => self.move_cursor( 0,  1),
			(_, Event::SpecialKey(SpecialKey::Left))  => self.move_cursor(-1,  0),
			(_, Event::SpecialKey(SpecialKey::Right)) => self.move_cursor( 1,  0),
			_ => {}
		}
	}
} impl<'a> IntoIterator for &'a Buffer {
	type IntoIter = RopeIterator<'a>;
	type Item = u8;
	fn into_iter(self) -> Self::IntoIter {
		self.rope.into_iter()
	}
}
