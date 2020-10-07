use bevy::prelude::*;
use crossterm::{
	cursor, queue,
	style::{ContentStyle, Print, StyledContent},
};
use std::io::{stdout, Write};

use crate::{Position, ScreenPosition, TermCamera, TermRender};

#[derive(Clone, Copy, Debug)]
pub struct RendersSize(pub usize);

fn build_buffer_string(
	cam: &TermCamera,
	cam_pos: &Position,
	renders: &mut Query<(&TermRender, &Position)>,
	renders_size: &mut RendersSize,
) -> String {
	let mut buffer = crate::Buffer::new(
		cam.size(),
		(
			StyledContent::new(ContentStyle::default(), ' '),
			std::f32::MIN,
		),
	);

	let mut bytes: usize = 0;

	for (render, render_pos) in &mut renders.iter() {
		use std::mem::size_of_val;
		bytes += size_of_val(&*render.body);
		bytes += size_of_val(&render.style);
		bytes += size_of_val(&render_pos);
	}

	renders_size.0 = bytes;

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
	mut renders_size: ResMut<RendersSize>,
	mut cameras: Query<(&TermCamera, &Position, &ScreenPosition)>,
	mut world_renders: Query<(&TermRender, &Position)>,
	mut screen_renders: Query<(&TermRender, &ScreenPosition)>,
) {
	let mut stdout = stdout();

	for (camera, camera_pos, camera_screen_pos) in &mut cameras.iter() {
		let buffer_string = build_buffer_string(
			camera,
			camera_pos,
			&mut world_renders,
			&mut renders_size,
		);

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
