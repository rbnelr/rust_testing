use bevy::prelude::*;
use bevy::ecs::system::{SystemState};
use bevy::window::{CursorIcon, CursorOptions, PrimaryWindow, WindowMode, PresentMode};
use serde::{Serialize, Deserialize};
use crate::phases::Phase;
use crate::serialization::{RenderSettings, Settings};
use bevy_egui::*;

pub struct AppControlPlugin;
impl Plugin for AppControlPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(WindowSettings::default())
			.add_systems(Update, window_control.in_set(Phase::Windowing))
			.add_systems(EguiPrimaryContextPass, ui_example_system //.in_set(Phase::Windowing) execute UI first?
			);
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

fn ui_example_system(
	world: &mut World,
	sys: &mut SystemState<(
		Res<Time>,
		ResMut<WindowSettings>,
		MessageWriter<AppExit>,
		Commands,
	)>
) -> Result {
	let mut egui_context = world
		.query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
		.single_mut(world)?.clone();
	
	egui::Window::new("Main").show(egui_context.get_mut(), |ui| {
		
		let mut do_load = false;
		let mut do_save = false;
		
		let (
			time,
			mut window_settings,
			mut exit,
			commands,
		) = sys.get_mut(world);
		
		ui.label("world");
	});
	Ok(())
}

/*
let ui = ctx.ui();
	let gui = ui.window("Hello world");
	gui
		.size([300.0, 100.0], Condition::FirstUseEver)
		.position([0.0, 0.0], Condition::FirstUseEver)
		.build(|| {
			let mut ws = *window_settings;
			
			ui.checkbox("Fullscreen", &mut ws.fullscreen);
			
			ui.same_line();
			ui.checkbox("Borderless", &mut ws.fullscreen_borderless);
			
			ui.same_line();
			ui.checkbox("Vsync", &mut ws.vsync);
			
			if ws != *window_settings { // for change-detection
				*window_settings = ws;
			}
			
			ui.same_line();
			ui.checkbox("ImGui Demo", &mut state.demo_window_open);
			
			// TODO: are these colors correct like this?
			let color1 = ui.push_style_color(StyleColor::Button, Color::srgba_u8(250, 66, 66, 102).to_linear().to_f32_array());
			let color2 = ui.push_style_color(StyleColor::ButtonActive, Color::srgba_u8(250, 66, 66, 255).to_linear().to_f32_array());
			let color3 = ui.push_style_color(StyleColor::ButtonHovered, Color::srgba_u8(250, 15, 15, 255).to_linear().to_f32_array());
			if ui.button("Quit") {
				exit.write(AppExit::Success);
			}
			color3.pop();
			color2.pop();
			color1.pop();
			
			ui.text("debug.json:");
			ui.same_line();
			do_load = ui.button("Load [;]");
			ui.same_line();
			do_save = ui.button("Save [']");
			
			ui.separator();
			imgui_fps_histogram(&ui, &mut state, &time);
		});
	
	if state.demo_window_open {
		ctx.ui().show_demo_window(&mut state.demo_window_open);
	}
	
	if do_load {
		world.run_system_once(load);
	}
	else if do_save {
		world.run_system_once(save);
	}
*/