use super::ansi::{consts::*, ToAnsi};

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Motion {
	None,
	Up,
	Down,
	Left,
	Right,
	UpBy(u16),
	DownBy(u16),
	LeftBy(u16),
	RightBy(u16),
	NextLine,
	PrevLine,
	NextLines(u16),
	PrevLines(u16),
	RelativeLine(i16),
	LineStart,
	ScreenStart,
	///Expects `(x, y)` as opposed to `(row, col)`. Zero-indexed.
	ToPosition(u16, u16),
	ScrollUp,
	ScrollDown,
} impl ToAnsi for Motion {
	fn to_ansi(&self) -> String {
		match self {
			Motion::None  => format!(""),
			Motion::Up    => format!("{CSI}A"),
			Motion::Down  => format!("{CSI}B"),
			Motion::Left  => format!("{CSI}C"),
			Motion::Right => format!("{CSI}D"),

			Motion::UpBy(n)    => format!("{CSI}{n}A"),
			Motion::DownBy(n)  => format!("{CSI}{n}B"),
			Motion::LeftBy(n)  => format!("{CSI}{n}C"),
			Motion::RightBy(n) => format!("{CSI}{n}D"),

			Motion::NextLine => format!("{CSI}E"),
			Motion::PrevLine => format!("{CSI}F"),
			Motion::NextLines(n) => format!("{CSI}{n}E"),
			Motion::PrevLines(n) => format!("{CSI}{n}E"),
			Motion::RelativeLine(n) => if n.is_positive() {
				format!("{CSI}{n}E")
			} else {
				format!("{CSI}{}F", n.abs())
			},

			Motion::LineStart   => format!("{CSI}G"),
			Motion::ScreenStart => format!("{CSI}H"),

			Motion::ToPosition(x, y) => format!("{CSI}{};{}H", y+1, x+1),

			Motion::ScrollUp   => format!("{CSI}S"),
			Motion::ScrollDown => format!("{CSI}T"),
		}
	}
}  impl Motion {
	#[allow(unused)]
	pub fn as_ansi(&self) -> String {
		self.to_ansi()
	}
} impl Default for Motion {
	fn default() -> Self {
		Motion::None
	}
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
	EraseToScreenEnd,
	EraseToScreenStart,
	EraseToLineEnd,
	EraseToLineStart,
	EraseLine,
	//Also moves the cursor to the top-left corner for compatibility.
	EraseScreen,
	//Also moves the cursor to the top-left corner for compatibility.
	EraseScrollback,

	CursorVisible(bool),
	AlternateBuffer(bool),
	BracketPaste(bool),
	FocusReport(bool),
} impl ToAnsi for Action {
	fn to_ansi(&self) -> String {
		match self {
			Action::EraseToLineEnd   => format!("{CSI}0K"),
			Action::EraseToLineStart => format!("{CSI}1K"),
			Action::EraseLine        => format!("{CSI}2K"),

			Action::EraseToScreenEnd      => format!("{CSI}0J"),
			Action::EraseToScreenStart    => format!("{CSI}1J"),

			Action::EraseScreen           => format!("{CSI}2J{CSI}H"),
			Action::EraseScrollback       => format!("{CSI}3J{CSI}H"),

			Action::CursorVisible(v)   => if *v { format!("{CSI}?25h")   } else { format!("{CSI}?25l")   },
			Action::AlternateBuffer(b) => if *b { format!("{CSI}?1049h") } else { format!("{CSI}?1049l") },
			Action::BracketPaste(b)    => if *b { format!("{CSI}?2004h") } else { format!("{CSI}?2004l") },
			Action::FocusReport(r)     => if *r { format!("{CSI}?1004h") } else { format!("{CSI}?1004l") },
		}
	}
}