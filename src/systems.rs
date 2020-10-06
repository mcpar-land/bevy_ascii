use bevy::prelude::*;
use crossterm::{
	cursor, queue,
	style::{ContentStyle, Print, StyledContent},
};
use std::io::{stdout, Write};

use crate::{Position, ScreenPosition, TermCamera, TermRender};

fn build_buffer_string(
	cam: &TermCamera,
	cam_pos: &Position,
	renders: &mut Query<(&TermRender, &Position)>,
) -> String {
	let mut buffer = crate::Buffer::new(
		cam.size(),
		(
			StyledContent::new(ContentStyle::default(), ' '),
			std::f32::MIN,
		),
	);
	for (render, render_pos) in &mut renders.iter() {
		for (c, p) in render.positions() {
			if let Some(c_pos) = (p + render_pos).camera_position(cam, cam_pos) {
				let (buf_val, buf_depth) = buffer.get_mut(&c_pos.0).unwrap();
				if *buf_depth < render_pos.0.z() {
					*buf_val = StyledContent::new(render.style, c);
					*buf_depth = render_pos.0.z();
				}
			}
		}
	}
	buffer.map(|v| v.0).to_string()
}

pub fn draw_cameras(
	mut cameras: Query<(&TermCamera, &Position, &ScreenPosition)>,
	mut renders: Query<(&TermRender, &Position)>,
) {
	let mut stdout = stdout();

	for (camera, camera_pos, camera_screen_pos) in &mut cameras.iter() {
		let buffer_string = build_buffer_string(camera, camera_pos, &mut renders);

		queue!(
			stdout,
			cursor::MoveTo(camera_screen_pos.0.w, camera_screen_pos.0.h),
			Print(buffer_string)
		)
		.unwrap();
	}
	stdout.flush().unwrap();
}

// TODO: Currently, this doesn't work. https://github.com/bevyengine/bevy/issues/636
pub fn handle_quit(
	options: Res<crate::TerminalOptions>,
	keyboard_input: Res<Input<KeyCode>>,
) {
	if keyboard_input.pressed(KeyCode::Q) && options.quit_on_esc {
		crate::util::quit();
	}
}
