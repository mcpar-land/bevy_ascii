use crossterm::{cursor, execute, terminal};
use std::io::{stdout, Write};

pub fn quit() {
	execute!(
		stdout(),
		terminal::LeaveAlternateScreen,
		terminal::EnableLineWrap,
		cursor::Show
	)
	.unwrap();
	terminal::disable_raw_mode().unwrap();
	std::process::exit(0);
}
