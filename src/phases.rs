use bevy::prelude::*;
use bevy::ecs::schedule::ScheduleConfigs;
use crate::phases;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Phase {
	Start,
	Gameplay,
	CameraUpdate,
}
// egui runs in PostUpdate via EguiPrimaryContextPass?
// which is probably fine

pub fn update_schedule_configs(app: &mut App) {
	app.configure_sets(Update, (
		Phase::Start
			.before(Phase::Gameplay)
			.before(Phase::CameraUpdate),
		Phase::CameraUpdate
			.after(Phase::Gameplay),
	));
}
