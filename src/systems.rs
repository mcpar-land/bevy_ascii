use bevy::prelude::*;
use crossterm::{
	cursor, queue,
	style::{PrintStyledContent, StyledContent},
	terminal,
};
use std::io::{stdout, Write};

use crate::{Position, ScreenPosition, TermCamera, TermRect, TermRender};

fn build_buffer(
	cam: &TermCamera,
	cam_pos: &Position,
	mut renders: Query<(&TermRender, &Position)>,
) -> crate::Buffer<char> {
	let mut buffer = crate::Buffer::new(cam.size(), ' ');
	for (render, render_pos) in &mut renders.iter() {
		for (c, p) in render.positions() {
			if let Some(c_pos) = (p + render_pos).camera_position(cam, cam_pos) {
				buffer.set(&c_pos.0, c);
			}
		}
	}
	buffer
}

static REFRESH_RATE: f32 = 0.25;
#[derive(Debug, Clone)]
pub struct DrawTimer(pub f32);

pub fn draw_cameras(
	time: Res<Time>,
	mut timer: ResMut<crate::DrawTimer>,
	mut cameras: Query<(&TermCamera, &Position, &ScreenPosition)>,
	mut renders: Query<(&TermRender, &Position)>,
) {
	let mut stdout = stdout();
	queue!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

	for (camera, camera_pos, camera_screen_pos) in &mut cameras.iter() {
		let mut depth_buffer = crate::Buffer::new(camera.size(), std::f32::MIN);

		for (render, render_pos) in &mut renders.iter() {
			for (c, p) in render.positions() {
				// println!("cam: {:?}, char: {:?}", camera.size(), c_pos);
				if let Some(depth_pos) =
					(p + render_pos).camera_position(camera, camera_pos)
				{
					if let Some(buf_val) = depth_buffer.get(&depth_pos.0) {
						// if buf_val < render_pos.0.z() {
						let screen_pos: TermRect = depth_pos.0 + camera_screen_pos.0;
						queue!(
							stdout,
							cursor::MoveTo(screen_pos.w, screen_pos.h),
							render.write_cmd(c)
						)
						.unwrap();
						depth_buffer.set(&depth_pos.0, render_pos.0.z());
					// }
					} else {
						panic!(
							"out of range of depthbuf at {:?} of size {:?}",
							depth_pos,
							depth_buffer.size()
						);
					}
				} else {
					panic!("Not in view of camera {:?}", render_pos);
				}
			}
		}
	}
	stdout.flush().unwrap();
	if timer.0 < REFRESH_RATE {
		timer.0 += time.delta_seconds;
		return;
	} else {
		timer.0 = 0.0;
	}
}
