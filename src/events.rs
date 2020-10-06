use crate::TermEvent;
use bevy::prelude::*;
use crossterm::event::{poll, read};
use std::time::Duration;

#[derive(Default)]
pub struct TermState {
	pub reader: EventReader<TermEvent>,
}

pub fn event_producer(mut events: ResMut<Events<TermEvent>>) {
	if poll(Duration::from_millis(16)).unwrap() {
		if let Ok(e) = read() {
			events.send(e);
		}
	}
}
