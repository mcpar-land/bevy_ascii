use bevy::prelude::*;
use crossterm::{execute, terminal};
use std::io::{stdout, Write};

use crate::systems::*;

fn startup() {
	execute!(
		stdout(),
		terminal::EnterAlternateScreen,
		terminal::Clear(terminal::ClearType::All),
		terminal::DisableLineWrap,
		crossterm::cursor::Hide
	)
	.unwrap();
	terminal::enable_raw_mode().unwrap();
}

pub struct TermPlugin;

impl Plugin for TermPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			.add_plugin(bevy::type_registry::TypeRegistryPlugin::default())
			.add_plugin(bevy::core::CorePlugin::default())
			.add_plugin(bevy::diagnostic::DiagnosticsPlugin::default())
			.add_plugin(bevy::input::InputPlugin::default())
			.add_plugin(bevy::window::WindowPlugin {
				add_primary_window: false,
				exit_on_close: true,
			})
			.add_plugin(bevy::asset::AssetPlugin::default())
			.add_plugin(bevy::scene::ScenePlugin::default())
			.add_plugin(bevy::audio::AudioPlugin::default())
			.add_plugin(bevy::winit::WinitPlugin::default())
			.add_event::<crate::TermEvent>()
			.add_resource(crate::TerminalOptions::default())
			.add_startup_system(startup.system())
			.add_system(crate::events::event_producer.system())
			.add_system(draw_cameras.system())
			.add_system(handle_quit.system());
	}
}
