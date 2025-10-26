use bevy::prelude::*;

pub struct DebugCameraPlugin;

impl Plugin for DebugCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, update);
    }
}

#[derive(Resource)]
struct DebugCameraSystem {
    viewing_debug_cam : bool,
}

fn startup (mut commands: Commands) {
    commands.insert_resource(DebugCameraSystem{ viewing_debug_cam: false });
}
fn update(sys: Res<DebugCameraSystem>,
    main_cam: Query<&mut Camera>) {
    let gravity: f32 = -10.0;

    for (e, mut particle, mut transform) in particles {
        particle.velocity += Vec3::new(0.0, gravity, 0.0) * time.delta_secs();
        transform.translation += particle.velocity * time.delta_secs();

        if transform.translation.y < 0.0 {
            commands.entity(e).despawn();
        }
    }
}