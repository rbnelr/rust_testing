use bevy::{asset::uuid::serde::compact::deserialize, prelude::*};
use bevy_egui::egui::TextBuffer;
use serde::{Serialize, de::DeserializeOwned, ser::Error};

// NOTE:
/*
serde is an API in rust meant to declare serialization for types
this is abstracted in such a way that the serde Serialize and Deserialize traits only actually declare the structure and fields of the data
but the actual serialization is handled by other crates like serde_json, this in theory allows serialization into any format
though this also makes implementing Serialize and Deserialize rather hard to understand and complicated,
though most will likely just use #[derive(Serialize, Deserialize)] to auto-implement it

Quick overview:

// serde core:
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer;
}

pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>;
    
    #[doc(hidden)]
    fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
    where D: Deserializer<'de> {
        // Default implementation just delegates to `deserialize` impl.
        *place = tri!(Deserialize::deserialize(deserializer));
        Ok(())
    }
}

/// ```edition2021
/// use serde::ser::{Serialize, SerializeStruct, Serializer};
///
/// struct Person {
///     name: String,
///     age: u8,
///     phones: Vec<String>,
/// }
///
/// // This is what #[derive(Serialize)] would generate.
/// impl Serialize for Person {
///     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
///     where
///         S: Serializer,
///     {
///         let mut s = serializer.serialize_struct("Person", 3)?;
///         s.serialize_field("name", &self.name)?;
///         s.serialize_field("age", &self.age)?;
///         s.serialize_field("phones", &self.phones)?;
///         s.end()
///     }
/// }
/// ```

// Deserialize has a weird lifetime complication
// which liekly is because it is supported that deserialization creates data which actually hold references to the source data, like strings simply pointing into the json string to avoid copies
// this might be relevant if that is not the case, but I dont't understand it
pub trait DeserializeOwned: for<'de> Deserialize<'de> {}
impl<T> DeserializeOwned for T where T: for<'de> Deserialize<'de> {}

// Deserialize actually involves writing a Visitor object, then passing to the api, which gets called back with the key names
// This seems overengineered and a pain, but could be justified since it allows deserializing without intermediary string-hashmap like json deserialization does
// though when using serde_json, I'm not sure if there's any benefit
// overall I feel like if serialization speed is ever something you need to optimize, you'd likely want to move bulk data out of the serializer and handle it manually in binary instead
// so there overall the visitor api might not be worth it

// Serializer & Deserializer are then what actually does the serialization, so serde_json for example
note that there are assumptions in this api:
-serializing happens on immutable objects by being implemented on the type
 This means you can't call serialize, then pass its return value into serialize_field!
-serialize_field expects something implementing Serialize serialize returns a serializer specific Result<Ok, Error>
-deserializing returns new instances, which means having fields that are skipped by serialization might set them to nonsense values like fov: 0
 not sure how struct default is used
 serializing in place, which is how I personally used to do it in c++ is discouraged, though there are likely good reasons for this
-deserializing fails if any fields are missing in the json, but that's actually desired for anything where setting individual fields to defaults makes sense, like in settings structures
 for more complex data structures like actual game save state there might be good reasons to fail if any part is missing, though

I tried to create custom serializers using traits in this way:

// Attach serializer to type which can be called by code which gets components from query
struct FlycamSer; // either fake type, or attach to tuple
impl SerializeEntity for FlycamSer {
	type SerComponentTuple<'a> = (&Transform, &Flycam) where Self: 'a;
	type DeComponentTuple<'a> = (&mut Transform, &mut Flycam) where Self: 'a;
	
	fn serialize<'a, S> (components: Self::SerComponentTuple<'a>, serializer: S) -> Result<S::Ok, S::Error>
	where S: serde::Serializer {
		// use serializer
	}
	fn deserialize<'a, 'de, D> (components: Self::DeComponentTuple<'a>, deserializer: D) -> Result<(), D::Error>
	where D: serde::Deserializer<'de> {
		// use deserializer with complicated visitor pattern or use local helper struct with auto-impl
	}
}

// The problem is that some things simply don't work with serde,
// like returning the equivalent of a json object from a custom serialize function, Result<S::Ok, S::Error> for serde::Serialize, but it turns out that can't be passed into serialize_field)
// the alternative, to create a serializeable wrapper object that works with the api does not work, because keeping a &mut World in a type to serialize is not allowed due to the api having immutable self
// (mut world allows for queries to be cached even when only reading data)

// So overall I'm left confused and annoyed at this API, which makes things complicated or impossible for seemingly no benefit
// serializing json was easier in c++ (nlohmann json), and in rust I could just use serde_json directly and sidestep all of these headaches
// the only benefit left seems to be the postcard format, which may be an actual binary format,
// so in theory, allowing both json and text based formats at the with the same api might be a benefit, but I'm unsure if that is ever actually useful
// I feel like for 90% of the data, you want to keep it text-based
// while for the 10% that might be bulk data you will want to keep it human readable as long as the performance is ok
// and the moment you run into issues with performance or file size, you go and write a custom binary serializer that you actually have control over
// doing your own data queries, deciding how to encode data, writing out a few Vec of structs or possibly u8, then writing them into a file would likely be superior

Note:
there is bevy_serde_lens, which attempts to implement serialization for resources and enities, but the only model it has is newly spawning deserialized entities
which is not my model, for example serializing the main camera involves writing serializers that define which camera data should be persisted, (position, orientation, certain settings like fov, speed)
and then updating data within the existing camera when deserializing, deserializing entire components could be made to work, but spawning entire new entities just does not work
it would either have to be heavily worked around, or we have to always serialize the all camera components, which includes things like bloom settings, which just don't belong there in the settings.json
overall I feel like giving the serialization code a query like Single<> would work better
*/

pub trait MyWorldSerializer {
	type Serializeable<'a>: serde::Serialize;
	
	fn serialize (world: &mut World) -> Result<Self::Serializeable<'_>, String>; // Error type?
	
	fn deserialize<'de, D> (world: &mut World, deserializer: D) -> Result<(), D::Error>
	where D: serde::Deserializer<'de>;
}
pub trait EntitySerializer where Self: Sized {
	type SerComponentTuple<'a> where Self: 'a;
	type DeComponentTuple<'a> where Self: 'a;
	
	fn serialize<'a, S> (components: Self::SerComponentTuple<'a>, serializer: S) -> Result<S::Ok, S::Error>
	where S: serde::Serializer;
	fn deserialize<'a, 'de, D> (components: Self::DeComponentTuple<'a>, deserializer: D) -> Result<(), D::Error>
	where D: serde::Deserializer<'de>;
}

macro_rules! world_serializer {
	($type:ty, $($field:ident: $world_serializer_type:ty),* $(,)?) => {
//		impl MyWorldSerializer for $type {
//			fn serialize<S> (world: &mut World, serializer: S) -> Result<S::Ok, S::Error>
//			where S: serde::Serializer {
//				//Ok(S::Ok())
//				$(
//				<$world_serializer_type as MyWorldSerializer>::serialize(world, serializer)?
//				)*
//				Ok()
//			}
//			fn deserialize<'de, D> (world: &mut World, deserializer: D) -> Result<(), D::Error>
//			where D: serde::Deserializer<'de> {
//				Ok(())
//			}
//		}
	};
}

impl<T> MyWorldSerializer for Res<'_, T>
where T: Resource + Serialize + DeserializeOwned {
	type Serializeable<'a> = &'a T;
	
	fn serialize (world: &mut World) -> Result<Self::Serializeable<'_>, String> {
		if let Some(res) = world.get_resource::<T>() {
			Ok(&res)
		}
		else {
			error!("Error serializing Resource {}: resource missing in world", std::any::type_name::<T>());
			Err("missing resource in world".into())
		}
	}
	fn deserialize<'de, D> (world: &mut World, deserializer: D) -> Result<(), D::Error>
	where D: serde::Deserializer<'de> {
		match T::deserialize(deserializer) {
			Ok(res) => {
				world.insert_resource(res); // replace or lazily insert TODO: ?
				Ok(())
			},
			Err(e) => {
				error!("Error deserializing Resource {}: {}", std::any::type_name::<T>(), e);
				Err(e)
			}
		}
	}
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
				// The visitor pattern for deserialize_struct is a bit complex, fall back to serde's derive for now
				#[derive(serde::Deserialize)]
				struct Helper {
					$($component_name: $component_type),*
				};
				// deserializes components into new structs
				let new_components = Helper::deserialize(deserializer)?;
				
				// Destructure tuple to make members accesible by macro as macros are to dumb to count indices
				#[allow(non_snake_case)]
				let ($(mut $component_name),*) = components;
				
				$(
				// Replace components via Mut<> component reference in tuple, which will trigger change detection
				*$component_name = new_components.$component_name;
				)*
				
				Ok(())
				
				//use serde::de::{self, MapAccess, Visitor};
				//use std::fmt;
				//
				//const FIELDS : &[&str] = &["transform", "flycam"];
				//struct ComponentsVisitor<'a> {
				//	$($component_name: Mut<'a, $component_type>),*
				//};
				//
				//impl<'a, 'de> Visitor<'de> for ComponentsVisitor<'a> {
				//	type Value = ();
				//	
				//	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				//		formatter.write_str("a map of components")
				//	}
				//
				//	fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
				//	where A: MapAccess<'de>
				//	{
				//		// Track which fields we've seen
				//		$(let mut $component_name = false;)*
				//		
				//		while let Some(key) = map.next_key::<String>()? {
				//			match key.as_str() {
				//			$(
				//				stringify!($component_name) => {
				//					if $component_name {
				//						return Err(de::Error::duplicate_field(stringify!($component_name)));
				//					}
				//					// Deserialize to Mut<> component reference in tuple, which will trigger change detection
				//					*self.$component_name = map.next_value()?;
				//					// TODO: any mismatch in component and json will cause error and stop serialization (but keep partial changes!)
				//					
				//					//self.$component_name.deserialize_in_place();
				//					
				//					$component_name = true;
				//				}
				//			)*
				//				"transform" => { println!("transform"); },
				//				"flycam" => { println!("flycam"); },
				//				
				//				//_ => Err(de::Error::unknown_field(value, FIELDS)),
				//				_ => {},
				//			}
				//		}
				//		Ok(())
				//	}
				//}
				//
				//let ($($component_name),*) = components;
				//let visitor = ComponentsVisitor{ $($component_name),* };
				//
				//deserializer.deserialize_struct(stringify!($type), FIELDS, visitor)?;
				//
				//Ok(())
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
