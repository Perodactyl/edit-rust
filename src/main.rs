use std::io::IsTerminal;

use terminil::input::{Event, Input};

mod terminil;
mod rope;

fn main() -> Result<(),u8> {
	if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
		eprintln!("Stdin and Stdout must be a terminal.");
		return Err(1);
	}
	let mut input = Input::new();

	'main: loop {
		if let Some(ev) = input.get_event() {
			match ev {
				Event::TextChar(c) => {
					send!("{c} ");
					// if c == 3 {
					// 	drop(input);
					// 	println!("Interrupted");
					// 	break 'main Ok(());
					// }
				},
				Event::Paste(data) => {
					println!("paste {data:?}");
				}
				_ => {},
			}
		}
	}
}
