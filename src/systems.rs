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

pub fn render(
	mut cameras: Query<(&TermCamera, &Position, &ScreenPosition)>,
	mut world_renders: Query<(&TermRender, &Position)>,
	mut screen_renders: Query<(&TermRender, &ScreenPosition)>,
) {
	let mut stdout = stdout();

	for (camera, camera_pos, camera_screen_pos) in &mut cameras.iter() {
		let buffer_string =
			build_buffer_string(camera, camera_pos, &mut world_renders);

		queue!(
			stdout,
			cursor::MoveTo(camera_screen_pos.0.w, camera_screen_pos.0.h),
			Print(buffer_string)
		)
		.unwrap();
	}
	for (render, render_pos) in &mut screen_renders.iter() {
		queue!(
			stdout,
			cursor::MoveTo(render_pos.0.w, render_pos.0.h),
			render.write_cmd()
		)
		.unwrap();
	}
	stdout.flush().unwrap();
}

pub fn handle_quit(
	options: Res<crate::TerminalOptions>,
	events: Res<Events<crate::TermEvent>>,
	mut state: Local<crate::TermState>,
) {
	if options.quit_on_esc {
		for event in state.reader.iter(&events) {
			if let crate::TermEvent::Key(e) = event {
				if let crate::TermKeyCode::Esc = e.code {
					crate::util::quit();
				}
			}
		}
	}
}
