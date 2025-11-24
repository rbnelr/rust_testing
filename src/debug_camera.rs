use bevy::prelude::*;
use crate::flycam::Flycam;
use crate::phases::Phase;
use serde::*;

/*
	Allows a secondary debug camera to exists which can be swapped to via hotkey
	TODO: visualize main camera when in debug camera mode
	TODO: potentially allow debug camera to be locked but still viewed, and switch over camera control to main camera
	       useful for viewing debugging movement from third person in otherwise first person game
	TODO: view debug cam but use culling and or LOD from main camera, to visualize those
	TODO: all of these would also benefit from splitscreen/picture in picture mode,
	      potentially could even use egui to create a mode where main/debug cam views become egui windows that can be moved and docked as desired (viewing_debug_cam = false switches back to just one main view)
*/

pub struct DebugCameraPlugin;
impl Plugin for DebugCameraPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(DebugCameraState::default())
			.add_systems(Update, update
			.before(Phase::CameraUpdate)
		);
	}
}

#[derive(Component, Serialize, Deserialize, Reflect)]
pub struct MainCamera;

#[derive(Component, Serialize, Deserialize, Reflect)]
pub struct DebugCamera;

#[derive(Resource, Default, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct DebugCameraState {
	viewing_debug_cam : bool,
}

fn update(
	mut state: ResMut<DebugCameraState>,
	keyboard: Res<ButtonInput<KeyCode>>,
	main_cam: Single<(&mut Camera, &Transform), (With<MainCamera>, Without<DebugCamera>)>,
	debug_cam: Single<(&mut Camera, &mut Transform), (With<DebugCamera>, Without<MainCamera>)>,
	mut commands: Commands
) {
	let (mut main_cam, main_transf) = main_cam.into_inner();
	let (mut debug_cam, mut debug_transf) = debug_cam.into_inner();
	
	if keyboard.just_pressed(KeyCode::KeyP) {
		state.viewing_debug_cam = !state.viewing_debug_cam;
		
		if state.viewing_debug_cam {
			*debug_transf = *main_transf;
		}
	}
	
	main_cam.is_active = !state.viewing_debug_cam;
	debug_cam.is_active = state.viewing_debug_cam;
}
