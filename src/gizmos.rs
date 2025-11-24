use bevy::{prelude::*, color::palettes::css::*};
use crate::phases::*;
use crate::debug_camera::*;

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, startup);
		app.add_systems(Update, xyz_gizmo.after(Phase::CameraUpdate));
	}
}

// Reference for later: https://bevy.org/examples/gizmos/3d-gizmos/

fn startup (mut config_store: ResMut<GizmoConfigStore>) {
	// Gizmos occluded by geometry by default, which is not what I (usually) want
	// Simply draw all gizmos on top for now WARNING: this caused disappearing lines in the online example
	for (_, config, _) in config_store.iter_mut() {
		config.depth_bias = -1.0;
	}
}

fn xyz_gizmo(mut gizmos: Gizmos, cam: Single<(&Transform, &Projection), With<MainCamera>>) {
	let dist_from_cam: f32 = 15.0;
	let scale: f32 = 1.0;
	let tip: f32 = 0.3;
	let edge_percent: f32 = 0.05;
	
	let (cam_transf, proj) = *cam;
	let corners= proj.get_frustum_corners(-dist_from_cam, -1000.0); // is negative actually correct? otherwise it's behind the camera
	let br : Vec3 = corners[0].into(); // bottom right in cam space
	let tl : Vec3 = corners[2].into(); // bottom left in cam space
	let pos_cam = Vec3::lerp(br, tl, edge_percent); // 5% into bottom right corner
	let pos = *cam_transf * pos_cam;
	
	gizmos.arrow(pos, pos + Vec3::X * scale, RED).with_tip_length(tip);
	gizmos.arrow(pos, pos + Vec3::Y * scale, GREEN).with_tip_length(tip);
	gizmos.arrow(pos, pos + Vec3::Z * scale, BLUE).with_tip_length(tip);
}
