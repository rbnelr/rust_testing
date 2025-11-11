use bevy::prelude::*;
use serde::*;
use bevy_serde_lens::*;
use crate::flycam;
use crate::debug_camera;

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

#[derive(Bundle, BevyObject)]
#[bevy_object(query)]
pub struct FlycamSer {
	pub _marker: debug_camera::MainCamera, // huh? How do we specify the filter without this dodgy thing?
	pub transform: Transform,
	pub flycam: flycam::Flycam,
}

type SettingsFile = bevy_serde_lens::batch!(
	SerializeResource<crate::app_control::WindowSettings>,
	SerializeResource<RenderSettings>,
	SerializeResource<crate::debug_camera::DebugCameraState>,
	FlycamSer,
);

fn serialize_to_json_pretty_tabs<T>(value: &T) -> Result<String>
where T: Serialize {
	let mut vec = Vec::with_capacity(1024);
	let mut ser = serde_json::Serializer::with_formatter(&mut vec,
		serde_json::ser::PrettyFormatter::with_indent(b"\t"));
	
	if let Err(e) = serde::Serialize::serialize(&value, &mut ser) {
		warn!("{e:?}");
		return Err(e.into());
	}
	
	unsafe { // copied from serde_json::to_string_pretty
		Ok(String::from_utf8_unchecked(vec))
	}
}

pub fn save(world: &mut World) {
	let value = &world.serialize_lens::<SettingsFile>();
	
	if let Ok(json_str) = serialize_to_json_pretty_tabs(&value) {
		if std::fs::write(SETTINGS_FILE, json_str).is_ok() {
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
		
			let render = serde_json::from_value::<RenderSettings>(loaded_json["render"].clone());
			let render = render.unwrap_or(RenderSettings::default());
			
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
	world.insert_resource(RenderSettings::default()); // Never inserted anywhere else
	
	if let Some(res) = res {
		world.despawn_bound_objects::<SettingsFile>();
		world.deserialize_scope(|| {
			let _ = serde_json::from_value::<InWorld<SettingsFile>>(res.loaded_json);
			// TODO: FIX: Because how serde lens works, camera gets respawned from scratch, so either I serialize everything,
			// despite it not making sense or I need to work around the fact that a bunch of components will be missing
		});
		
		info!("Fully Loaded {SETTINGS_FILE}!");
	}
}

pub fn load(world: &mut World) {
	let res = early_load_settings();
	load_settings(world, res);
}
