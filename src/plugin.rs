use bevy::app::AppExit;
use bevy::prelude::*;
use crossterm::{
	cursor,
	event::{poll, read, KeyCode},
	execute, terminal,
};
use std::io::{stdout, Write};
use std::thread;
use std::time::{Duration, Instant};

use crate::systems::*;

fn startup() {
	terminal_init();
}

fn terminal_init() {
	execute!(
		stdout(),
		terminal::EnterAlternateScreen,
		terminal::Clear(terminal::ClearType::All),
		terminal::DisableLineWrap,
		cursor::Hide
	)
	.unwrap();
	terminal::enable_raw_mode().unwrap();
}

fn terminal_exit() {
	execute!(
		stdout(),
		terminal::LeaveAlternateScreen,
		terminal::EnableLineWrap,
		cursor::Show
	)
	.unwrap();
	terminal::disable_raw_mode().unwrap();
}

fn term_runner(mut app: App) {
	let wait = Duration::from_secs_f64(1.0 / 60.0);
	let mut app_exit_event_reader = EventReader::<AppExit>::default();
	let mut tick = move |app: &mut App,
	                     wait: Duration|
	      -> Result<Option<Duration>, AppExit> {
		let start_time = Instant::now();
		if let Some(app_exit_events) = app.resources.get_mut::<Events<AppExit>>() {
			if let Some(_) = app_exit_event_reader.latest(&app_exit_events) {
				return Err(AppExit);
			}
		}

		while poll(Duration::from_millis(0)).unwrap() {
			if let crossterm::event::Event::Key(key) = read().unwrap() {
				if key.code == KeyCode::Esc {
					terminal_exit();
					return Err(AppExit);
				}
			}
		}

		app.update();
		stdout().flush().unwrap();

		if let Some(app_exit_events) = app.resources.get_mut::<Events<AppExit>>() {
			if let Some(_) = app_exit_event_reader.latest(&app_exit_events) {
				return Err(AppExit);
			}
		}

		let end_time = Instant::now();

		let exe_time = end_time - start_time;
		if exe_time < wait {
			return Ok(Some(wait - exe_time));
		}

		Ok(None)
	};

	while let Ok(delay) = tick(&mut app, wait) {
		if let Some(delay) = delay {
			thread::sleep(delay);
		}
	}
}

/// Core plugin for the library
pub struct TermPlugin;

impl Plugin for TermPlugin {
	fn build(&self, app: &mut AppBuilder) {
		app
			.add_plugin(bevy::type_registry::TypeRegistryPlugin::default())
			.add_plugin(bevy::core::CorePlugin::default())
			.add_plugin(bevy::asset::AssetPlugin::default())
			.add_plugin(bevy::scene::ScenePlugin::default())
			.add_plugin(bevy::audio::AudioPlugin::default())
			.add_event::<crate::TermEvent>()
			.add_resource(crate::TerminalOptions::default())
			.add_resource(crate::systems::RendersSize(0))
			.add_startup_system(startup.system())
			.add_system(crate::events::event_producer.system())
			.add_system(render.system())
			.set_runner(term_runner);
	}
}
