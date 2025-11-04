use bevy::prelude::*;
use bevy::ecs::system::{SystemState, RunSystemOnce};
use bevy::window::{CursorIcon, CursorOptions, PrimaryWindow, WindowMode, PresentMode};
use bevy_egui::*;
use egui::{Ui, RichText, Color32};
use crate::phases::Phase;
use serde::{Serialize, Deserialize};
use crate::serialization;

pub struct AppControlPlugin;
impl Plugin for AppControlPlugin {
	fn build(&self, app: &mut App) {
		//app.insert_resource(WindowSettings::default());
		app.add_systems(Startup, serialization::load); // Load at startup
		app.add_systems(Update, (
			save_load_controls,
			window_control.after(save_load_controls)
		).in_set(Phase::Start));
		app.add_systems(EguiPrimaryContextPass, main_ui); //.in_set(Phase::Windowing) execute UI first?);
	}
}

#[derive(Resource, Reflect, Copy, Clone, PartialEq, Serialize, Deserialize)]
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

pub fn save_load_controls(
	world: &mut World,
	params: &mut SystemState<(
		Res<ButtonInput<KeyCode>>
	)>
) {
	let (do_load, do_save) = {
		let keyboard = params.get(world);
		(keyboard.just_pressed(KeyCode::Semicolon), keyboard.just_pressed(KeyCode::Quote))
	};
	if do_load {
		world.run_system_once(serialization::load);
	}
	else if do_save {
		world.run_system_once(serialization::save);
	}
}

fn main_ui(
	world: &mut World,
	sys: &mut SystemState<(
		Res<Time>,
		ResMut<WindowSettings>,
		MessageWriter<AppExit>,
		Local<crate::egui_histogram::Frametimes>,
		Commands,
	)>
) -> Result {
	let mut egui_context = world
		.query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
		.single_mut(world)?.clone();
	
	let mut do_load = false;
	let mut do_save = false;
	
	egui::Window::new("Main").show(egui_context.get_mut(), |ui| {
		
		let (
			time,
			mut window_settings,
			mut exit,
			mut frametimes,
			commands,
		) = sys.get_mut(world);
		
		let mut ws = *window_settings;
		
		ui.horizontal(|ui| {
			ui.checkbox(&mut ws.fullscreen, "Fullscreen");
			ui.checkbox(&mut ws.fullscreen_borderless, "Borderless");
			ui.checkbox(&mut ws.vsync, "Vsync");
			
			ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
				if ui.button(RichText::new("Quit").color(Color32::RED)).clicked() {
					exit.write(AppExit::Success);
				}
			});
		});
			
		if ws != *window_settings { // for change-detection
			*window_settings = ws;
		}
		
		frametimes.gui(ui, time);
		
		ui.add_space(6.0);
		
		ui.horizontal(|ui| {
			ui.label("settings.json:");
			if ui.button("Load [;]").clicked() {
				do_load = true;
			}
			if ui.button("Save [']").clicked() {
				do_save = true;
			}
		});
	});
	
	if do_load {
		world.run_system_once(serialization::load);
	}
	else if do_save {
		world.run_system_once(serialization::save);
	}
	
	Ok(())
}
