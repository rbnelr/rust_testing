use bevy::prelude::*;
use bevy::ecs::schedule::ScheduleConfigs;
use crate::phases;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Phase {
	SerializationAndImgui,
	Windowing,
	Gameplay,
	CameraUpdate,
}

pub fn update_schedule_configs(app: &mut App) {
	app.configure_sets(Update, (
		Phase::SerializationAndImgui
			.before(Phase::Windowing)
			.before(Phase::Gameplay)
			.before(Phase::CameraUpdate),
		Phase::CameraUpdate
			.after(Phase::Gameplay),
	));
}
