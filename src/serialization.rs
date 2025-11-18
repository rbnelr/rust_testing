use bevy::prelude::*;
use serde::*;

//pub trait MyWorldSerializer {
//	fn serialize<S> (world: &mut World, serializer: S)
//	where S: serde::Serialize;
//	fn deserialize<D> (world: &mut World, deserializer: D)
//	where D: serde::Deserialize<>;
//}
pub trait EntitySerializer where Self: Sized {
	type SerComponentTuple<'a> where Self: 'a;
	type DeComponentTuple<'a> where Self: 'a;
	
	fn serialize<'a, S> (components: Self::SerComponentTuple<'a>, serializer: S) -> Result<S::Ok, S::Error>
	where S: serde::Serializer;
	fn deserialize<'a, 'de, D> (components: Self::DeComponentTuple<'a>, deserializer: D) -> Result<(), D::Error>
	where D: serde::Deserializer<'de>;
}

macro_rules! world_serializer {
	($type:ty, $($field:ident: $query:ty),* $(,)?) => {
		//impl MyWorldSerializer for $type {
		//	fn serialize<S> (world: &mut World, serializer: S) {
		//		
		//	}
		//	fn deserialize<D> (world: &mut World, deserializer: D) {
		//		
		//	}
		//}
	};
}

macro_rules! world_serializer_entity {
	// give custom names for json key of components
	($type:ty, $($component_name:ident : $component_type:ty),* $(,)?) => {
		impl EntitySerializer for $type {
			type SerComponentTuple<'a> = ($(&'a $component_type),*) where Self: 'a;
			type DeComponentTuple<'a> = ($(Mut<'a, $component_type>),*) where Self: 'a;
			
			fn serialize<'a, S> (components: Self::SerComponentTuple<'a>, serializer: S) -> Result<S::Ok, S::Error>
			where S: serde::Serializer {
				//#[derive(serde::Serialize)]
				//struct Helper<'a> {
				//	$($component_name: &'a $component_type),*
				//};
				//let ($($field),*) = components;
				//Helper{ $($field),* }.serialize(serializer)
				
				use serde::ser::SerializeStruct;
				
				// Destructure tuple to make members accesible by macro as macros are to dumb to count indices
				#[allow(non_snake_case)]
				let ($($component_name),*) = components;
				
				let mut _struct = serializer.serialize_struct(stringify!($type), crate::util::_ident_count!($($component_name),*))?;
				$(
				// Serialize from component reference in tuple
				_struct.serialize_field(stringify!($component_name), $component_name)?;
				)*
				_struct.end()
			}
			fn deserialize<'a, 'de, D> (components: Self::DeComponentTuple<'a>, deserializer: D) -> Result<(), D::Error>
			where D: serde::Deserializer<'de> {
				// TODO: can we access the members in the serialized thing and actually deserialize in place?
				// maybe this requires the vistor?
				
				// The visitor pattern for deserialize_struct is a bit complex, fall back to serde's derive for now
				#[derive(serde::Deserialize)]
				struct Helper {
					$($component_name: $component_type),*
				};
				let tmp = Helper::deserialize(deserializer)?;
				
				// Destructure tuple to make members accesible by macro as macros are to dumb to count indices
				#[allow(non_snake_case)]
				let ($(mut $component_name),*) = components;
				
				$(
				// Deserialize to Mut<> component reference in tuple, which will trigger change detection
				*$component_name = tmp.$component_name;
				)*
				
				Ok(())
			}
		}
	};
	// use type as json key of components
	// NOTE: does not support type names like, do use flycam::Flycam first or use above pattern of macro
	($type:ty, $($component_type:tt),* $(,)?) => {
		world_serializer_entity!($type, $($component_type : $component_type),*);
	}
}

pub(crate) use world_serializer;
pub(crate) use world_serializer_entity;
