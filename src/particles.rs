use std::f32::INFINITY;
use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, setup_assets)
			.add_systems(Update, spawn_particles)
			.add_systems(Update, update_particles.after(spawn_particles));
	}
}


#[derive(Resource)]
struct ParticleSystem {
	mesh: Handle<Mesh>,
	material: Handle<StandardMaterial>,
	rng: ChaCha8Rng,
}
fn setup_assets (
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>
) {
	let mesh = meshes.add(Cuboid::new(0.1, 0.1, 0.1));
	let material = materials.add(Color::srgb_u8(60, 255, 70));
	
	let mut rng = ChaCha8Rng::seed_from_u64(19878367467711);
	
	commands.insert_resource(ParticleSystem{ mesh, material, rng });
}


#[derive(Component)]
pub struct ParticleEmitter {
	pub spawn_period: f32,
	time_since_last_spawn: f32,
}
impl ParticleEmitter {
	pub fn new(spawn_period: f32) -> Self { Self{
		spawn_period,
		time_since_last_spawn: INFINITY
	} }
}

#[derive(Component)]
struct Particle {
	velocity: Vec3,
}

fn spawn_particles(
		time: Res<Time>,
		spawners: Query<(&mut ParticleEmitter, &GlobalTransform)>,
		mut sys: ResMut<ParticleSystem>,
		mut commands: Commands) {
	let emit_speed: f32 = 7.0;
	let speed_variation: f32 = 2.0;
	
	for (mut spawner, transform) in spawners {
		spawner.time_since_last_spawn += time.delta_secs();
		if spawner.time_since_last_spawn >= spawner.spawn_period {
			spawner.time_since_last_spawn -= spawner.spawn_period;
			
			let var = Vec3::new(sys.rng.random_range(-1.0..1.0),
			                          sys.rng.random_range(-1.0..1.0),
			                          sys.rng.random_range(-1.0..1.0));
			
			commands.spawn((
				Mesh3d(sys.mesh.clone()),
				MeshMaterial3d(sys.material.clone()),
				Transform {
					translation: transform.translation(),
					rotation: transform.rotation(),
					..Default::default()
				},
				Particle {
					velocity: -transform.forward() * emit_speed + var * speed_variation,
				},
			));
		}
	}
}
fn update_particles(
		time: Res<Time>,
		particles: Query<(Entity, &mut Particle, &mut Transform)>,
		mut commands: Commands) {
	let gravity: f32 = -10.0;
	
	for (e, mut particle, mut transform) in particles {
		particle.velocity += Vec3::new(0.0, gravity, 0.0) * time.delta_secs();
		transform.translation += particle.velocity * time.delta_secs();
		
		if transform.translation.y < 0.0 {
			commands.entity(e).despawn();
		}
	}
}