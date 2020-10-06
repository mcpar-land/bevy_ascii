use crate::Position;
use bevy::prelude::*;
use crossterm::style::{ContentStyle, PrintStyledContent, StyledContent};

/// Bundle to display a [TermRender](struct.TermRender.html) in world space
#[derive(Bundle)]
pub struct TermRenderWorldBundle {
	pub render: TermRender,
	pub position: crate::Position,
}

/// budnle to display a [TermRender](struct.TermRender.html) in screen space
#[derive(Bundle)]
pub struct TermRenderScreenBundle {
	pub render: TermRender,
	pub position: crate::ScreenPosition,
}

pub struct TermRender {
	pub body: String,
	pub style: ContentStyle,
}

impl TermRender {
	pub fn new<T: Into<String>>(body: T) -> Self {
		Self {
			body: body.into(),
			..Default::default()
		}
	}

	pub fn write_cmd(&self) -> PrintStyledContent<String> {
		let styled = StyledContent::new(self.style, self.body.clone());
		PrintStyledContent(styled)
	}
	pub fn positions_rect(&self) -> Vec<(char, crate::TermRect)> {
		let mut ps = vec![];
		let mut h: u16 = 0;
		let lines = self.body.split('\n');
		for line in lines {
			for (i, c) in line.char_indices() {
				if c != ' ' {
					ps.push((c, crate::TermRect { w: i as u16, h }));
				}
			}
			h += 1;
		}
		return ps;
	}
	pub fn positions(&self) -> Vec<(char, crate::Position)> {
		self
			.positions_rect()
			.iter()
			.map(|(c, r)| (*c, Position(Vec3::new(r.w as f32, r.h as f32, 0.0))))
			.collect::<Vec<(char, Position)>>()
	}
}

impl Default for TermRender {
	fn default() -> Self {
		Self {
			body: "?".to_string(),
			style: ContentStyle::default(),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::TermRect;

	#[test]
	fn test_positions() {
		assert_eq!(
			TermRender::new("0").positions_rect(),
			vec![('0', TermRect { w: 0, h: 0 })]
		);
		assert_eq!(
			TermRender::new("four").positions_rect(),
			vec![
				('f', TermRect { w: 0, h: 0 }),
				('o', TermRect { w: 1, h: 0 }),
				('u', TermRect { w: 2, h: 0 }),
				('r', TermRect { w: 3, h: 0 }),
			]
		);
		assert_eq!(
			TermRender::new("12\n34").positions_rect(),
			vec![
				('1', TermRect { w: 0, h: 0 }),
				('2', TermRect { w: 1, h: 0 }),
				('3', TermRect { w: 0, h: 1 }),
				('4', TermRect { w: 1, h: 1 }),
			]
		);
		assert_eq!(
			TermRender::new("7 8 9\n4   6\n1 2 3").positions_rect(),
			vec![
				('7', TermRect { w: 0, h: 0 }),
				('8', TermRect { w: 2, h: 0 }),
				('9', TermRect { w: 4, h: 0 }),
				('4', TermRect { w: 0, h: 1 }),
				('6', TermRect { w: 4, h: 1 }),
				('1', TermRect { w: 0, h: 2 }),
				('2', TermRect { w: 2, h: 2 }),
				('3', TermRect { w: 4, h: 2 }),
			]
		)
	}
}
