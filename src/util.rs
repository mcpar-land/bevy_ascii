use crossterm::{execute, terminal};
use std::io::{stdout, Write};

pub fn quit() {
	execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
	terminal::disable_raw_mode().unwrap();
	std::process::exit(0);
}
