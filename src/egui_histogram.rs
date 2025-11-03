use bevy::prelude::*;
use bevy_egui::egui::{
	Color32, Context, Pos2, Rect, Ui, Rangef, Stroke,
	containers::{Frame, Window},
	emath, epaint,
	lerp, pos2, remap, vec2,
};

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct Frametimes {
	frametimes: Vec<f32>,
	frametime_cur: usize,
	frametimes_avg_rate : Timer,
	frametime_avg : FrametimeAvg,
}
impl Default for Frametimes {
	fn default() -> Self {
		Self{
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
impl Frametimes {
	pub fn gui(&mut self, ui: &mut Ui, time: Res<Time>) {
		let idx = self.frametime_cur;
		self.frametimes[idx] = time.delta_secs();
		self.frametime_cur = (self.frametime_cur + 1) % self.frametimes.len();
		
		if self.frametimes_avg_rate.tick(time.delta()).just_finished() {
			self.frametime_avg = calc_frametime_avg(&self.frametimes);
		}
		let avg = &self.frametime_avg;
		let avg_hz = 1.0 / avg.mean;
		
		ui.label(format!("avg: {:5.1} hz ({:6.3} ms  min: {:6.3}  max: {:6.3}  stddev: {:6.3})",
						avg_hz, avg.mean * 1000.0, avg.min * 1000.0, avg.max * 1000.0, avg.std_dev * 1000.0));
		
		self.plot_histogram(ui, &self.frametimes);
		
		//ui.plot("Frametimes", ft.frametimes.as_slice())
		//	.scale_min(0.0)
		//	.scale_max(20.0 / 1000.0)
		//	.graph_size([ 0.0, 60.0 ])
		//	.build();
	}
	
	pub fn plot_histogram(&self, ui: &mut Ui, numbers: &Vec<f32>) {
		Frame::canvas(ui.style()).show(ui, |ui| {
			ui.ctx().request_repaint();
			
			let n = numbers.len();
			let max_y : f32 = 20.0;
			let height : f32 = 60.0;
			
			let desired_size = vec2(ui.available_width(), height);
			let (_id, rect) = ui.allocate_space(desired_size);
			let width = desired_size.x;
			let bar_width = width / (n as f32);

			let to_screen =
				emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=n as f32, 0.0..=max_y), rect);

			let mut shapes = vec![];
			
			for i in 0..n {
				let a = to_screen * Pos2::new(i as f32, max_y);
				let b = to_screen * Pos2::new(i as f32, numbers[i] * 1000.0);
				shapes.push(epaint::Shape::line_segment(
					[a, b],
					Stroke { width: bar_width.floor().max(0.0), color:Color32::WHITE }
				));
			}
			
			ui.painter().extend(shapes);
		});
	}
}

#[derive(Reflect)]
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
