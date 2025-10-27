use core::f32;
use bevy::color::Color::Srgba;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::{CursorIcon, CursorOptions, PrimaryWindow, WindowMode};
use bevy_mod_imgui::prelude::*;
use bevy::window::*;
use crate::flycam::Flycam;

pub struct ImguiPlugin;

impl Plugin for ImguiPlugin {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(ImguiState::default())
			.add_plugins(bevy_mod_imgui::ImguiPlugin{
				ini_filename: Some("imgui.ini".into()),
				font_oversample_h: 2,
				font_oversample_v: 2,
				..default()
			})
			.add_systems(Update, (imgui_main_ui, window_key_actions));
	}
}

#[derive(Resource)]
struct ImguiState {
	demo_window_open: bool,
	
	// TODO: put into separate struct & file
	frametimes: Vec<f32>,
	frametime_cur: usize,
	frametimes_avg_rate : Timer,
	frametime_avg : FrametimeAvg,
}
impl Default for ImguiState {
	fn default() -> Self {
		Self{
			demo_window_open: false,
			
			frametimes: vec![0.0; 120],
			frametime_cur: 0,
			frametimes_avg_rate: {
				let freq = std::time::Duration::from_secs_f32(0.5);
				Timer::new(freq, TimerMode::Repeating)
			},
			frametime_avg: FrametimeAvg{ mean:0.0, min:0.0, max:0.0, std_dev:0.0 },
		}
	}
}

struct FrametimeAvg {
	mean: f32,
	min: f32,
	max: f32,
	std_dev: f32,
}
fn calc_frametime_avg (frametimes: &Vec<f32>) -> FrametimeAvg {
	let total : f32 = frametimes.iter().sum();
	let count = frametimes.len() as f32;
	let mean = total / frametimes.len() as f32;
	
	let mut min = f32::INFINITY;
	let mut max = f32::NEG_INFINITY;
	let mut variance: f32 = 0.0;
	
	for val in frametimes {
		min = min.min(*val);
		max = max.max(*val);

		let tmp = val - mean;
		variance += tmp*tmp;
	}
	
	let std_dev = (variance / (count - 1.0)).sqrt();
	
	FrametimeAvg { mean, min, max, std_dev }
}
	
fn imgui_main_ui(
	mut context: NonSendMut<ImguiContext>,
	mut state: ResMut<ImguiState>,
	time: Res<Time>,
	window: Single<&mut Window>,
	mut exit: MessageWriter<AppExit>
) {
	let ui = context.ui();
	let gui = ui.window("Hello world");
	gui
		.size([300.0, 100.0], Condition::FirstUseEver)
		.position([0.0, 0.0], Condition::FirstUseEver)
		.build(|| {
			ui.text("Hello world!");
			ui.text("This...is...bevy_mod_imgui!");
			ui.separator();
			let mouse_pos = ui.io().mouse_pos;
			ui.text(format!(
				"Mouse Position: ({:.1},{:.1})",
				mouse_pos[0], mouse_pos[1]
			));
			
			{
				let mut wind = window.into_inner();
				
				let mut fullscreen = is_fullscreen(&wind);
				if ui.checkbox("Fullscreen", &mut fullscreen) {
					set_fullscreen(&mut wind, fullscreen);
				}
				
				ui.same_line();
				let mut vsync = is_vsync(&wind);
				if ui.checkbox("Vsync", &mut vsync) {
					set_vsync(&mut wind, vsync);
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
			}
	
			let idx = state.frametime_cur;
			state.frametimes[idx] = time.delta_secs();
			state.frametime_cur = (state.frametime_cur + 1) % state.frametimes.len();
			
			if state.frametimes_avg_rate.tick(time.delta()).just_finished() {
				state.frametime_avg = calc_frametime_avg(&state.frametimes);
			}
			let avg = &state.frametime_avg;
			let avg_hz = 1.0 / avg.mean;
			
			ui.text(format!("avg: {:5.1} hz ({:6.3} ms  min: {:6.3}  max: {:6.3}  stddev: {:6.3})",
				avg_hz, avg.mean * 1000.0, avg.min * 1000.0, avg.max * 1000.0, avg.std_dev * 1000.0));
			
			ui.set_next_item_width(-1.0);
			ui.plot_histogram("Frametimes", state.frametimes.as_slice())
			  .scale_min(0.0)
			  .scale_max(20.0 / 1000.0)
			  .graph_size([ 0.0, 60.0 ])
			  .build();
		});

	if state.demo_window_open {
		ui.show_demo_window(&mut state.demo_window_open);
	}
}

// TODO: move out of imgui file
// TODO: Could use something like this to set fullscreen both from imgui and settings.json
// Now imgui does not have to manually modify window, instead we can just use Res<WindowSettings> and set a bool
// And a separate system can check for changes here, you could even use one shot systems to essentially only apply setting changes at startup, at imgui or other UI modification, or via keypress
// Or use change detection, even cleaner?
//#[derive(Resource)]
//struct WindowSettings {
//	fullscreen: bool,
//	fullscreen_borderless: bool,
//}

fn is_fullscreen (window: &Window) -> bool {
	match window.mode {
		WindowMode::Windowed => false,
		WindowMode::BorderlessFullscreen(_) => true,
		WindowMode::Fullscreen(_, _) => true
	}
}
fn set_fullscreen (window: &mut Window, fullscreen: bool) {
	window.mode = match fullscreen {
		false => WindowMode::Windowed,
		true => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
	};
}
fn is_vsync (window: &Window) -> bool {
	match window.present_mode {
		PresentMode::AutoNoVsync => false,
		PresentMode::AutoVsync => true,
		_ => true // Should not happen
	}
}
fn set_vsync (window: &mut Window, vsync: bool) {
	window.present_mode = match vsync {
		false => PresentMode::AutoNoVsync,
		true => PresentMode::AutoVsync,
	};
}

fn window_key_actions(
	keyboard: Res<ButtonInput<KeyCode>>,
	window: Single<&mut Window>
) {
	if keyboard.just_pressed(KeyCode::F11) ||
		(keyboard.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) && keyboard.just_pressed(KeyCode::Enter)) {
		
		let mut wind = window.into_inner();
		let fullscreen = !is_fullscreen(&wind);
		set_fullscreen(&mut wind, fullscreen);
	}
}
