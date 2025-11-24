#![allow(unused)]

mod util;
mod phases;
mod serialization;
mod settings_file;
mod egui_histogram;
mod app_control;
mod gizmos;
mod debug_camera;
mod flycam;
mod particles;
mod many_cubes;

use bevy::{
	prelude::*,
	ecs::name::*,
	ecs::query::{QueryFilter},
	ecs::schedule::{ScheduleBuildSettings, LogLevel},
	render::*,
	render::settings::Backends,
	camera::*,
	camera::visibility::RenderLayers,
	scene::SceneInstanceReady,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::f32::consts::*;

use debug_camera::{DebugCamera, MainCamera};
use flycam::FlycamBundle;
use phases::*;

fn main() {
	let mut app = App::new();
	app.configure_schedules(ScheduleBuildSettings {
		ambiguity_detection: LogLevel::Error,
		..default()
	});
	
	
	let asset_path = std::env::current_dir().unwrap()
		.join("assets")
		.to_string_lossy().to_string();
	
	// info!() not working in main, but works inside serialization::early_load_settings, what!?
	println!("Working directory: {:?}", std::env::current_dir().unwrap());
	println!("Exe path: {:?}", std::env::current_exe().unwrap());
	println!("Asset path: {:?}", asset_path);
	
	let (settings_json, render_settings) = settings_file::early_load_settings();
	
	app.add_plugins({
		let mut plugins = DefaultPlugins
		.set(WindowPlugin {
			primary_window: Some(Window {
				title: app_control::APP_NAME.into(),
				name: Some(app_control::APP_NAME.into()),
				resolution: (1152, 720).into(),
				resize_constraints: WindowResizeConstraints { min_width: 100.0, min_height: 100.0, ..default() },
				resizable: true,
				..default()
			}),
			..default()
		})
		.set(RenderPlugin {
			render_creation: settings::RenderCreation::Automatic({
				let mut set = settings::WgpuSettings::default();
				
				set.backends = Some(Backends::from_comma_list(render_settings.backends.as_str()));
				
				// Allow disabling validation layers since there seem to be bugs(?) in some part of bevy/wgpu?
				// Unfortunately I'm unsure how to only disable them for vulkan, as backends still allows Bevy to select it on its own
				if render_settings.disable_validation_in_debug {
					set.instance_flags = bevy::render::settings::InstanceFlags::empty();
				}
				set
			}),
			..default()
		})
		.set(AssetPlugin {
			file_path: asset_path,
			..default()
		});
		
		plugins
	});
	app.add_plugins((
		app_control::AppControlPlugin,
		debug_camera::DebugCameraPlugin,
		gizmos::GizmosPlugin,
		flycam::FlycamPlugin,
		particles::ParticlePlugin,
		many_cubes::ManyCubesPlugin,
	));
	
	app.add_observer(init_wiggly_cylinder);
	
	app.add_systems(Startup, (
		startup,
		spawn_animated_gltf,
		(move |world: &mut World| {
			settings_file::load_settings(world, settings_json.clone());
		}).after(startup)
	));
	
	app.add_systems(Update, (
		update_animation,
	));
	
	phases::update_schedule_configs(&mut app);
	
	app.run();
}

fn startup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	asset_server: Res<AssetServer>,
) {
	let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);
	let cube_mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
	let blue = materials.add(Color::srgb_u8(124, 144, 255));
	let red = materials.add(Color::srgb_u8(255, 144, 124));
	
	// cameras
	
	// Overlay camera for Egui, as egui does not handle switching between main/debug camera
	commands.spawn((
		bevy_egui::PrimaryEguiContext,
		Camera2d::default(),
		RenderLayers::from_layers(&[31]), // Avoid entities accidentally rendering twice (NOTE: entities by default (without RenderLayers component) on layer 0)
		Camera {
			order: 10,
			output_mode: CameraOutputMode::Write {
				blend_state: Some(render_resource::BlendState::ALPHA_BLENDING),
				clear_color: ClearColorConfig::None,
			},
			clear_color: ClearColorConfig::Custom(Color::NONE),
			..default()
		},
		Name::new("EguiCamera"),
	));
	
	commands.spawn((
		MainCamera,
		flycam::FlycamBundle::new( Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y) ),
		bevy::render::view::Hdr,
		bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
		bevy::post_process::bloom::Bloom::NATURAL,
		Name::new("MainCamera"),
			Mesh3d(cube_mesh.clone()),
			MeshMaterial3d(red.clone()), // just for debugging
	));
	commands.spawn((
		DebugCamera,
		flycam::FlycamBundle::new( Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y) ),
		bevy::render::view::Hdr,
		bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
		bevy::post_process::bloom::Bloom::NATURAL,
		Name::new("DebugFlycam"),
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
		Name::new("Light")
	));
	
	// ground plane
	commands.spawn((
		Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
		MeshMaterial3d(materials.add(Color::srgb(0.4, 0.55, 0.6))),
		Name::new("Ground Plane")
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
				Name::new("Cube")
			)
		})
		.take(10),
	);
	
	commands.spawn((
		Transform { translation: Vec3::new(5.0, 0.0, 0.0), ..default() },
		SceneRoot(asset_server.load("basic_assets.glb#Scene0")),
	));
}

#[derive(Component)]
struct WigglyCylinderScene;
#[derive(Component)]
struct WigglyCylinder;

fn spawn_animated_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn((
		SceneRoot(asset_server.load("rig.glb#Scene0")),
		WigglyCylinderScene,
	));
}
fn init_wiggly_cylinder(scene_ready: On<SceneInstanceReady>,
		q_children: Query<&Children>,
		mut q_filter: Query<(), With<WigglyCylinderScene>>,
		mut q_skinned_mesh: Query<(&bevy::mesh::skinning::SkinnedMesh, &mut MeshMaterial3d<StandardMaterial>)>,
		mut materials: ResMut<Assets<StandardMaterial>>,
		mut commands: Commands) {
	if q_filter.get(scene_ready.entity).is_err() { return }
	
	println!("init_wiggly_cylinder() {}", q_filter.count());
	
	let material = materials.add(Color::srgb_u8(255, 144, 50));
	
	let scene_root = scene_ready.entity;
	for entity in q_children.iter_descendants(scene_root) {
		if let Ok((skinned_mesh, mut mat)) = q_skinned_mesh.get_mut(entity) {
			*mat = MeshMaterial3d(material.clone());
			
			commands.entity(entity).insert(WigglyCylinder);
			commands.entity(skinned_mesh.joints[2]).insert((
				particles::ParticleEmitter::new(0.2),
			));
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
	animated_entities: Query<&bevy::mesh::skinning::SkinnedMesh, With<WigglyCylinder>>,
	mut transform_query: Query<&mut Transform>,
) {
	for animated in &animated_entities {
		
		let second_joint_entity = animated.joints[1];
		let mut second_joint_transform = transform_query.get_mut(second_joint_entity).unwrap();
		
		second_joint_transform.rotation =
			Quat::from_rotation_z(FRAC_PI_2 * ops::sin(time.elapsed_secs()*3.0));
	}
}
