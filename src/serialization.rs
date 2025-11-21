use bevy::{ecs::query::{QueryData, QueryFilter, ROQueryItem, QueryItem}, prelude::*};
use bevy_egui::egui::TextBuffer;
use serde::Deserialize;
use std::marker::PhantomData;

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

pub type Error = Box<dyn std::error::Error>;
type Json = serde_json::Value;
pub type JsonResult = Result<Json, Error>;

// TODO: auto serialization of entity references using hashmap based serializer state (lazily assign incrementing IDs)
// could also use this to implement "additive/partial" serialization, where we can choose to serialize ex. all vehicle components in one list,
// then serialize all bus components of vehicles that are busses
// advantage:
//  -homogenous data in save file instead of heterogenous list (possibly faster?)
//  if vehicle is the superset of all 'additive' components,
// deserializer can first despawn all vehicles, then spawn empty entities for all, then insert vehicle components from vehicles list, then insert bug components for all busses using id ...
// alternative is having heterogenous serializer where code needs to somehow insert only those components that are in the json

pub trait WorldSerializer {
	fn serialize (world: &mut World) -> Result<serde_json::Value, Error>;
	fn deserialize (world: &mut World, json: &serde_json::Value) -> Result<(), Error>;
}
pub trait EntitySerializer {
	type ComponentTuple;
	type RefTuple<'a>: QueryData;
	type MutTuple<'a>: QueryData;
	
	fn serialize (components: ROQueryItem<'_, '_, Self::RefTuple<'_>>) -> serde_json::Value;
	fn deserialize (components: QueryItem<'_, '_, Self::MutTuple<'_>>, json: &serde_json::Value) -> Result<(), Error>;
	
	// With existing entities, I have to use a query, which right now will return existing component references
	// and for those passing in a MutTuple and having deserialize assign them makes more sense
	// TODO: for entities which get newly spawned we could make a version like this
	//fn deserialize (json: &serde_json::Value) -> Result<Self::ComponentTuple, Error>;
}

pub struct WorldRes<T: Resource>(PhantomData<T>);

// TODO: add different modes when is missing from world or json?
impl<T> WorldSerializer for WorldRes<T>
where T: Resource + serde::Serialize + serde::de::DeserializeOwned {
	// missing resource from world: log error, insert null in json, but continue
	// TODO: make mode where missing is expected and thus does not log error?
	fn serialize (world: &mut World) -> crate::serialization::JsonResult {
		if let Some(res) = world.get_resource::<T>() {
			Ok(serde_json::to_value(res)?)
		}
		else {
			Err(format!("Error serializing Resource {}: missing in world", std::any::type_name::<T>()).into())
		}
	}
	// insert or replace resource in world
	fn deserialize (world: &mut World, json: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
		match <T>::deserialize(json) {
			Ok(res) => {
				world.insert_resource(res);
				Ok(())
			},
			Err(e) => Err(format!("Error deserializing Resource {}: {}", std::any::type_name::<T>(), e).into()),
		}
	}
}

pub struct WorldSingleEntity<T: EntitySerializer, F=()>(PhantomData<T>, PhantomData<F>);

impl<T, F> WorldSerializer for WorldSingleEntity<T, F>
where T: EntitySerializer, F: QueryFilter {
	fn serialize (world: &mut World) -> Result<serde_json::Value, Error> {
		let mut query = world.query_filtered::<T::RefTuple<'_>, F>();
		match query.single(world) {
			Ok(components) => {
				Ok(T::serialize(components))
			},
			Err(e) => {
				Err(format!("Error serializing Single Entity <{},{}>: {}", std::any::type_name::<T>(), std::any::type_name::<F>(), e).into())
			},
		}
	}
	fn deserialize (world: &mut World, json: &serde_json::Value) -> Result<(), Error> {
		let mut query = world.query_filtered::<T::MutTuple<'_>, F>();
		match query.single_mut(world) {
			Ok(components) => {
				T::deserialize(components, json)?;
				Ok(())
			},
			Err(e) => {
				Err(format!("Error deserializing Single Entity <{},{}>: {}", std::any::type_name::<T>(), std::any::type_name::<F>(), e).into())
			},
		}
	}
}

macro_rules! serializer_entity {
	// give custom names for json key of components
	($type:ty, $($component_name:ident : $component_type:ty),* $(,)?) => {
		impl EntitySerializer for $type {
			type ComponentTuple = ($($component_type),*);
			type RefTuple<'a> = ($(&'a $component_type),*);
			type MutTuple<'a> = ($(&'a mut $component_type),*);
			
			#[allow(non_snake_case)] // we want type name as key, and need temp vars using key here, which is not snake case
			fn serialize (components: bevy::ecs::query::ROQueryItem<'_, '_, Self::RefTuple<'_>>) -> serde_json::Value {
				let ( $($component_name),* ) = components;
				serde_json::json!({
				$(
					stringify!($component_name): $component_name,
				)*
				})
			}
			#[allow(non_snake_case)]
			fn deserialize (
				components: bevy::ecs::query::QueryItem<'_, '_, Self::MutTuple<'_>>,
				json: &serde_json::Value
			) -> Result<(), Error> {
				let ( $(mut $component_name),* ) = components;
				$(
				*$component_name = match serde::Deserialize::deserialize(&json[stringify!($component_name)]) {
					Ok(val) => val,
					Err(e) => return Err(format!("Error deserializing Entity {}: {:?}", stringify!($component_name), e).into()),
				};
				)*
				Ok(())
			}
		}
	};
	// use type as json key of components
	// NOTE: does not support type names like flycam::Flycam, in that case use above pattern
	($type:ty, $($component_type:tt),* $(,)?) => {
		serializer_entity!($type, $($component_type : $component_type),*);
	}
}

macro_rules! world_serializer {
	($type:ty, $($field:ident: $world_serializer_type:ty),* $(,)?) => {
		impl WorldSerializer for $type {
			fn serialize (world: &mut World) -> crate::serialization::JsonResult {
				Ok(serde_json::json!({
				$(
					stringify!($field): <$world_serializer_type as WorldSerializer>::serialize(world)?,
				)*
				}))
			}
			fn deserialize (world: &mut World, json: &serde_json::Value) -> Result<(), Error> {
				$(
				<$world_serializer_type as WorldSerializer>::deserialize(world, &json[stringify!($field)])?;
				)*
				Ok(())
			}
		}
	};
}

pub(crate) use world_serializer;
pub(crate) use serializer_entity;
