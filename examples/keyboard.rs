use bevy::prelude::*;
use bevy_ascii::*;

// Bevy keyboard events require a window to be present.
// Instead, we use crossterm's event system.

fn setup(mut commands: Commands, mut options: ResMut<TerminalOptions>) {
	options.display_fps_counter = true;
	commands.spawn(TermCameraComponents::default());

	commands.spawn(TermRenderWorldBundle {
		render: TermRender {
			body: String::from("? ctrl: _, alt: _"),
			..Default::default()
		},
		position: Position(Vec3::new(0.0, 0.0, 0.0)),
	});
}

fn handle_keyboard(
	mut state: Local<TermState>,
	events: Res<Events<TermEvent>>,
	mut render: Mut<TermRender>,
) {
	for event in state.reader.iter(&events) {
		if let TermEvent::Key(e) = event {
			if let TermKeyCode::Char(c) = e.code {
				render.body = String::from(c);
				let ctrl_down = e.modifiers.contains(TermKeyModifiers::CONTROL);
				let alt_down = e.modifiers.contains(TermKeyModifiers::ALT);

				render.body = format!(
					"{} ctrl: {}, alt: {}",
					c,
					if ctrl_down { "X" } else { "_" },
					if alt_down { "X" } else { "_" }
				);
			}
		}
	}
}

fn main() {
	App::build()
		.add_plugin(TermPlugin)
		.add_startup_system(setup.system())
		.add_system(handle_keyboard.system())
		.run();
}
