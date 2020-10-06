pub struct TerminalOptions {
	pub quit_on_esc: bool,
}

impl Default for TerminalOptions {
	fn default() -> Self {
		Self { quit_on_esc: true }
	}
}
