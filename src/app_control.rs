use bevy::prelude::*;
use bevy::window::{CursorIcon, CursorOptions, PrimaryWindow, WindowMode, PresentMode};
use serde::{Serialize, Deserialize};
use crate::phases::Phase;
use crate::serialization::{RenderSettings, Settings};

pub struct AppControlPlugin;
impl Plugin for AppControlPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(WindowSettings::default())
			.add_systems(Update, (
				window_control.in_set(Phase::Windowing),
			));
	}
}

#[derive(Resource, Reflect, Serialize, Deserialize, Copy, Clone, PartialEq)]
#[reflect(Resource)]
pub struct WindowSettings {
	// Ideally I'd be able to restore windowing state after restarting
	// Like floating window position/size, window maximized (which monitor?), window docked, like docked to left size
	// But this probably requires access to Winit
	//pub windowed_position: Option<IVec2>,
	//pub windowed_size: Vec2,
	
	pub fullscreen: bool,
	pub fullscreen_borderless: bool,
	pub vsync: bool,
}
impl Default for WindowSettings {
	fn default() -> Self {
		Self {
			fullscreen: false,
			fullscreen_borderless: true,
			vsync: true,
		}
	}
}

pub const APP_NAME : &str = "Bevy Test Project";

impl WindowSettings {
	fn update(mut window: Mut<Window>, mut settings: ResMut<WindowSettings>) {
		if settings.is_changed() {
			//println!("WindowSettings Change");
			window.mode = match (settings.fullscreen, settings.fullscreen_borderless) {
				(false, _) => WindowMode::Windowed,
				(true, false) => WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current),
				(true, true) => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
			};
			window.present_mode = match settings.vsync {
				false => PresentMode::AutoNoVsync,
				true => PresentMode::AutoVsync,
			};
		}
	}
}

// Can potentially derive settings from bevy state like this (without WindowSettings being a Resource)
// could be useful for other applications, but WindowSettings in my case need to be Resource
//impl WindowSettings {
//	fn save(world: &mut World) -> Self {
//
//		let mut system_state: SystemState<Single<&Window>> = SystemState::new(world);
//		let (query) = system_state.get(world);
//		let w = query.into_inner();
//
//		Self {
//			windowed_position: IVec2::new(0,0),
//			windowed_size: w.physical_size(),
//			fullscreen: matches!(w.mode, WindowMode::Fullscreen(_, _)),
//			fullscreen_borderless: matches!(w.mode, WindowMode::BorderlessFullscreen(_)),
//			vsync: matches!(w.present_mode, PresentMode::AutoVsync),
//		}
//	}
//}

fn window_control(
	keyboard: Res<ButtonInput<KeyCode>>,
	window: Single<&mut Window>,
	mut settings: ResMut<WindowSettings>
) {
	if keyboard.just_pressed(KeyCode::F11) ||
		(keyboard.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) && keyboard.just_pressed(KeyCode::Enter)) {
		
		settings.fullscreen = !settings.fullscreen;
	}
	
	WindowSettings::update(window.into_inner(), settings);
}
