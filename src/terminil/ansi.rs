pub mod consts {
	use const_format::formatcp;
	pub const ESC: &str = "\x1b";
	pub const CSI: &str = formatcp!("{ESC}[");
	pub const RESET: &str = formatcp!("{CSI}0m");
}

pub trait ToAnsi {
	fn to_ansi(&self) -> String;
}

impl<U: std::fmt::Display> ToAnsi for U {
	fn to_ansi(&self) -> String {
		self.to_string()
	}
}

#[allow(unused)]
///Returns a duplicate string with all ANSI escape codes stripped.
pub fn strip(text: &str) -> String {
	let r = regex::Regex::new(r"[\u001b\u009b][[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]").expect("this to be a valid regex");
	r.replace_all(text, "").to_string()
}