use bevy::prelude::*;
use bevy_ascii::*;

struct Tracker;

fn setup(mut commands: Commands) {
	commands.spawn(TermCameraComponents::default());

	for (i, (fg, bg)) in [
		(TermColor::Red, TermColor::Black),
		(TermColor::Red, TermColor::Cyan),
		(TermColor::Green, TermColor::Black),
		(TermColor::DarkGreen, TermColor::Blue),
		(TermColor::Yellow, TermColor::Black),
		(TermColor::DarkYellow, TermColor::Black),
		(TermColor::Blue, TermColor::Black),
		(TermColor::Magenta, TermColor::Black),
		(TermColor::Cyan, TermColor::Black),
	]
	.iter()
	.enumerate()
	{
		commands.spawn(TermRenderWorldBundle {
			render: TermRender {
				body: format!("This is in world space! "),
				style: ContentStyle::new().foreground(*fg).background(*bg),
			},
			position: Position(Vec3::new(i as f32, i as f32, 0.0)),
		});
	}

	commands.spawn(TermRenderScreenBundle {
		render: TermRender {
			body: format!("This is in screen space!"),
			..Default::default()
		},
		position: ScreenPosition(TermRect { w: 30, h: 5 }),
	});

	commands
		.spawn(TermRenderWorldBundle {
			render: TermRender {
				body: format!("This is in both screen and world space!"),
				..Default::default()
			},
			position: Position(Vec3::new(-10.0, -5.0, 0.0)),
		})
		.with(ScreenPosition(TermRect { w: 20, h: 8 }))
		.with(Tracker);
}

fn wiggle_camera(
	time: Res<Time>,
	mut cameras: Query<(&TermCamera, &mut Position)>,
) {
	let wiggle_amount = 10f64;
	let sin =
		time.seconds_since_startup.sin() * wiggle_amount - wiggle_amount / 2.0;

	for (_, mut cam_pos) in &mut cameras.iter() {
		cam_pos.0.set_x(sin as f32);
	}
}

fn print_time(time: Res<Time>, mut render: Mut<TermRender>, _: &Tracker) {
	render.body = format!(
		"This is in both screen and world space! delta time: {:.2}",
		time.delta_seconds
	);
}

fn main() {
	App::build()
		.add_plugin(TermPlugin)
		.add_startup_system(setup.system())
		.add_system(wiggle_camera.system())
		.add_system(print_time.system())
		.run();
}
