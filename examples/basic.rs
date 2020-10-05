use bevy::prelude::*;
use bevy_ascii::*;
use crossterm::{
	execute,
	style::{Color, ContentStyle},
	terminal,
};
use std::io::{stdout, Write};

fn setup(mut commands: Commands) {
	commands
		.spawn(TermCameraComponents::default())
		.spawn(TermDrawComponents {
			c: TermDraw {
				body: "h".to_string(),
				..Default::default()
			},
			position: Position(Vec3::new(5.0, 5.0, 0.0)),
		});

	for (i, (fg, bg)) in [
		(Color::Red, Color::Black),
		(Color::Red, Color::Cyan),
		(Color::Green, Color::Black),
		(Color::DarkGreen, Color::Blue),
		(Color::Yellow, Color::Black),
		(Color::DarkYellow, Color::Black),
		(Color::Blue, Color::Black),
		(Color::Magenta, Color::Black),
		(Color::Cyan, Color::Black),
	]
	.iter()
	.enumerate()
	{
		commands.spawn(TermDrawComponents {
			c: TermDraw {
				body: format!("{}", i),
				style: ContentStyle::new().foreground(*fg).background(*bg),
			},
			position: Position(Vec3::new(i as f32, i as f32, 0.0)),
		});
	}
}

fn wiggle_camera(
	time: Res<Time>,
	mut cameras: Query<(&TermCamera, &mut Position)>,
) {
	let wiggle_amount = 1f64;
	let sin = time.seconds_since_startup.sin().to_degrees() * wiggle_amount
		- wiggle_amount / 2.0;

	for (cam, mut cam_pos) in &mut cameras.iter() {
		cam_pos.0.set_x(sin as f32);
	}
}

fn main() {
	App::build()
		.add_plugin(TermPlugin)
		.add_startup_system(setup.system())
		.add_system(wiggle_camera.system())
		.run();
	execute!(stdout(), terminal::LeaveAlternateScreen).unwrap();
}
