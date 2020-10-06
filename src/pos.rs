use crate::TermRect;
use bevy::prelude::*;

/// Component for giving an entity a position in 3d space.
///
/// Consider it to be `Transform` without the rotation or scale capabilities.
#[derive(Debug, Clone)]
pub struct Position(pub Vec3);

impl Position {
	pub fn screen_position(
		&self,
		camera: &crate::TermCamera,
		camera_pos: &Position,
		camera_screen_pos: &ScreenPosition,
	) -> Option<ScreenPosition> {
		self
			.camera_position(camera, camera_pos)
			.map(|p| ScreenPosition(camera_screen_pos.0 + p.0))
	}

	pub fn camera_position(
		&self,
		camera: &crate::TermCamera,
		camera_pos: &Position,
	) -> Option<ScreenPosition> {
		let offset: Vec3 = self.0 - camera_pos.0;
		let offset_x = offset.x().round() as i32;
		let offset_y = offset.y().round() as i32;
		let on_screen = (offset_x.abs() as u16) < camera.size().w / 2
			&& (offset_y.abs() as u16) < camera.size().h / 2;

		if on_screen {
			let mut screen_pos: TermRect = camera.size() / 2;
			screen_pos.w = (screen_pos.w as i32 + offset_x) as u16;
			screen_pos.h = (screen_pos.h as i32 + offset_y) as u16;

			Some(ScreenPosition(screen_pos))
		} else {
			None
		}
	}
}

impl std::ops::Add for Position {
	type Output = Position;

	fn add(self, rhs: Self) -> Self::Output {
		Position(self.0 + rhs.0)
	}
}

impl std::ops::Add<&Position> for Position {
	type Output = Position;

	fn add(self, rhs: &Position) -> Self::Output {
		Position(self.0 + rhs.0)
	}
}

/// Similar to [Position](struct.Position.html), but for locations within screen space.
///
/// Used to put an entity in a location on-screen, rather than in the world.
/// Does not require a camera to view.
#[derive(Debug, Clone)]
pub struct ScreenPosition(pub TermRect);

impl ScreenPosition {
	// TODO
	// pub fn world_position(
	// 	&self,
	// 	camera: &crate::TermCamera,
	// 	camera_pos: &Position,
	// 	camera_screen_pos: &ScreenPosition,
	// ) -> Option<Position> {
	// 	todo!()
	// }
}

impl std::ops::Add for ScreenPosition {
	type Output = ScreenPosition;

	fn add(self, rhs: Self) -> Self::Output {
		ScreenPosition(self.0 + rhs.0)
	}
}

impl std::ops::Add<TermRect> for ScreenPosition {
	type Output = ScreenPosition;

	fn add(self, rhs: TermRect) -> Self::Output {
		ScreenPosition(self.0 + rhs)
	}
}
