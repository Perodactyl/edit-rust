use super::ansi::{consts::*, ToAnsi};
use const_format::formatcp;

#[allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Color {
	///Uses the last color that was defined.
	Unset,
	///Uses the terminal default colors.
	Uncolored,
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	White,

	BrightBlack,
	BrightRed,
	BrightGreen,
	BrightYellow,
	BrightBlue,
	BrightMagenta,
	BrightCyan,
	BrightWhite,

	#[serde(untagged)]
	///RGB TrueColor value
	RGB(u8, u8, u8),
	#[serde(untagged)]
	///256-color value
	Index(u8),
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

	fn as_fg_codes(self) -> String {
		match self {
			Color::Unset         => "".to_string(),
			Color::Uncolored     => "".to_string(),
			Color::Black         => "30".to_string(),
			Color::Red           => "31".to_string(),
			Color::Green         => "32".to_string(),
			Color::Yellow        => "33".to_string(),
			Color::Blue          => "34".to_string(),
			Color::Magenta       => "35".to_string(),
			Color::Cyan          => "36".to_string(),
			Color::White         => "37".to_string(),

			Color::BrightBlack   => "90".to_string(),
			Color::BrightRed     => "91".to_string(),
			Color::BrightGreen   => "92".to_string(),
			Color::BrightYellow  => "93".to_string(),
			Color::BrightBlue    => "94".to_string(),
			Color::BrightMagenta => "95".to_string(),
			Color::BrightCyan    => "96".to_string(),
			Color::BrightWhite   => "97".to_string(),

			Color::Index(i) => format!("38;5;{i}"),
			Color::RGB(r,g,b) => format!("38;2;{r};{g};{b}"),
		}
	}

	fn as_bg_codes(self) -> String {
		match self {
			Color::Unset         => "".to_string(),
			Color::Uncolored     => "".to_string(),
			Color::Black         => "40".to_string(),
			Color::Red           => "41".to_string(),
			Color::Green         => "42".to_string(),
			Color::Yellow        => "43".to_string(),
			Color::Blue          => "44".to_string(),
			Color::Magenta       => "45".to_string(),
			Color::Cyan          => "46".to_string(),
			Color::White         => "47".to_string(),

			Color::BrightBlack   => "100".to_string(),
			Color::BrightRed     => "101".to_string(),
			Color::BrightGreen   => "102".to_string(),
			Color::BrightYellow  => "103".to_string(),
			Color::BrightBlue    => "104".to_string(),
			Color::BrightMagenta => "105".to_string(),
			Color::BrightCyan    => "106".to_string(),
			Color::BrightWhite   => "107".to_string(),

			Color::Index(i) => format!("48;5;{i}"),
			Color::RGB(r,g,b) => format!("48;2;{r};{g};{b}"),
		}
	}
	
	pub fn as_fg(self) -> String {
		if let Color::Uncolored = self {
			format!("{CSI}39m")
		} else if let Color::Unset = self {
			format!("")
		} else {
			format!("{CSI}{}m", self.as_fg_codes())
		}
	}
	
	pub fn as_bg(self) -> String {
		if let Color::Uncolored = self {
			format!("{CSI}49m")
		} else if let Color::Unset = self {
			format!("")
		} else {
			format!("{CSI}{}m", self.as_bg_codes())
		}
	}
}

#[derive(Default, Debug, Clone, Copy, serde::Deserialize)]
pub struct Style {
	fg: Option<Color>,
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
	pub fn fg(color: Color) -> Style {
		Style {
			fg: Some(color),
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
		return format!("{}{}{}",
			if self.bold.unwrap_or_default() {formatcp!("{CSI}1m")} else {formatcp!("{CSI}22m")},
			self.fg.map_or(String::default(), |v| v.as_fg()),
			self.bg.map_or(String::default(), |v| v.as_bg()),
		);
	}
}