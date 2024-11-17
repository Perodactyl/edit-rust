use std::io::IsTerminal;

use editor::{style::Stylesheet, Editor};
use terminil::input::{Event, Input, SpecialKey};

mod terminil;
mod editor;

fn main() -> Result<(),u8> {
	if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
		eprintln!("Stdin and Stdout must be a terminal.");
		return Err(1);
	}

	let styles = match toml::from_str::<Stylesheet>(include_str!("../theme.toml")) {
		Ok(t) => t,
		Err(e) => {
			panic!("{}", e);
		}
	};
	
	let mut editor = Editor::new();
	editor.set_style(styles);
	printnl!("{:?}", editor.open(editor::Source::File(editor::File::open("test.txt", false))));
	editor.run();

	println!("Exited.");
	Ok(())
}
