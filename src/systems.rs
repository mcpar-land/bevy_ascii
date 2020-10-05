use bevy::prelude::*;
use crossterm::{cursor, queue, terminal};
use std::io::{stdout, Write};

use crate::{Position, ScreenPosition, TermCamera, TermDraw};

pub fn draw_cameras(
	mut cameras: Query<(&TermCamera, &Position, &ScreenPosition)>,
	mut drawables: Query<(&TermDraw, &Position)>,
) {
	let mut stdout = stdout();
	queue!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
	for (camera, camera_pos, camera_screen_pos) in &mut cameras.iter() {
		for (c, c_pos) in &mut drawables.iter() {
			// println!("cam: {:?}, char: {:?}", camera.size(), c_pos);
			if let Some(screen_pos) =
				c_pos.screen_position(camera, camera_pos, camera_screen_pos)
			{
				queue!(
					stdout,
					cursor::MoveTo(screen_pos.0.w as u16, screen_pos.0.h as u16),
					c.write_cmd()
				)
				.unwrap();
			}
		}
	}
	stdout.flush().unwrap();
}
