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

#[derive(Resource, Reflect, Serialize, Deserialize, Clone)]
#[reflect(Resource)]
pub struct RenderSettings {
	pub backends: String,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
	pub render: RenderSettings,
	window: WindowSettings,
}

pub fn save(
	world: &mut World,
	params: &mut SystemState<(
		Option<Res<RenderSettings>>,
		Res<WindowSettings>,
	)>
) {
	let (
		render_settings,
		window_settings,
	) = params.get(world);
	
	let render_settings = match render_settings {
		Some(s) => s.clone(),
		_ => RenderSettings { backends: "vulkan, dx12, opengl, webgpu".to_string() }
	};
	
	let settings = Settings{
		render: render_settings,
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
// load_settings() + apply_settings() Allows loading Settings before app starts
pub fn load_settings() -> Option<Settings> {
	if let Ok(json_str) = std::fs::read_to_string(SETTINGS_FILE) {
		if let Ok(settings) = serde_json::from_str::<Settings>(&json_str) {
			println!("Loaded!");
			return settings.into();
		}
	}
	
	println!("Failed to load {SETTINGS_FILE}!");
	None
}
pub fn apply_settings(
	world: &mut World,
	settings: Option<Settings>
) {
	let mut params: SystemState<(
		ResMut<WindowSettings>,
		Commands,
	)> = SystemState::new(world);
	let (
		mut window_settings,
		mut commands,
	) = params.get_mut(world);
	
	if let Some(settings) = settings {
		commands.insert_resource(settings.render);
		*window_settings = settings.window;
	}
	
	params.apply(world);
}
pub fn load(world: &mut World) {
	let settings = load_settings();
	apply_settings(world, settings);
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
