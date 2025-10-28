use core::f32;
use bevy::prelude::*;
use bevy::color::Color::Srgba;
use bevy::ecs::system::{RunSystemOnce, SystemState};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy_mod_imgui::prelude::*;
use crate::flycam::Flycam;
use crate::app_control;
use crate::phases::Phase;
use crate::serialization::{load, save};

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
			.add_systems(Update, imgui_main_ui.in_set(Phase::SerializationAndImgui));
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
	world: &mut World,
	params: &mut SystemState<(
		NonSendMut<ImguiContext>,
		ResMut<ImguiState>,
		Res<Time>,
		ResMut<app_control::WindowSettings>,
		MessageWriter<AppExit>,
		Commands,
	)>
) {
	let mut do_load = false;
	let mut do_save = false;
	
	let (
		mut ctx,
		mut state,
		time,
		mut window_settings,
		mut exit,
		commands,
	) = params.get_mut(world);
	
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
}

fn imgui_fps_histogram(
	ui: &&mut imgui::Ui,
	mut state: &mut ResMut<ImguiState>,
	time: &Res<Time>
) {
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
}

/* // Can't even change styles currently?
fn set_imgui_style(mut ctx: NonSendMut<ImguiContext>) {
	unsafe {
	let mut style = ctx.ctx.write().unwrap();
	
	auto& style = ImGui::GetStyle();
	ImVec4* colors = style.Colors;
	
	colors[ImGuiCol_Text]                   = ImVec4(0.90f, 0.90f, 0.90f, 1.00f);
	colors[ImGuiCol_TextDisabled]           = ImVec4(0.60f, 0.60f, 0.60f, 1.00f);
	colors[ImGuiCol_WindowBg]               = ImVec4(0.09f, 0.09f, 0.11f, 0.83f);
	colors[ImGuiCol_ChildBg]                = ImVec4(0.00f, 0.00f, 0.00f, 0.00f);
	colors[ImGuiCol_PopupBg]                = ImVec4(0.11f, 0.11f, 0.14f, 0.92f);
	colors[ImGuiCol_Border]                 = ImVec4(0.50f, 0.50f, 0.50f, 0.50f);
	colors[ImGuiCol_BorderShadow]           = ImVec4(0.05f, 0.06f, 0.07f, 0.80f);
	colors[ImGuiCol_FrameBg]                = ImVec4(0.43f, 0.43f, 0.43f, 0.39f);
	colors[ImGuiCol_FrameBgHovered]         = ImVec4(0.47f, 0.47f, 0.69f, 0.40f);
	colors[ImGuiCol_FrameBgActive]          = ImVec4(0.42f, 0.41f, 0.64f, 0.69f);
	colors[ImGuiCol_TitleBg]                = ImVec4(0.27f, 0.27f, 0.54f, 0.83f);
	colors[ImGuiCol_TitleBgActive]          = ImVec4(0.32f, 0.32f, 0.63f, 0.87f);
	colors[ImGuiCol_TitleBgCollapsed]       = ImVec4(0.40f, 0.40f, 0.80f, 0.20f);
	colors[ImGuiCol_MenuBarBg]              = ImVec4(0.40f, 0.40f, 0.55f, 0.80f);
	colors[ImGuiCol_ScrollbarBg]            = ImVec4(0.20f, 0.25f, 0.30f, 0.60f);
	colors[ImGuiCol_ScrollbarGrab]          = ImVec4(0.40f, 0.40f, 0.80f, 0.30f);
	colors[ImGuiCol_ScrollbarGrabHovered]   = ImVec4(0.40f, 0.40f, 0.80f, 0.40f);
	colors[ImGuiCol_ScrollbarGrabActive]    = ImVec4(0.41f, 0.39f, 0.80f, 0.60f);
	colors[ImGuiCol_CheckMark]              = ImVec4(0.90f, 0.90f, 0.90f, 0.50f);
	colors[ImGuiCol_SliderGrab]             = ImVec4(1.00f, 1.00f, 1.00f, 0.30f);
	colors[ImGuiCol_SliderGrabActive]       = ImVec4(0.41f, 0.39f, 0.80f, 0.60f);
	colors[ImGuiCol_Button]                 = ImVec4(0.35f, 0.40f, 0.61f, 0.62f);
	colors[ImGuiCol_ButtonHovered]          = ImVec4(0.40f, 0.48f, 0.71f, 0.79f);
	colors[ImGuiCol_ButtonActive]           = ImVec4(0.46f, 0.54f, 0.80f, 1.00f);
	colors[ImGuiCol_Header]                 = ImVec4(0.40f, 0.40f, 0.90f, 0.45f);
	colors[ImGuiCol_HeaderHovered]          = ImVec4(0.45f, 0.45f, 0.90f, 0.80f);
	colors[ImGuiCol_HeaderActive]           = ImVec4(0.53f, 0.53f, 0.87f, 0.80f);
	colors[ImGuiCol_Separator]              = ImVec4(0.50f, 0.50f, 0.50f, 0.60f);
	colors[ImGuiCol_SeparatorHovered]       = ImVec4(0.60f, 0.60f, 0.70f, 1.00f);
	colors[ImGuiCol_SeparatorActive]        = ImVec4(0.70f, 0.70f, 0.90f, 1.00f);
	colors[ImGuiCol_ResizeGrip]             = ImVec4(1.00f, 1.00f, 1.00f, 0.10f);
	colors[ImGuiCol_ResizeGripHovered]      = ImVec4(0.78f, 0.82f, 1.00f, 0.60f);
	colors[ImGuiCol_ResizeGripActive]       = ImVec4(0.78f, 0.82f, 1.00f, 0.90f);
	colors[ImGuiCol_Tab]                    = ImVec4(0.34f, 0.34f, 0.68f, 0.79f);
	colors[ImGuiCol_TabHovered]             = ImVec4(0.45f, 0.45f, 0.90f, 0.80f);
	colors[ImGuiCol_TabActive]              = ImVec4(0.40f, 0.40f, 0.73f, 0.84f);
	colors[ImGuiCol_TabUnfocused]           = ImVec4(0.28f, 0.28f, 0.57f, 0.82f);
	colors[ImGuiCol_TabUnfocusedActive]     = ImVec4(0.35f, 0.35f, 0.65f, 0.84f);
	colors[ImGuiCol_DockingPreview]         = ImVec4(0.40f, 0.40f, 0.90f, 0.31f);
	colors[ImGuiCol_DockingEmptyBg]         = ImVec4(0.20f, 0.20f, 0.20f, 1.00f);
	colors[ImGuiCol_PlotLines]              = ImVec4(1.00f, 1.00f, 1.00f, 1.00f);
	colors[ImGuiCol_PlotLinesHovered]       = ImVec4(0.90f, 0.70f, 0.00f, 1.00f);
	colors[ImGuiCol_PlotHistogram]          = ImVec4(0.90f, 0.70f, 0.00f, 1.00f);
	colors[ImGuiCol_PlotHistogramHovered]   = ImVec4(1.00f, 0.60f, 0.00f, 1.00f);
	colors[ImGuiCol_TableHeaderBg]          = ImVec4(0.27f, 0.27f, 0.38f, 1.00f);
	colors[ImGuiCol_TableBorderStrong]      = ImVec4(0.31f, 0.31f, 0.45f, 1.00f);
	colors[ImGuiCol_TableBorderLight]       = ImVec4(0.26f, 0.26f, 0.28f, 1.00f);
	colors[ImGuiCol_TableRowBg]             = ImVec4(0.00f, 0.00f, 0.00f, 0.29f);
	colors[ImGuiCol_TableRowBgAlt]          = ImVec4(0.19f, 0.19f, 0.19f, 0.29f);
	colors[ImGuiCol_TextSelectedBg]         = ImVec4(0.00f, 0.00f, 1.00f, 0.35f);
	colors[ImGuiCol_DragDropTarget]         = ImVec4(1.00f, 1.00f, 0.00f, 0.90f);
	colors[ImGuiCol_NavHighlight]           = ImVec4(0.45f, 0.45f, 0.90f, 0.80f);
	colors[ImGuiCol_NavWindowingHighlight]  = ImVec4(1.00f, 1.00f, 1.00f, 0.70f);
	colors[ImGuiCol_NavWindowingDimBg]      = ImVec4(0.80f, 0.80f, 0.80f, 0.20f);
	colors[ImGuiCol_ModalWindowDimBg]       = ImVec4(0.20f, 0.20f, 0.20f, 0.35f);
		
		
		style.window_padding      = ImVec2(5,5);
		style.frame_padding       = ImVec2(6,2);
		style.cell_padding        = ImVec2(4,2);
		style.item_spacing        = ImVec2(12,3);
		style.item_inner_spacing  = ImVec2(3,3);
		style.indent_spacing      = 18;
		style.grab_min_size       = 14;
		
		style.window_rounding     = 3.0;
		style.frame_rounding      = 6.0;
		style.popup_rounding      = 3.0;
		style.grab_rounding       = 6.0;
		
		style.window_title_align  = ImVec2(0.5f, 0.5f);
		
		*ctx.ui().push_style_var(StyleVar::WindowPadding(style.window_padding));
	}
}*/
