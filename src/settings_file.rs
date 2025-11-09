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
	pub name: bevy::ecs::name::Name,
	pub transform: Transform,
	pub flycam: flycam::Flycam,
}

type SettingsFile = bevy_serde_lens::batch!(
	FlycamSer,
	SerializeResource<crate::app_control::WindowSettings>,
	SerializeResource<RenderSettings>,
	//SerializeResource<crate::debug_camera::DebugCameraState>,
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
			//let render = RenderSettings::deserialize_new(loaded_json["render"].take());
			
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
	// deserialize only updates things, never inserts them itself
	world.insert_resource(RenderSettings::default());
	
	if let Some(res) = res {
		//SettingsFile::deserialize(world, res.loaded_json);
		
		//world.despawn_bound_objects::<SettingsFile>();
		//world.deserialize_scope(|| {
		//	let _ = serde_json::from_value::<InWorld<SettingsFile>>(res.loaded_json);
		//});
		
		info!("Fully Loaded {SETTINGS_FILE}!");
	}
	
	test();
}

pub fn load(world: &mut World) {
	let res = early_load_settings();
	load_settings(world, res);
}



use bevy::ecs::{bundle::Bundle, component::Component, resource::Resource, world::World};
use bevy::reflect::TypePath;
use bevy_serde_lens::{BevyObject, InWorld, SerializeResource, WorldExtension, batch};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Component, TypePath)]
#[serde(transparent)]
pub struct A(char);

#[derive(Serialize, Deserialize, Component, TypePath)]
#[serde(transparent)]
pub struct B(f32);

#[derive(Serialize, Deserialize, Component, TypePath)]
#[serde(transparent)]
pub struct C(String);

#[derive(Serialize, Deserialize, Component, TypePath)]
#[serde(transparent)]
pub struct D(usize);

#[derive(Serialize, Deserialize, Resource, TypePath)]
#[serde(transparent)]
pub struct R(usize);

#[derive(Bundle, BevyObject)]
pub struct AbBundle {
    a: A,
    b: B,
}

#[derive(Bundle, BevyObject)]
pub struct ABundle {
    a: A,
}

#[derive(Bundle, BevyObject)]
pub struct AbcdBundle {
    a: A,
    b: B,
    c: C,
    d: D,
}

type AB = batch!(A, B);
type BD = batch!(B, D);
type CD = batch!(C, D);
type ABCD = batch!(A, B, C, D);

type ABCDR = batch!(A, B, C, D, SerializeResource<R>);

pub fn test() {
    let mut world = World::new();
    world.spawn(A('b'));
    world.spawn(A('e'));
    world.spawn(A('v'));
    world.spawn(A('y'));
    world.spawn(B(3.0));
    world.spawn(B(0.5));
    world.spawn(C("Ferris".to_owned()));
    world.spawn(C("Crab".to_owned()));
    world.spawn(D(69));
    world.spawn(D(420));

    let value = world.save::<A, _>(serde_json::value::Serializer).unwrap();

    assert_eq!(value, json!(["b", "e", "v", "y"]));

    let value = world.save::<B, _>(serde_json::value::Serializer).unwrap();

    assert_eq!(value, json!([3.0, 0.5]));

    let value = world.save::<C, _>(serde_json::value::Serializer).unwrap();

    assert_eq!(value, json!(["Ferris", "Crab"]));

    let value = world.save::<D, _>(serde_json::value::Serializer).unwrap();

    assert_eq!(value, json!([69, 420]));

    //let value = world.save::<AB, _>(serde_json::value::Serializer).unwrap(); // works
	let value = serde_json::to_value(world.serialize_lens::<AB>()).unwrap(); // works as well
	let str1 = serde_json::to_string_pretty(&world.serialize_lens::<AB>()).unwrap(); // works as well
	
	let mut vec = Vec::with_capacity(1024);
	let mut ser = serde_json::Serializer::with_formatter(&mut vec,
		serde_json::ser::PrettyFormatter::with_indent(b"\t"));
	
	let res = serde::Serialize::serialize(&world.serialize_lens::<AB>(), &mut ser).unwrap();
	let str = String::from_utf8(vec).unwrap();

    assert_eq!(
        value,
        json!({
            "A": ["b", "e", "v", "y"],
            "B": [3.0, 0.5]
        })
    );
	println!("{}", value);
	println!("{}", str1);
	println!("{}", str);
	println!("{}", serde_json::to_string(&value).unwrap());

    let value = world.save::<BD, _>(serde_json::value::Serializer).unwrap();

    assert_eq!(
        value,
        json!({
            "B": [3.0, 0.5],
            "D": [69, 420],
        })
    );

    let value = world
        .save::<ABCD, _>(serde_json::value::Serializer)
        .unwrap();

    assert_eq!(
        value,
        json!({
            "A": ["b", "e", "v", "y"],
            "B": [3.0, 0.5],
            "C": ["Ferris", "Crab"],
            "D": [69, 420],
        })
    );

    world.despawn_bound_objects::<AB>();
    assert_eq!(world.entities().len(), 4);

    world.despawn_bound_objects::<CD>();
    assert_eq!(world.entities().len(), 0);

    world.load::<ABCD, _>(&value).unwrap();

    assert_eq!(world.entities().len(), 10);

    let value = world
        .save::<ABCD, _>(serde_json::value::Serializer)
        .unwrap();

    assert_eq!(
        value,
        json!({
            "A": ["b", "e", "v", "y"],
            "B": [3.0, 0.5],
            "C": ["Ferris", "Crab"],
            "D": [69, 420],
        })
    );

    world.despawn_bound_objects::<ABCD>();
    assert_eq!(world.entities().len(), 0);

    world.load::<ABCD, _>(value).unwrap();

    world.insert_resource(R(12));

    let lens = world.serialize_lens::<ABCDR>();

    let value = serde_json::to_value(lens).unwrap();

    assert_eq!(
        value,
        json!({
            "A": ["b", "e", "v", "y"],
            "B": [3.0, 0.5],
            "C": ["Ferris", "Crab"],
            "D": [69, 420],
            "R": 12,
        })
    );

    world.despawn_bound_objects::<ABCDR>();

    assert_eq!(world.entities().len(), 0);

    assert!(!world.contains_resource::<R>());

    world.load::<ABCDR, _>(value.clone()).unwrap();

    assert_eq!(world.entities().len(), 10);

    assert!(world.contains_resource::<R>());

    world.despawn_bound_objects::<ABCDR>();

    world.deserialize_scope(|| {
        let _: InWorld<ABCDR> = serde_json::from_value(value).unwrap();
    });

    assert_eq!(world.entities().len(), 10);

    assert!(world.contains_resource::<R>());

    world.despawn_bound_objects::<ABCDR>();

    world.spawn((A('y'), B(3.0), C("Ferris".to_owned()), D(69)));

    world.spawn((A('z'), B(4.0)));
    let value = world
        .save::<AbBundle, _>(serde_json::value::Serializer)
        .unwrap();

    assert_eq!(
        value,
        json!([
            {
                "a": "y",
                "b": 3.0,
            },
            {
                "a": "z",
                "b": 4.0,
            }
        ])
    );
    world.clear_all();

    world.spawn((A('y'), B(3.0), C("Ferris".to_owned()), D(69)));

    world.spawn((A('z'), B(4.5), C("Gopher".to_owned()), D(32)));

    let value = world
        .save::<AbcdBundle, _>(serde_json::value::Serializer)
        .unwrap();

    assert_eq!(
        value,
        json!([
            {
                "a": "y",
                "b": 3.0,
                "c": "Ferris",
                "d": 69
            },
            {
                "a": "z",
                "b": 4.5,
                "c": "Gopher",
                "d": 32
            },
        ])
    );
}
