#![allow(unused)]

mod camera_util;
mod debug_camera;
mod flycam;
mod imgui;
mod particles;

use bevy::{
	prelude::*,
	ecs::query::{QueryFilter},
	ecs::schedule::{ScheduleBuildSettings, LogLevel},
	dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
	render::*,
	camera::Viewport,
	scene::SceneInstanceReady,
	//math::*,
	//diagnostic::*,
	//text::FontSmoothing,
};
//use bevy::dev_tools::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::{f32::consts::*, env};
use camera_util::CameraUpdateSet;


fn main() {
	
	let asset_path = env::current_dir().unwrap()
		.join("assets")
		.to_string_lossy().to_string();
		
	println!("Working directory: {:?}", std::env::current_dir().unwrap());
	println!("Exe path: {:?}", std::env::current_exe().unwrap());
	println!("Asset path: {:?}", asset_path);
	
	let mut app = App::new();
	app.configure_schedules(ScheduleBuildSettings {
		ambiguity_detection: LogLevel::Error,
		..default()
	});

	app.add_plugins(DefaultPlugins
			.set(RenderPlugin {
				render_creation: settings::RenderCreation::Automatic(settings::WgpuSettings {
					backends: Some(settings::Backends::DX12),
					..default()
				}),
				..default()
			})
			.set(AssetPlugin {
				file_path: asset_path,
				..default()
			})
		)
		//.insert_resource(bevy::ecs::schedule::ReportExecutionOrderAmbiguities) // Does not seem to exists, how do I schedule plugins correctly to avoid 1 frame latency?
		.add_plugins((
			imgui::ImguiPlugin,
			debug_camera::DebugCameraPlugin,
			flycam::FlycamPlugin,
			particles::ParticlePlugin
		))
		
		//.add_plugins(FrameTimeDiagnosticsPlugin::default())
		//.add_plugins(LogDiagnosticsPlugin::default())
		//.add_plugins(FpsOverlayPlugin {
		//	config: FpsOverlayConfig {
		//		text_config: TextFont {
		//			font_size: 20.0,
		//			..default()
		//		},
		//		refresh_interval: core::time::Duration::from_millis(100),
		//		enabled: true,
		//		frame_time_graph_config: FrameTimeGraphConfig {
		//			enabled: true,
		//			min_fps: 30.0,
		//			target_fps: 144.0,
		//		},
		//		..default()
		//	},
		//})

		.add_observer(do_very_specific_thing_to_object)
		.add_systems(Startup, (startup, spawn_animated_gltf))
		.add_systems(Update, update_animation)
		//.add_systems(Update, _log_scene_hierarchy)
		.run();
}

fn startup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>
) {
	
	let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);
	let cube_mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
	let blue = materials.add(Color::srgb_u8(124, 144, 255));
	let red = materials.add(Color::srgb_u8(255, 144, 124));
	
	// camera
	commands.spawn((
		flycam::Flycam::new( Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y) ),
		Camera::default(),
		bevy::render::view::Hdr,
		bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
		bevy::post_process::bloom::Bloom::NATURAL,
		Name("Camera".to_string()),
			Mesh3d(cube_mesh.clone()),
			MeshMaterial3d(red.clone()), // just for debugging
	));
	commands.spawn((
		debug_camera::DebugCamera,
		flycam::Flycam::new( Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y) ),
		Camera::default(),
		bevy::render::view::Hdr,
		bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
		bevy::post_process::bloom::Bloom::NATURAL,
		Name("DebugFlycam".to_string()),
			Mesh3d(cube_mesh.clone()),
			MeshMaterial3d(red.clone()), // just for debugging
	));
	
	// light
	commands.spawn((
		Transform::from_xyz(0.0,0.0,0.0).looking_at(Vec3::new(1.0, -6.0, 3.0), Vec3::Y),
		DirectionalLight  {
			shadows_enabled: true,
			..default()
		},
		Name("Light".to_string())
	));
	
	// ground plane
	commands.spawn((
		Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
		MeshMaterial3d(materials.add(Color::WHITE)),
		Name("Ground Plane".to_string())
	));

	commands.spawn_batch(
		std::iter::repeat_with(move || {
			let x = rng.random_range(-5.0..5.0);
			let y = rng.random_range(0.0..3.0);
			let z = rng.random_range(-5.0..5.0);

			(
				Mesh3d(cube_mesh.clone()),
				MeshMaterial3d(blue.clone()),
				Transform::from_xyz(x, y, z),
				Name("Cube".to_string())
			)
		})
		.take(10),
	);
}

#[derive(Component)]
struct ThisVerySpecificObject();

fn spawn_animated_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn((
		SceneRoot(asset_server.load("rig.glb#Scene0")),
		ThisVerySpecificObject()
	));
}
fn do_very_specific_thing_to_object(scene_ready: On<SceneInstanceReady>,
		q_children: Query<&Children>,
		mut q_skinned_mesh: Query<(&bevy::mesh::skinning::SkinnedMesh, &mut MeshMaterial3d<StandardMaterial>)>,
		mut materials: ResMut<Assets<StandardMaterial>>,
		mut commands: Commands) {
	let material = materials.add(Color::srgb_u8(255, 144, 50));
	
	let scene_root = scene_ready.entity;
	for entity in q_children.iter_descendants(scene_root) {
		if let Ok((skinned_mesh, mut mat)) = q_skinned_mesh.get_mut(entity) {
			*mat = MeshMaterial3d(material.clone());
			
			commands.entity(skinned_mesh.joints[2]).insert(
				particles::ParticleEmitter::new(0.2)
			);
		}
	}
}

fn spin_camera(time: Res<Time>, mut query: Query<&mut Transform, With<Camera3d>>) {
	for mut transf in &mut query {
		transf.rotate_around(Vec3::ZERO, Quat::from_rotation_y(1.0*time.delta_secs()));
	}
}

fn update_animation(
	time: Res<Time>,
	animated_entities: Query<&bevy::mesh::skinning::SkinnedMesh>,
	mut transform_query: Query<&mut Transform>,
) {
	for animated in &animated_entities {
		
		let second_joint_entity = animated.joints[1];
		let mut second_joint_transform = transform_query.get_mut(second_joint_entity).unwrap();
		
		second_joint_transform.rotation =
			Quat::from_rotation_z(FRAC_PI_2 * ops::sin(time.elapsed_secs()*3.0));
	}
}

#[derive(Component)]
struct Name(String);

fn _log_entity_tree (world: &World, entity: Entity, ident: &str, indent2: &str) {
	let entity_ref = world.entity(entity);
	let all_component_names = entity_ref.archetype().components().into_iter()
		.map(|component_id| world.components().get_info(*component_id).unwrap().name().split("::").last().unwrap().to_string())
		.collect::<Vec<String>>().join(", ");
		
	if let Some(name) = world.get::<Name>(entity) {
		println!("{ident}Entity: {:?} \"{}\", [{}]", entity, name.0.as_str(), all_component_names);
	} else {
		println!("{ident}Entity: {:?}, [{}]", entity, all_component_names);
	}
	
	// No idea how to implement this generically
	if let Some(skinned_mesh) = world.get::<bevy::mesh::skinning::SkinnedMesh>(entity) {
		println!("{indent2}  SkinnedMesh: {:?}", skinned_mesh);
	}
	
	if let Some(children) = world.get::<Children>(entity) {
		let last = children.len()-1;
		for (i, child) in children.iter().enumerate() {
			// Generate the nice ascii tree indentation
			let indent  = format!("{}{}", indent2, (if i==last {"└─"} else {"├─"}));
			let indent2 = format!("{}{}", indent2, (if i==last {"  "} else {"│ "}));
			
			_log_entity_tree(world, child, indent.as_str(), indent2.as_str());
		}
	}
}
fn _log_entity_trees<F: QueryFilter> (world: &World, root_entities: Query<Entity, F>) {
	for entity in &root_entities {
		if world.get::<ChildOf>(entity).is_none() {
			_log_entity_tree(world, entity, "", "");
		}
	}
}

fn _log_scene_hierarchy (world: &World, query: Query<Entity>) {
	_log_entity_trees(world, query);
	//println!("------------------------");
}
