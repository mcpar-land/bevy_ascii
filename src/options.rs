pub struct TerminalOptions {
	pub quit_on_esc: bool,
	pub display_fps_counter: bool,
}

impl Default for TerminalOptions {
	fn default() -> Self {
		Self {
			quit_on_esc: true,
			display_fps_counter: false,
		}
	}
}
