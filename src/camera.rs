use crate::pos::{Position, ScreenPosition};
use crate::primitives::TermRect;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct TermCameraComponents {
	camera: TermCamera,
	world_position: Position,
	screen_position: ScreenPosition,
}

impl Default for TermCameraComponents {
	fn default() -> Self {
		Self {
			camera: TermCamera::default(),
			world_position: Position(Vec3::zero()),
			screen_position: ScreenPosition(TermRect { w: 0, h: 0 }),
		}
	}
}

#[derive(Debug, Clone)]
pub struct TermCamera {
	pub name: Option<String>,
	pub sizing: TermCameraSizing,
	pub culling: TermCameraCulling,
}

impl TermCamera {
	pub fn size(&self) -> TermRect {
		match self.sizing {
			TermCameraSizing::Auto => {
				let (term_w, term_h) = crossterm::terminal::size().unwrap();
				TermRect {
					w: term_w,
					h: term_h,
				}
			}
			TermCameraSizing::Fixed(size) => size,
		}
	}
}

impl Default for TermCamera {
	fn default() -> Self {
		Self {
			name: None,
			sizing: TermCameraSizing::Auto,
			culling: TermCameraCulling::All,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum TermCameraSizing {
	Auto,
	Fixed(TermRect),
}

#[derive(Debug, Clone, Copy)]
pub enum TermCameraCulling {
	/// For each square on screen, draw the object with the largest Z value.
	All,
	/// For each square on the screen, draw the object with the largezt Z value
	/// that is smaller than the camera's Z value.
	/// (Don't draw anything that's behind the camera)
	FrontOnly,
}
