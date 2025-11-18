#![feature(prelude_import)]
#![allow(unused)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod util {
    pub(crate) use _ident_count;
}
mod phases {
    use bevy::prelude::*;
    use bevy::ecs::schedule::ScheduleConfigs;
    use crate::phases;
    pub enum Phase {
        Start,
        Gameplay,
        CameraUpdate,
    }
    const _: () = {
        extern crate alloc;
        impl ::bevy::ecs::schedule::SystemSet for Phase
        where
            Self: 'static + Send + Sync + Clone + Eq + ::core::fmt::Debug
                + ::core::hash::Hash,
        {
            fn dyn_clone(
                &self,
            ) -> alloc::boxed::Box<dyn ::bevy::ecs::schedule::SystemSet> {
                alloc::boxed::Box::new(::core::clone::Clone::clone(self))
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Phase {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Phase::Start => "Start",
                    Phase::Gameplay => "Gameplay",
                    Phase::CameraUpdate => "CameraUpdate",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Phase {
        #[inline]
        fn clone(&self) -> Phase {
            match self {
                Phase::Start => Phase::Start,
                Phase::Gameplay => Phase::Gameplay,
                Phase::CameraUpdate => Phase::CameraUpdate,
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Phase {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Phase {
        #[inline]
        fn eq(&self, other: &Phase) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Phase {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Phase {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    pub fn update_schedule_configs(app: &mut App) {
        app.configure_sets(
            Update,
            (
                Phase::Start.before(Phase::Gameplay).before(Phase::CameraUpdate),
                Phase::CameraUpdate.after(Phase::Gameplay),
            ),
        );
    }
}
mod serialization {
    use bevy::prelude::*;
    use serde::*;
    pub trait EntitySerializer
    where
        Self: Sized,
    {
        type SerComponentTuple<'a> where Self: 'a;
        type DeComponentTuple<'a> where Self: 'a;
        fn serialize<'a, S>(
            components: Self::SerComponentTuple<'a>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer;
        fn deserialize<'a, 'de, D>(
            components: Self::DeComponentTuple<'a>,
            deserializer: D,
        ) -> Result<(), D::Error>
        where
            D: serde::Deserializer<'de>;
    }
    pub(crate) use world_serializer;
    pub(crate) use world_serializer_entity;
}
mod settings_file {
    use bevy::prelude::*;
    use serde::*;
    use crate::flycam;
    use crate::debug_camera;
    use crate::serialization::*;
    const SETTINGS_FILE: &'static str = "settings.json";
    #[reflect(Resource)]
    pub struct RenderSettings {
        pub backends: String,
        pub disable_validation_in_debug: bool,
    }
    impl ::bevy::ecs::resource::Resource for RenderSettings
    where
        Self: Send + Sync + 'static,
    {}
    const _: () = {
        impl ::bevy::reflect::GetTypeRegistration for RenderSettings {
            fn get_type_registration() -> ::bevy::reflect::TypeRegistration {
                let mut registration = ::bevy::reflect::TypeRegistration::of::<Self>();
                registration
                    .insert::<
                        ::bevy::reflect::ReflectFromPtr,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ::bevy::reflect::ReflectFromReflect,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ReflectResource,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
            }
            #[inline(never)]
            fn register_type_dependencies(registry: &mut ::bevy::reflect::TypeRegistry) {
                <String as ::bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                    registry,
                );
                <bool as ::bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                    registry,
                );
            }
        }
        impl ::bevy::reflect::Typed for RenderSettings {
            #[inline]
            fn type_info() -> &'static ::bevy::reflect::TypeInfo {
                static CELL: ::bevy::reflect::utility::NonGenericTypeInfoCell = ::bevy::reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(|| {
                    ::bevy::reflect::TypeInfo::Struct(
                        ::bevy::reflect::StructInfo::new::<
                            Self,
                        >(
                            &[
                                ::bevy::reflect::NamedField::new::<String>("backends"),
                                ::bevy::reflect::NamedField::new::<
                                    bool,
                                >("disable_validation_in_debug"),
                            ],
                        ),
                    )
                })
            }
        }
        impl ::bevy::reflect::TypePath for RenderSettings {
            fn type_path() -> &'static str {
                "rust_bevy_test::settings_file::RenderSettings"
            }
            fn short_type_path() -> &'static str {
                "RenderSettings"
            }
            fn type_ident() -> Option<&'static str> {
                ::core::option::Option::Some("RenderSettings")
            }
            fn crate_name() -> Option<&'static str> {
                ::core::option::Option::Some(
                    "rust_bevy_test::settings_file".split(':').next().unwrap(),
                )
            }
            fn module_path() -> Option<&'static str> {
                ::core::option::Option::Some("rust_bevy_test::settings_file")
            }
        }
        impl ::bevy::reflect::Reflect for RenderSettings {
            #[inline]
            fn into_any(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::core::any::Any,
            > {
                self
            }
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }
            #[inline]
            fn into_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::bevy::reflect::Reflect,
            > {
                self
            }
            #[inline]
            fn as_reflect(&self) -> &dyn ::bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn ::bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn set(
                &mut self,
                value: ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
            ) -> ::core::result::Result<
                (),
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
            > {
                *self = <dyn ::bevy::reflect::Reflect>::take(value)?;
                ::core::result::Result::Ok(())
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    ::bevy::reflect::__macro_exports::auto_register::AutomaticReflectRegistrations(
                        <RenderSettings as ::bevy::reflect::__macro_exports::auto_register::RegisterForReflection>::__register,
                    )
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = ".CRT$XCU"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
        impl ::bevy::reflect::Struct for RenderSettings {
            fn field(
                &self,
                name: &str,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::PartialReflect> {
                match name {
                    "backends" => ::core::option::Option::Some(&self.backends),
                    "disable_validation_in_debug" => {
                        ::core::option::Option::Some(&self.disable_validation_in_debug)
                    }
                    _ => ::core::option::Option::None,
                }
            }
            fn field_mut(
                &mut self,
                name: &str,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::PartialReflect> {
                match name {
                    "backends" => ::core::option::Option::Some(&mut self.backends),
                    "disable_validation_in_debug" => {
                        ::core::option::Option::Some(
                            &mut self.disable_validation_in_debug,
                        )
                    }
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at(
                &self,
                index: usize,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::PartialReflect> {
                match index {
                    0usize => ::core::option::Option::Some(&self.backends),
                    1usize => {
                        ::core::option::Option::Some(&self.disable_validation_in_debug)
                    }
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at_mut(
                &mut self,
                index: usize,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::PartialReflect> {
                match index {
                    0usize => ::core::option::Option::Some(&mut self.backends),
                    1usize => {
                        ::core::option::Option::Some(
                            &mut self.disable_validation_in_debug,
                        )
                    }
                    _ => ::core::option::Option::None,
                }
            }
            fn name_at(&self, index: usize) -> ::core::option::Option<&str> {
                match index {
                    0usize => ::core::option::Option::Some("backends"),
                    1usize => ::core::option::Option::Some("disable_validation_in_debug"),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_len(&self) -> usize {
                2usize
            }
            fn iter_fields(&self) -> ::bevy::reflect::FieldIter {
                ::bevy::reflect::FieldIter::new(self)
            }
            fn to_dynamic_struct(&self) -> ::bevy::reflect::DynamicStruct {
                let mut dynamic: ::bevy::reflect::DynamicStruct = ::core::default::Default::default();
                dynamic
                    .set_represented_type(
                        ::bevy::reflect::PartialReflect::get_represented_type_info(self),
                    );
                dynamic
                    .insert_boxed(
                        "backends",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.backends),
                    );
                dynamic
                    .insert_boxed(
                        "disable_validation_in_debug",
                        ::bevy::reflect::PartialReflect::to_dynamic(
                            &self.disable_validation_in_debug,
                        ),
                    );
                dynamic
            }
        }
        impl ::bevy::reflect::PartialReflect for RenderSettings {
            #[inline]
            fn get_represented_type_info(
                &self,
            ) -> ::core::option::Option<&'static ::bevy::reflect::TypeInfo> {
                ::core::option::Option::Some(
                    <Self as ::bevy::reflect::Typed>::type_info(),
                )
            }
            #[inline]
            fn try_apply(
                &mut self,
                value: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::result::Result<(), ::bevy::reflect::ApplyError> {
                if let ::bevy::reflect::ReflectRef::Struct(struct_value) = ::bevy::reflect::PartialReflect::reflect_ref(
                    value,
                ) {
                    for (i, value) in ::core::iter::Iterator::enumerate(
                        ::bevy::reflect::Struct::iter_fields(struct_value),
                    ) {
                        let name = ::bevy::reflect::Struct::name_at(struct_value, i)
                            .unwrap();
                        if let ::core::option::Option::Some(v) = ::bevy::reflect::Struct::field_mut(
                            self,
                            name,
                        ) {
                            ::bevy::reflect::PartialReflect::try_apply(v, value)?;
                        }
                    }
                } else {
                    return ::core::result::Result::Err(::bevy::reflect::ApplyError::MismatchedKinds {
                        from_kind: ::bevy::reflect::PartialReflect::reflect_kind(value),
                        to_kind: ::bevy::reflect::ReflectKind::Struct,
                    });
                }
                ::core::result::Result::Ok(())
            }
            #[inline]
            fn reflect_kind(&self) -> ::bevy::reflect::ReflectKind {
                ::bevy::reflect::ReflectKind::Struct
            }
            #[inline]
            fn reflect_ref(&self) -> ::bevy::reflect::ReflectRef {
                ::bevy::reflect::ReflectRef::Struct(self)
            }
            #[inline]
            fn reflect_mut(&mut self) -> ::bevy::reflect::ReflectMut {
                ::bevy::reflect::ReflectMut::Struct(self)
            }
            #[inline]
            fn reflect_owned(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::ReflectOwned {
                ::bevy::reflect::ReflectOwned::Struct(self)
            }
            #[inline]
            fn try_into_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::core::result::Result<
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::PartialReflect,
                >,
            > {
                ::core::result::Result::Ok(self)
            }
            #[inline]
            fn try_as_reflect(
                &self,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn try_as_reflect_mut(
                &mut self,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn into_partial_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::bevy::reflect::PartialReflect,
            > {
                self
            }
            #[inline]
            fn as_partial_reflect(&self) -> &dyn ::bevy::reflect::PartialReflect {
                self
            }
            #[inline]
            fn as_partial_reflect_mut(
                &mut self,
            ) -> &mut dyn ::bevy::reflect::PartialReflect {
                self
            }
            fn reflect_partial_eq(
                &self,
                value: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<bool> {
                (::bevy::reflect::struct_partial_eq)(self, value)
            }
            #[inline]
            #[allow(
                unreachable_code,
                reason = "Ignored fields without a `clone` attribute will early-return with an error"
            )]
            fn reflect_clone(
                &self,
            ) -> ::core::result::Result<
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
                ::bevy::reflect::ReflectCloneError,
            > {
                ::core::result::Result::Ok(
                    ::bevy::reflect::__macro_exports::alloc_utils::Box::new(Self {
                        backends: <String as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.backends,
                        )?,
                        disable_validation_in_debug: <bool as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.disable_validation_in_debug,
                        )?,
                    }),
                )
            }
        }
        impl ::bevy::reflect::FromReflect for RenderSettings {
            fn from_reflect(
                reflect: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<Self> {
                if let ::bevy::reflect::ReflectRef::Struct(__ref_struct) = ::bevy::reflect::PartialReflect::reflect_ref(
                    reflect,
                ) {
                    let __this = Self {
                        backends: <String as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "backends")?,
                        )?,
                        disable_validation_in_debug: <bool as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(
                                __ref_struct,
                                "disable_validation_in_debug",
                            )?,
                        )?,
                    };
                    ::core::option::Option::Some(__this)
                } else {
                    ::core::option::Option::None
                }
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for RenderSettings {
        #[inline]
        fn clone(&self) -> RenderSettings {
            RenderSettings {
                backends: ::core::clone::Clone::clone(&self.backends),
                disable_validation_in_debug: ::core::clone::Clone::clone(
                    &self.disable_validation_in_debug,
                ),
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for RenderSettings {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "RenderSettings",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "backends",
                    &self.backends,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "disable_validation_in_debug",
                    &self.disable_validation_in_debug,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RenderSettings {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "backends" => _serde::__private228::Ok(__Field::__field0),
                            "disable_validation_in_debug" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"backends" => _serde::__private228::Ok(__Field::__field0),
                            b"disable_validation_in_debug" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<RenderSettings>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RenderSettings;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct RenderSettings",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct RenderSettings with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct RenderSettings with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(RenderSettings {
                            backends: __field0,
                            disable_validation_in_debug: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<bool> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "backends",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "disable_validation_in_debug",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("backends")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field(
                                    "disable_validation_in_debug",
                                )?
                            }
                        };
                        _serde::__private228::Ok(RenderSettings {
                            backends: __field0,
                            disable_validation_in_debug: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "backends",
                    "disable_validation_in_debug",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "RenderSettings",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<RenderSettings>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for RenderSettings {
        fn default() -> Self {
            Self {
                backends: "vk, dx12, gl".into(),
                disable_validation_in_debug: true,
            }
        }
    }
    use flycam::*;
    impl EntitySerializer for FlycamBundle {
        type SerComponentTuple<'a> = (&'a Transform, &'a flycam::Flycam) where Self: 'a;
        type DeComponentTuple<'a> = (Mut<'a, Transform>, Mut<'a, flycam::Flycam>)
        where
            Self: 'a;
        fn serialize<'a, S>(
            components: Self::SerComponentTuple<'a>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            use serde::ser::SerializeStruct;
            #[allow(non_snake_case)]
            let (transform, flycam) = components;
            let mut _struct = serializer.serialize_struct("FlycamBundle", 2)?;
            _struct.serialize_field("transform", transform)?;
            _struct.serialize_field("flycam", flycam)?;
            _struct.end()
        }
        fn deserialize<'a, 'de, D>(
            components: Self::DeComponentTuple<'a>,
            deserializer: D,
        ) -> Result<(), D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            use std::fmt;
            struct ComponentStructVisitor;
            impl<'de> serde::de::Visitor<'de> for ComponentStructVisitor {
                type Value = ();
                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("a map of components")
                }
                fn visit_str<E>(self, value: &str) -> Result<(), E>
                where
                    E: de::Error,
                {
                    match value {
                        "transform" => Ok(()),
                        "flycam" => Ok(()),
                        _ => {}
                    }
                    Ok(())
                }
            }
            Ok(())
        }
    }
    struct Nested;
    struct SettingsFile;
    fn serialize_to_json_pretty_tabs(json: &serde_json::Value) -> Result<String> {
        let mut vec = Vec::with_capacity(1024);
        let mut ser = serde_json::Serializer::with_formatter(
            &mut vec,
            serde_json::ser::PrettyFormatter::with_indent(b"\t"),
        );
        if let Err(e) = serde::Serialize::serialize(&json, &mut ser) {
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event src\\settings_file.rs:66",
                            "rust_bevy_test::settings_file",
                            ::tracing::Level::WARN,
                            ::tracing_core::__macro_support::Option::Some(
                                "src\\settings_file.rs",
                            ),
                            ::tracing_core::__macro_support::Option::Some(66u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "rust_bevy_test::settings_file",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::WARN
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::WARN
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!("{0:?}", e) as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            };
            return Err(e.into());
        }
        unsafe { Ok(String::from_utf8_unchecked(vec)) }
    }
    pub fn save(world: &mut World) {
        let mut query = world
            .query_filtered::<
                (&Transform, &flycam::Flycam),
                With<crate::debug_camera::MainCamera>,
            >();
        let cam = query.single(world).unwrap();
        let mut json = <crate::flycam::FlycamBundle as EntitySerializer>::serialize(
                cam,
                serde_json::value::Serializer,
            )
            .unwrap();
        json.as_object_mut().unwrap().shift_insert(0, "version".into(), "0.1".into());
        if let Ok(json_str) = serialize_to_json_pretty_tabs(&json) {
            if std::fs::write(SETTINGS_FILE, json_str).is_ok() {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src\\settings_file.rs:87",
                                "rust_bevy_test::settings_file",
                                ::tracing::Level::INFO,
                                ::tracing_core::__macro_support::Option::Some(
                                    "src\\settings_file.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(87u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "rust_bevy_test::settings_file",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::INFO
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &format_args!("Saved!") as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return;
            }
        }
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src\\settings_file.rs:92",
                        "rust_bevy_test::settings_file",
                        ::tracing::Level::WARN,
                        ::tracing_core::__macro_support::Option::Some(
                            "src\\settings_file.rs",
                        ),
                        ::tracing_core::__macro_support::Option::Some(92u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "rust_bevy_test::settings_file",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::WARN
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::WARN
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Failed to save {0}!", SETTINGS_FILE)
                                            as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
    }
    pub struct LoadResult {
        loaded_json: serde_json::Value,
        pub render: RenderSettings,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for LoadResult {
        #[inline]
        fn clone(&self) -> LoadResult {
            LoadResult {
                loaded_json: ::core::clone::Clone::clone(&self.loaded_json),
                render: ::core::clone::Clone::clone(&self.render),
            }
        }
    }
    pub fn early_load_settings() -> Option<LoadResult> {
        if let Ok(json_str) = std::fs::read_to_string(SETTINGS_FILE) {
            if let Ok(mut loaded_json) = serde_json::from_str::<
                serde_json::Value,
            >(&json_str) {
                let render = serde_json::from_value::<
                    RenderSettings,
                >(loaded_json["render"].clone());
                let render = render.unwrap_or(RenderSettings::default());
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src\\settings_file.rs:109",
                                "rust_bevy_test::settings_file",
                                ::tracing::Level::INFO,
                                ::tracing_core::__macro_support::Option::Some(
                                    "src\\settings_file.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(109u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "rust_bevy_test::settings_file",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::INFO
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::INFO
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &format_args!("Early loaded {0}!", SETTINGS_FILE)
                                                    as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                return LoadResult { loaded_json, render }.into();
            }
        }
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src\\settings_file.rs:116",
                        "rust_bevy_test::settings_file",
                        ::tracing::Level::WARN,
                        ::tracing_core::__macro_support::Option::Some(
                            "src\\settings_file.rs",
                        ),
                        ::tracing_core::__macro_support::Option::Some(116u32),
                        ::tracing_core::__macro_support::Option::Some(
                            "rust_bevy_test::settings_file",
                        ),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::WARN
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::WARN
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    let mut iter = __CALLSITE.metadata().fields().iter();
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set(
                            &[
                                (
                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                        .expect("FieldSet corrupted (this is a bug)"),
                                    ::tracing::__macro_support::Option::Some(
                                        &format_args!("Failed to load {0}!", SETTINGS_FILE)
                                            as &dyn Value,
                                    ),
                                ),
                            ],
                        )
                });
            } else {
            }
        };
        None
    }
    use crate::util::*;
    pub fn load_settings(world: &mut World, res: Option<LoadResult>) {
        world.insert_resource(RenderSettings::default());
        if let Some(res) = res {
            let mut query = world
                .query_filtered::<
                    (&mut Transform, &mut flycam::Flycam),
                    With<crate::debug_camera::MainCamera>,
                >();
            let cam = query.single_mut(world).unwrap();
            <crate::flycam::FlycamBundle as EntitySerializer>::deserialize(
                    cam,
                    res.loaded_json,
                )
                .unwrap();
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event src\\settings_file.rs:136",
                            "rust_bevy_test::settings_file",
                            ::tracing::Level::INFO,
                            ::tracing_core::__macro_support::Option::Some(
                                "src\\settings_file.rs",
                            ),
                            ::tracing_core::__macro_support::Option::Some(136u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "rust_bevy_test::settings_file",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::INFO
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::INFO
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        let mut iter = __CALLSITE.metadata().fields().iter();
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set(
                                &[
                                    (
                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                            .expect("FieldSet corrupted (this is a bug)"),
                                        ::tracing::__macro_support::Option::Some(
                                            &format_args!("Fully Loaded {0}!", SETTINGS_FILE)
                                                as &dyn Value,
                                        ),
                                    ),
                                ],
                            )
                    });
                } else {
                }
            };
        }
    }
    pub fn load(world: &mut World) {
        let res = early_load_settings();
        load_settings(world, res);
    }
}
mod egui_histogram {
    use bevy::prelude::*;
    use bevy_egui::egui::{
        Color32, Context, Pos2, Rect, Ui, Rangef, Stroke, RichText, Label,
        containers::{Frame, Window},
        emath, epaint, lerp, pos2, remap, vec2,
    };
    use std::collections::VecDeque;
    #[reflect(Resource)]
    pub struct Frametimes {
        frametimes: VecDeque<f32>,
        frametimes_avg_rate: Timer,
        frametime_avg: FrametimeAvg,
    }
    impl ::bevy::ecs::resource::Resource for Frametimes
    where
        Self: Send + Sync + 'static,
    {}
    const _: () = {
        impl ::bevy::reflect::GetTypeRegistration for Frametimes {
            fn get_type_registration() -> ::bevy::reflect::TypeRegistration {
                let mut registration = ::bevy::reflect::TypeRegistration::of::<Self>();
                registration
                    .insert::<
                        ::bevy::reflect::ReflectFromPtr,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ::bevy::reflect::ReflectFromReflect,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ReflectResource,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
            }
            #[inline(never)]
            fn register_type_dependencies(registry: &mut ::bevy::reflect::TypeRegistry) {
                <VecDeque<
                    f32,
                > as ::bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                    registry,
                );
                <Timer as ::bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                    registry,
                );
                <FrametimeAvg as ::bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                    registry,
                );
            }
        }
        impl ::bevy::reflect::Typed for Frametimes {
            #[inline]
            fn type_info() -> &'static ::bevy::reflect::TypeInfo {
                static CELL: ::bevy::reflect::utility::NonGenericTypeInfoCell = ::bevy::reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(|| {
                    ::bevy::reflect::TypeInfo::Struct(
                        ::bevy::reflect::StructInfo::new::<
                            Self,
                        >(
                            &[
                                ::bevy::reflect::NamedField::new::<
                                    VecDeque<f32>,
                                >("frametimes"),
                                ::bevy::reflect::NamedField::new::<
                                    Timer,
                                >("frametimes_avg_rate"),
                                ::bevy::reflect::NamedField::new::<
                                    FrametimeAvg,
                                >("frametime_avg"),
                            ],
                        ),
                    )
                })
            }
        }
        impl ::bevy::reflect::TypePath for Frametimes {
            fn type_path() -> &'static str {
                "rust_bevy_test::egui_histogram::Frametimes"
            }
            fn short_type_path() -> &'static str {
                "Frametimes"
            }
            fn type_ident() -> Option<&'static str> {
                ::core::option::Option::Some("Frametimes")
            }
            fn crate_name() -> Option<&'static str> {
                ::core::option::Option::Some(
                    "rust_bevy_test::egui_histogram".split(':').next().unwrap(),
                )
            }
            fn module_path() -> Option<&'static str> {
                ::core::option::Option::Some("rust_bevy_test::egui_histogram")
            }
        }
        impl ::bevy::reflect::Reflect for Frametimes {
            #[inline]
            fn into_any(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::core::any::Any,
            > {
                self
            }
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }
            #[inline]
            fn into_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::bevy::reflect::Reflect,
            > {
                self
            }
            #[inline]
            fn as_reflect(&self) -> &dyn ::bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn ::bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn set(
                &mut self,
                value: ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
            ) -> ::core::result::Result<
                (),
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
            > {
                *self = <dyn ::bevy::reflect::Reflect>::take(value)?;
                ::core::result::Result::Ok(())
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    ::bevy::reflect::__macro_exports::auto_register::AutomaticReflectRegistrations(
                        <Frametimes as ::bevy::reflect::__macro_exports::auto_register::RegisterForReflection>::__register,
                    )
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = ".CRT$XCU"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
        impl ::bevy::reflect::Struct for Frametimes {
            fn field(
                &self,
                name: &str,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::PartialReflect> {
                match name {
                    "frametimes" => ::core::option::Option::Some(&self.frametimes),
                    "frametimes_avg_rate" => {
                        ::core::option::Option::Some(&self.frametimes_avg_rate)
                    }
                    "frametime_avg" => ::core::option::Option::Some(&self.frametime_avg),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_mut(
                &mut self,
                name: &str,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::PartialReflect> {
                match name {
                    "frametimes" => ::core::option::Option::Some(&mut self.frametimes),
                    "frametimes_avg_rate" => {
                        ::core::option::Option::Some(&mut self.frametimes_avg_rate)
                    }
                    "frametime_avg" => {
                        ::core::option::Option::Some(&mut self.frametime_avg)
                    }
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at(
                &self,
                index: usize,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::PartialReflect> {
                match index {
                    0usize => ::core::option::Option::Some(&self.frametimes),
                    1usize => ::core::option::Option::Some(&self.frametimes_avg_rate),
                    2usize => ::core::option::Option::Some(&self.frametime_avg),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at_mut(
                &mut self,
                index: usize,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::PartialReflect> {
                match index {
                    0usize => ::core::option::Option::Some(&mut self.frametimes),
                    1usize => ::core::option::Option::Some(&mut self.frametimes_avg_rate),
                    2usize => ::core::option::Option::Some(&mut self.frametime_avg),
                    _ => ::core::option::Option::None,
                }
            }
            fn name_at(&self, index: usize) -> ::core::option::Option<&str> {
                match index {
                    0usize => ::core::option::Option::Some("frametimes"),
                    1usize => ::core::option::Option::Some("frametimes_avg_rate"),
                    2usize => ::core::option::Option::Some("frametime_avg"),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_len(&self) -> usize {
                3usize
            }
            fn iter_fields(&self) -> ::bevy::reflect::FieldIter {
                ::bevy::reflect::FieldIter::new(self)
            }
            fn to_dynamic_struct(&self) -> ::bevy::reflect::DynamicStruct {
                let mut dynamic: ::bevy::reflect::DynamicStruct = ::core::default::Default::default();
                dynamic
                    .set_represented_type(
                        ::bevy::reflect::PartialReflect::get_represented_type_info(self),
                    );
                dynamic
                    .insert_boxed(
                        "frametimes",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.frametimes),
                    );
                dynamic
                    .insert_boxed(
                        "frametimes_avg_rate",
                        ::bevy::reflect::PartialReflect::to_dynamic(
                            &self.frametimes_avg_rate,
                        ),
                    );
                dynamic
                    .insert_boxed(
                        "frametime_avg",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.frametime_avg),
                    );
                dynamic
            }
        }
        impl ::bevy::reflect::PartialReflect for Frametimes {
            #[inline]
            fn get_represented_type_info(
                &self,
            ) -> ::core::option::Option<&'static ::bevy::reflect::TypeInfo> {
                ::core::option::Option::Some(
                    <Self as ::bevy::reflect::Typed>::type_info(),
                )
            }
            #[inline]
            fn try_apply(
                &mut self,
                value: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::result::Result<(), ::bevy::reflect::ApplyError> {
                if let ::bevy::reflect::ReflectRef::Struct(struct_value) = ::bevy::reflect::PartialReflect::reflect_ref(
                    value,
                ) {
                    for (i, value) in ::core::iter::Iterator::enumerate(
                        ::bevy::reflect::Struct::iter_fields(struct_value),
                    ) {
                        let name = ::bevy::reflect::Struct::name_at(struct_value, i)
                            .unwrap();
                        if let ::core::option::Option::Some(v) = ::bevy::reflect::Struct::field_mut(
                            self,
                            name,
                        ) {
                            ::bevy::reflect::PartialReflect::try_apply(v, value)?;
                        }
                    }
                } else {
                    return ::core::result::Result::Err(::bevy::reflect::ApplyError::MismatchedKinds {
                        from_kind: ::bevy::reflect::PartialReflect::reflect_kind(value),
                        to_kind: ::bevy::reflect::ReflectKind::Struct,
                    });
                }
                ::core::result::Result::Ok(())
            }
            #[inline]
            fn reflect_kind(&self) -> ::bevy::reflect::ReflectKind {
                ::bevy::reflect::ReflectKind::Struct
            }
            #[inline]
            fn reflect_ref(&self) -> ::bevy::reflect::ReflectRef {
                ::bevy::reflect::ReflectRef::Struct(self)
            }
            #[inline]
            fn reflect_mut(&mut self) -> ::bevy::reflect::ReflectMut {
                ::bevy::reflect::ReflectMut::Struct(self)
            }
            #[inline]
            fn reflect_owned(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::ReflectOwned {
                ::bevy::reflect::ReflectOwned::Struct(self)
            }
            #[inline]
            fn try_into_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::core::result::Result<
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::PartialReflect,
                >,
            > {
                ::core::result::Result::Ok(self)
            }
            #[inline]
            fn try_as_reflect(
                &self,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn try_as_reflect_mut(
                &mut self,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn into_partial_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::bevy::reflect::PartialReflect,
            > {
                self
            }
            #[inline]
            fn as_partial_reflect(&self) -> &dyn ::bevy::reflect::PartialReflect {
                self
            }
            #[inline]
            fn as_partial_reflect_mut(
                &mut self,
            ) -> &mut dyn ::bevy::reflect::PartialReflect {
                self
            }
            fn reflect_partial_eq(
                &self,
                value: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<bool> {
                (::bevy::reflect::struct_partial_eq)(self, value)
            }
            #[inline]
            #[allow(
                unreachable_code,
                reason = "Ignored fields without a `clone` attribute will early-return with an error"
            )]
            fn reflect_clone(
                &self,
            ) -> ::core::result::Result<
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
                ::bevy::reflect::ReflectCloneError,
            > {
                ::core::result::Result::Ok(
                    ::bevy::reflect::__macro_exports::alloc_utils::Box::new(Self {
                        frametimes: <VecDeque<
                            f32,
                        > as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.frametimes,
                        )?,
                        frametimes_avg_rate: <Timer as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.frametimes_avg_rate,
                        )?,
                        frametime_avg: <FrametimeAvg as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.frametime_avg,
                        )?,
                    }),
                )
            }
        }
        impl ::bevy::reflect::FromReflect for Frametimes {
            fn from_reflect(
                reflect: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<Self> {
                if let ::bevy::reflect::ReflectRef::Struct(__ref_struct) = ::bevy::reflect::PartialReflect::reflect_ref(
                    reflect,
                ) {
                    let __this = Self {
                        frametimes: <VecDeque<
                            f32,
                        > as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "frametimes")?,
                        )?,
                        frametimes_avg_rate: <Timer as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(
                                __ref_struct,
                                "frametimes_avg_rate",
                            )?,
                        )?,
                        frametime_avg: <FrametimeAvg as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(
                                __ref_struct,
                                "frametime_avg",
                            )?,
                        )?,
                    };
                    ::core::option::Option::Some(__this)
                } else {
                    ::core::option::Option::None
                }
            }
        }
    };
    impl Default for Frametimes {
        fn default() -> Self {
            Self {
                frametimes: VecDeque::with_capacity(64),
                frametimes_avg_rate: {
                    let freq = std::time::Duration::from_secs_f32(0.5);
                    Timer::new(freq, TimerMode::Repeating)
                },
                frametime_avg: FrametimeAvg {
                    mean: 0.0,
                    min: 0.0,
                    max: 0.0,
                    std_dev: 0.0,
                },
            }
        }
    }
    impl Frametimes {
        pub fn gui(&mut self, ui: &mut Ui, time: Res<Time>) {
            let height: f32 = 60.0;
            let bar_width: f32 = 1.0;
            let max_y: f32 = 20.0;
            Frame::canvas(ui.style())
                .show(
                    ui,
                    |ui| {
                        ui.ctx().request_repaint();
                        let desired_size = vec2(ui.available_width(), height);
                        let (_id, rect) = ui.allocate_space(desired_size);
                        let count = (rect.width() / bar_width).floor() as usize;
                        let draw_width = count as f32 * bar_width;
                        while self.frametimes.len() >= count {
                            self.frametimes.pop_front();
                        }
                        self.frametimes.push_back(time.delta_secs());
                        let to_screen = emath::RectTransform::from_to(
                            Rect::from_x_y_ranges(0.0..=draw_width, max_y..=0.0),
                            rect,
                        );
                        let thres: [f32; 2] = [1000.0 / 120.0, 1000.0 / 60.0];
                        let cols = [Color32::GREEN, Color32::YELLOW, Color32::RED];
                        let mut shapes = ::alloc::vec::Vec::new();
                        let mut x = 0;
                        for val in &self.frametimes {
                            shapes
                                .push(
                                    epaint::Shape::line_segment(
                                        [
                                            to_screen * Pos2::new((x as f32 + 0.5) * bar_width, 0.0),
                                            to_screen
                                                * Pos2::new((x as f32 + 0.5) * bar_width, val * 1000.0),
                                        ],
                                        Stroke {
                                            width: bar_width,
                                            color: {
                                                let mut col = cols[2];
                                                for i in 0..2 {
                                                    if val * 1000.0 <= thres[i] {
                                                        col = cols[i];
                                                        break;
                                                    }
                                                }
                                                Color32::from_rgba_unmultiplied(
                                                    col.r(),
                                                    col.g(),
                                                    col.b(),
                                                    200,
                                                )
                                            },
                                        },
                                    ),
                                );
                            x += 1;
                        }
                        for i in 0..2 {
                            shapes
                                .push(
                                    epaint::Shape::line_segment(
                                        [
                                            to_screen * Pos2::new(0.0, thres[i]),
                                            to_screen * Pos2::new(draw_width, thres[i]),
                                        ],
                                        Stroke {
                                            width: 1.0,
                                            color: Color32::from_rgba_unmultiplied(
                                                cols[i].r(),
                                                cols[i].g(),
                                                cols[i].b(),
                                                150,
                                            ),
                                        },
                                    ),
                                );
                        }
                        ui.painter().with_clip_rect(rect).extend(shapes);
                        {
                            if self
                                .frametimes_avg_rate
                                .tick(time.delta())
                                .just_finished()
                            {
                                self.frametime_avg = calc_frametime_avg(&self.frametimes);
                            }
                            let avg = &self.frametime_avg;
                            let avg_hz = 1.0 / avg.mean;
                            let txt = ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "avg: {0:5.1} hz ({1:6.3} ms  min: {2:6.3}  max: {3:6.3}  stddev: {4:6.3})",
                                        avg_hz,
                                        avg.mean * 1000.0,
                                        avg.min * 1000.0,
                                        avg.max * 1000.0,
                                        avg.std_dev * 1000.0,
                                    ),
                                )
                            });
                            ui.add(
                                Label::new(
                                        RichText::new(txt).color(Color32::from_white_alpha(220)),
                                    )
                                    .truncate(),
                            );
                        }
                    },
                );
        }
    }
    struct FrametimeAvg {
        mean: f32,
        min: f32,
        max: f32,
        std_dev: f32,
    }
    const _: () = {
        impl ::bevy::reflect::GetTypeRegistration for FrametimeAvg {
            fn get_type_registration() -> ::bevy::reflect::TypeRegistration {
                let mut registration = ::bevy::reflect::TypeRegistration::of::<Self>();
                registration
                    .insert::<
                        ::bevy::reflect::ReflectFromPtr,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ::bevy::reflect::ReflectFromReflect,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
            }
            #[inline(never)]
            fn register_type_dependencies(registry: &mut ::bevy::reflect::TypeRegistry) {
                <f32 as ::bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                    registry,
                );
            }
        }
        impl ::bevy::reflect::Typed for FrametimeAvg {
            #[inline]
            fn type_info() -> &'static ::bevy::reflect::TypeInfo {
                static CELL: ::bevy::reflect::utility::NonGenericTypeInfoCell = ::bevy::reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(|| {
                    ::bevy::reflect::TypeInfo::Struct(
                        ::bevy::reflect::StructInfo::new::<
                            Self,
                        >(
                            &[
                                ::bevy::reflect::NamedField::new::<f32>("mean"),
                                ::bevy::reflect::NamedField::new::<f32>("min"),
                                ::bevy::reflect::NamedField::new::<f32>("max"),
                                ::bevy::reflect::NamedField::new::<f32>("std_dev"),
                            ],
                        ),
                    )
                })
            }
        }
        impl ::bevy::reflect::TypePath for FrametimeAvg {
            fn type_path() -> &'static str {
                "rust_bevy_test::egui_histogram::FrametimeAvg"
            }
            fn short_type_path() -> &'static str {
                "FrametimeAvg"
            }
            fn type_ident() -> Option<&'static str> {
                ::core::option::Option::Some("FrametimeAvg")
            }
            fn crate_name() -> Option<&'static str> {
                ::core::option::Option::Some(
                    "rust_bevy_test::egui_histogram".split(':').next().unwrap(),
                )
            }
            fn module_path() -> Option<&'static str> {
                ::core::option::Option::Some("rust_bevy_test::egui_histogram")
            }
        }
        impl ::bevy::reflect::Reflect for FrametimeAvg {
            #[inline]
            fn into_any(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::core::any::Any,
            > {
                self
            }
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }
            #[inline]
            fn into_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::bevy::reflect::Reflect,
            > {
                self
            }
            #[inline]
            fn as_reflect(&self) -> &dyn ::bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn ::bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn set(
                &mut self,
                value: ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
            ) -> ::core::result::Result<
                (),
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
            > {
                *self = <dyn ::bevy::reflect::Reflect>::take(value)?;
                ::core::result::Result::Ok(())
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    ::bevy::reflect::__macro_exports::auto_register::AutomaticReflectRegistrations(
                        <FrametimeAvg as ::bevy::reflect::__macro_exports::auto_register::RegisterForReflection>::__register,
                    )
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = ".CRT$XCU"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
        impl ::bevy::reflect::Struct for FrametimeAvg {
            fn field(
                &self,
                name: &str,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::PartialReflect> {
                match name {
                    "mean" => ::core::option::Option::Some(&self.mean),
                    "min" => ::core::option::Option::Some(&self.min),
                    "max" => ::core::option::Option::Some(&self.max),
                    "std_dev" => ::core::option::Option::Some(&self.std_dev),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_mut(
                &mut self,
                name: &str,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::PartialReflect> {
                match name {
                    "mean" => ::core::option::Option::Some(&mut self.mean),
                    "min" => ::core::option::Option::Some(&mut self.min),
                    "max" => ::core::option::Option::Some(&mut self.max),
                    "std_dev" => ::core::option::Option::Some(&mut self.std_dev),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at(
                &self,
                index: usize,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::PartialReflect> {
                match index {
                    0usize => ::core::option::Option::Some(&self.mean),
                    1usize => ::core::option::Option::Some(&self.min),
                    2usize => ::core::option::Option::Some(&self.max),
                    3usize => ::core::option::Option::Some(&self.std_dev),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at_mut(
                &mut self,
                index: usize,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::PartialReflect> {
                match index {
                    0usize => ::core::option::Option::Some(&mut self.mean),
                    1usize => ::core::option::Option::Some(&mut self.min),
                    2usize => ::core::option::Option::Some(&mut self.max),
                    3usize => ::core::option::Option::Some(&mut self.std_dev),
                    _ => ::core::option::Option::None,
                }
            }
            fn name_at(&self, index: usize) -> ::core::option::Option<&str> {
                match index {
                    0usize => ::core::option::Option::Some("mean"),
                    1usize => ::core::option::Option::Some("min"),
                    2usize => ::core::option::Option::Some("max"),
                    3usize => ::core::option::Option::Some("std_dev"),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_len(&self) -> usize {
                4usize
            }
            fn iter_fields(&self) -> ::bevy::reflect::FieldIter {
                ::bevy::reflect::FieldIter::new(self)
            }
            fn to_dynamic_struct(&self) -> ::bevy::reflect::DynamicStruct {
                let mut dynamic: ::bevy::reflect::DynamicStruct = ::core::default::Default::default();
                dynamic
                    .set_represented_type(
                        ::bevy::reflect::PartialReflect::get_represented_type_info(self),
                    );
                dynamic
                    .insert_boxed(
                        "mean",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.mean),
                    );
                dynamic
                    .insert_boxed(
                        "min",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.min),
                    );
                dynamic
                    .insert_boxed(
                        "max",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.max),
                    );
                dynamic
                    .insert_boxed(
                        "std_dev",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.std_dev),
                    );
                dynamic
            }
        }
        impl ::bevy::reflect::PartialReflect for FrametimeAvg {
            #[inline]
            fn get_represented_type_info(
                &self,
            ) -> ::core::option::Option<&'static ::bevy::reflect::TypeInfo> {
                ::core::option::Option::Some(
                    <Self as ::bevy::reflect::Typed>::type_info(),
                )
            }
            #[inline]
            fn try_apply(
                &mut self,
                value: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::result::Result<(), ::bevy::reflect::ApplyError> {
                if let ::bevy::reflect::ReflectRef::Struct(struct_value) = ::bevy::reflect::PartialReflect::reflect_ref(
                    value,
                ) {
                    for (i, value) in ::core::iter::Iterator::enumerate(
                        ::bevy::reflect::Struct::iter_fields(struct_value),
                    ) {
                        let name = ::bevy::reflect::Struct::name_at(struct_value, i)
                            .unwrap();
                        if let ::core::option::Option::Some(v) = ::bevy::reflect::Struct::field_mut(
                            self,
                            name,
                        ) {
                            ::bevy::reflect::PartialReflect::try_apply(v, value)?;
                        }
                    }
                } else {
                    return ::core::result::Result::Err(::bevy::reflect::ApplyError::MismatchedKinds {
                        from_kind: ::bevy::reflect::PartialReflect::reflect_kind(value),
                        to_kind: ::bevy::reflect::ReflectKind::Struct,
                    });
                }
                ::core::result::Result::Ok(())
            }
            #[inline]
            fn reflect_kind(&self) -> ::bevy::reflect::ReflectKind {
                ::bevy::reflect::ReflectKind::Struct
            }
            #[inline]
            fn reflect_ref(&self) -> ::bevy::reflect::ReflectRef {
                ::bevy::reflect::ReflectRef::Struct(self)
            }
            #[inline]
            fn reflect_mut(&mut self) -> ::bevy::reflect::ReflectMut {
                ::bevy::reflect::ReflectMut::Struct(self)
            }
            #[inline]
            fn reflect_owned(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::ReflectOwned {
                ::bevy::reflect::ReflectOwned::Struct(self)
            }
            #[inline]
            fn try_into_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::core::result::Result<
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::PartialReflect,
                >,
            > {
                ::core::result::Result::Ok(self)
            }
            #[inline]
            fn try_as_reflect(
                &self,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn try_as_reflect_mut(
                &mut self,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn into_partial_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::bevy::reflect::PartialReflect,
            > {
                self
            }
            #[inline]
            fn as_partial_reflect(&self) -> &dyn ::bevy::reflect::PartialReflect {
                self
            }
            #[inline]
            fn as_partial_reflect_mut(
                &mut self,
            ) -> &mut dyn ::bevy::reflect::PartialReflect {
                self
            }
            fn reflect_partial_eq(
                &self,
                value: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<bool> {
                (::bevy::reflect::struct_partial_eq)(self, value)
            }
            #[inline]
            #[allow(
                unreachable_code,
                reason = "Ignored fields without a `clone` attribute will early-return with an error"
            )]
            fn reflect_clone(
                &self,
            ) -> ::core::result::Result<
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
                ::bevy::reflect::ReflectCloneError,
            > {
                ::core::result::Result::Ok(
                    ::bevy::reflect::__macro_exports::alloc_utils::Box::new(Self {
                        mean: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.mean,
                        )?,
                        min: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.min,
                        )?,
                        max: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.max,
                        )?,
                        std_dev: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.std_dev,
                        )?,
                    }),
                )
            }
        }
        impl ::bevy::reflect::FromReflect for FrametimeAvg {
            fn from_reflect(
                reflect: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<Self> {
                if let ::bevy::reflect::ReflectRef::Struct(__ref_struct) = ::bevy::reflect::PartialReflect::reflect_ref(
                    reflect,
                ) {
                    let __this = Self {
                        mean: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "mean")?,
                        )?,
                        min: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "min")?,
                        )?,
                        max: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "max")?,
                        )?,
                        std_dev: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "std_dev")?,
                        )?,
                    };
                    ::core::option::Option::Some(__this)
                } else {
                    ::core::option::Option::None
                }
            }
        }
    };
    fn calc_frametime_avg(frametimes: &VecDeque<f32>) -> FrametimeAvg {
        let total: f32 = frametimes.iter().sum();
        let count = frametimes.len() as f32;
        let mean = total / frametimes.len() as f32;
        let mut min = f32::INFINITY;
        let mut max = f32::NEG_INFINITY;
        let mut variance: f32 = 0.0;
        for val in frametimes {
            min = min.min(*val);
            max = max.max(*val);
            let tmp = val - mean;
            variance += tmp * tmp;
        }
        let std_dev = (variance / (count - 1.0)).sqrt();
        FrametimeAvg {
            mean,
            min,
            max,
            std_dev,
        }
    }
}
mod app_control {
    use bevy::prelude::*;
    use bevy::ecs::system::{SystemState, RunSystemOnce};
    use bevy::window::{
        CursorIcon, CursorOptions, PrimaryWindow, WindowMode, PresentMode,
    };
    use bevy_egui::*;
    use egui::{Ui, RichText, Color32};
    use crate::phases::Phase;
    use crate::settings_file;
    use serde::*;
    pub struct AppControlPlugin;
    impl Plugin for AppControlPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(WindowSettings::default());
            app.add_systems(
                Update,
                (save_load_controls, window_control.after(save_load_controls))
                    .in_set(Phase::Start),
            );
            app.add_systems(EguiPrimaryContextPass, main_ui);
        }
    }
    #[reflect(Resource)]
    pub struct WindowSettings {
        pub fullscreen: bool,
        pub fullscreen_borderless: bool,
        pub vsync: bool,
    }
    impl ::bevy::ecs::resource::Resource for WindowSettings
    where
        Self: Send + Sync + 'static,
    {}
    const _: () = {
        impl ::bevy::reflect::GetTypeRegistration for WindowSettings {
            fn get_type_registration() -> ::bevy::reflect::TypeRegistration {
                let mut registration = ::bevy::reflect::TypeRegistration::of::<Self>();
                registration
                    .insert::<
                        ::bevy::reflect::ReflectFromPtr,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ::bevy::reflect::ReflectFromReflect,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ReflectResource,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
            }
            #[inline(never)]
            fn register_type_dependencies(registry: &mut ::bevy::reflect::TypeRegistry) {
                <bool as ::bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                    registry,
                );
            }
        }
        impl ::bevy::reflect::Typed for WindowSettings {
            #[inline]
            fn type_info() -> &'static ::bevy::reflect::TypeInfo {
                static CELL: ::bevy::reflect::utility::NonGenericTypeInfoCell = ::bevy::reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(|| {
                    ::bevy::reflect::TypeInfo::Struct(
                        ::bevy::reflect::StructInfo::new::<
                            Self,
                        >(
                            &[
                                ::bevy::reflect::NamedField::new::<bool>("fullscreen"),
                                ::bevy::reflect::NamedField::new::<
                                    bool,
                                >("fullscreen_borderless"),
                                ::bevy::reflect::NamedField::new::<bool>("vsync"),
                            ],
                        ),
                    )
                })
            }
        }
        impl ::bevy::reflect::TypePath for WindowSettings {
            fn type_path() -> &'static str {
                "rust_bevy_test::app_control::WindowSettings"
            }
            fn short_type_path() -> &'static str {
                "WindowSettings"
            }
            fn type_ident() -> Option<&'static str> {
                ::core::option::Option::Some("WindowSettings")
            }
            fn crate_name() -> Option<&'static str> {
                ::core::option::Option::Some(
                    "rust_bevy_test::app_control".split(':').next().unwrap(),
                )
            }
            fn module_path() -> Option<&'static str> {
                ::core::option::Option::Some("rust_bevy_test::app_control")
            }
        }
        impl ::bevy::reflect::Reflect for WindowSettings {
            #[inline]
            fn into_any(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::core::any::Any,
            > {
                self
            }
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }
            #[inline]
            fn into_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::bevy::reflect::Reflect,
            > {
                self
            }
            #[inline]
            fn as_reflect(&self) -> &dyn ::bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn ::bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn set(
                &mut self,
                value: ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
            ) -> ::core::result::Result<
                (),
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
            > {
                *self = <dyn ::bevy::reflect::Reflect>::take(value)?;
                ::core::result::Result::Ok(())
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    ::bevy::reflect::__macro_exports::auto_register::AutomaticReflectRegistrations(
                        <WindowSettings as ::bevy::reflect::__macro_exports::auto_register::RegisterForReflection>::__register,
                    )
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = ".CRT$XCU"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
        impl ::bevy::reflect::Struct for WindowSettings {
            fn field(
                &self,
                name: &str,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::PartialReflect> {
                match name {
                    "fullscreen" => ::core::option::Option::Some(&self.fullscreen),
                    "fullscreen_borderless" => {
                        ::core::option::Option::Some(&self.fullscreen_borderless)
                    }
                    "vsync" => ::core::option::Option::Some(&self.vsync),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_mut(
                &mut self,
                name: &str,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::PartialReflect> {
                match name {
                    "fullscreen" => ::core::option::Option::Some(&mut self.fullscreen),
                    "fullscreen_borderless" => {
                        ::core::option::Option::Some(&mut self.fullscreen_borderless)
                    }
                    "vsync" => ::core::option::Option::Some(&mut self.vsync),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at(
                &self,
                index: usize,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::PartialReflect> {
                match index {
                    0usize => ::core::option::Option::Some(&self.fullscreen),
                    1usize => ::core::option::Option::Some(&self.fullscreen_borderless),
                    2usize => ::core::option::Option::Some(&self.vsync),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at_mut(
                &mut self,
                index: usize,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::PartialReflect> {
                match index {
                    0usize => ::core::option::Option::Some(&mut self.fullscreen),
                    1usize => {
                        ::core::option::Option::Some(&mut self.fullscreen_borderless)
                    }
                    2usize => ::core::option::Option::Some(&mut self.vsync),
                    _ => ::core::option::Option::None,
                }
            }
            fn name_at(&self, index: usize) -> ::core::option::Option<&str> {
                match index {
                    0usize => ::core::option::Option::Some("fullscreen"),
                    1usize => ::core::option::Option::Some("fullscreen_borderless"),
                    2usize => ::core::option::Option::Some("vsync"),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_len(&self) -> usize {
                3usize
            }
            fn iter_fields(&self) -> ::bevy::reflect::FieldIter {
                ::bevy::reflect::FieldIter::new(self)
            }
            fn to_dynamic_struct(&self) -> ::bevy::reflect::DynamicStruct {
                let mut dynamic: ::bevy::reflect::DynamicStruct = ::core::default::Default::default();
                dynamic
                    .set_represented_type(
                        ::bevy::reflect::PartialReflect::get_represented_type_info(self),
                    );
                dynamic
                    .insert_boxed(
                        "fullscreen",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.fullscreen),
                    );
                dynamic
                    .insert_boxed(
                        "fullscreen_borderless",
                        ::bevy::reflect::PartialReflect::to_dynamic(
                            &self.fullscreen_borderless,
                        ),
                    );
                dynamic
                    .insert_boxed(
                        "vsync",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.vsync),
                    );
                dynamic
            }
        }
        impl ::bevy::reflect::PartialReflect for WindowSettings {
            #[inline]
            fn get_represented_type_info(
                &self,
            ) -> ::core::option::Option<&'static ::bevy::reflect::TypeInfo> {
                ::core::option::Option::Some(
                    <Self as ::bevy::reflect::Typed>::type_info(),
                )
            }
            #[inline]
            fn try_apply(
                &mut self,
                value: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::result::Result<(), ::bevy::reflect::ApplyError> {
                if let ::bevy::reflect::ReflectRef::Struct(struct_value) = ::bevy::reflect::PartialReflect::reflect_ref(
                    value,
                ) {
                    for (i, value) in ::core::iter::Iterator::enumerate(
                        ::bevy::reflect::Struct::iter_fields(struct_value),
                    ) {
                        let name = ::bevy::reflect::Struct::name_at(struct_value, i)
                            .unwrap();
                        if let ::core::option::Option::Some(v) = ::bevy::reflect::Struct::field_mut(
                            self,
                            name,
                        ) {
                            ::bevy::reflect::PartialReflect::try_apply(v, value)?;
                        }
                    }
                } else {
                    return ::core::result::Result::Err(::bevy::reflect::ApplyError::MismatchedKinds {
                        from_kind: ::bevy::reflect::PartialReflect::reflect_kind(value),
                        to_kind: ::bevy::reflect::ReflectKind::Struct,
                    });
                }
                ::core::result::Result::Ok(())
            }
            #[inline]
            fn reflect_kind(&self) -> ::bevy::reflect::ReflectKind {
                ::bevy::reflect::ReflectKind::Struct
            }
            #[inline]
            fn reflect_ref(&self) -> ::bevy::reflect::ReflectRef {
                ::bevy::reflect::ReflectRef::Struct(self)
            }
            #[inline]
            fn reflect_mut(&mut self) -> ::bevy::reflect::ReflectMut {
                ::bevy::reflect::ReflectMut::Struct(self)
            }
            #[inline]
            fn reflect_owned(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::ReflectOwned {
                ::bevy::reflect::ReflectOwned::Struct(self)
            }
            #[inline]
            fn try_into_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::core::result::Result<
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::PartialReflect,
                >,
            > {
                ::core::result::Result::Ok(self)
            }
            #[inline]
            fn try_as_reflect(
                &self,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn try_as_reflect_mut(
                &mut self,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn into_partial_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::bevy::reflect::PartialReflect,
            > {
                self
            }
            #[inline]
            fn as_partial_reflect(&self) -> &dyn ::bevy::reflect::PartialReflect {
                self
            }
            #[inline]
            fn as_partial_reflect_mut(
                &mut self,
            ) -> &mut dyn ::bevy::reflect::PartialReflect {
                self
            }
            fn reflect_partial_eq(
                &self,
                value: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<bool> {
                (::bevy::reflect::struct_partial_eq)(self, value)
            }
            #[inline]
            #[allow(
                unreachable_code,
                reason = "Ignored fields without a `clone` attribute will early-return with an error"
            )]
            fn reflect_clone(
                &self,
            ) -> ::core::result::Result<
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
                ::bevy::reflect::ReflectCloneError,
            > {
                ::core::result::Result::Ok(
                    ::bevy::reflect::__macro_exports::alloc_utils::Box::new(Self {
                        fullscreen: <bool as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.fullscreen,
                        )?,
                        fullscreen_borderless: <bool as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.fullscreen_borderless,
                        )?,
                        vsync: <bool as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.vsync,
                        )?,
                    }),
                )
            }
        }
        impl ::bevy::reflect::FromReflect for WindowSettings {
            fn from_reflect(
                reflect: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<Self> {
                if let ::bevy::reflect::ReflectRef::Struct(__ref_struct) = ::bevy::reflect::PartialReflect::reflect_ref(
                    reflect,
                ) {
                    let __this = Self {
                        fullscreen: <bool as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "fullscreen")?,
                        )?,
                        fullscreen_borderless: <bool as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(
                                __ref_struct,
                                "fullscreen_borderless",
                            )?,
                        )?,
                        vsync: <bool as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "vsync")?,
                        )?,
                    };
                    ::core::option::Option::Some(__this)
                } else {
                    ::core::option::Option::None
                }
            }
        }
    };
    #[automatically_derived]
    impl ::core::marker::Copy for WindowSettings {}
    #[automatically_derived]
    impl ::core::clone::Clone for WindowSettings {
        #[inline]
        fn clone(&self) -> WindowSettings {
            let _: ::core::clone::AssertParamIsClone<bool>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for WindowSettings {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for WindowSettings {
        #[inline]
        fn eq(&self, other: &WindowSettings) -> bool {
            self.fullscreen == other.fullscreen
                && self.fullscreen_borderless == other.fullscreen_borderless
                && self.vsync == other.vsync
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for WindowSettings {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "WindowSettings",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "fullscreen",
                    &self.fullscreen,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "fullscreen_borderless",
                    &self.fullscreen_borderless,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "vsync",
                    &self.vsync,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for WindowSettings {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            2u64 => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "fullscreen" => _serde::__private228::Ok(__Field::__field0),
                            "fullscreen_borderless" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            "vsync" => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"fullscreen" => _serde::__private228::Ok(__Field::__field0),
                            b"fullscreen_borderless" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            b"vsync" => _serde::__private228::Ok(__Field::__field2),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<WindowSettings>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = WindowSettings;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct WindowSettings",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct WindowSettings with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct WindowSettings with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct WindowSettings with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(WindowSettings {
                            fullscreen: __field0,
                            fullscreen_borderless: __field1,
                            vsync: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<bool> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<bool> = _serde::__private228::None;
                        let mut __field2: _serde::__private228::Option<bool> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "fullscreen",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "fullscreen_borderless",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private228::Option::is_some(&__field2) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("vsync"),
                                        );
                                    }
                                    __field2 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("fullscreen")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field(
                                    "fullscreen_borderless",
                                )?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private228::Some(__field2) => __field2,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("vsync")?
                            }
                        };
                        _serde::__private228::Ok(WindowSettings {
                            fullscreen: __field0,
                            fullscreen_borderless: __field1,
                            vsync: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "fullscreen",
                    "fullscreen_borderless",
                    "vsync",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "WindowSettings",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<WindowSettings>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    impl Default for WindowSettings {
        fn default() -> Self {
            Self {
                fullscreen: false,
                fullscreen_borderless: true,
                vsync: true,
            }
        }
    }
    pub const APP_NAME: &str = "Bevy Test Project";
    impl WindowSettings {
        fn update(mut window: Mut<Window>, mut settings: ResMut<WindowSettings>) {
            if settings.is_changed() {
                window.mode = match (
                    settings.fullscreen,
                    settings.fullscreen_borderless,
                ) {
                    (false, _) => WindowMode::Windowed,
                    (true, false) => {
                        WindowMode::Fullscreen(
                            MonitorSelection::Current,
                            VideoModeSelection::Current,
                        )
                    }
                    (true, true) => {
                        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
                    }
                };
                window.present_mode = match settings.vsync {
                    false => PresentMode::AutoNoVsync,
                    true => PresentMode::AutoVsync,
                };
            }
        }
    }
    fn window_control(
        keyboard: Res<ButtonInput<KeyCode>>,
        window: Single<&mut Window>,
        mut settings: ResMut<WindowSettings>,
    ) {
        if keyboard.just_pressed(KeyCode::F11)
            || (keyboard.any_pressed([KeyCode::AltLeft, KeyCode::AltRight])
                && keyboard.just_pressed(KeyCode::Enter))
        {
            settings.fullscreen = !settings.fullscreen;
        }
        WindowSettings::update(window.into_inner(), settings);
    }
    pub fn save_load_controls(
        world: &mut World,
        params: &mut SystemState<(Res<ButtonInput<KeyCode>>)>,
    ) {
        let (do_load, do_save) = {
            let keyboard = params.get(world);
            (
                keyboard.just_pressed(KeyCode::Semicolon),
                keyboard.just_pressed(KeyCode::Quote),
            )
        };
        if do_load {
            world.run_system_once(settings_file::load);
        } else if do_save {
            world.run_system_once(settings_file::save);
        }
    }
    fn main_ui(
        world: &mut World,
        sys: &mut SystemState<
            (
                Res<Time>,
                ResMut<WindowSettings>,
                MessageWriter<AppExit>,
                Local<crate::egui_histogram::Frametimes>,
                Commands,
            ),
        >,
    ) -> Result {
        let mut egui_context = world
            .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
            .single_mut(world)?
            .clone();
        let mut do_load = false;
        let mut do_save = false;
        egui::Window::new("Main")
            .show(
                egui_context.get_mut(),
                |ui| {
                    let (
                        time,
                        mut window_settings,
                        mut exit,
                        mut frametimes,
                        commands,
                    ) = sys.get_mut(world);
                    let mut ws = *window_settings;
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut ws.fullscreen, "Fullscreen");
                        ui.checkbox(&mut ws.fullscreen_borderless, "Borderless");
                        ui.checkbox(&mut ws.vsync, "Vsync");
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::TOP),
                            |ui| {
                                if ui
                                    .button(RichText::new("Quit").color(Color32::RED))
                                    .clicked()
                                {
                                    exit.write(AppExit::Success);
                                }
                            },
                        );
                    });
                    if ws != *window_settings {
                        *window_settings = ws;
                    }
                    frametimes.gui(ui, time);
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.label("settings.json:");
                        if ui.button("Load [;]").clicked() {
                            do_load = true;
                        }
                        if ui.button("Save [']").clicked() {
                            do_save = true;
                        }
                    });
                },
            );
        if do_load {
            world.run_system_once(settings_file::load);
        } else if do_save {
            world.run_system_once(settings_file::save);
        }
        Ok(())
    }
}
mod debug_camera {
    use bevy::prelude::*;
    use crate::flycam::Flycam;
    use crate::phases::Phase;
    use serde::*;
    pub struct DebugCameraPlugin;
    impl Plugin for DebugCameraPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(DebugCameraState::default())
                .add_systems(Update, update.before(Phase::CameraUpdate));
        }
    }
    pub struct MainCamera;
    impl ::bevy::ecs::component::Component for MainCamera
    where
        Self: Send + Sync + 'static,
    {
        const STORAGE_TYPE: ::bevy::ecs::component::StorageType = ::bevy::ecs::component::StorageType::Table;
        type Mutability = ::bevy::ecs::component::Mutable;
        fn register_required_components(
            _requiree: ::bevy::ecs::component::ComponentId,
            required_components: &mut ::bevy::ecs::component::RequiredComponentsRegistrator,
        ) {}
        fn clone_behavior() -> ::bevy::ecs::component::ComponentCloneBehavior {
            use ::bevy::ecs::component::{
                DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
            };
            (&&&::bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                Self,
            >::default())
                .default_clone_behavior()
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for MainCamera {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_unit_struct(__serializer, "MainCamera")
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MainCamera {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<MainCamera>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MainCamera;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "unit struct MainCamera",
                        )
                    }
                    #[inline]
                    fn visit_unit<__E>(
                        self,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private228::Ok(MainCamera)
                    }
                }
                _serde::Deserializer::deserialize_unit_struct(
                    __deserializer,
                    "MainCamera",
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<MainCamera>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        impl ::bevy::reflect::GetTypeRegistration for MainCamera {
            fn get_type_registration() -> ::bevy::reflect::TypeRegistration {
                let mut registration = ::bevy::reflect::TypeRegistration::of::<Self>();
                registration
                    .insert::<
                        ::bevy::reflect::ReflectFromPtr,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ::bevy::reflect::ReflectFromReflect,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
            }
            #[inline(never)]
            fn register_type_dependencies(registry: &mut ::bevy::reflect::TypeRegistry) {}
        }
        impl ::bevy::reflect::Typed for MainCamera {
            #[inline]
            fn type_info() -> &'static ::bevy::reflect::TypeInfo {
                static CELL: ::bevy::reflect::utility::NonGenericTypeInfoCell = ::bevy::reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(|| {
                    ::bevy::reflect::TypeInfo::Struct(
                        ::bevy::reflect::StructInfo::new::<Self>(&[]),
                    )
                })
            }
        }
        impl ::bevy::reflect::TypePath for MainCamera {
            fn type_path() -> &'static str {
                "rust_bevy_test::debug_camera::MainCamera"
            }
            fn short_type_path() -> &'static str {
                "MainCamera"
            }
            fn type_ident() -> Option<&'static str> {
                ::core::option::Option::Some("MainCamera")
            }
            fn crate_name() -> Option<&'static str> {
                ::core::option::Option::Some(
                    "rust_bevy_test::debug_camera".split(':').next().unwrap(),
                )
            }
            fn module_path() -> Option<&'static str> {
                ::core::option::Option::Some("rust_bevy_test::debug_camera")
            }
        }
        impl ::bevy::reflect::Reflect for MainCamera {
            #[inline]
            fn into_any(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::core::any::Any,
            > {
                self
            }
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }
            #[inline]
            fn into_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::bevy::reflect::Reflect,
            > {
                self
            }
            #[inline]
            fn as_reflect(&self) -> &dyn ::bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn ::bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn set(
                &mut self,
                value: ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
            ) -> ::core::result::Result<
                (),
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
            > {
                *self = <dyn ::bevy::reflect::Reflect>::take(value)?;
                ::core::result::Result::Ok(())
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    ::bevy::reflect::__macro_exports::auto_register::AutomaticReflectRegistrations(
                        <MainCamera as ::bevy::reflect::__macro_exports::auto_register::RegisterForReflection>::__register,
                    )
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = ".CRT$XCU"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
        impl ::bevy::reflect::Struct for MainCamera {
            fn field(
                &self,
                name: &str,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::PartialReflect> {
                match name {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_mut(
                &mut self,
                name: &str,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::PartialReflect> {
                match name {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at(
                &self,
                index: usize,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::PartialReflect> {
                match index {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at_mut(
                &mut self,
                index: usize,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::PartialReflect> {
                match index {
                    _ => ::core::option::Option::None,
                }
            }
            fn name_at(&self, index: usize) -> ::core::option::Option<&str> {
                match index {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_len(&self) -> usize {
                0usize
            }
            fn iter_fields(&self) -> ::bevy::reflect::FieldIter {
                ::bevy::reflect::FieldIter::new(self)
            }
            fn to_dynamic_struct(&self) -> ::bevy::reflect::DynamicStruct {
                let mut dynamic: ::bevy::reflect::DynamicStruct = ::core::default::Default::default();
                dynamic
                    .set_represented_type(
                        ::bevy::reflect::PartialReflect::get_represented_type_info(self),
                    );
                dynamic
            }
        }
        impl ::bevy::reflect::PartialReflect for MainCamera {
            #[inline]
            fn get_represented_type_info(
                &self,
            ) -> ::core::option::Option<&'static ::bevy::reflect::TypeInfo> {
                ::core::option::Option::Some(
                    <Self as ::bevy::reflect::Typed>::type_info(),
                )
            }
            #[inline]
            fn try_apply(
                &mut self,
                value: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::result::Result<(), ::bevy::reflect::ApplyError> {
                if let ::bevy::reflect::ReflectRef::Struct(struct_value) = ::bevy::reflect::PartialReflect::reflect_ref(
                    value,
                ) {
                    for (i, value) in ::core::iter::Iterator::enumerate(
                        ::bevy::reflect::Struct::iter_fields(struct_value),
                    ) {
                        let name = ::bevy::reflect::Struct::name_at(struct_value, i)
                            .unwrap();
                        if let ::core::option::Option::Some(v) = ::bevy::reflect::Struct::field_mut(
                            self,
                            name,
                        ) {
                            ::bevy::reflect::PartialReflect::try_apply(v, value)?;
                        }
                    }
                } else {
                    return ::core::result::Result::Err(::bevy::reflect::ApplyError::MismatchedKinds {
                        from_kind: ::bevy::reflect::PartialReflect::reflect_kind(value),
                        to_kind: ::bevy::reflect::ReflectKind::Struct,
                    });
                }
                ::core::result::Result::Ok(())
            }
            #[inline]
            fn reflect_kind(&self) -> ::bevy::reflect::ReflectKind {
                ::bevy::reflect::ReflectKind::Struct
            }
            #[inline]
            fn reflect_ref(&self) -> ::bevy::reflect::ReflectRef {
                ::bevy::reflect::ReflectRef::Struct(self)
            }
            #[inline]
            fn reflect_mut(&mut self) -> ::bevy::reflect::ReflectMut {
                ::bevy::reflect::ReflectMut::Struct(self)
            }
            #[inline]
            fn reflect_owned(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::ReflectOwned {
                ::bevy::reflect::ReflectOwned::Struct(self)
            }
            #[inline]
            fn try_into_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::core::result::Result<
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::PartialReflect,
                >,
            > {
                ::core::result::Result::Ok(self)
            }
            #[inline]
            fn try_as_reflect(
                &self,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn try_as_reflect_mut(
                &mut self,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn into_partial_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::bevy::reflect::PartialReflect,
            > {
                self
            }
            #[inline]
            fn as_partial_reflect(&self) -> &dyn ::bevy::reflect::PartialReflect {
                self
            }
            #[inline]
            fn as_partial_reflect_mut(
                &mut self,
            ) -> &mut dyn ::bevy::reflect::PartialReflect {
                self
            }
            fn reflect_partial_eq(
                &self,
                value: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<bool> {
                (::bevy::reflect::struct_partial_eq)(self, value)
            }
            #[inline]
            #[allow(
                unreachable_code,
                reason = "Ignored fields without a `clone` attribute will early-return with an error"
            )]
            fn reflect_clone(
                &self,
            ) -> ::core::result::Result<
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
                ::bevy::reflect::ReflectCloneError,
            > {
                ::core::result::Result::Ok(
                    ::bevy::reflect::__macro_exports::alloc_utils::Box::new(Self {}),
                )
            }
        }
        impl ::bevy::reflect::FromReflect for MainCamera {
            fn from_reflect(
                reflect: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<Self> {
                if let ::bevy::reflect::ReflectRef::Struct(__ref_struct) = ::bevy::reflect::PartialReflect::reflect_ref(
                    reflect,
                ) {
                    let __this = Self {};
                    ::core::option::Option::Some(__this)
                } else {
                    ::core::option::Option::None
                }
            }
        }
    };
    pub struct DebugCamera;
    impl ::bevy::ecs::component::Component for DebugCamera
    where
        Self: Send + Sync + 'static,
    {
        const STORAGE_TYPE: ::bevy::ecs::component::StorageType = ::bevy::ecs::component::StorageType::Table;
        type Mutability = ::bevy::ecs::component::Mutable;
        fn register_required_components(
            _requiree: ::bevy::ecs::component::ComponentId,
            required_components: &mut ::bevy::ecs::component::RequiredComponentsRegistrator,
        ) {}
        fn clone_behavior() -> ::bevy::ecs::component::ComponentCloneBehavior {
            use ::bevy::ecs::component::{
                DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
            };
            (&&&::bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                Self,
            >::default())
                .default_clone_behavior()
        }
    }
    #[reflect(Resource)]
    pub struct DebugCameraState {
        viewing_debug_cam: bool,
    }
    impl ::bevy::ecs::resource::Resource for DebugCameraState
    where
        Self: Send + Sync + 'static,
    {}
    #[automatically_derived]
    impl ::core::default::Default for DebugCameraState {
        #[inline]
        fn default() -> DebugCameraState {
            DebugCameraState {
                viewing_debug_cam: ::core::default::Default::default(),
            }
        }
    }
    const _: () = {
        impl ::bevy::reflect::GetTypeRegistration for DebugCameraState {
            fn get_type_registration() -> ::bevy::reflect::TypeRegistration {
                let mut registration = ::bevy::reflect::TypeRegistration::of::<Self>();
                registration
                    .insert::<
                        ::bevy::reflect::ReflectFromPtr,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ::bevy::reflect::ReflectFromReflect,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ReflectResource,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
            }
            #[inline(never)]
            fn register_type_dependencies(registry: &mut ::bevy::reflect::TypeRegistry) {
                <bool as ::bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                    registry,
                );
            }
        }
        impl ::bevy::reflect::Typed for DebugCameraState {
            #[inline]
            fn type_info() -> &'static ::bevy::reflect::TypeInfo {
                static CELL: ::bevy::reflect::utility::NonGenericTypeInfoCell = ::bevy::reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(|| {
                    ::bevy::reflect::TypeInfo::Struct(
                        ::bevy::reflect::StructInfo::new::<
                            Self,
                        >(
                            &[
                                ::bevy::reflect::NamedField::new::<
                                    bool,
                                >("viewing_debug_cam"),
                            ],
                        ),
                    )
                })
            }
        }
        impl ::bevy::reflect::TypePath for DebugCameraState {
            fn type_path() -> &'static str {
                "rust_bevy_test::debug_camera::DebugCameraState"
            }
            fn short_type_path() -> &'static str {
                "DebugCameraState"
            }
            fn type_ident() -> Option<&'static str> {
                ::core::option::Option::Some("DebugCameraState")
            }
            fn crate_name() -> Option<&'static str> {
                ::core::option::Option::Some(
                    "rust_bevy_test::debug_camera".split(':').next().unwrap(),
                )
            }
            fn module_path() -> Option<&'static str> {
                ::core::option::Option::Some("rust_bevy_test::debug_camera")
            }
        }
        impl ::bevy::reflect::Reflect for DebugCameraState {
            #[inline]
            fn into_any(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::core::any::Any,
            > {
                self
            }
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }
            #[inline]
            fn into_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::bevy::reflect::Reflect,
            > {
                self
            }
            #[inline]
            fn as_reflect(&self) -> &dyn ::bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn ::bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn set(
                &mut self,
                value: ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
            ) -> ::core::result::Result<
                (),
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
            > {
                *self = <dyn ::bevy::reflect::Reflect>::take(value)?;
                ::core::result::Result::Ok(())
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    ::bevy::reflect::__macro_exports::auto_register::AutomaticReflectRegistrations(
                        <DebugCameraState as ::bevy::reflect::__macro_exports::auto_register::RegisterForReflection>::__register,
                    )
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = ".CRT$XCU"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
        impl ::bevy::reflect::Struct for DebugCameraState {
            fn field(
                &self,
                name: &str,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::PartialReflect> {
                match name {
                    "viewing_debug_cam" => {
                        ::core::option::Option::Some(&self.viewing_debug_cam)
                    }
                    _ => ::core::option::Option::None,
                }
            }
            fn field_mut(
                &mut self,
                name: &str,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::PartialReflect> {
                match name {
                    "viewing_debug_cam" => {
                        ::core::option::Option::Some(&mut self.viewing_debug_cam)
                    }
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at(
                &self,
                index: usize,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::PartialReflect> {
                match index {
                    0usize => ::core::option::Option::Some(&self.viewing_debug_cam),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at_mut(
                &mut self,
                index: usize,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::PartialReflect> {
                match index {
                    0usize => ::core::option::Option::Some(&mut self.viewing_debug_cam),
                    _ => ::core::option::Option::None,
                }
            }
            fn name_at(&self, index: usize) -> ::core::option::Option<&str> {
                match index {
                    0usize => ::core::option::Option::Some("viewing_debug_cam"),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_len(&self) -> usize {
                1usize
            }
            fn iter_fields(&self) -> ::bevy::reflect::FieldIter {
                ::bevy::reflect::FieldIter::new(self)
            }
            fn to_dynamic_struct(&self) -> ::bevy::reflect::DynamicStruct {
                let mut dynamic: ::bevy::reflect::DynamicStruct = ::core::default::Default::default();
                dynamic
                    .set_represented_type(
                        ::bevy::reflect::PartialReflect::get_represented_type_info(self),
                    );
                dynamic
                    .insert_boxed(
                        "viewing_debug_cam",
                        ::bevy::reflect::PartialReflect::to_dynamic(
                            &self.viewing_debug_cam,
                        ),
                    );
                dynamic
            }
        }
        impl ::bevy::reflect::PartialReflect for DebugCameraState {
            #[inline]
            fn get_represented_type_info(
                &self,
            ) -> ::core::option::Option<&'static ::bevy::reflect::TypeInfo> {
                ::core::option::Option::Some(
                    <Self as ::bevy::reflect::Typed>::type_info(),
                )
            }
            #[inline]
            fn try_apply(
                &mut self,
                value: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::result::Result<(), ::bevy::reflect::ApplyError> {
                if let ::bevy::reflect::ReflectRef::Struct(struct_value) = ::bevy::reflect::PartialReflect::reflect_ref(
                    value,
                ) {
                    for (i, value) in ::core::iter::Iterator::enumerate(
                        ::bevy::reflect::Struct::iter_fields(struct_value),
                    ) {
                        let name = ::bevy::reflect::Struct::name_at(struct_value, i)
                            .unwrap();
                        if let ::core::option::Option::Some(v) = ::bevy::reflect::Struct::field_mut(
                            self,
                            name,
                        ) {
                            ::bevy::reflect::PartialReflect::try_apply(v, value)?;
                        }
                    }
                } else {
                    return ::core::result::Result::Err(::bevy::reflect::ApplyError::MismatchedKinds {
                        from_kind: ::bevy::reflect::PartialReflect::reflect_kind(value),
                        to_kind: ::bevy::reflect::ReflectKind::Struct,
                    });
                }
                ::core::result::Result::Ok(())
            }
            #[inline]
            fn reflect_kind(&self) -> ::bevy::reflect::ReflectKind {
                ::bevy::reflect::ReflectKind::Struct
            }
            #[inline]
            fn reflect_ref(&self) -> ::bevy::reflect::ReflectRef {
                ::bevy::reflect::ReflectRef::Struct(self)
            }
            #[inline]
            fn reflect_mut(&mut self) -> ::bevy::reflect::ReflectMut {
                ::bevy::reflect::ReflectMut::Struct(self)
            }
            #[inline]
            fn reflect_owned(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::ReflectOwned {
                ::bevy::reflect::ReflectOwned::Struct(self)
            }
            #[inline]
            fn try_into_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::core::result::Result<
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::PartialReflect,
                >,
            > {
                ::core::result::Result::Ok(self)
            }
            #[inline]
            fn try_as_reflect(
                &self,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn try_as_reflect_mut(
                &mut self,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn into_partial_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::bevy::reflect::PartialReflect,
            > {
                self
            }
            #[inline]
            fn as_partial_reflect(&self) -> &dyn ::bevy::reflect::PartialReflect {
                self
            }
            #[inline]
            fn as_partial_reflect_mut(
                &mut self,
            ) -> &mut dyn ::bevy::reflect::PartialReflect {
                self
            }
            fn reflect_partial_eq(
                &self,
                value: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<bool> {
                (::bevy::reflect::struct_partial_eq)(self, value)
            }
            #[inline]
            #[allow(
                unreachable_code,
                reason = "Ignored fields without a `clone` attribute will early-return with an error"
            )]
            fn reflect_clone(
                &self,
            ) -> ::core::result::Result<
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
                ::bevy::reflect::ReflectCloneError,
            > {
                ::core::result::Result::Ok(
                    ::bevy::reflect::__macro_exports::alloc_utils::Box::new(Self {
                        viewing_debug_cam: <bool as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.viewing_debug_cam,
                        )?,
                    }),
                )
            }
        }
        impl ::bevy::reflect::FromReflect for DebugCameraState {
            fn from_reflect(
                reflect: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<Self> {
                if let ::bevy::reflect::ReflectRef::Struct(__ref_struct) = ::bevy::reflect::PartialReflect::reflect_ref(
                    reflect,
                ) {
                    let __this = Self {
                        viewing_debug_cam: <bool as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(
                                __ref_struct,
                                "viewing_debug_cam",
                            )?,
                        )?,
                    };
                    ::core::option::Option::Some(__this)
                } else {
                    ::core::option::Option::None
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for DebugCameraState {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "DebugCameraState",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "viewing_debug_cam",
                    &self.viewing_debug_cam,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for DebugCameraState {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "viewing_debug_cam" => {
                                _serde::__private228::Ok(__Field::__field0)
                            }
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"viewing_debug_cam" => {
                                _serde::__private228::Ok(__Field::__field0)
                            }
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<DebugCameraState>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = DebugCameraState;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct DebugCameraState",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct DebugCameraState with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(DebugCameraState {
                            viewing_debug_cam: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<bool> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "viewing_debug_cam",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field(
                                    "viewing_debug_cam",
                                )?
                            }
                        };
                        _serde::__private228::Ok(DebugCameraState {
                            viewing_debug_cam: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["viewing_debug_cam"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "DebugCameraState",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<DebugCameraState>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    fn update(
        mut state: ResMut<DebugCameraState>,
        keyboard: Res<ButtonInput<KeyCode>>,
        main_cam: Single<
            (&mut Camera, &Transform),
            (With<MainCamera>, Without<DebugCamera>),
        >,
        debug_cam: Single<
            (&mut Camera, &mut Transform),
            (With<DebugCamera>, Without<MainCamera>),
        >,
        mut commands: Commands,
    ) {
        let (mut main_cam, main_transf) = main_cam.into_inner();
        let (mut debug_cam, mut debug_transf) = debug_cam.into_inner();
        if keyboard.just_pressed(KeyCode::KeyP) {
            state.viewing_debug_cam = !state.viewing_debug_cam;
            if state.viewing_debug_cam {
                *debug_transf = *main_transf;
            }
        }
        main_cam.is_active = !state.viewing_debug_cam;
        debug_cam.is_active = state.viewing_debug_cam;
    }
}
mod flycam {
    use bevy::{
        prelude::*, math, input::mouse::{MouseMotion, MouseWheel, MouseScrollUnit},
        window::{CursorGrabMode, CursorIcon, CursorOptions, SystemCursorIcon},
    };
    use bevy_egui::render::systems::EguiTransform;
    use core::f32;
    use std::fmt;
    use crate::app_control::WindowSettings;
    use crate::phases::Phase;
    use serde::*;
    pub struct FlycamPlugin;
    impl Plugin for FlycamPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Update, update_cursor.before(update_camera))
                .add_systems(Update, update_camera.in_set(Phase::CameraUpdate));
        }
    }
    const MOUSELOOK_BTN: MouseButton = MouseButton::Middle;
    #[require(Transform, Camera3d, Camera, Projection)]
    #[reflect(Component)]
    pub struct Flycam {
        pub move_planar: bool,
        pub vfov_multiplied_sensitivity: bool,
        pub mouse_sens: f32,
        pub default_vfov: f32,
        pub vfov_target: f32,
        pub vfov_smooth: f32,
        pub zoom_speed: f32,
        pub speed: f32,
        pub base_speed: f32,
        pub max_speed: f32,
        pub speedup_factor: f32,
        pub fast_multiplier: f32,
        #[serde(skip)]
        pub test: f32,
    }
    #[doc = "**Required Components**: [`Transform`], [`Camera3d`], [`Camera`], [`Projection`]. \n\n A component's Required Components are inserted whenever it is inserted. Note that this will also insert the required components _of_ the required components, recursively, in depth-first order."]
    impl ::bevy::ecs::component::Component for Flycam
    where
        Self: Send + Sync + 'static,
    {
        const STORAGE_TYPE: ::bevy::ecs::component::StorageType = ::bevy::ecs::component::StorageType::Table;
        type Mutability = ::bevy::ecs::component::Mutable;
        fn register_required_components(
            _requiree: ::bevy::ecs::component::ComponentId,
            required_components: &mut ::bevy::ecs::component::RequiredComponentsRegistrator,
        ) {
            required_components
                .register_required::<Transform>(<Transform as Default>::default);
            required_components
                .register_required::<Camera3d>(<Camera3d as Default>::default);
            required_components
                .register_required::<Camera>(<Camera as Default>::default);
            required_components
                .register_required::<Projection>(<Projection as Default>::default);
        }
        fn clone_behavior() -> ::bevy::ecs::component::ComponentCloneBehavior {
            use ::bevy::ecs::component::{
                DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
            };
            (&&&::bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                Self,
            >::default())
                .default_clone_behavior()
        }
    }
    const _: () = {
        impl ::bevy::reflect::GetTypeRegistration for Flycam {
            fn get_type_registration() -> ::bevy::reflect::TypeRegistration {
                let mut registration = ::bevy::reflect::TypeRegistration::of::<Self>();
                registration
                    .insert::<
                        ::bevy::reflect::ReflectFromPtr,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ::bevy::reflect::ReflectFromReflect,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ReflectComponent,
                    >(::bevy::reflect::FromType::<Self>::from_type());
                registration
            }
            #[inline(never)]
            fn register_type_dependencies(registry: &mut ::bevy::reflect::TypeRegistry) {
                <bool as ::bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                    registry,
                );
                <f32 as ::bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                    registry,
                );
            }
        }
        impl ::bevy::reflect::Typed for Flycam {
            #[inline]
            fn type_info() -> &'static ::bevy::reflect::TypeInfo {
                static CELL: ::bevy::reflect::utility::NonGenericTypeInfoCell = ::bevy::reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(|| {
                    ::bevy::reflect::TypeInfo::Struct(
                        ::bevy::reflect::StructInfo::new::<
                            Self,
                        >(
                            &[
                                ::bevy::reflect::NamedField::new::<bool>("move_planar"),
                                ::bevy::reflect::NamedField::new::<
                                    bool,
                                >("vfov_multiplied_sensitivity"),
                                ::bevy::reflect::NamedField::new::<f32>("mouse_sens"),
                                ::bevy::reflect::NamedField::new::<f32>("default_vfov"),
                                ::bevy::reflect::NamedField::new::<f32>("vfov_target"),
                                ::bevy::reflect::NamedField::new::<f32>("vfov_smooth"),
                                ::bevy::reflect::NamedField::new::<f32>("zoom_speed"),
                                ::bevy::reflect::NamedField::new::<f32>("speed"),
                                ::bevy::reflect::NamedField::new::<f32>("base_speed"),
                                ::bevy::reflect::NamedField::new::<f32>("max_speed"),
                                ::bevy::reflect::NamedField::new::<f32>("speedup_factor"),
                                ::bevy::reflect::NamedField::new::<f32>("fast_multiplier"),
                                ::bevy::reflect::NamedField::new::<f32>("test"),
                            ],
                        ),
                    )
                })
            }
        }
        impl ::bevy::reflect::TypePath for Flycam {
            fn type_path() -> &'static str {
                "rust_bevy_test::flycam::Flycam"
            }
            fn short_type_path() -> &'static str {
                "Flycam"
            }
            fn type_ident() -> Option<&'static str> {
                ::core::option::Option::Some("Flycam")
            }
            fn crate_name() -> Option<&'static str> {
                ::core::option::Option::Some(
                    "rust_bevy_test::flycam".split(':').next().unwrap(),
                )
            }
            fn module_path() -> Option<&'static str> {
                ::core::option::Option::Some("rust_bevy_test::flycam")
            }
        }
        impl ::bevy::reflect::Reflect for Flycam {
            #[inline]
            fn into_any(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::core::any::Any,
            > {
                self
            }
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }
            #[inline]
            fn into_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::bevy::reflect::Reflect,
            > {
                self
            }
            #[inline]
            fn as_reflect(&self) -> &dyn ::bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn ::bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn set(
                &mut self,
                value: ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
            ) -> ::core::result::Result<
                (),
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
            > {
                *self = <dyn ::bevy::reflect::Reflect>::take(value)?;
                ::core::result::Result::Ok(())
            }
        }
        #[allow(non_upper_case_globals)]
        const _: () = {
            static __INVENTORY: ::inventory::Node = ::inventory::Node {
                value: &{
                    ::bevy::reflect::__macro_exports::auto_register::AutomaticReflectRegistrations(
                        <Flycam as ::bevy::reflect::__macro_exports::auto_register::RegisterForReflection>::__register,
                    )
                },
                next: ::inventory::core::cell::UnsafeCell::new(
                    ::inventory::core::option::Option::None,
                ),
            };
            unsafe extern "C" fn __ctor() {
                unsafe {
                    ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY)
                }
            }
            #[used]
            #[link_section = ".CRT$XCU"]
            static __CTOR: unsafe extern "C" fn() = __ctor;
        };
        impl ::bevy::reflect::Struct for Flycam {
            fn field(
                &self,
                name: &str,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::PartialReflect> {
                match name {
                    "move_planar" => ::core::option::Option::Some(&self.move_planar),
                    "vfov_multiplied_sensitivity" => {
                        ::core::option::Option::Some(&self.vfov_multiplied_sensitivity)
                    }
                    "mouse_sens" => ::core::option::Option::Some(&self.mouse_sens),
                    "default_vfov" => ::core::option::Option::Some(&self.default_vfov),
                    "vfov_target" => ::core::option::Option::Some(&self.vfov_target),
                    "vfov_smooth" => ::core::option::Option::Some(&self.vfov_smooth),
                    "zoom_speed" => ::core::option::Option::Some(&self.zoom_speed),
                    "speed" => ::core::option::Option::Some(&self.speed),
                    "base_speed" => ::core::option::Option::Some(&self.base_speed),
                    "max_speed" => ::core::option::Option::Some(&self.max_speed),
                    "speedup_factor" => {
                        ::core::option::Option::Some(&self.speedup_factor)
                    }
                    "fast_multiplier" => {
                        ::core::option::Option::Some(&self.fast_multiplier)
                    }
                    "test" => ::core::option::Option::Some(&self.test),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_mut(
                &mut self,
                name: &str,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::PartialReflect> {
                match name {
                    "move_planar" => ::core::option::Option::Some(&mut self.move_planar),
                    "vfov_multiplied_sensitivity" => {
                        ::core::option::Option::Some(
                            &mut self.vfov_multiplied_sensitivity,
                        )
                    }
                    "mouse_sens" => ::core::option::Option::Some(&mut self.mouse_sens),
                    "default_vfov" => {
                        ::core::option::Option::Some(&mut self.default_vfov)
                    }
                    "vfov_target" => ::core::option::Option::Some(&mut self.vfov_target),
                    "vfov_smooth" => ::core::option::Option::Some(&mut self.vfov_smooth),
                    "zoom_speed" => ::core::option::Option::Some(&mut self.zoom_speed),
                    "speed" => ::core::option::Option::Some(&mut self.speed),
                    "base_speed" => ::core::option::Option::Some(&mut self.base_speed),
                    "max_speed" => ::core::option::Option::Some(&mut self.max_speed),
                    "speedup_factor" => {
                        ::core::option::Option::Some(&mut self.speedup_factor)
                    }
                    "fast_multiplier" => {
                        ::core::option::Option::Some(&mut self.fast_multiplier)
                    }
                    "test" => ::core::option::Option::Some(&mut self.test),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at(
                &self,
                index: usize,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::PartialReflect> {
                match index {
                    0usize => ::core::option::Option::Some(&self.move_planar),
                    1usize => {
                        ::core::option::Option::Some(&self.vfov_multiplied_sensitivity)
                    }
                    2usize => ::core::option::Option::Some(&self.mouse_sens),
                    3usize => ::core::option::Option::Some(&self.default_vfov),
                    4usize => ::core::option::Option::Some(&self.vfov_target),
                    5usize => ::core::option::Option::Some(&self.vfov_smooth),
                    6usize => ::core::option::Option::Some(&self.zoom_speed),
                    7usize => ::core::option::Option::Some(&self.speed),
                    8usize => ::core::option::Option::Some(&self.base_speed),
                    9usize => ::core::option::Option::Some(&self.max_speed),
                    10usize => ::core::option::Option::Some(&self.speedup_factor),
                    11usize => ::core::option::Option::Some(&self.fast_multiplier),
                    12usize => ::core::option::Option::Some(&self.test),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at_mut(
                &mut self,
                index: usize,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::PartialReflect> {
                match index {
                    0usize => ::core::option::Option::Some(&mut self.move_planar),
                    1usize => {
                        ::core::option::Option::Some(
                            &mut self.vfov_multiplied_sensitivity,
                        )
                    }
                    2usize => ::core::option::Option::Some(&mut self.mouse_sens),
                    3usize => ::core::option::Option::Some(&mut self.default_vfov),
                    4usize => ::core::option::Option::Some(&mut self.vfov_target),
                    5usize => ::core::option::Option::Some(&mut self.vfov_smooth),
                    6usize => ::core::option::Option::Some(&mut self.zoom_speed),
                    7usize => ::core::option::Option::Some(&mut self.speed),
                    8usize => ::core::option::Option::Some(&mut self.base_speed),
                    9usize => ::core::option::Option::Some(&mut self.max_speed),
                    10usize => ::core::option::Option::Some(&mut self.speedup_factor),
                    11usize => ::core::option::Option::Some(&mut self.fast_multiplier),
                    12usize => ::core::option::Option::Some(&mut self.test),
                    _ => ::core::option::Option::None,
                }
            }
            fn name_at(&self, index: usize) -> ::core::option::Option<&str> {
                match index {
                    0usize => ::core::option::Option::Some("move_planar"),
                    1usize => ::core::option::Option::Some("vfov_multiplied_sensitivity"),
                    2usize => ::core::option::Option::Some("mouse_sens"),
                    3usize => ::core::option::Option::Some("default_vfov"),
                    4usize => ::core::option::Option::Some("vfov_target"),
                    5usize => ::core::option::Option::Some("vfov_smooth"),
                    6usize => ::core::option::Option::Some("zoom_speed"),
                    7usize => ::core::option::Option::Some("speed"),
                    8usize => ::core::option::Option::Some("base_speed"),
                    9usize => ::core::option::Option::Some("max_speed"),
                    10usize => ::core::option::Option::Some("speedup_factor"),
                    11usize => ::core::option::Option::Some("fast_multiplier"),
                    12usize => ::core::option::Option::Some("test"),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_len(&self) -> usize {
                13usize
            }
            fn iter_fields(&self) -> ::bevy::reflect::FieldIter {
                ::bevy::reflect::FieldIter::new(self)
            }
            fn to_dynamic_struct(&self) -> ::bevy::reflect::DynamicStruct {
                let mut dynamic: ::bevy::reflect::DynamicStruct = ::core::default::Default::default();
                dynamic
                    .set_represented_type(
                        ::bevy::reflect::PartialReflect::get_represented_type_info(self),
                    );
                dynamic
                    .insert_boxed(
                        "move_planar",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.move_planar),
                    );
                dynamic
                    .insert_boxed(
                        "vfov_multiplied_sensitivity",
                        ::bevy::reflect::PartialReflect::to_dynamic(
                            &self.vfov_multiplied_sensitivity,
                        ),
                    );
                dynamic
                    .insert_boxed(
                        "mouse_sens",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.mouse_sens),
                    );
                dynamic
                    .insert_boxed(
                        "default_vfov",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.default_vfov),
                    );
                dynamic
                    .insert_boxed(
                        "vfov_target",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.vfov_target),
                    );
                dynamic
                    .insert_boxed(
                        "vfov_smooth",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.vfov_smooth),
                    );
                dynamic
                    .insert_boxed(
                        "zoom_speed",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.zoom_speed),
                    );
                dynamic
                    .insert_boxed(
                        "speed",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.speed),
                    );
                dynamic
                    .insert_boxed(
                        "base_speed",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.base_speed),
                    );
                dynamic
                    .insert_boxed(
                        "max_speed",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.max_speed),
                    );
                dynamic
                    .insert_boxed(
                        "speedup_factor",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.speedup_factor),
                    );
                dynamic
                    .insert_boxed(
                        "fast_multiplier",
                        ::bevy::reflect::PartialReflect::to_dynamic(
                            &self.fast_multiplier,
                        ),
                    );
                dynamic
                    .insert_boxed(
                        "test",
                        ::bevy::reflect::PartialReflect::to_dynamic(&self.test),
                    );
                dynamic
            }
        }
        impl ::bevy::reflect::PartialReflect for Flycam {
            #[inline]
            fn get_represented_type_info(
                &self,
            ) -> ::core::option::Option<&'static ::bevy::reflect::TypeInfo> {
                ::core::option::Option::Some(
                    <Self as ::bevy::reflect::Typed>::type_info(),
                )
            }
            #[inline]
            fn try_apply(
                &mut self,
                value: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::result::Result<(), ::bevy::reflect::ApplyError> {
                if let ::bevy::reflect::ReflectRef::Struct(struct_value) = ::bevy::reflect::PartialReflect::reflect_ref(
                    value,
                ) {
                    for (i, value) in ::core::iter::Iterator::enumerate(
                        ::bevy::reflect::Struct::iter_fields(struct_value),
                    ) {
                        let name = ::bevy::reflect::Struct::name_at(struct_value, i)
                            .unwrap();
                        if let ::core::option::Option::Some(v) = ::bevy::reflect::Struct::field_mut(
                            self,
                            name,
                        ) {
                            ::bevy::reflect::PartialReflect::try_apply(v, value)?;
                        }
                    }
                } else {
                    return ::core::result::Result::Err(::bevy::reflect::ApplyError::MismatchedKinds {
                        from_kind: ::bevy::reflect::PartialReflect::reflect_kind(value),
                        to_kind: ::bevy::reflect::ReflectKind::Struct,
                    });
                }
                ::core::result::Result::Ok(())
            }
            #[inline]
            fn reflect_kind(&self) -> ::bevy::reflect::ReflectKind {
                ::bevy::reflect::ReflectKind::Struct
            }
            #[inline]
            fn reflect_ref(&self) -> ::bevy::reflect::ReflectRef {
                ::bevy::reflect::ReflectRef::Struct(self)
            }
            #[inline]
            fn reflect_mut(&mut self) -> ::bevy::reflect::ReflectMut {
                ::bevy::reflect::ReflectMut::Struct(self)
            }
            #[inline]
            fn reflect_owned(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::ReflectOwned {
                ::bevy::reflect::ReflectOwned::Struct(self)
            }
            #[inline]
            fn try_into_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::core::result::Result<
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::PartialReflect,
                >,
            > {
                ::core::result::Result::Ok(self)
            }
            #[inline]
            fn try_as_reflect(
                &self,
            ) -> ::core::option::Option<&dyn ::bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn try_as_reflect_mut(
                &mut self,
            ) -> ::core::option::Option<&mut dyn ::bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn into_partial_reflect(
                self: ::bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn ::bevy::reflect::PartialReflect,
            > {
                self
            }
            #[inline]
            fn as_partial_reflect(&self) -> &dyn ::bevy::reflect::PartialReflect {
                self
            }
            #[inline]
            fn as_partial_reflect_mut(
                &mut self,
            ) -> &mut dyn ::bevy::reflect::PartialReflect {
                self
            }
            fn reflect_partial_eq(
                &self,
                value: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<bool> {
                (::bevy::reflect::struct_partial_eq)(self, value)
            }
            #[inline]
            #[allow(
                unreachable_code,
                reason = "Ignored fields without a `clone` attribute will early-return with an error"
            )]
            fn reflect_clone(
                &self,
            ) -> ::core::result::Result<
                ::bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn ::bevy::reflect::Reflect,
                >,
                ::bevy::reflect::ReflectCloneError,
            > {
                ::core::result::Result::Ok(
                    ::bevy::reflect::__macro_exports::alloc_utils::Box::new(Self {
                        move_planar: <bool as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.move_planar,
                        )?,
                        vfov_multiplied_sensitivity: <bool as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.vfov_multiplied_sensitivity,
                        )?,
                        mouse_sens: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.mouse_sens,
                        )?,
                        default_vfov: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.default_vfov,
                        )?,
                        vfov_target: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.vfov_target,
                        )?,
                        vfov_smooth: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.vfov_smooth,
                        )?,
                        zoom_speed: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.zoom_speed,
                        )?,
                        speed: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.speed,
                        )?,
                        base_speed: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.base_speed,
                        )?,
                        max_speed: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.max_speed,
                        )?,
                        speedup_factor: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.speedup_factor,
                        )?,
                        fast_multiplier: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.fast_multiplier,
                        )?,
                        test: <f32 as ::bevy::reflect::PartialReflect>::reflect_clone_and_take(
                            &self.test,
                        )?,
                    }),
                )
            }
        }
        impl ::bevy::reflect::FromReflect for Flycam {
            fn from_reflect(
                reflect: &dyn ::bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<Self> {
                if let ::bevy::reflect::ReflectRef::Struct(__ref_struct) = ::bevy::reflect::PartialReflect::reflect_ref(
                    reflect,
                ) {
                    let __this = Self {
                        move_planar: <bool as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "move_planar")?,
                        )?,
                        vfov_multiplied_sensitivity: <bool as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(
                                __ref_struct,
                                "vfov_multiplied_sensitivity",
                            )?,
                        )?,
                        mouse_sens: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "mouse_sens")?,
                        )?,
                        default_vfov: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "default_vfov")?,
                        )?,
                        vfov_target: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "vfov_target")?,
                        )?,
                        vfov_smooth: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "vfov_smooth")?,
                        )?,
                        zoom_speed: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "zoom_speed")?,
                        )?,
                        speed: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "speed")?,
                        )?,
                        base_speed: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "base_speed")?,
                        )?,
                        max_speed: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "max_speed")?,
                        )?,
                        speedup_factor: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(
                                __ref_struct,
                                "speedup_factor",
                            )?,
                        )?,
                        fast_multiplier: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(
                                __ref_struct,
                                "fast_multiplier",
                            )?,
                        )?,
                        test: <f32 as ::bevy::reflect::FromReflect>::from_reflect(
                            ::bevy::reflect::Struct::field(__ref_struct, "test")?,
                        )?,
                    };
                    ::core::option::Option::Some(__this)
                } else {
                    ::core::option::Option::None
                }
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for Flycam {
        #[inline]
        fn clone(&self) -> Flycam {
            Flycam {
                move_planar: ::core::clone::Clone::clone(&self.move_planar),
                vfov_multiplied_sensitivity: ::core::clone::Clone::clone(
                    &self.vfov_multiplied_sensitivity,
                ),
                mouse_sens: ::core::clone::Clone::clone(&self.mouse_sens),
                default_vfov: ::core::clone::Clone::clone(&self.default_vfov),
                vfov_target: ::core::clone::Clone::clone(&self.vfov_target),
                vfov_smooth: ::core::clone::Clone::clone(&self.vfov_smooth),
                zoom_speed: ::core::clone::Clone::clone(&self.zoom_speed),
                speed: ::core::clone::Clone::clone(&self.speed),
                base_speed: ::core::clone::Clone::clone(&self.base_speed),
                max_speed: ::core::clone::Clone::clone(&self.max_speed),
                speedup_factor: ::core::clone::Clone::clone(&self.speedup_factor),
                fast_multiplier: ::core::clone::Clone::clone(&self.fast_multiplier),
                test: ::core::clone::Clone::clone(&self.test),
            }
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Flycam {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Flycam",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "move_planar",
                    &self.move_planar,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "vfov_multiplied_sensitivity",
                    &self.vfov_multiplied_sensitivity,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "mouse_sens",
                    &self.mouse_sens,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "default_vfov",
                    &self.default_vfov,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "vfov_target",
                    &self.vfov_target,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "vfov_smooth",
                    &self.vfov_smooth,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "zoom_speed",
                    &self.zoom_speed,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "speed",
                    &self.speed,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "base_speed",
                    &self.base_speed,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "max_speed",
                    &self.max_speed,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "speedup_factor",
                    &self.speedup_factor,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "fast_multiplier",
                    &self.fast_multiplier,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Flycam {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            2u64 => _serde::__private228::Ok(__Field::__field2),
                            3u64 => _serde::__private228::Ok(__Field::__field3),
                            4u64 => _serde::__private228::Ok(__Field::__field4),
                            5u64 => _serde::__private228::Ok(__Field::__field5),
                            6u64 => _serde::__private228::Ok(__Field::__field6),
                            7u64 => _serde::__private228::Ok(__Field::__field7),
                            8u64 => _serde::__private228::Ok(__Field::__field8),
                            9u64 => _serde::__private228::Ok(__Field::__field9),
                            10u64 => _serde::__private228::Ok(__Field::__field10),
                            11u64 => _serde::__private228::Ok(__Field::__field11),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "move_planar" => _serde::__private228::Ok(__Field::__field0),
                            "vfov_multiplied_sensitivity" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            "mouse_sens" => _serde::__private228::Ok(__Field::__field2),
                            "default_vfov" => _serde::__private228::Ok(__Field::__field3),
                            "vfov_target" => _serde::__private228::Ok(__Field::__field4),
                            "vfov_smooth" => _serde::__private228::Ok(__Field::__field5),
                            "zoom_speed" => _serde::__private228::Ok(__Field::__field6),
                            "speed" => _serde::__private228::Ok(__Field::__field7),
                            "base_speed" => _serde::__private228::Ok(__Field::__field8),
                            "max_speed" => _serde::__private228::Ok(__Field::__field9),
                            "speedup_factor" => {
                                _serde::__private228::Ok(__Field::__field10)
                            }
                            "fast_multiplier" => {
                                _serde::__private228::Ok(__Field::__field11)
                            }
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"move_planar" => _serde::__private228::Ok(__Field::__field0),
                            b"vfov_multiplied_sensitivity" => {
                                _serde::__private228::Ok(__Field::__field1)
                            }
                            b"mouse_sens" => _serde::__private228::Ok(__Field::__field2),
                            b"default_vfov" => {
                                _serde::__private228::Ok(__Field::__field3)
                            }
                            b"vfov_target" => _serde::__private228::Ok(__Field::__field4),
                            b"vfov_smooth" => _serde::__private228::Ok(__Field::__field5),
                            b"zoom_speed" => _serde::__private228::Ok(__Field::__field6),
                            b"speed" => _serde::__private228::Ok(__Field::__field7),
                            b"base_speed" => _serde::__private228::Ok(__Field::__field8),
                            b"max_speed" => _serde::__private228::Ok(__Field::__field9),
                            b"speedup_factor" => {
                                _serde::__private228::Ok(__Field::__field10)
                            }
                            b"fast_multiplier" => {
                                _serde::__private228::Ok(__Field::__field11)
                            }
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<Flycam>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Flycam;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct Flycam",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Flycam with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            bool,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Flycam with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Flycam with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Flycam with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Flycam with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct Flycam with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct Flycam with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field7 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct Flycam with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field8 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        8usize,
                                        &"struct Flycam with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field9 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        9usize,
                                        &"struct Flycam with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field10 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        10usize,
                                        &"struct Flycam with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field11 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        11usize,
                                        &"struct Flycam with 12 elements",
                                    ),
                                );
                            }
                        };
                        let __field12 = _serde::__private228::Default::default();
                        _serde::__private228::Ok(Flycam {
                            move_planar: __field0,
                            vfov_multiplied_sensitivity: __field1,
                            mouse_sens: __field2,
                            default_vfov: __field3,
                            vfov_target: __field4,
                            vfov_smooth: __field5,
                            zoom_speed: __field6,
                            speed: __field7,
                            base_speed: __field8,
                            max_speed: __field9,
                            speedup_factor: __field10,
                            fast_multiplier: __field11,
                            test: __field12,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<bool> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<bool> = _serde::__private228::None;
                        let mut __field2: _serde::__private228::Option<f32> = _serde::__private228::None;
                        let mut __field3: _serde::__private228::Option<f32> = _serde::__private228::None;
                        let mut __field4: _serde::__private228::Option<f32> = _serde::__private228::None;
                        let mut __field5: _serde::__private228::Option<f32> = _serde::__private228::None;
                        let mut __field6: _serde::__private228::Option<f32> = _serde::__private228::None;
                        let mut __field7: _serde::__private228::Option<f32> = _serde::__private228::None;
                        let mut __field8: _serde::__private228::Option<f32> = _serde::__private228::None;
                        let mut __field9: _serde::__private228::Option<f32> = _serde::__private228::None;
                        let mut __field10: _serde::__private228::Option<f32> = _serde::__private228::None;
                        let mut __field11: _serde::__private228::Option<f32> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "move_planar",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "vfov_multiplied_sensitivity",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private228::Option::is_some(&__field2) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "mouse_sens",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private228::Option::is_some(&__field3) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "default_vfov",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private228::Option::is_some(&__field4) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "vfov_target",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private228::Option::is_some(&__field5) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "vfov_smooth",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private228::Option::is_some(&__field6) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "zoom_speed",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private228::Option::is_some(&__field7) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("speed"),
                                        );
                                    }
                                    __field7 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::__private228::Option::is_some(&__field8) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "base_speed",
                                            ),
                                        );
                                    }
                                    __field8 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field9 => {
                                    if _serde::__private228::Option::is_some(&__field9) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "max_speed",
                                            ),
                                        );
                                    }
                                    __field9 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field10 => {
                                    if _serde::__private228::Option::is_some(&__field10) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "speedup_factor",
                                            ),
                                        );
                                    }
                                    __field10 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field11 => {
                                    if _serde::__private228::Option::is_some(&__field11) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "fast_multiplier",
                                            ),
                                        );
                                    }
                                    __field11 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("move_planar")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field(
                                    "vfov_multiplied_sensitivity",
                                )?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private228::Some(__field2) => __field2,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("mouse_sens")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private228::Some(__field3) => __field3,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("default_vfov")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private228::Some(__field4) => __field4,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("vfov_target")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private228::Some(__field5) => __field5,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("vfov_smooth")?
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private228::Some(__field6) => __field6,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("zoom_speed")?
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private228::Some(__field7) => __field7,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("speed")?
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::__private228::Some(__field8) => __field8,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("base_speed")?
                            }
                        };
                        let __field9 = match __field9 {
                            _serde::__private228::Some(__field9) => __field9,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("max_speed")?
                            }
                        };
                        let __field10 = match __field10 {
                            _serde::__private228::Some(__field10) => __field10,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("speedup_factor")?
                            }
                        };
                        let __field11 = match __field11 {
                            _serde::__private228::Some(__field11) => __field11,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("fast_multiplier")?
                            }
                        };
                        _serde::__private228::Ok(Flycam {
                            move_planar: __field0,
                            vfov_multiplied_sensitivity: __field1,
                            mouse_sens: __field2,
                            default_vfov: __field3,
                            vfov_target: __field4,
                            vfov_smooth: __field5,
                            zoom_speed: __field6,
                            speed: __field7,
                            base_speed: __field8,
                            max_speed: __field9,
                            speedup_factor: __field10,
                            fast_multiplier: __field11,
                            test: _serde::__private228::Default::default(),
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "move_planar",
                    "vfov_multiplied_sensitivity",
                    "mouse_sens",
                    "default_vfov",
                    "vfov_target",
                    "vfov_smooth",
                    "zoom_speed",
                    "speed",
                    "base_speed",
                    "max_speed",
                    "speedup_factor",
                    "fast_multiplier",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Flycam",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<Flycam>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    pub struct FlycamBundle {
        pub flycam: Flycam,
        pub transf: Transform,
        pub proj: Projection,
        pub cam3d: Camera3d,
        pub cam: Camera,
    }
    #[allow(deprecated)]
    unsafe impl ::bevy::ecs::bundle::Bundle for FlycamBundle {
        fn component_ids(
            components: &mut ::bevy::ecs::component::ComponentsRegistrator,
            ids: &mut impl FnMut(::bevy::ecs::component::ComponentId),
        ) {
            <Flycam as ::bevy::ecs::bundle::Bundle>::component_ids(components, ids);
            <Transform as ::bevy::ecs::bundle::Bundle>::component_ids(components, ids);
            <Projection as ::bevy::ecs::bundle::Bundle>::component_ids(components, ids);
            <Camera3d as ::bevy::ecs::bundle::Bundle>::component_ids(components, ids);
            <Camera as ::bevy::ecs::bundle::Bundle>::component_ids(components, ids);
        }
        fn get_component_ids(
            components: &::bevy::ecs::component::Components,
            ids: &mut impl FnMut(Option<::bevy::ecs::component::ComponentId>),
        ) {
            <Flycam as ::bevy::ecs::bundle::Bundle>::get_component_ids(
                components,
                &mut *ids,
            );
            <Transform as ::bevy::ecs::bundle::Bundle>::get_component_ids(
                components,
                &mut *ids,
            );
            <Projection as ::bevy::ecs::bundle::Bundle>::get_component_ids(
                components,
                &mut *ids,
            );
            <Camera3d as ::bevy::ecs::bundle::Bundle>::get_component_ids(
                components,
                &mut *ids,
            );
            <Camera as ::bevy::ecs::bundle::Bundle>::get_component_ids(
                components,
                &mut *ids,
            );
        }
    }
    #[allow(deprecated)]
    unsafe impl ::bevy::ecs::bundle::BundleFromComponents for FlycamBundle {
        #[allow(unused_variables, non_snake_case)]
        unsafe fn from_components<__T, __F>(ctx: &mut __T, func: &mut __F) -> Self
        where
            __F: FnMut(&mut __T) -> ::bevy::ecs::ptr::OwningPtr<'_>,
        {
            Self {
                flycam: <Flycam as ::bevy::ecs::bundle::BundleFromComponents>::from_components(
                    ctx,
                    &mut *func,
                ),
                transf: <Transform as ::bevy::ecs::bundle::BundleFromComponents>::from_components(
                    ctx,
                    &mut *func,
                ),
                proj: <Projection as ::bevy::ecs::bundle::BundleFromComponents>::from_components(
                    ctx,
                    &mut *func,
                ),
                cam3d: <Camera3d as ::bevy::ecs::bundle::BundleFromComponents>::from_components(
                    ctx,
                    &mut *func,
                ),
                cam: <Camera as ::bevy::ecs::bundle::BundleFromComponents>::from_components(
                    ctx,
                    &mut *func,
                ),
            }
        }
    }
    impl ::bevy::ecs::bundle::DynamicBundle for FlycamBundle {
        type Effect = ();
        #[allow(unused_variables)]
        #[inline]
        unsafe fn get_components(
            ptr: ::bevy::ecs::ptr::MovingPtr<'_, Self>,
            func: &mut impl FnMut(
                ::bevy::ecs::component::StorageType,
                ::bevy::ecs::ptr::OwningPtr<'_>,
            ),
        ) {
            use ::bevy::ecs::__macro_exports::DebugCheckedUnwrap;
            let mut ptr: ::bevy_ptr::MovingPtr<_, _> = ptr;
            let _ = || {
                let value = &mut *ptr;
                let FlycamBundle { flycam: _, transf: _, proj: _, cam3d: _, cam: _ } = value;
                core::hint::black_box((
                    &mut value.flycam,
                    &mut value.transf,
                    &mut value.proj,
                    &mut value.cam3d,
                    &mut value.cam,
                ));
                let value: *mut _ = value;
                FlycamBundle {
                    ..unsafe { value.read() }
                };
            };
            let field_0 = unsafe { ptr.move_field(|f| &raw mut (*f).flycam) };
            let field_1 = unsafe { ptr.move_field(|f| &raw mut (*f).transf) };
            let field_2 = unsafe { ptr.move_field(|f| &raw mut (*f).proj) };
            let field_3 = unsafe { ptr.move_field(|f| &raw mut (*f).cam3d) };
            let field_4 = unsafe { ptr.move_field(|f| &raw mut (*f).cam) };
            core::mem::forget(ptr);
            <Flycam as ::bevy::ecs::bundle::DynamicBundle>::get_components(
                field_0,
                func,
            );
            <Transform as ::bevy::ecs::bundle::DynamicBundle>::get_components(
                field_1,
                func,
            );
            <Projection as ::bevy::ecs::bundle::DynamicBundle>::get_components(
                field_2,
                func,
            );
            <Camera3d as ::bevy::ecs::bundle::DynamicBundle>::get_components(
                field_3,
                func,
            );
            <Camera as ::bevy::ecs::bundle::DynamicBundle>::get_components(
                field_4,
                func,
            );
        }
        #[allow(unused_variables)]
        #[inline]
        unsafe fn apply_effect(
            ptr: ::bevy::ecs::ptr::MovingPtr<'_, core::mem::MaybeUninit<Self>>,
            func: &mut ::bevy::ecs::world::EntityWorldMut<'_>,
        ) {}
    }
    impl FlycamBundle {
        pub fn new(transf: Transform) -> Self {
            let vfov = 70.0_f32.to_radians();
            Self {
                flycam: Flycam {
                    move_planar: true,
                    vfov_multiplied_sensitivity: true,
                    mouse_sens: 2.0 / 1000.0,
                    default_vfov: vfov,
                    vfov_target: vfov,
                    vfov_smooth: 25.0,
                    zoom_speed: 1.5,
                    speed: 4.0,
                    base_speed: 4.0,
                    max_speed: 1000000.0,
                    speedup_factor: 2.0,
                    fast_multiplier: 4.0,
                    test: 99.0,
                },
                transf,
                proj: Projection::Perspective(PerspectiveProjection {
                    fov: vfov,
                    near: 0.1,
                    far: 10000.0,
                    ..default()
                }),
                cam3d: Camera3d::default(),
                cam: Camera::default(),
            }
        }
    }
    fn get_mouse_scroll_delta(mouse_wheel: &mut MessageReader<MouseWheel>) -> f32 {
        let mut total_lines: f32 = 0.0;
        for event in mouse_wheel.read() {
            total_lines
                += match event.unit {
                    MouseScrollUnit::Line => event.y,
                    MouseScrollUnit::Pixel => event.y / 100.0,
                };
        }
        total_lines
    }
    fn get_mouselook_sensitivity(flycam: &Flycam, proj: &Projection) -> f32 {
        if flycam.vfov_multiplied_sensitivity {
            if let Projection::Perspective(persp) = proj {
                return flycam.mouse_sens * persp.fov;
            }
        }
        return flycam.mouse_sens;
    }
    fn zoom(
        time: &Res<Time>,
        keyboard: &Res<ButtonInput<KeyCode>>,
        mouse_wheel: &mut MessageReader<MouseWheel>,
        flycam: &mut Flycam,
        proj: &mut Projection,
    ) {
        let mut zoom_dir: f32 = 0.0;
        if keyboard.any_pressed([KeyCode::Equal, KeyCode::NumpadAdd]) {
            zoom_dir += 1.0;
        }
        if keyboard.any_pressed([KeyCode::Minus, KeyCode::NumpadSubtract]) {
            zoom_dir -= 1.0;
        }
        let mut zoom_delta = zoom_dir * flycam.zoom_speed * time.delta_secs();
        if zoom_delta == 0.0 {
            zoom_delta = 0.125 * get_mouse_scroll_delta(mouse_wheel);
        }
        if keyboard.pressed(KeyCode::KeyF) {
            let mut fov = flycam.vfov_target;
            fov = 2.0_f32.powf(fov.log2() - zoom_delta);
            let min_vfov = 0.1_f32.to_radians();
            let max_vfov = 170.0_f32.to_radians();
            fov = fov.clamp(min_vfov, max_vfov);
            if keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight])
                && zoom_delta != 0.0
            {
                fov = flycam.default_vfov;
            }
            flycam.vfov_target = fov;
        } else {
            flycam.base_speed = 2.0_f32.powf(flycam.base_speed.log2() + zoom_delta);
        }
        if let Projection::Perspective(persp) = proj {
            persp
                .fov
                .smooth_nudge(
                    &flycam.vfov_target,
                    flycam.vfov_smooth,
                    time.delta_secs(),
                );
        }
    }
    fn mouselook(
        time: &Res<Time>,
        keyboard: &Res<ButtonInput<KeyCode>>,
        mouse: &Res<ButtonInput<MouseButton>>,
        mut mouse_motion: &mut MessageReader<MouseMotion>,
        cursor_options: &CursorOptions,
        transf: &mut Transform,
        flycam: &Flycam,
        proj: &Projection,
    ) {
        let pitch_min = (-90.0_f32 + 5.0).to_radians();
        let pitch_max = (90.0_f32 - 5.0).to_radians();
        let euler = EulerRot::YXZ;
        let (mut yaw, mut pitch, mut roll) = transf.rotation.to_euler(euler);
        if mouse.pressed(MOUSELOOK_BTN)
            || cursor_options.grab_mode != CursorGrabMode::None
        {
            let sens = get_mouselook_sensitivity(flycam, proj);
            for event in mouse_motion.read() {
                yaw -= event.delta.x * sens;
                pitch -= event.delta.y * sens;
            }
        }
        pitch = pitch.clamp(pitch_min, pitch_max);
        let enable_roll = false;
        if enable_roll {
            let mut roll_dir = 0.0_f32;
            let roll_speed = 90_f32.to_radians();
            if keyboard.pressed(KeyCode::KeyQ) {
                roll_dir += 1.0;
            }
            if keyboard.pressed(KeyCode::KeyE) {
                roll_dir -= 1.0;
            }
            roll -= roll_dir * (time.delta_secs() * roll_speed);
        } else {
            roll = 0.0;
        }
        transf.rotation = Quat::from_euler(euler, yaw, pitch, roll);
    }
    fn movement(
        time: &Res<Time>,
        keyboard: &Res<ButtonInput<KeyCode>>,
        transf: &mut Transform,
        flycam: &mut Flycam,
    ) {
        fn get_move3d(keyboard: &Res<ButtonInput<KeyCode>>) -> Vec3 {
            let mut dir_local = Vec3::ZERO;
            if keyboard.pressed(KeyCode::KeyA) {
                dir_local.x -= 1.0;
            }
            if keyboard.pressed(KeyCode::KeyD) {
                dir_local.x += 1.0;
            }
            if keyboard.pressed(KeyCode::KeyS) {
                dir_local.z += 1.0;
            }
            if keyboard.pressed(KeyCode::KeyW) {
                dir_local.z -= 1.0;
            }
            if keyboard.pressed(KeyCode::KeyQ) {
                dir_local.y -= 1.0;
            }
            if keyboard.pressed(KeyCode::KeyE) {
                dir_local.y += 1.0;
            }
            dir_local.normalize_or_zero()
        }
        let dir_local = get_move3d(&keyboard);
        let mut move_speed = dir_local.length();
        if move_speed == 0.0 {
            flycam.speed = flycam.base_speed;
        }
        if keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            move_speed *= flycam.fast_multiplier;
            flycam.speed
                += flycam.base_speed * flycam.speedup_factor * time.delta_secs();
        }
        flycam.speed = flycam.speed.clamp(flycam.base_speed, flycam.max_speed);
        let delta_local = dir_local * (time.delta_secs() * flycam.speed);
        if flycam.move_planar {
            let (yaw, _, _) = transf.rotation.to_euler(EulerRot::YXZ);
            let move_2d = Quat::from_rotation_y(yaw)
                * Vec3::new(delta_local.x, 0.0, delta_local.z);
            transf.translation += move_2d;
            transf.translation.y += delta_local.y;
        } else {
            transf.translation += transf.rotation * delta_local;
        }
    }
    fn update_camera(
        time: Res<Time>,
        keyboard: Res<ButtonInput<KeyCode>>,
        mouse: Res<ButtonInput<MouseButton>>,
        mut mouse_motion: MessageReader<MouseMotion>,
        mut mouse_wheel: MessageReader<MouseWheel>,
        mut cursor_options: Single<&mut CursorOptions>,
        mut query: Query<
            (&mut Transform, &mut Flycam, &Camera, &mut Projection),
            With<Camera3d>,
        >,
    ) {
        let mut cursor_opt = cursor_options.into_inner();
        for (mut transf, mut flycam, cam, mut proj) in &mut query {
            if cam.is_active {
                zoom(&time, &keyboard, &mut mouse_wheel, &mut flycam, proj.as_mut());
                mouselook(
                    &time,
                    &keyboard,
                    &mouse,
                    &mut mouse_motion,
                    &cursor_opt,
                    &mut transf,
                    &mut flycam,
                    &proj,
                );
                movement(&time, &keyboard, &mut transf, &mut flycam);
            }
        }
    }
    fn update_cursor(
        keyboard: Res<ButtonInput<KeyCode>>,
        mouse: Res<ButtonInput<MouseButton>>,
        window: Single<(Entity, &Window)>,
        mut cursor_options: Single<&mut CursorOptions>,
        mut commands: Commands,
    ) {
        let (window_e, window) = *window;
        if !window.focused {
            cursor_options.visible = true;
            cursor_options.grab_mode = CursorGrabMode::None;
            commands.entity(window_e).remove::<CursorIcon>();
            return;
        }
        if keyboard.just_pressed(KeyCode::F2) {
            cursor_options.visible = !cursor_options.visible;
        }
        let mouselook = !cursor_options.visible || mouse.pressed(MOUSELOOK_BTN);
        let was_mouselook = cursor_options.grab_mode != CursorGrabMode::None;
        if mouselook != was_mouselook {
            if mouselook {
                cursor_options.grab_mode = CursorGrabMode::Locked;
                let icon: CursorIcon = SystemCursorIcon::AllScroll.into();
                commands.entity(window_e).insert(icon);
            } else {
                cursor_options.grab_mode = CursorGrabMode::None;
                commands.entity(window_e).remove::<CursorIcon>();
            }
        }
    }
}
mod particles {
    use std::f32::INFINITY;
    use bevy::prelude::*;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;
    use crate::app_control::WindowSettings;
    use crate::flycam::Flycam;
    pub struct ParticlePlugin;
    impl Plugin for ParticlePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, setup_data)
                .add_systems(Update, spawn_particles)
                .add_systems(Update, update_particles.after(spawn_particles));
        }
    }
    struct ParticleSystem {
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
        rng: ChaCha8Rng,
    }
    impl ::bevy::ecs::resource::Resource for ParticleSystem
    where
        Self: Send + Sync + 'static,
    {}
    fn setup_data(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let mesh = meshes.add(Cuboid::new(0.1, 0.1, 0.1));
        let material = materials.add(Color::srgb_u8(60, 255, 70));
        let rng = ChaCha8Rng::seed_from_u64(19878367467711);
        commands
            .insert_resource(ParticleSystem {
                mesh,
                material,
                rng,
            });
    }
    pub struct ParticleEmitter {
        pub spawn_period: f32,
        time_since_last_spawn: f32,
    }
    impl ::bevy::ecs::component::Component for ParticleEmitter
    where
        Self: Send + Sync + 'static,
    {
        const STORAGE_TYPE: ::bevy::ecs::component::StorageType = ::bevy::ecs::component::StorageType::Table;
        type Mutability = ::bevy::ecs::component::Mutable;
        fn register_required_components(
            _requiree: ::bevy::ecs::component::ComponentId,
            required_components: &mut ::bevy::ecs::component::RequiredComponentsRegistrator,
        ) {}
        fn clone_behavior() -> ::bevy::ecs::component::ComponentCloneBehavior {
            use ::bevy::ecs::component::{
                DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
            };
            (&&&::bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                Self,
            >::default())
                .default_clone_behavior()
        }
    }
    impl ParticleEmitter {
        pub fn new(spawn_period: f32) -> Self {
            Self {
                spawn_period,
                time_since_last_spawn: INFINITY,
            }
        }
    }
    struct Particle {
        velocity: Vec3,
    }
    impl ::bevy::ecs::component::Component for Particle
    where
        Self: Send + Sync + 'static,
    {
        const STORAGE_TYPE: ::bevy::ecs::component::StorageType = ::bevy::ecs::component::StorageType::Table;
        type Mutability = ::bevy::ecs::component::Mutable;
        fn register_required_components(
            _requiree: ::bevy::ecs::component::ComponentId,
            required_components: &mut ::bevy::ecs::component::RequiredComponentsRegistrator,
        ) {}
        fn clone_behavior() -> ::bevy::ecs::component::ComponentCloneBehavior {
            use ::bevy::ecs::component::{
                DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
            };
            (&&&::bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                Self,
            >::default())
                .default_clone_behavior()
        }
    }
    fn spawn_particles(
        time: Res<Time>,
        spawners: Query<(&mut ParticleEmitter, &GlobalTransform)>,
        mut sys: ResMut<ParticleSystem>,
        mut commands: Commands,
    ) {
        let emit_speed: f32 = 7.0;
        let speed_variation: f32 = 1.0;
        for (mut spawner, transform) in spawners {
            spawner.time_since_last_spawn += time.delta_secs();
            if spawner.time_since_last_spawn >= spawner.spawn_period {
                spawner.time_since_last_spawn -= spawner.spawn_period;
                let var = Vec3::new(
                    sys.rng.random_range(-1.0..1.0),
                    sys.rng.random_range(-1.0..1.0),
                    sys.rng.random_range(-1.0..1.0),
                );
                commands
                    .spawn((
                        Mesh3d(sys.mesh.clone()),
                        MeshMaterial3d(sys.material.clone()),
                        Transform {
                            translation: transform.translation(),
                            rotation: transform.rotation(),
                            ..Default::default()
                        },
                        Particle {
                            velocity: -transform.forward() * emit_speed
                                + var * speed_variation,
                        },
                    ));
            }
        }
    }
    fn update_particles(
        time: Res<Time>,
        particles: Query<(Entity, &mut Particle, &mut Transform)>,
        mut commands: Commands,
    ) {
        let gravity: f32 = -10.0;
        for (e, mut particle, mut transform) in particles {
            particle.velocity += Vec3::new(0.0, gravity, 0.0) * time.delta_secs();
            transform.translation += particle.velocity * time.delta_secs();
            if transform.translation.y < 0.0 {
                commands.entity(e).despawn();
            }
        }
    }
}
use bevy::{
    prelude::*, ecs::name::*, ecs::query::QueryFilter,
    ecs::schedule::{ScheduleBuildSettings, LogLevel},
    render::*, render::settings::Backends, camera::*, scene::SceneInstanceReady,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::f32::consts::*;
use bevy_egui::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use debug_camera::{DebugCamera, MainCamera};
use flycam::FlycamBundle;
fn main() {
    let mut app = App::new();
    app.configure_schedules(ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Error,
        ..default()
    });
    let asset_path = std::env::current_dir()
        .unwrap()
        .join("assets")
        .to_string_lossy()
        .to_string();
    {
        ::std::io::_print(
            format_args!("Working directory: {0:?}\n", std::env::current_dir().unwrap()),
        );
    };
    {
        ::std::io::_print(
            format_args!("Exe path: {0:?}\n", std::env::current_exe().unwrap()),
        );
    };
    {
        ::std::io::_print(format_args!("Asset path: {0:?}\n", asset_path));
    };
    let settings = settings_file::early_load_settings();
    let settings2 = settings.clone();
    app.add_plugins({
        let mut plugins = DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: app_control::APP_NAME.into(),
                    name: Some(app_control::APP_NAME.into()),
                    resolution: (1152, 720).into(),
                    resize_constraints: WindowResizeConstraints {
                        min_width: 100.0,
                        min_height: 100.0,
                        ..default()
                    },
                    resizable: true,
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                render_creation: settings::RenderCreation::Automatic({
                    let mut set = settings::WgpuSettings::default();
                    if let Some(s) = &settings {
                        set.backends = Some(
                            Backends::from_comma_list(s.render.backends.as_str()),
                        );
                        if s.render.disable_validation_in_debug {
                            set.instance_flags = bevy::render::settings::InstanceFlags::empty();
                        }
                    }
                    set
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: asset_path,
                ..default()
            });
        plugins
    });
    app.insert_resource(EguiGlobalSettings {
        auto_create_primary_context: false,
        ..default()
    });
    app.add_plugins((EguiPlugin::default(), WorldInspectorPlugin::new()));
    app.add_plugins((
        app_control::AppControlPlugin,
        debug_camera::DebugCameraPlugin,
        flycam::FlycamPlugin,
        particles::ParticlePlugin,
    ));
    app.add_observer(do_very_specific_thing_to_object);
    app.add_systems(
        Startup,
        (
            startup,
            spawn_animated_gltf,
            (move |world: &mut World| {
                settings_file::load_settings(world, settings.clone());
            })
                .after(startup),
        ),
    );
    app.add_systems(Update, (update_animation,));
    phases::update_schedule_configs(&mut app);
    app.run();
}
fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);
    let cube_mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
    let blue = materials.add(Color::srgb_u8(124, 144, 255));
    let red = materials.add(Color::srgb_u8(255, 144, 124));
    commands
        .spawn((
            PrimaryEguiContext,
            Camera3d::default(),
            Camera {
                order: 10,
                output_mode: CameraOutputMode::Write {
                    blend_state: Some(render_resource::BlendState::ALPHA_BLENDING),
                    clear_color: ClearColorConfig::None,
                },
                clear_color: ClearColorConfig::Custom(Color::NONE),
                ..default()
            },
            Name::new("EguiCamera"),
        ));
    commands
        .spawn((
            MainCamera,
            flycam::FlycamBundle::new(
                Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ),
            bevy::render::view::Hdr,
            bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
            bevy::post_process::bloom::Bloom::NATURAL,
            Name::new("MainCamera"),
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(red.clone()),
        ));
    commands
        .spawn((
            DebugCamera,
            flycam::FlycamBundle::new(
                Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ),
            bevy::render::view::Hdr,
            bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface,
            bevy::post_process::bloom::Bloom::NATURAL,
            Name::new("DebugFlycam"),
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(red.clone()),
        ));
    commands
        .spawn((
            Transform::from_xyz(0.0, 0.0, 0.0)
                .looking_at(Vec3::new(1.0, -6.0, 3.0), Vec3::Y),
            DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            Name::new("Light"),
        ));
    commands
        .spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.4, 0.55, 0.6))),
            Name::new("Ground Plane"),
        ));
    commands
        .spawn_batch(
            std::iter::repeat_with(move || {
                    let x = rng.random_range(-5.0..5.0);
                    let y = rng.random_range(0.0..3.0);
                    let z = rng.random_range(-5.0..5.0);
                    (
                        Mesh3d(cube_mesh.clone()),
                        MeshMaterial3d(blue.clone()),
                        Transform::from_xyz(x, y, z),
                        Name::new("Cube"),
                    )
                })
                .take(10),
        );
}
struct ThisVerySpecificObject();
impl ::bevy::ecs::component::Component for ThisVerySpecificObject
where
    Self: Send + Sync + 'static,
{
    const STORAGE_TYPE: ::bevy::ecs::component::StorageType = ::bevy::ecs::component::StorageType::Table;
    type Mutability = ::bevy::ecs::component::Mutable;
    fn register_required_components(
        _requiree: ::bevy::ecs::component::ComponentId,
        required_components: &mut ::bevy::ecs::component::RequiredComponentsRegistrator,
    ) {}
    fn clone_behavior() -> ::bevy::ecs::component::ComponentCloneBehavior {
        use ::bevy::ecs::component::{
            DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
        };
        (&&&::bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
            Self,
        >::default())
            .default_clone_behavior()
    }
}
fn spawn_animated_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            SceneRoot(asset_server.load("rig.glb#Scene0")),
            ThisVerySpecificObject(),
        ));
}
fn do_very_specific_thing_to_object(
    scene_ready: On<SceneInstanceReady>,
    q_children: Query<&Children>,
    mut q_skinned_mesh: Query<
        (&bevy::mesh::skinning::SkinnedMesh, &mut MeshMaterial3d<StandardMaterial>),
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let material = materials.add(Color::srgb_u8(255, 144, 50));
    let scene_root = scene_ready.entity;
    for entity in q_children.iter_descendants(scene_root) {
        if let Ok((skinned_mesh, mut mat)) = q_skinned_mesh.get_mut(entity) {
            *mat = MeshMaterial3d(material.clone());
            commands
                .entity(skinned_mesh.joints[2])
                .insert(particles::ParticleEmitter::new(0.2));
        }
    }
}
fn spin_camera(time: Res<Time>, mut query: Query<&mut Transform, With<Camera3d>>) {
    for mut transf in &mut query {
        transf.rotate_around(Vec3::ZERO, Quat::from_rotation_y(1.0 * time.delta_secs()));
    }
}
fn update_animation(
    time: Res<Time>,
    animated_entities: Query<&bevy::mesh::skinning::SkinnedMesh>,
    mut transform_query: Query<&mut Transform>,
) {
    for animated in &animated_entities {
        let second_joint_entity = animated.joints[1];
        let mut second_joint_transform = transform_query
            .get_mut(second_joint_entity)
            .unwrap();
        second_joint_transform.rotation = Quat::from_rotation_z(
            FRAC_PI_2 * ops::sin(time.elapsed_secs() * 3.0),
        );
    }
}
