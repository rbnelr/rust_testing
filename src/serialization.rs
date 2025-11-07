use bevy::prelude::*;
use bevy::ecs::{*, query::*};
use serde::{Serialize, Deserialize};
use serde_json::Value as Json;

pub trait Serializer {
	type SerializeFrom<'a> where Self: 'a;
	type DeserializeInto<'a> where Self: 'a;

	// Serialize to serde_json value
	fn serialize(from: Self::SerializeFrom<'_>) -> Json;
	// Update existing object from serde_json value
	fn deserialize(into: Self::DeserializeInto<'_>, json: Json);
	
	// Create object from defaults and serde_json value
	fn deserialize_new(value: Json) -> Self
	where Self: Default + Sized,
	      for<'a> Self::DeserializeInto<'a>: From<&'a mut Self>,
	{
		let mut obj = Self::default();
		Self::deserialize((&mut obj).into(), value);
		obj
	}
}

// Rely on serde for serialization of basic types
// This blanked impl fails with things like  impl Serializer for (Flycam, Transform)
//impl<T> Serializer for T
//where T: Serialize + for<'de> Deserialize<'de> {
//	fn serialize(&self) -> Json {
//		serde_json::to_value(self)
//			.unwrap() // Not sure how serializing something can fail or how to handle
//	}
//
//	fn deserialize(&mut self, json: Json) {
//		if let Ok(new_val) = serde_json::from_value::<T>(json) {
//			*self = new_val;
//		}
//	}
//}
pub trait _SerializerSerde {
	fn serialize(&self) -> Json;
	fn deserialize(&mut self, json: Json);
}
impl<T> _SerializerSerde for T
where T: Serialize + for<'de> Deserialize<'de> {
	fn serialize(&self) -> Json {
		serde_json::to_value(self)
			.unwrap() // Not sure how serializing something can fail or how to handle
	}

	fn deserialize(&mut self, json: Json) {
		if let Json::Null = json {
			// Json value either explicitly null or key missing
			// don't deserialize, this is intentional behavoir
		}
		else {
			match serde_json::from_value::<T>(json) {
				Ok(new_val) => {
					*self = new_val;
				},
				Err(err) => {
					// Json value likely has wrong type, this is important to know
					panic!(err);
				}
			}
		}
	}
}

macro_rules! serializer {
	($struct:tt { $($field:ident),* }) => {
		impl crate::serialization::Serializer for $struct {
			type SerializeFrom<'a> = &'a $struct;
			type DeserializeInto<'a> = &'a mut $struct;
			
			fn serialize(from: Self::SerializeFrom<'_>) -> Json {
				serde_json::json!({
					$(stringify!($field): from.$field.serialize()),*
				})
			}
			fn deserialize(into: Self::DeserializeInto<'_>, mut json: Json) {
				if let Json::Null = json { return; }
				$(
					// Will deserialize only if field key is found, arrays or missing json container will silently not skip
					into.$field.deserialize(json[stringify!($field)].take());
				)*
			}
		}
	};
}

pub trait WorldSerializer {
	// world mutable to allow getting queries (which are cached)
	fn serialize(world: &mut World) -> Json;
	fn deserialize(world: &mut World, json: Json);
}

// These match:
// item: (Res<ResourceType>) => serialize single Resource
// item: (Single<Components>) or (Single<Components, Filters>) => serialize single Entity as component_map

// Could implement Query returning multiple entiteies, but then I might need to despawn the previous query and deserialize_new and respawn
// Alternatively we could support some kind of id component to match them, possibly using bevy::ecs::name
// item: (Query<Components>) or (Query<Components, Filters>) => serialize many Entities as List?

// Do ids like this?
// item: EntityLookup(Components, IdComponent) => serialize Query<Components> as map { id: component_map }
macro_rules! serializer_world {
	
	// WorldSerializer with single query
	($type:tt, $($query:tt)+) => {
		impl WorldSerializer for $type {
			fn serialize(world: &mut World) -> Json {
				crate::serialization::serialize_world!(world, $($query)*)
			}
			fn deserialize(world: &mut World, mut json: Json) {
				crate::serialization::deserialize_world!(world, json, $($query)*);
			}
		}
	};
	
	// WorldSerializer nesting other WorldSerializers into json map
	($type:tt { $($item:ident : $serializer:ty),* $(,)? }) => {
		impl WorldSerializer for $type {
			fn serialize(world: &mut World) -> Json {
				serde_json::json!({
				$(
					stringify!($item): <$serializer as WorldSerializer>::serialize(world),
				)*
				})
			}
			fn deserialize(world: &mut World, mut json: Json) {
				$(
					<$serializer as WorldSerializer>::deserialize(world, json[stringify!($item)].take());
				)*
			}
		}
	};
}

macro_rules! serialize_world {
	($world:ident, Res<$type:ty>) => {
		if let Some(resource) = $world.get_resource::<$type>() {
			<$type as Serializer>::serialize(resource)
		}
		else {
			// resource not in world, could be a hard error
			// instead insert null
			Json::Null
		}
	};
	//($world:ident, Single<$type:tt>) => {
	//	serialize_world($world, Single<$type, ()>)
	//};
	($world:ident, Single<($($component:tt),*), $filter:tt>) => {{
		let mut query = $world.query_filtered::<($(& $component),*), $filter>();
		match query.single($world) {
			Ok(components) => {
				<($($component),*) as Serializer>::serialize(&components)
			},
			Err(err) => match err {
				bevy::ecs::query::QuerySingleError::NoEntities(_) => {
					// entity not in world, could be a hard error
					// instead insert null
					Json::Null
				}
				bevy::ecs::query::QuerySingleError::MultipleEntities(_) => panic!("Multiple entities found!"), // TODO: return error and stop?
			},
		}}
	};
}
macro_rules! deserialize_world {
	($world:ident, $value:expr, Res<$type:ty>) => {
		if let Some(mut resource) = $world.get_resource_mut::<$type>() {
			<$type as Serializer>::deserialize(resource.into_inner(), $value);
		}
		else {
			// resource not in world, don't deserialize
			// could also insert automatically, but not my desired behavoir
		}
	};
	//($world:ident, $value:expr, Single<$type:tt>) => {
	//	//deserialize_world($world, $value, Single<$type, ()>)
	//};
	($world:ident, $value:expr, Single<$type:tt, $filter:tt>) => {
		//crate::serialization::_deserialize_single::<$type, $filter>($world, $value)
	};
}

pub(crate) use serializer;
pub(crate) use serializer_world;
pub(crate) use serialize_world;
pub(crate) use deserialize_world;
