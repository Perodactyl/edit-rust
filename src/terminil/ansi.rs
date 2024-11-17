
pub mod consts {
	use const_format::formatcp;
	pub const ESC: &str = "\x1b";
	pub const CSI: &str = formatcp!("{ESC}[");
	pub const RESET: &str = formatcp!("{CSI}0m");
}

pub trait ToAnsi {
	fn to_ansi(&self) -> String;
}

// impl<U: std::fmt::Display> ToAnsi for U {
// 	fn to_ansi(&self) -> String {
// 		self.to_string()
// 	}
// }

///Returns a duplicate string with all ANSI escape codes stripped.
pub fn strip(text: &str) -> String {
	let r = regex::Regex::new(r"[\u001b\u009b][\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]").expect("this to be a valid regex");
	r.replace_all(text, "").to_string()
}

///This macro acts like `print!` but it flushes stdout afterward, which forces the data through.
///Calling it with no parameters only flushes stdout.
#[macro_export]
macro_rules! send {
	($fmt:literal) => {{
		use std::io::{stdout, Write};
		print!($fmt);
		let _ = stdout().flush();
	}};
	($fmt:literal,$($args:expr),*) => {{
		use std::io::{stdout, Write};
		print!($fmt, $($args),*);
		let _ = stdout().flush();
	}};
	() => {{
		use std::io::{stdout, Write};
		let _ = stdout().flush();
	}}
}

///This macro acts like `println!` but it resets the cursor position to the start of the next line.
#[macro_export]
macro_rules! printnl {
	($fmt:literal) => {{
		print!($fmt);
		send!("\n\x1B[G");
	}};
	($fmt:literal,$($args:expr),*) => {{
		print!($fmt, $($args),*);
		send!("\n\x1B[G");
	}};
}