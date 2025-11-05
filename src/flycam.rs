use bevy::{
	prelude::*,
	math,
	input::mouse::{ MouseMotion, MouseWheel, MouseScrollUnit },
	window::{ CursorGrabMode, CursorIcon, CursorOptions, SystemCursorIcon },
};
use core::f32;
use std::fmt;
use crate::app_control::WindowSettings;
use crate::phases::Phase;
use crate::serialization::*;

pub struct FlycamPlugin;

impl Plugin for FlycamPlugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Update, update_cursor.before(update_camera))
			.add_systems(Update, update_camera.in_set(Phase::CameraUpdate));
	}
}

const MOUSELOOK_BTN : MouseButton = MouseButton::Middle;

#[derive(Component, Reflect)]
#[require(Transform, Camera3d, Camera)]
#[reflect(Component)]
pub struct Flycam {
	pub move_planar : bool,
	pub vfov_multiplied_sensitivity : bool,
	pub mouse_sens : f32,
	
	pub default_vfov : f32,
	pub vfov_target : f32,
	pub vfov_smooth : f32,
	
	pub zoom_speed : f32,
	
	pub speed : f32,
	pub base_speed : f32,
	pub max_speed : f32,
	pub speedup_factor : f32,
	pub fast_multiplier : f32,
}
impl Flycam {
	pub fn new(transf: Transform) -> (Transform, Camera3d, Projection, Flycam) {
		let vfov = 70.0_f32.to_radians();
		(
			transf,
			Camera3d::default(),
			Projection::Perspective(PerspectiveProjection {
				fov: vfov,
				near: 0.1,
				far: 10000.0,
				..default()
			}),
			Flycam {
				move_planar: true,
				vfov_multiplied_sensitivity: true,
				// if vfov_multiplied_sensitivity == false:
				//mouse_sens: 120_f32.to_radians() / 1000.0, // degrees / mouse 'dots'
				// if vfov_multiplied_sensitivity == true:
				mouse_sens: 2.0 / 1000.0, // screen heights / mouse 'dots'
				
				default_vfov: vfov,
				vfov_target: vfov,
				vfov_smooth: 25.0,
				zoom_speed: 1.5,
				
				speed: 4.0,
				base_speed: 4.0,
				
				max_speed: 1000000.0,
				speedup_factor: 2.0,
				fast_multiplier: 4.0,
			}
		)
	}
}
serializer!(Flycam, move_planar, mouse_sens, default_vfov, base_speed, speedup_factor);

fn wrap(x: f32, y: f32) -> f32 {
	((x % y) + y) % y
}
fn get_mouse_scroll_delta(mouse_wheel: &mut MessageReader<MouseWheel>) -> f32 {
	let mut total_lines : f32 = 0.0;
	for event in mouse_wheel.read() {
		//match event.unit {
		//	MouseScrollUnit::Line => println!("Scroll (Lines): {:8.3}", event.y),
		//	MouseScrollUnit::Pixel => println!("Scroll (Pixels): {:8.3}", event.y),
		//}
		
		// Scrolling is weird, normally on windows each scroll tick results in +-120 (pixels?)
		// Supposedly there are also smooth scrolling mice, which would presumably return smaller increments?
		// Here we seem to get Line with +-1 (or more if the OS combined events)
		// But on wasm supposedly it returns +-100
		// What would other OSes or smooth scrolling mice result in?
		// This is my best guess at what makes sense
		total_lines += match event.unit {
			MouseScrollUnit::Line => event.y,
			MouseScrollUnit::Pixel => event.y / 100.0,
		}
	}
	total_lines
}
fn get_mouselook_sensitivity(flycam: &Flycam, proj: &Projection) -> f32 {
	if flycam.vfov_multiplied_sensitivity {
		if let Projection::Perspective(persp) = proj {
			return flycam.mouse_sens * persp.fov;
		}
	}
	
	return flycam.mouse_sens;
}

fn zoom(
		time: &Res<Time>,
		keyboard: &Res<ButtonInput<KeyCode>>,
		mouse_wheel: &mut MessageReader<MouseWheel>,
		flycam: &mut Flycam, proj: &mut Projection) {
	// key zoom
	let mut zoom_dir: f32 = 0.0;
	if keyboard.any_pressed([KeyCode::Equal, KeyCode::NumpadAdd]) { zoom_dir += 1.0; }
	if keyboard.any_pressed([KeyCode::Minus, KeyCode::NumpadSubtract]) { zoom_dir -= 1.0; }
	
	let mut zoom_delta = zoom_dir * flycam.zoom_speed * time.delta_secs();
	
	// mousewheel zoom
	if zoom_delta == 0.0 {
		// 0.125 to kinda bring it in line with keyboard based zooming
		zoom_delta = 0.125*get_mouse_scroll_delta(mouse_wheel);
	}
	
	// F + Mousewheel or +/- Zooms FOV
	if keyboard.pressed(KeyCode::KeyF) {
		let mut fov = flycam.vfov_target;
		fov = 2.0_f32.powf(fov.log2() - zoom_delta);
		
		let min_vfov = 0.1_f32.to_radians();
		let max_vfov = 170.0_f32.to_radians();
		fov = fov.clamp(min_vfov, max_vfov);
		
		if keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) && zoom_delta != 0.0 {
			fov = flycam.default_vfov;
		}
		flycam.vfov_target = fov;
	}
	// Otherwise Mousewheel or +/- Zooms FOV changes base speed (later clamped in movement)
	else {
		flycam.base_speed = 2.0_f32.powf(flycam.base_speed.log2() + zoom_delta);
	}
	
	if let Projection::Perspective(persp) = proj {
		
		persp.fov.smooth_nudge(&flycam.vfov_target, flycam.vfov_smooth, time.delta_secs());
		
		//println!("VFov: {:8.3}", persp.fov.to_degrees());
	}
}
fn mouselook(
		time: &Res<Time>,
		keyboard: &Res<ButtonInput<KeyCode>>,
		mouse: &Res<ButtonInput<MouseButton>>,
		mut mouse_motion: &mut MessageReader<MouseMotion>,
		cursor_options: &CursorOptions,
		transf: &mut Transform, flycam: &Flycam, proj: &Projection) {
	
	let pitch_min = (-90.0_f32 + 5.0).to_radians();
	let pitch_max = ( 90.0_f32 - 5.0).to_radians();
	
	let euler = EulerRot::YXZ;
	let (mut yaw, mut pitch, mut roll) = transf.rotation.to_euler(euler);
	
	if mouse.pressed(MOUSELOOK_BTN) || cursor_options.grab_mode != CursorGrabMode::None {
		let sens = get_mouselook_sensitivity(flycam, proj);
		for event in mouse_motion.read() {
			// NOTE: For this camera it makes sense to scale mouselook with fov
			// This is not always the case but would fit an FPS games
			// where muscle memory likely works based on distances on screen (which do depend on fov if zoomed in)
			yaw   -= event.delta.x * sens;
			pitch -= event.delta.y * sens;
		}
	}
	// due to Quat.to_euler, we seem to automatically get a [-180, 180] wrapping for yaw and roll, which is good enough
	//yaw = wrap(yaw, f32::consts::TAU); // wrap into [0, 360deg] range
	pitch = pitch.clamp(pitch_min, pitch_max);
	
	let enable_roll = false;
	if enable_roll {
		let mut roll_dir = 0.0_f32;
		let roll_speed = 90_f32.to_radians();
		if keyboard.pressed(KeyCode::KeyQ) { roll_dir += 1.0; }
		if keyboard.pressed(KeyCode::KeyE) { roll_dir -= 1.0; }
		
		roll -= roll_dir * (time.delta_secs() * roll_speed);
	}
	else {
		roll = 0.0;
	}
	//roll = (roll + f32::consts::PI) % f32::consts::TAU - f32::consts::PI; // wrap into [-180deg, 180deg] range
	
	transf.rotation = Quat::from_euler(euler, yaw, pitch, roll);
	
	//println!("Rot: {:8.3}, {:8.3}, {:8.3}", yaw.to_degrees(), pitch.to_degrees(), roll.to_degrees());
}
fn movement(
		time: &Res<Time>,
		keyboard: &Res<ButtonInput<KeyCode>>,
		transf: &mut Transform, flycam: &mut Flycam) {
	
	fn get_move3d(keyboard: &Res<ButtonInput<KeyCode>>) -> Vec3 {
		let mut dir_local = Vec3::ZERO;
		
		if keyboard.pressed(KeyCode::KeyA) { dir_local.x -= 1.0; }
		if keyboard.pressed(KeyCode::KeyD) { dir_local.x += 1.0; }
		
		if keyboard.pressed(KeyCode::KeyS) { dir_local.z += 1.0; }
		if keyboard.pressed(KeyCode::KeyW) { dir_local.z -= 1.0; }
		
		if keyboard.pressed(KeyCode::KeyQ) { dir_local.y -= 1.0; }
		if keyboard.pressed(KeyCode::KeyE) { dir_local.y += 1.0; }
		
		dir_local.normalize_or_zero()
	}
	let dir_local = get_move3d(&keyboard);
	let mut move_speed = dir_local.length(); // could be analog with gamepad input for get_move3d()
	
	// no movement resets speed
	if move_speed == 0.0 {
		flycam.speed = flycam.base_speed;
	}
	
	if keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
		move_speed *= flycam.fast_multiplier;

		flycam.speed += flycam.base_speed * flycam.speedup_factor * time.delta_secs();
	}

	flycam.speed = flycam.speed.clamp(flycam.base_speed, flycam.max_speed);
	
	//
	let delta_local = dir_local * (time.delta_secs() * flycam.speed);
	
	// WASD move only horizontally, even if looking up/down
	// QE move up/down
	if flycam.move_planar {
		let (yaw, _, _) = transf.rotation.to_euler(EulerRot::YXZ);
		let move_2d = Quat::from_rotation_y(yaw) * Vec3::new(delta_local.x, 0.0, delta_local.z);
		
		transf.translation += move_2d;
		transf.translation.y += delta_local.y;
	}
	// Move forward while looking down will move downwards
	else {
		transf.translation += transf.rotation * delta_local;
	}
}

fn update_camera(
		time: Res<Time>,
		keyboard: Res<ButtonInput<KeyCode>>,
		mouse: Res<ButtonInput<MouseButton>>,
		mut mouse_motion: MessageReader<MouseMotion>,
		mut mouse_wheel: MessageReader<MouseWheel>,
		mut cursor_options: Single<&mut CursorOptions>,
		mut query: Query<(&mut Transform, &mut Flycam, &Camera, &mut Projection), With<Camera3d>>) {
	
	let mut cursor_opt = cursor_options.into_inner();
	
	for (mut transf, mut flycam, cam, mut proj) in &mut query {
		if cam.is_active { // disabling rendering also disables controls
			zoom(&time, &keyboard, &mut mouse_wheel, &mut flycam, proj.as_mut());
			mouselook(&time, &keyboard, &mouse, &mut mouse_motion, &cursor_opt, &mut transf, &mut flycam, &proj);
			movement(&time, &keyboard, &mut transf, &mut flycam);
			// NOTE: controlling multiple cameras does not work since MessageReaders eat input
		}
	}
}

// TODO: move this somewhere else but make it possible for other systems to share control of the cursor in some way
// Ex: In-game Translation gizmos might want to change cursor icon, but not lock it, etc.
// Maybe other users of cursor might want to know about mouselook and freeze during it etc.
// Cameras then could ask cursor for mouselook state (ie. MOUSELOOK_BTN moves to cursor system)

// Make sure to add and remove CursorIcon from Window when needed
// both writing SystemCursorIcon::Default to it every frame and writing SystemCursorIcon::Default to it only to reset
// both cause flickering when resizing window (bug in bevy?)
fn update_cursor(
		keyboard: Res<ButtonInput<KeyCode>>,
		mouse: Res<ButtonInput<MouseButton>>,
		window: Single<(Entity, &Window)>,
		mut cursor_options: Single<&mut CursorOptions>,
		mut commands: Commands) {
	
	let (window_e, window) = *window;
	
	// Handle alt-tab gracefully by fulling resetting everything
	if !window.focused {
		//println!("No Window Focus!");
		cursor_options.visible = true;
		cursor_options.grab_mode = CursorGrabMode::None;
		commands.entity(window_e).remove::<CursorIcon>();
		return;
	}
	
	// Toggle mouse cursor visible via F2 (invisible cursor = FPS style mouselook)
	if keyboard.just_pressed(KeyCode::F2) {
		//println!("Toggle Cursor Visible");
		cursor_options.visible = !cursor_options.visible;
	}
	
	// Mouselooking using held MOUSELOOK_BTN or when cursor invisible
	let mouselook = !cursor_options.visible || mouse.pressed(MOUSELOOK_BTN);
	// Detect mouselook using cursor_options.grab_mode
	let was_mouselook = cursor_options.grab_mode != CursorGrabMode::None;
	
	// Only update grab_mode and cursor_icon when needed instead of overwriting every frame to avoid glitches
	// TODO: 
	if mouselook != was_mouselook {
		//println!("Toggle Mouselook");
		if mouselook {
			//println!("Mouselook=1");
			// Lock mouse cursor, which means freeze in place, and change icon to a "We are mouselooking/dragging" style icon
			// This feels more professional and clean than making it invisible or only constraining it to the window
			cursor_options.grab_mode = CursorGrabMode::Locked;
			let icon : CursorIcon = SystemCursorIcon::AllScroll.into();
			commands.entity(window_e).insert(icon);
		}
		else {
			//println!("Mouselook=0");
			cursor_options.grab_mode = CursorGrabMode::None;
			commands.entity(window_e).remove::<CursorIcon>();
		}
	}
}
