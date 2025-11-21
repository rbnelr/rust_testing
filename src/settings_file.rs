use bevy::prelude::*;
use serde::*;
use crate::flycam;
use crate::debug_camera;
use crate::serialization;
use crate::serialization::*;

const SETTINGS_FILE: &'static str = "settings.json";
// TODO: use ron instead? advantages: enums, comments, trailing comma

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

use flycam::*;
serializer_entity!(FlycamBundle,
	Transform, Flycam,
);

struct Nested;
world_serializer!(Nested,
	debug_cam: WorldRes<crate::debug_camera::DebugCameraState>,
	main_cam: WorldSingleEntity<FlycamBundle, With<crate::debug_camera::MainCamera>>,
);

struct SettingsFile;
world_serializer!(SettingsFile,
	window: WorldRes<crate::app_control::WindowSettings>,
	render: WorldRes<RenderSettings>,
	nested: Nested,
);

fn load_json_value() -> serialization::JsonResult {
	let json_str : Result<String, serialization::Error> = std::fs::read_to_string(SETTINGS_FILE)
		.map_err(|e| e.into());
	serde_json::from_str::<serde_json::Value>(&json_str?).map_err(|e| e.into())
}
// TODO: can we write this helper function to take in WorldSerializer directly?
fn serialize_to_json_pretty_tabs(json: &serde_json::Value) -> Result<String, crate::serialization::Error> {
	let mut vec = Vec::with_capacity(1024);
	let mut ser = serde_json::Serializer::with_formatter(&mut vec,
		serde_json::ser::PrettyFormatter::with_indent(b"\t"));
	
	serde::Serialize::serialize(&json, &mut ser)?;
	
	unsafe { // copied from serde_json::to_string_pretty
		Ok(String::from_utf8_unchecked(vec))
	}
}

pub fn save(world: &mut World) {
	fn inner(world: &mut World) -> Result<(), crate::serialization::Error> {
		let mut json = <SettingsFile as WorldSerializer>::serialize(world)?;
		
		// Important to version save files when releasing applications
		json.as_object_mut().unwrap().shift_insert(0, "version".into(), "0.1".into());
		
		let json_str = serialize_to_json_pretty_tabs(&json)?;
		std::fs::write(SETTINGS_FILE, json_str)?;
		Ok(())
	}
	
	if let Err(e) = inner(world) {
		warn!("Failed to save {SETTINGS_FILE}!\n{:?}", e);
		return;
	}
	info!("Saved {SETTINGS_FILE}!");
}

// Early load RenderSettings to allow backend selection before bevy is actually loaded and use loaded_json later
pub type LoadResult = (Option<serde_json::Value>, RenderSettings);

pub fn early_load_settings() -> LoadResult {
	let json = match load_json_value() {
		Ok(val) => Some(val),
		Err(e) => {
			warn!("Failed to load {SETTINGS_FILE}!\n{:?}", e); // Need to report error early and skip later as error can't be cloned and we need to clone it for startup systems
			None
		},
	};
	
	let render = if let Some(json) = &json {
		serde::Deserialize::deserialize(&json["render"]).unwrap_or(RenderSettings::default())
	}
	else {
		RenderSettings::default()
	};
	
	(json, render)
}

use crate::util::*;
pub fn load_settings(world: &mut World, json: Option<serde_json::Value>) {
	world.insert_resource(RenderSettings::default()); // Never inserted anywhere else
	
	if let Some(json) = json {
		if let Err(e) = <SettingsFile as WorldSerializer>::deserialize(world, &json) {
			warn!("Failed to load {SETTINGS_FILE}!\n{:?}", e);
			return;
		}
		info!("Fully Loaded {SETTINGS_FILE}!");
	}
}

pub fn load(world: &mut World) {
	load_settings(world, early_load_settings().0);
}
