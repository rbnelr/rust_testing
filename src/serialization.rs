use bevy::prelude::*;
use bevy::ecs::system::{RunSystemOnce, SystemState};
use bevy_mod_imgui::ImguiContext;
use bevy_mod_imgui::prelude::{Condition, StyleColor};
use serde::{Serialize, Deserialize};
use serde_json;
use crate::app_control::WindowSettings;
use bevy_mod_imgui::prelude::*;
use crate::phases::Phase;

pub struct SerializationPlugin;
impl Plugin for SerializationPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, load) // Load at startup
			.add_systems(Update,
				save_load_controls.in_set(Phase::SerializationAndImgui)
			);
	}
}

const SETTINGS_FILE: &'static str = "settings.json";

#[derive(Serialize, Deserialize)]
struct Settings {
	window: WindowSettings,
}

pub fn save(
	world: &mut World,
	params: &mut SystemState<(
		Res<WindowSettings>
	)>
) {
	let (
		window_settings
	) = params.get(world);
	
	let settings = Settings{
		window: *window_settings
	};
	
	let mut writer = Vec::with_capacity(1024);
	let tabs_pretty = serde_json::ser::PrettyFormatter::with_indent(b"\t");
	let mut ser = serde_json::Serializer::with_formatter(&mut writer, tabs_pretty);
	
	if serde::Serialize::serialize(&settings, &mut ser).is_ok() {
		if std::fs::write(SETTINGS_FILE, writer).is_ok() {
			println!("Saved!");
			return;
		}
	}
	
	println!("Failed to save {SETTINGS_FILE}!");
}
pub fn load(
	world: &mut World,
	params: &mut SystemState<(
		ResMut<WindowSettings>
	)>
) {
	let (
		mut window_settings
	) = params.get_mut(world);
	
	if let Ok(json_str) = std::fs::read_to_string(SETTINGS_FILE) {
		if let Ok(settings) = serde_json::from_str::<Settings>(&json_str) {
			*window_settings = settings.window;
			println!("Loaded!");
			return;
		}
	}
	
	println!("Failed to load {SETTINGS_FILE}!");
}

fn save_load_controls(
	world: &mut World,
	params: &mut SystemState<(
		Res<ButtonInput<KeyCode>>
	)>
) {
	let (do_load, do_save) = {
		let keyboard = params.get(world);
		(keyboard.just_pressed(KeyCode::Semicolon), keyboard.just_pressed(KeyCode::Quote))
	};
	if do_load {
		world.run_system_once(load);
	}
	else if do_save {
		world.run_system_once(save);
	}
}
