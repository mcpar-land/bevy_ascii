use crate::TermRect;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct Position(pub Vec3);

impl Position {
	pub fn screen_position(
		&self,
		camera: &crate::TermCamera,
		camera_pos: &Position,
		camera_screen_pos: &ScreenPosition,
	) -> Option<ScreenPosition> {
		let offset: Vec3 = self.0 - camera_pos.0;
		let offset = TermRect {
			w: offset.x().round() as i32,
			h: offset.y().round() as i32,
		};
		let on_screen = offset.w.abs() < camera.size().w / 2
			&& offset.h.abs() < camera.size().h / 2;

		if on_screen {
			Some(ScreenPosition(
				camera_screen_pos.0 + (camera.size() / 2) + offset,
			))
		} else {
			None
		}
	}
}

#[derive(Debug, Clone)]
pub struct ScreenPosition(pub TermRect);

impl ScreenPosition {
	pub fn world_position(
		&self,
		camera: &crate::TermCamera,
		camera_pos: &Position,
		camera_screen_pos: &ScreenPosition,
	) -> Option<Position> {
		todo!()
	}
}
