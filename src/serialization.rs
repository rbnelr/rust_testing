use bevy::prelude::*;
use bevy::ecs::{*, query::*};
use serde::{Serialize, Deserialize};

pub trait Serializer {
	// Serialize to serde_json value
	fn serialize(&self) -> serde_json::Value;
	// Update existing object from serde_json value
	fn deserialize(&mut self, json: serde_json::Value);
	
	// Create object from defaults and serde_json value
	fn deserialize_new(value: serde_json::Value) -> Self
	where Self: Default + Sized {
		let mut obj = Self::default();
		obj.deserialize(value);
		obj
	}
}

// Rely on serde for serialization of basic types
impl<T> Serializer for T
where T: Serialize + for<'de> Deserialize<'de> {
	fn serialize(&self) -> serde_json::Value {
		serde_json::to_value(self)
			.unwrap() // Not sure how serializing something can fail or how to handle
	}

	fn deserialize(&mut self, json: serde_json::Value) {
		if let Ok(new_val) = serde_json::from_value::<T>(json) {
			*self = new_val;
		}
	}
}

macro_rules! serializer {
	($struct:ty, $($field:ident),*) => {
		impl crate::serialization::Serializer for $struct {
			fn serialize(&self) -> serde_json::Value {
				serde_json::json!({
					$(stringify!($field): crate::serialization::Serializer::serialize(&self.$field)),*
				})
			}
			fn deserialize(&mut self, mut json: serde_json::Value) {
				$(
					// Overwrite field if value exists in json
					if let Some(value) = json.get_mut(stringify!($field)).take() {
						self.$field.deserialize(value.take());
					}
				)*
			}
		}
	};
}

pub trait WorldSerializer {
	// world mutable to allow getting queries (which are cached)
	fn serialize(world: &mut World) -> serde_json::Value;
	fn deserialize(world: &mut World, json: serde_json::Value);
}

macro_rules! serializer_world {
	
	// WorldSerializer with single query
	($type:tt, $($query_type:tt)+) => {
		impl WorldSerializer for $type {
			fn serialize(world: &mut World) -> serde_json::Value {
				crate::serialization::serialize_world!(world, $($query_type)*)
			}
			fn deserialize(world: &mut World, mut json: serde_json::Value) {
				crate::serialization::deserialize_world!(world, json, $($query_type)*);
			}
		}
	};
	
	// WorldSerializer nesting other WorldSerializers into json map
	($type:tt { $($item:ident : $serializer:ty),* $(,)? }) => {
		impl WorldSerializer for $type {
			fn serialize(world: &mut World) -> serde_json::Value {
				serde_json::json!({
				$(
					stringify!($item): <$serializer as WorldSerializer>::serialize(world),
				)*
				})
			}
			fn deserialize(world: &mut World, mut json: serde_json::Value) {
				$(
					if let Some(value) = json.get_mut(stringify!($item)).take() {
						<$serializer as WorldSerializer>::deserialize(world, value.take());
					}
				)*
			}
		}
	};
}

// These match:
// item: (Res<ResourceType>) => serialize single Resource
// item: (Single<Components>) or (Single<Components, Filters>) => serialize single Entity as component_map

// Could implement Query returning multiple entiteies, but then I might need to despawn the previous query and deserialize_new and respawn
// Alternatively we could support some kind of id component to match them, possibly using bevy::ecs::name
// item: (Query<Components>) or (Query<Components, Filters>) => serialize many Entities as List?

// Do ids like this?
// item: EntityLookup(Components, IdComponent) => serialize Query<Components> as map { id: component_map }
macro_rules! serialize_world {
	($world:ident, Res<$type:ty>) => {
		if let Some(resource) = $world.get_resource::<$type>() {
			resource.serialize()
		}
		else {
			// resource not in world, could be a hard error
			// instead insert null
			serde_json::Value::Null
		}
	};
	($world:ident, Single<$D:ty>) => {
		crate::serialization::_serialize_single::<$D, ()>($world)
	};
	($world:ident, Single<$D:ty, $F:ty>) => {
		crate::serialization::_serialize_single::<$D, $F>($world)
	};
}
macro_rules! deserialize_world {
	($world:ident, $value:expr, Res<$type:ty>) => {
		if let Some(mut resource) = $world.get_resource_mut::<$type>() {
			resource.deserialize($value);
		}
		else {
			// resource not in world, don't deserialize
			// could also insert automatically, but not my desired behavoir
		}
	};
	($world:ident, $value:expr, Single<$type:ty>) => {
		crate::serialization::_deserialize_single::<$D, ()>($world, $value)
	};
	($world:ident, $value:expr, Single<$D:ty, $F:ty>) => {
		crate::serialization::_deserialize_single::<$D, $F>($world, $value)
	};
}

pub fn _serialize_single<D, F>(world: &mut World) -> serde_json::Value
//where D: QueryData, F: QueryFilter
where D: Component + Serializer, F: QueryFilter // TODO: support multiple components
{
	let mut query = world.query_filtered::<(&D), F>();
	match query.single(world) {
		Ok(components) => components.serialize(),
		Err(err) => match err {
			QuerySingleError::NoEntities(_) => {
				// entity not in world, could be a hard error
				// instead insert null
				serde_json::Value::Null
			}
			QuerySingleError::MultipleEntities(_) => panic!("Multiple entities found!"), // TODO: return error and stop?
		},
	}
}
pub fn _deserialize_single<D, F>(world: &mut World, json: serde_json::Value)
//where D: QueryData, F: QueryFilter
where D: Component<Mutability = bevy::ecs::component::Mutable> + Serializer, F: QueryFilter // TODO: support multiple components
{
	let mut query = world.query_filtered::<(Mut<D>), F>();
	match query.single_mut(world) {
		Ok(mut components) => components.deserialize(json),
		Err(err) => match err {
			QuerySingleError::NoEntities(_) => {
				// entity not in world, don't deserialize
				warn!("Failed to deserialize single {}", std::any::type_name::<D>())
			}
			QuerySingleError::MultipleEntities(_) => panic!("Multiple entities found!"), // TODO: return error and stop?
		},
	}
}

pub(crate) use serializer;
pub(crate) use serializer_world;
pub(crate) use serialize_world;
pub(crate) use deserialize_world;
