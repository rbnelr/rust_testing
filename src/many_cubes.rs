use core::f32;

use bevy::prelude::*;
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::*;
use bevy_egui::*;
use egui::{Ui, RichText, Color32};
use crate::util::*;

pub struct ManyCubesPlugin;

impl Plugin for ManyCubesPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, startup);
		app.add_systems(Update, respawn);
		app.add_systems(Update, update_cubes.run_if(|set: Res<ManyCubesSystemSettings>| set.spinning));
		app.add_systems(EguiPrimaryContextPass, ui);
	}
}

// Split Resources since Handles do not support egui or serialization currently
// TODO: implement myself?
#[derive(Resource)]
struct ManyCubesSystem {
	mesh: Handle<Mesh>,
	materials: Vec<Handle<StandardMaterial>>,
}
#[derive(Resource, Reflect, Serialize, Deserialize, Copy, Clone, PartialEq)]
#[reflect(Resource)]
pub struct ManyCubesSystemSettings {
	random_material: bool,
	rng_seed: u32,
	count: u32,
	spacing: f32,
	spinning: bool,
}

fn ui(mut egui: Single<&mut EguiContext, With<PrimaryEguiContext>>, mut set: ResMut<ManyCubesSystemSettings>) -> Result {
	let mut s = *set;
	egui::Window::new("Many Cubes").show(egui.get_mut(), |ui| {
		ui.checkbox(&mut s.random_material, "random_material");
		ui.add(egui::DragValue::new(&mut s.rng_seed));
		ui.add(egui::Slider::new(&mut s.count, 0..=100000));
		ui.add(egui::DragValue::new(&mut s.spacing).speed(0.1));
		ui.checkbox(&mut s.spinning, "spinning");
	});
	if s != *set { *set = s; }
	Ok(())
}

#[derive(Component, Serialize, Deserialize, Reflect)]
struct ManyCubesCube;

fn startup (
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>
) {
	let mesh = meshes.add(Cuboid::new(0.2, 0.2, 0.2));
	
	let materials = vec![
		materials.add(Color::srgb_u8(255, 60, 70)),
		materials.add(Color::srgb_u8(60, 255, 70)),
		materials.add(Color::srgb_u8(60, 70, 255)),
		materials.add(Color::srgb_u8(255, 255, 60)),
		materials.add(Color::srgb_u8(255, 60, 255)),
		materials.add(Color::srgb_u8(60, 255, 255)),
		materials.add(Color::srgb_u8(255, 255, 255)),
		materials.add(Color::srgb_u8(100, 100, 100)),
		materials.add(Color::srgb_u8(20, 20, 20)),
	];
	
	commands.insert_resource(ManyCubesSystem{ mesh, materials });
	commands.insert_resource(ManyCubesSystemSettings{
		random_material: true,
		rng_seed: 1,
		count: 500,
		spacing: 0.25,
		spinning: true,
	});
}

fn respawn(
		mut sys: ResMut<ManyCubesSystem>,
		mut set: Res<ManyCubesSystemSettings>,
		mut commands: Commands,
		cubes: Query<Entity, With<ManyCubesCube>>) {
	
	if set.is_changed() {
		for e in cubes {
			commands.entity(e).despawn(); // despawn batch?
		}
		
		let base_pos = Vec3::new(5.0, set.spacing, 5.0);
		let width = (set.count as f32).sqrt().ceil() as u32;
		let mut rng = ChaCha8Rng::seed_from_u64(set.rng_seed as u64);
		
		for i in 0..set.count {
			let x = i / width;
			let y = i % width;
			let pos = base_pos + Vec3::new(x as f32 * set.spacing, 0.0, y as f32 * set.spacing);
			
			let mat = if set.random_material {
				rng.random_item(&sys.materials).clone()
			}
			else {
				sys.materials[0].clone()
			};
			
			commands.spawn((
				ManyCubesCube,
				Mesh3d(sys.mesh.clone()),
				MeshMaterial3d(mat),
				Transform {
					translation: pos,
					..default()
				},
				bevy_camera::visibility::NoFrustumCulling,
			));
		}
	}
}

fn update_cubes(mut time: Res<Time>, cubes: Query<&mut Transform, With<ManyCubesCube>>) {
	let _span = info_span!("update_cubes").entered();
	
	let speed0 : f32 = 45.0_f32.to_radians() * time.delta_secs();
	let speed1 : f32 = 360.0_f32.to_radians() * time.delta_secs();
	
	let mut rng = ChaCha8Rng::seed_from_u64(20);
	for mut transf in cubes {
		let speed = rng.random_range(speed0..speed1);
		let axis = rng.random_direction3d();
		
		transf.rotate(Quat::from_axis_angle(axis, speed));
	}
}
