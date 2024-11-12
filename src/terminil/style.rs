use super::ansi::{consts::*, ToAnsi};
use const_format::formatcp;

#[allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
	///RGB TrueColor value
	RGB(u8, u8, u8),
	///256-color value
	Index(u8),
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White
} impl Color {
	#[allow(unused)]
	///Converts a string like "rrggbb" into an instance of Color::RGB
	pub fn try_from_hex(data: &str) -> Option<Self> {
		if data.len() != 6 {
			return None;
		}

		let red = &data[0..=1];
		let green = &data[2..=3];
		let blue = &data[4..=5];

		Some(Color::RGB(
			u8::from_str_radix(red, 16).ok()?,
			u8::from_str_radix(green, 16).ok()?,
			u8::from_str_radix(blue, 16).ok()?
		))
	}

	pub fn as_fg_codes(self, bright: bool) -> String {
		match (bright, self) {
			(false, Color::Black  ) => "30".to_string(),
			(false, Color::Red    ) => "31".to_string(),
			(false, Color::Green  ) => "32".to_string(),
			(false, Color::Yellow ) => "33".to_string(),
			(false, Color::Blue   ) => "34".to_string(),
			(false, Color::Magenta) => "35".to_string(),
			(false, Color::Cyan   ) => "36".to_string(),
			(false, Color::White  ) => "37".to_string(),

			(true,  Color::Black  ) => "90".to_string(),
			(true,  Color::Red    ) => "91".to_string(),
			(true,  Color::Green  ) => "92".to_string(),
			(true,  Color::Yellow ) => "93".to_string(),
			(true,  Color::Blue   ) => "94".to_string(),
			(true,  Color::Magenta) => "95".to_string(),
			(true,  Color::Cyan   ) => "96".to_string(),
			(true,  Color::White  ) => "97".to_string(),

			(_, Color::Index(i)) => format!("38;5;{i}"),
			(_, Color::RGB(r,g,b)) => format!("38;2;{r};{g};{b}"),
		}
	}

	pub fn as_bg_codes(self) -> String {
		match self {
			Color::Black   => "40".to_string(),
			Color::Red     => "41".to_string(),
			Color::Green   => "42".to_string(),
			Color::Yellow  => "44".to_string(),
			Color::Blue    => "44".to_string(),
			Color::Magenta => "45".to_string(),
			Color::Cyan    => "46".to_string(),
			Color::White   => "47".to_string(),

			Color::Index(i) => format!("48;5;{i}"),
			Color::RGB(r,g,b) => format!("48;2;{r};{g};{b}"),
		}
	}
	
	pub fn as_fg(self, bright: bool) -> String {
		format!("{CSI}{}m", self.as_fg_codes(bright))
	}
	
	pub fn as_bg(self) -> String {
		format!("{CSI}{}m", self.as_bg_codes())
	}
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Style {
	fg: Option<(Color, bool)>,
	bg: Option<Color>,
	bold: Option<bool>,
} impl Style {
	#[allow(unused)]
	pub const BOLD: Style = Style {
		fg: None,
		bg: None,
		bold: Some(true),
	};

	#[allow(unused)]
	pub fn fg_bright(color: Color) -> Style {
		Style {
			fg: Some((color, true)),
			bg: None,
			bold: None,
		}
	}

	#[allow(unused)]
	pub fn fg(color: Color) -> Style {
		Style {
			fg: Some((color, false)),
			bg: None,
			bold: None,
		}
	}
	
	#[allow(unused)]
	pub fn bg(color: Color) -> Style {
		Style {
			fg: None,
			bg: Some(color),
			bold: None,
		}
	}

	#[allow(unused)]
	///Returns a new Style, with unset values defaulting to values within `other`.
	pub fn inherit(self, other: Style) -> Style {
		Style {
			fg: self.fg.or(other.fg),
			bg: self.bg.or(other.bg),
			bold: self.bold.or(other.bold),
		}
	}

	#[allow(unused)]
	///Computes a Style from a set of nested styling rules.
	pub fn from_stack(stack: &Vec<Style>) -> Style {
		let mut base = Style::default();
		for style in stack {
			base = style.inherit(base);
		}

		base
	}

	#[allow(unused)]
	pub fn stylize(&self, data: &str) -> String {
		self.to_ansi() + data
	}
}

impl ToAnsi for Style {
	fn to_ansi(&self) -> String {
		return format!("{RESET}{}{}{}",
			if self.bold.unwrap_or_default() {formatcp!("{CSI}1m")} else {""},
			self.fg.map_or(String::default(), |v| v.0.as_fg(v.1)),
			self.bg.map_or(String::default(), |v| v.as_bg()),
		);
	}
}