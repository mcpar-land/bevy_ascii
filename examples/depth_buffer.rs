use bevy::prelude::*;
use bevy_ascii::*;

pub struct Speed(pub f32);
pub struct Wiggle;

fn setup(mut commands: Commands, mut options: ResMut<TerminalOptions>) {
	options.display_fps_counter = true;
	commands.spawn(TermCameraComponents::default());

	let a_str = "0000\n0000\n0000\n0000";
	let b_str = "======== Hello, world! ========";
	let c_str = concat!(
		"+---+---+\n",
		"|   |   |\n",
		"+---+---+\n",
		"|   |   |\n",
		"+---+---+\n"
	);

	let a = TermRenderWorldBundle {
		render: TermRender {
			body: String::from(a_str),
			style: ContentStyle::new()
				.foreground(TermColor::Black)
				.background(TermColor::White),
		},
		position: Position(Vec3::new(0.0, 0.0, -10.0)),
	};
	let b = TermRenderWorldBundle {
		render: TermRender {
			body: String::from(b_str),
			style: ContentStyle::new().foreground(TermColor::Magenta),
		},
		position: Position(Vec3::new(0.0, 1.0, 0.0)),
	};
	let c = TermRenderWorldBundle {
		render: TermRender {
			body: String::from(c_str),
			style: ContentStyle::new().foreground(TermColor::Red),
		},
		position: Position(Vec3::new(0.0, 0.0, 10.0)),
	};

	commands.spawn(a).with(Speed(15.0)).with(Wiggle);
	commands.spawn(b).with(Speed(20.0));
	commands.spawn(c).with(Speed(15.0));
}

static LOOP_DISTANCE: f32 = 35.0;

fn slide_to_the_left(time: Res<Time>, mut pos: Mut<Position>, speed: &Speed) {
	let x = pos.0.x_mut();
	*x += speed.0 * time.delta_seconds;
	if *x > LOOP_DISTANCE {
		*x = -LOOP_DISTANCE
	}
}

fn wiggle_up_down(time: Res<Time>, mut pos: Mut<Position>, _: &Wiggle) {
	let wiggle_amount = 8f64;
	let sin =
		time.seconds_since_startup.sin() * wiggle_amount - wiggle_amount / 2.0;
	pos.0.set_y(sin as f32);
}

fn main() {
	App::build()
		.add_plugin(TermPlugin)
		.add_startup_system(setup.system())
		.add_system(wiggle_up_down.system())
		.add_system(slide_to_the_left.system())
		.run();
}
