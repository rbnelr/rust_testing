use bevy::prelude::*;
use bevy::ecs::system::{RunSystemOnce, SystemState};
use serde::{Serialize, Deserialize};
use serde_json;
use bevy_serde_lens::{SerializeResource, WorldExtension};
use crate::app_control::WindowSettings;

const SETTINGS_FILE: &'static str = "settings.json";

#[derive(Resource, Reflect, Clone, Serialize, Deserialize)]
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

pub struct Settings {
	window: WindowSettings,
	pub render: RenderSettings,
}

type SettingsFile = bevy_serde_lens::batch!(
	SerializeResource<WindowSettings>,
	SerializeResource<RenderSettings>,
	
);

pub fn save(world: &mut World) {
	let mut writer = Vec::with_capacity(1024);
	let tabs_pretty = serde_json::ser::PrettyFormatter::with_indent(b"\t");
	let mut ser = serde_json::Serializer::with_formatter(&mut writer, tabs_pretty);
	
	if serde::Serialize::serialize(&world.serialize_lens::<SettingsFile>(), &mut ser).is_ok() {
		if std::fs::write(SETTINGS_FILE, writer).is_ok() {
			info!("Saved!");
			return;
		}
	}
	
	warn!("Failed to save {SETTINGS_FILE}!");
}

// Early load RenderSettings to allow backend selection before bevy is actually loaded and defer 
pub struct LoadResult {
	loaded_json: serde_json::Value,
	pub render: RenderSettings,
}

// load_settings() + apply_settings() Allows loading Settings before app starts
pub fn early_load_settings() -> Option<LoadResult> {
	if let Ok(json_str) = std::fs::read_to_string(SETTINGS_FILE) {
		if let Ok(json) = serde_json::from_str::<serde_json::Value>(&json_str) {
			let render : RenderSettings = serde_json::from_value(json["RenderSettings"].clone()).unwrap_or_default();
			
			info!("Early loaded {SETTINGS_FILE}!");
			return LoadResult {
				loaded_json: json,
				render
			}.into();
		}
	}
	
	warn!("Failed to load {SETTINGS_FILE}!");
	None
}
pub fn load_settings(
	world: &mut World,
	settings: Option<LoadResult>
) {
	//let mut params: SystemState<(
	//	ResMut<WindowSettings>,
	//	Commands,
	//)> = SystemState::new(world);
	//let (
	//	mut window_settings,
	//	mut commands,
	//) = params.get_mut(world);
	//
	//if let Some(settings) = settings {
	//	commands.insert_resource(settings.render);
	//	*window_settings = settings.window;
	//}
	//
	//params.apply(world);
	
	if let Some(settings) = settings {
		world.despawn_bound_objects::<SettingsFile>();
		world.deserialize_scope(|| {
			let res = serde_json::from_value::<bevy_serde_lens::InWorld<SettingsFile>>(settings.loaded_json);
			if let Err(error) = res {
				info!("Err {error}!");
			}
		});
		
		info!("Fully Loaded {SETTINGS_FILE}!");
	}
	
	//world.insert_resource(RenderSettings::default());
}
pub fn load(world: &mut World) {
	let res = early_load_settings();
	load_settings(world, res);
}
