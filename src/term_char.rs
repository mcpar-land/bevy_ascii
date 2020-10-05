use bevy::prelude::*;
use crossterm::style::{ContentStyle, PrintStyledContent, StyledContent};

#[derive(Bundle)]
pub struct TermDrawComponents {
	pub c: TermDraw,
	pub position: crate::Position,
}

pub struct TermDraw {
	pub body: String,
	pub style: ContentStyle,
}

impl TermDraw {
	pub fn write_cmd(&self) -> PrintStyledContent<String> {
		let styled = StyledContent::new(self.style, self.body.clone());
		PrintStyledContent(styled)
	}
}

impl Default for TermDraw {
	fn default() -> Self {
		Self {
			body: "?".to_string(),
			style: ContentStyle::default(),
		}
	}
}
