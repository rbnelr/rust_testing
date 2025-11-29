use bevy::prelude::*;
use core::f32;
use std::f32::consts::*;
	
// This is stupid, as this literally produces 0+1+1+1 etc.
macro_rules! _ident_count {
	//($a:ident) => { 1 };
	//($($a:ident),*) => {
	//	0 $(
	//	+ count!($a)
	//	)*
	//};
	
	($a:ident) => { 1 };
	($a:ident,$b:ident) => { 2 };
	($a:ident,$b:ident,$c:ident) => { 3 };
	($a:ident,$b:ident,$c:ident,$d:ident) => { 4 };
	($a:ident,$b:ident,$c:ident,$d:ident,$e:ident) => { 5 };
	($a:ident,$b:ident,$c:ident,$d:ident,$e:ident,$f:ident) => { 6 };
	($a:ident,$b:ident,$c:ident,$d:ident,$e:ident,$f:ident,$g:ident) => { 7 };
	($a:ident,$b:ident,$c:ident,$d:ident,$e:ident,$f:ident,$g:ident,$h:ident) => { 8 };
	($a:ident,$b:ident,$c:ident,$d:ident,$e:ident,$f:ident,$g:ident,$h:ident, $($rest:ident),*) => {
		8 + _ident_count!($($rest),*)
	};
}

pub trait RngExt {
	// Pick random item from slice uniformly
	fn random_item<'a, T> (&mut self, items: &'a [T]) -> &'a T;
	// Random direction, ie. uniform random point on unit sphere
	// Implemented via math instead of rejection based method like rand_distr crate
	fn random_direction3d (&mut self) -> Vec3;
}
impl<R> RngExt for R where R: rand::Rng {
	// NOTE: supposedly "Uniform" Distribution type is faster if sampling a bunch of uniform numbers in a row
	
	fn random_item<'a, T> (&mut self, items: &'a [T]) -> &'a T {
		let i = self.random_range(..items.len());
		&items[i]
	}
	fn random_direction3d (&mut self) -> Vec3 {
		// Rejection based would look like this:
		//float3 pos;
		//float len;
		//do {
		//	pos = uniform3f(-1.0f, +1.0f);
		//	len = length_sqr(pos);
		//} while (len > 1.0f || len == 0.0f);
		//
		//return pos / sqrt(len);
		
		// Don't know 
		let azim = self.random_range(0.0..f32::consts::TAU);
		let elev = self.random_range(-1.0_f32..1.0); // Originally did an acos here, but then did sin/cos again on that, so we can skip the cos

		let es = elev.acos().sin();
		return Vec3::new(azim.cos()*es, azim.sin()*es, elev);
	}
}

pub(crate) use _ident_count;
