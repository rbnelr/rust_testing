use bevy::{
	prelude::*,
	render::*,
	ecs::query::{QueryFilter},
	//math::*,
	scene::SceneInstanceReady,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::{f32::consts::*};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

mod particles;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(RenderPlugin {
			render_creation: settings::RenderCreation::Automatic(settings::WgpuSettings {
				backends: Some(settings::Backends::DX12),
				..default()
			}),
			..default()
		}))
		.add_plugins(FrameTimeDiagnosticsPlugin::default())
		.add_plugins(particles::ParticlePlugin)
		.add_observer(do_very_specific_thing_to_object)
		.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
		.add_systems(Startup, startup)
		.add_systems(Startup, spawn_animated_gltf)
		.add_systems(Update, (update_camera, update_animation).chain())
		//.add_systems(Update, (update_people, greet_people, log_scene_hierarchy).chain())
		.run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
	commands.spawn((Person, Name("Elaina Proctor".to_string())));
	commands.spawn((Person, Name("Renzo Hume".to_string())));
	commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn startup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>
) {
	// camera
	commands.spawn((
		Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
		Camera3d::default(),
		Camera::default(),
		bevy::render::view::Hdr,
		bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
		bevy::post_process::bloom::Bloom::NATURAL,
		Name("Camera".to_string())
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
	
	let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);
	let cube_mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
	let blue = materials.add(Color::srgb_u8(124, 144, 255));

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
	
	add_people(commands);
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

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
	if timer.0.tick(time.delta()).just_finished() {
		for name in &query {
			println!("hello {}!", name.0);
		}
	}
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
	for mut name in &mut query {
		if name.0 == "Elaina Proctor" {
			name.0 = "Elaina Hume".to_string();
			break; // We don't need to change any other names.
		}
	}
}

fn update_camera(time: Res<Time>, mut query: Query<&mut Transform, With<Camera3d>>) {
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
fn log_entity_trees<F: QueryFilter> (world: &World, root_entities: Query<Entity, F>) {
	for entity in &root_entities {
		if world.get::<ChildOf>(entity).is_none() {
			_log_entity_tree(world, entity, "", "");
		}
	}
}

fn log_scene_hierarchy (world: &World, query: Query<Entity>) {
	log_entity_trees(world, query);
	//println!("------------------------");
}
