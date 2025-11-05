use bevy::prelude::*;
use serde_json;
use crate::serialization::*;

const SETTINGS_FILE: &'static str = "settings.json";
// TODO: use ron instead? advantages: enums, comments, trailing comma

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct RenderSettings {
	pub backends: String,
	pub disable_validation_in_debug: bool,
}
impl Default for RenderSettings {
	fn default() -> Self {
		Self {
			backends: "vk, dx12, gl".into(),
			disable_validation_in_debug: true,
		}
	}
}
serializer!(RenderSettings, backends, disable_validation_in_debug);
serializer_world!(RenderSettings, Res<RenderSettings>);

struct SettingsFile();

serializer_world!(SettingsFile{
	window: crate::app_control::WindowSettings,
	render: RenderSettings,
	debug_cam: crate::debug_camera::DebugCameraState,
	main_cam: crate::flycam::Flycam,
});

pub fn save(world: &mut World) {
	let json = SettingsFile::serialize(world);
	
	let mut writer = Vec::with_capacity(1024);
	let mut ser = serde_json::Serializer::with_formatter(&mut writer,
		serde_json::ser::PrettyFormatter::with_indent(b"\t"));
	
	if serde::Serialize::serialize(&json, &mut ser).is_ok() {
		if std::fs::write(SETTINGS_FILE, writer).is_ok() {
			info!("Saved!");
			return;
		}
	}
	
	warn!("Failed to save {SETTINGS_FILE}!");
}

// Early load RenderSettings to allow backend selection before bevy is actually loaded and use loaded_json later
#[derive(Clone)]
pub struct LoadResult {
	loaded_json: serde_json::Value,
	pub render: RenderSettings,
}

pub fn early_load_settings() -> Option<LoadResult> {
	if let Ok(json_str) = std::fs::read_to_string(SETTINGS_FILE) {
		if let Ok(mut loaded_json) = serde_json::from_str::<serde_json::Value>(&json_str) {
			let render = RenderSettings::deserialize_new(loaded_json["render"].take());
			
			info!("Early loaded {SETTINGS_FILE}!");
			return LoadResult {
				loaded_json, render
			}.into();
		}
	}
	
	warn!("Failed to load {SETTINGS_FILE}!");
	None
}
pub fn load_settings(world: &mut World, res: Option<LoadResult>) {
	// deserialize only updates things, never inserts them itself
	world.insert_resource(RenderSettings::default());
	
	if let Some(res) = res {
		SettingsFile::deserialize(world, res.loaded_json);
		
		info!("Fully Loaded {SETTINGS_FILE}!");
	}
}

pub fn load(world: &mut World) {
	let res = early_load_settings();
	load_settings(world, res);
}
