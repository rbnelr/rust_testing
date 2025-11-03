use bevy::prelude::*;
use bevy_egui::egui::{
	Color32, Context, Pos2, Rect, Ui, Rangef, Stroke, RichText, Label,
	containers::{Frame, Window},
	emath, epaint,
	lerp, pos2, remap, vec2,
};
use std::collections::VecDeque;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct Frametimes {
	frametimes: VecDeque<f32>,
	frametimes_avg_rate : Timer,
	frametime_avg : FrametimeAvg,
}
impl Default for Frametimes {
	fn default() -> Self {
		Self{
			frametimes: VecDeque::with_capacity(64),
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
		
		let height : f32 = 60.0;
		
		let bar_width : f32 = 1.0;
		let max_y : f32 = 20.0;
		
		Frame::canvas(ui.style()).show(ui, |ui| {
			ui.ctx().request_repaint();
		
			let desired_size = vec2(ui.available_width(), height);
			let (_id, rect) = ui.allocate_space(desired_size);
			
			let count = (rect.width() / bar_width).floor() as usize;
			let draw_width = count as f32 * bar_width;
			
			// enforce max measurement buffer (now that correct count is known)
			while self.frametimes.len() >= count {
				self.frametimes.pop_front();
			}
			// push new measurement
			self.frametimes.push_back(time.delta_secs());
			
			// Plot
			let to_screen = emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=draw_width, max_y..=0.0), rect);
			
			let thres : [f32; 2] = [1000.0 / 120.0, 1000.0 / 60.0];
			let cols = [Color32::GREEN, Color32::YELLOW, Color32::RED];
			
			let mut shapes = vec![];
			let mut x = 0;
			for val in &self.frametimes {
				shapes.push(epaint::Shape::line_segment([
						to_screen * Pos2::new((x as f32 + 0.5) * bar_width, 0.0),
						to_screen * Pos2::new((x as f32 + 0.5) * bar_width, val * 1000.0)
					],
					Stroke { width: bar_width, color: {
							let mut col = cols[2];
							for i in 0..2 {
								if val * 1000.0 <= thres[i] { col = cols[i]; break; }
							};
							Color32::from_rgba_unmultiplied(col.r(), col.g(), col.b(), 200)
						}
					}
				));
				x += 1;
			}
			
			for i in 0..2 {
				shapes.push(epaint::Shape::line_segment([
						to_screen * Pos2::new(0.0, thres[i]),
						to_screen * Pos2::new(draw_width, thres[i])
					],
					Stroke { width: 1.0, color: Color32::from_rgba_unmultiplied(cols[i].r(), cols[i].g(), cols[i].b(), 150) }
				));
			}
			
			ui.painter().with_clip_rect(rect).extend(shapes);
			
			{ // update averages and display after histogram
				if self.frametimes_avg_rate.tick(time.delta()).just_finished() {
					self.frametime_avg = calc_frametime_avg(&self.frametimes);
				}
				let avg = &self.frametime_avg;
				let avg_hz = 1.0 / avg.mean;
				
				let txt = format!("avg: {:5.1} hz ({:6.3} ms  min: {:6.3}  max: {:6.3}  stddev: {:6.3})",
							avg_hz, avg.mean * 1000.0, avg.min * 1000.0, avg.max * 1000.0, avg.std_dev * 1000.0);
				ui.add(Label::new(RichText::new(txt).color(Color32::from_white_alpha(220))).truncate());
			}
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
fn calc_frametime_avg (frametimes: &VecDeque<f32>) -> FrametimeAvg {
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
