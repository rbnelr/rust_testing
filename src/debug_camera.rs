use bevy::prelude::*;
use crate::phases::Phase;

pub struct DebugCameraPlugin;
impl Plugin for DebugCameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, update
			.after(Phase::SerializationAndImgui)
			.before(Phase::CameraUpdate)
		);
	}
}

#[derive(Component)]
pub struct DebugCamera;

#[derive(Default)]
struct DebugCameraState {
	viewing_debug_cam : bool,
}

fn update(
	mut state: Local<DebugCameraState>,
	keyboard: Res<ButtonInput<KeyCode>>,
	main_cam: Single<(&mut Camera, &Transform), Without<DebugCamera>>,
	debug_cam: Single<(&mut Camera, &mut Transform), With<DebugCamera>>
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
