#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
fn main() {
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
}
mod schema {
    mod feature {
        use serde::{Deserialize, Serialize};
        pub struct Feature {
            /// Passes docker capabilities to include when creating the dev container.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "capAdd")]
            pub cap_add: Option<Vec<String>>,
            /// Container environment variables.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "containerEnv")]
            pub container_env: Option<::std::collections::BTreeMap<String, String>>,
            /// Tool-specific configuration. Each tool should use a JSON object subproperty with a unique
            /// name to group its customizations.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub customizations: Option<
                ::std::collections::BTreeMap<String, serde_json::Value>,
            >,
            /// Indicates that the Feature is deprecated, and will not receive any further updates/support.
            /// This property is intended to be used by the supporting tools for highlighting Feature
            /// deprecation.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub deprecated: Option<bool>,
            /// Description of the Feature. For the best appearance in an implementing tool, refrain from
            /// including markdown or HTML in the description.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub description: Option<String>,
            /// URL to documentation for the Feature.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "documentationURL")]
            pub documentation_url: Option<String>,
            /// Entrypoint script that should fire at container start up.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub entrypoint: Option<String>,
            /// ID of the Feature. The id should be unique in the context of the repository/published
            /// package where the feature exists and must match the name of the directory where the
            /// devcontainer-feature.json resides.
            pub id: String,
            /// Adds the tiny init process to the container (--init) when the Feature is used.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub init: Option<bool>,
            /// Array of ID's of Features that should execute before this one. Allows control for feature
            /// authors on soft dependencies between different Features.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "installsAfter")]
            pub installs_after: Option<Vec<String>>,
            /// Array of old IDs used to publish this Feature. The property is useful for renaming a
            /// currently published Feature within a single namespace.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "legacyIds")]
            pub legacy_ids: Option<Vec<String>>,
            /// URL to the license for the Feature.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "licenseURL")]
            pub license_url: Option<String>,
            /// Mounts a volume or bind mount into the container.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mounts: Option<Vec<Mount>>,
            /// Display name of the Feature.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: Option<String>,
            /// A command to run when creating the container. This command is run after "initializeCommand"
            /// and before "updateContentCommand". If this is a single string, it will be run in a shell.
            /// If this is an array of strings, it will be run as a single command without shell. If this
            /// is an object, each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "onCreateCommand")]
            pub on_create_command: Option<serde_json::Value>,
            /// Possible user-configurable options for this Feature. The selected options will be passed as
            /// environment variables when installing the Feature into the container.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub options: Option<::std::collections::BTreeMap<String, FeatureOption>>,
            /// A command to run when attaching to the container. This command is run after
            /// "postStartCommand". If this is a single string, it will be run in a shell. If this is an
            /// array of strings, it will be run as a single command without shell. If this is an object,
            /// each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "postAttachCommand")]
            pub post_attach_command: Option<serde_json::Value>,
            /// A command to run after creating the container. This command is run after
            /// "updateContentCommand" and before "postStartCommand". If this is a single string, it will
            /// be run in a shell. If this is an array of strings, it will be run as a single command
            /// without shell. If this is an object, each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "postCreateCommand")]
            pub post_create_command: Option<serde_json::Value>,
            /// A command to run after starting the container. This command is run after
            /// "postCreateCommand" and before "postAttachCommand". If this is a single string, it will be
            /// run in a shell. If this is an array of strings, it will be run as a single command without
            /// shell. If this is an object, each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "postStartCommand")]
            pub post_start_command: Option<serde_json::Value>,
            /// Sets privileged mode (--privileged) for the container.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub privileged: Option<bool>,
            /// Sets container security options to include when creating the container.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "securityOpt")]
            pub security_opt: Option<Vec<String>>,
            /// A command to run when creating the container and rerun when the workspace content was
            /// updated while creating the container. This command is run after "onCreateCommand" and
            /// before "postCreateCommand". If this is a single string, it will be run in a shell. If this
            /// is an array of strings, it will be run as a single command without shell. If this is an
            /// object, each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "updateContentCommand")]
            pub update_content_command: Option<serde_json::Value>,
            /// The version of the Feature. Follows the semanatic versioning (semver) specification.
            pub version: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Feature {
            #[inline]
            fn clone(&self) -> Feature {
                Feature {
                    cap_add: ::core::clone::Clone::clone(&self.cap_add),
                    container_env: ::core::clone::Clone::clone(&self.container_env),
                    customizations: ::core::clone::Clone::clone(&self.customizations),
                    deprecated: ::core::clone::Clone::clone(&self.deprecated),
                    description: ::core::clone::Clone::clone(&self.description),
                    documentation_url: ::core::clone::Clone::clone(
                        &self.documentation_url,
                    ),
                    entrypoint: ::core::clone::Clone::clone(&self.entrypoint),
                    id: ::core::clone::Clone::clone(&self.id),
                    init: ::core::clone::Clone::clone(&self.init),
                    installs_after: ::core::clone::Clone::clone(&self.installs_after),
                    legacy_ids: ::core::clone::Clone::clone(&self.legacy_ids),
                    license_url: ::core::clone::Clone::clone(&self.license_url),
                    mounts: ::core::clone::Clone::clone(&self.mounts),
                    name: ::core::clone::Clone::clone(&self.name),
                    on_create_command: ::core::clone::Clone::clone(
                        &self.on_create_command,
                    ),
                    options: ::core::clone::Clone::clone(&self.options),
                    post_attach_command: ::core::clone::Clone::clone(
                        &self.post_attach_command,
                    ),
                    post_create_command: ::core::clone::Clone::clone(
                        &self.post_create_command,
                    ),
                    post_start_command: ::core::clone::Clone::clone(
                        &self.post_start_command,
                    ),
                    privileged: ::core::clone::Clone::clone(&self.privileged),
                    security_opt: ::core::clone::Clone::clone(&self.security_opt),
                    update_content_command: ::core::clone::Clone::clone(
                        &self.update_content_command,
                    ),
                    version: ::core::clone::Clone::clone(&self.version),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Feature {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Feature {
            #[inline]
            fn eq(&self, other: &Feature) -> bool {
                self.cap_add == other.cap_add
                    && self.container_env == other.container_env
                    && self.customizations == other.customizations
                    && self.deprecated == other.deprecated
                    && self.description == other.description
                    && self.documentation_url == other.documentation_url
                    && self.entrypoint == other.entrypoint && self.id == other.id
                    && self.init == other.init
                    && self.installs_after == other.installs_after
                    && self.legacy_ids == other.legacy_ids
                    && self.license_url == other.license_url
                    && self.mounts == other.mounts && self.name == other.name
                    && self.on_create_command == other.on_create_command
                    && self.options == other.options
                    && self.post_attach_command == other.post_attach_command
                    && self.post_create_command == other.post_create_command
                    && self.post_start_command == other.post_start_command
                    && self.privileged == other.privileged
                    && self.security_opt == other.security_opt
                    && self.update_content_command == other.update_content_command
                    && self.version == other.version
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Feature {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "cap_add",
                    "container_env",
                    "customizations",
                    "deprecated",
                    "description",
                    "documentation_url",
                    "entrypoint",
                    "id",
                    "init",
                    "installs_after",
                    "legacy_ids",
                    "license_url",
                    "mounts",
                    "name",
                    "on_create_command",
                    "options",
                    "post_attach_command",
                    "post_create_command",
                    "post_start_command",
                    "privileged",
                    "security_opt",
                    "update_content_command",
                    "version",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.cap_add,
                    &self.container_env,
                    &self.customizations,
                    &self.deprecated,
                    &self.description,
                    &self.documentation_url,
                    &self.entrypoint,
                    &self.id,
                    &self.init,
                    &self.installs_after,
                    &self.legacy_ids,
                    &self.license_url,
                    &self.mounts,
                    &self.name,
                    &self.on_create_command,
                    &self.options,
                    &self.post_attach_command,
                    &self.post_create_command,
                    &self.post_start_command,
                    &self.privileged,
                    &self.security_opt,
                    &self.update_content_command,
                    &&self.version,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Feature",
                    names,
                    values,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Feature {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
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
                        __field12,
                        __field13,
                        __field14,
                        __field15,
                        __field16,
                        __field17,
                        __field18,
                        __field19,
                        __field20,
                        __field21,
                        __field22,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                10u64 => _serde::__private::Ok(__Field::__field10),
                                11u64 => _serde::__private::Ok(__Field::__field11),
                                12u64 => _serde::__private::Ok(__Field::__field12),
                                13u64 => _serde::__private::Ok(__Field::__field13),
                                14u64 => _serde::__private::Ok(__Field::__field14),
                                15u64 => _serde::__private::Ok(__Field::__field15),
                                16u64 => _serde::__private::Ok(__Field::__field16),
                                17u64 => _serde::__private::Ok(__Field::__field17),
                                18u64 => _serde::__private::Ok(__Field::__field18),
                                19u64 => _serde::__private::Ok(__Field::__field19),
                                20u64 => _serde::__private::Ok(__Field::__field20),
                                21u64 => _serde::__private::Ok(__Field::__field21),
                                22u64 => _serde::__private::Ok(__Field::__field22),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "capAdd" => _serde::__private::Ok(__Field::__field0),
                                "containerEnv" => _serde::__private::Ok(__Field::__field1),
                                "customizations" => _serde::__private::Ok(__Field::__field2),
                                "deprecated" => _serde::__private::Ok(__Field::__field3),
                                "description" => _serde::__private::Ok(__Field::__field4),
                                "documentationURL" => {
                                    _serde::__private::Ok(__Field::__field5)
                                }
                                "entrypoint" => _serde::__private::Ok(__Field::__field6),
                                "id" => _serde::__private::Ok(__Field::__field7),
                                "init" => _serde::__private::Ok(__Field::__field8),
                                "installsAfter" => _serde::__private::Ok(__Field::__field9),
                                "legacyIds" => _serde::__private::Ok(__Field::__field10),
                                "licenseURL" => _serde::__private::Ok(__Field::__field11),
                                "mounts" => _serde::__private::Ok(__Field::__field12),
                                "name" => _serde::__private::Ok(__Field::__field13),
                                "onCreateCommand" => {
                                    _serde::__private::Ok(__Field::__field14)
                                }
                                "options" => _serde::__private::Ok(__Field::__field15),
                                "postAttachCommand" => {
                                    _serde::__private::Ok(__Field::__field16)
                                }
                                "postCreateCommand" => {
                                    _serde::__private::Ok(__Field::__field17)
                                }
                                "postStartCommand" => {
                                    _serde::__private::Ok(__Field::__field18)
                                }
                                "privileged" => _serde::__private::Ok(__Field::__field19),
                                "securityOpt" => _serde::__private::Ok(__Field::__field20),
                                "updateContentCommand" => {
                                    _serde::__private::Ok(__Field::__field21)
                                }
                                "version" => _serde::__private::Ok(__Field::__field22),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"capAdd" => _serde::__private::Ok(__Field::__field0),
                                b"containerEnv" => _serde::__private::Ok(__Field::__field1),
                                b"customizations" => {
                                    _serde::__private::Ok(__Field::__field2)
                                }
                                b"deprecated" => _serde::__private::Ok(__Field::__field3),
                                b"description" => _serde::__private::Ok(__Field::__field4),
                                b"documentationURL" => {
                                    _serde::__private::Ok(__Field::__field5)
                                }
                                b"entrypoint" => _serde::__private::Ok(__Field::__field6),
                                b"id" => _serde::__private::Ok(__Field::__field7),
                                b"init" => _serde::__private::Ok(__Field::__field8),
                                b"installsAfter" => _serde::__private::Ok(__Field::__field9),
                                b"legacyIds" => _serde::__private::Ok(__Field::__field10),
                                b"licenseURL" => _serde::__private::Ok(__Field::__field11),
                                b"mounts" => _serde::__private::Ok(__Field::__field12),
                                b"name" => _serde::__private::Ok(__Field::__field13),
                                b"onCreateCommand" => {
                                    _serde::__private::Ok(__Field::__field14)
                                }
                                b"options" => _serde::__private::Ok(__Field::__field15),
                                b"postAttachCommand" => {
                                    _serde::__private::Ok(__Field::__field16)
                                }
                                b"postCreateCommand" => {
                                    _serde::__private::Ok(__Field::__field17)
                                }
                                b"postStartCommand" => {
                                    _serde::__private::Ok(__Field::__field18)
                                }
                                b"privileged" => _serde::__private::Ok(__Field::__field19),
                                b"securityOpt" => _serde::__private::Ok(__Field::__field20),
                                b"updateContentCommand" => {
                                    _serde::__private::Ok(__Field::__field21)
                                }
                                b"version" => _serde::__private::Ok(__Field::__field22),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Feature>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Feature;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Feature",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<::std::collections::BTreeMap<String, String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<
                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                >,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field10 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            10usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field11 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            11usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field12 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<Mount>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            12usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field13 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            13usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field14 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            14usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field15 = match match _serde::de::SeqAccess::next_element::<
                                Option<::std::collections::BTreeMap<String, FeatureOption>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            15usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field16 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            16usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field17 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            17usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field18 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            18usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field19 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            19usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field20 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            20usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field21 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            21usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            let __field22 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            22usize,
                                            &"struct Feature with 23 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Feature {
                                cap_add: __field0,
                                container_env: __field1,
                                customizations: __field2,
                                deprecated: __field3,
                                description: __field4,
                                documentation_url: __field5,
                                entrypoint: __field6,
                                id: __field7,
                                init: __field8,
                                installs_after: __field9,
                                legacy_ids: __field10,
                                license_url: __field11,
                                mounts: __field12,
                                name: __field13,
                                on_create_command: __field14,
                                options: __field15,
                                post_attach_command: __field16,
                                post_create_command: __field17,
                                post_start_command: __field18,
                                privileged: __field19,
                                security_opt: __field20,
                                update_content_command: __field21,
                                version: __field22,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<Vec<String>>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<::std::collections::BTreeMap<String, String>>,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<
                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                >,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field9: _serde::__private::Option<
                                Option<Vec<String>>,
                            > = _serde::__private::None;
                            let mut __field10: _serde::__private::Option<
                                Option<Vec<String>>,
                            > = _serde::__private::None;
                            let mut __field11: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field12: _serde::__private::Option<
                                Option<Vec<Mount>>,
                            > = _serde::__private::None;
                            let mut __field13: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field14: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field15: _serde::__private::Option<
                                Option<::std::collections::BTreeMap<String, FeatureOption>>,
                            > = _serde::__private::None;
                            let mut __field16: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field17: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field18: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field19: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field20: _serde::__private::Option<
                                Option<Vec<String>>,
                            > = _serde::__private::None;
                            let mut __field21: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field22: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("capAdd"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "containerEnv",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<::std::collections::BTreeMap<String, String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "customizations",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<
                                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                                >,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "deprecated",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "description",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "documentationURL",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "entrypoint",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("init"),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "installsAfter",
                                                ),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field10 => {
                                        if _serde::__private::Option::is_some(&__field10) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "legacyIds",
                                                ),
                                            );
                                        }
                                        __field10 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field11 => {
                                        if _serde::__private::Option::is_some(&__field11) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "licenseURL",
                                                ),
                                            );
                                        }
                                        __field11 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field12 => {
                                        if _serde::__private::Option::is_some(&__field12) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("mounts"),
                                            );
                                        }
                                        __field12 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<Mount>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field13 => {
                                        if _serde::__private::Option::is_some(&__field13) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field13 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field14 => {
                                        if _serde::__private::Option::is_some(&__field14) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "onCreateCommand",
                                                ),
                                            );
                                        }
                                        __field14 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field15 => {
                                        if _serde::__private::Option::is_some(&__field15) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "options",
                                                ),
                                            );
                                        }
                                        __field15 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<::std::collections::BTreeMap<String, FeatureOption>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field16 => {
                                        if _serde::__private::Option::is_some(&__field16) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "postAttachCommand",
                                                ),
                                            );
                                        }
                                        __field16 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field17 => {
                                        if _serde::__private::Option::is_some(&__field17) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "postCreateCommand",
                                                ),
                                            );
                                        }
                                        __field17 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field18 => {
                                        if _serde::__private::Option::is_some(&__field18) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "postStartCommand",
                                                ),
                                            );
                                        }
                                        __field18 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field19 => {
                                        if _serde::__private::Option::is_some(&__field19) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "privileged",
                                                ),
                                            );
                                        }
                                        __field19 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field20 => {
                                        if _serde::__private::Option::is_some(&__field20) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "securityOpt",
                                                ),
                                            );
                                        }
                                        __field20 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field21 => {
                                        if _serde::__private::Option::is_some(&__field21) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "updateContentCommand",
                                                ),
                                            );
                                        }
                                        __field21 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field22 => {
                                        if _serde::__private::Option::is_some(&__field22) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "version",
                                                ),
                                            );
                                        }
                                        __field22 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("capAdd") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("containerEnv") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "customizations",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("deprecated") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("description") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "documentationURL",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("entrypoint") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("init") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "installsAfter",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field10 = match __field10 {
                                _serde::__private::Some(__field10) => __field10,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("legacyIds") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field11 = match __field11 {
                                _serde::__private::Some(__field11) => __field11,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("licenseURL") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field12 = match __field12 {
                                _serde::__private::Some(__field12) => __field12,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("mounts") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field13 = match __field13 {
                                _serde::__private::Some(__field13) => __field13,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("name") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field14 = match __field14 {
                                _serde::__private::Some(__field14) => __field14,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "onCreateCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field15 = match __field15 {
                                _serde::__private::Some(__field15) => __field15,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("options") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field16 = match __field16 {
                                _serde::__private::Some(__field16) => __field16,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "postAttachCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field17 = match __field17 {
                                _serde::__private::Some(__field17) => __field17,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "postCreateCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field18 = match __field18 {
                                _serde::__private::Some(__field18) => __field18,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "postStartCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field19 = match __field19 {
                                _serde::__private::Some(__field19) => __field19,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("privileged") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field20 = match __field20 {
                                _serde::__private::Some(__field20) => __field20,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("securityOpt") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field21 = match __field21 {
                                _serde::__private::Some(__field21) => __field21,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "updateContentCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field22 = match __field22 {
                                _serde::__private::Some(__field22) => __field22,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("version") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Feature {
                                cap_add: __field0,
                                container_env: __field1,
                                customizations: __field2,
                                deprecated: __field3,
                                description: __field4,
                                documentation_url: __field5,
                                entrypoint: __field6,
                                id: __field7,
                                init: __field8,
                                installs_after: __field9,
                                legacy_ids: __field10,
                                license_url: __field11,
                                mounts: __field12,
                                name: __field13,
                                on_create_command: __field14,
                                options: __field15,
                                post_attach_command: __field16,
                                post_create_command: __field17,
                                post_start_command: __field18,
                                privileged: __field19,
                                security_opt: __field20,
                                update_content_command: __field21,
                                version: __field22,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "capAdd",
                        "containerEnv",
                        "customizations",
                        "deprecated",
                        "description",
                        "documentationURL",
                        "entrypoint",
                        "id",
                        "init",
                        "installsAfter",
                        "legacyIds",
                        "licenseURL",
                        "mounts",
                        "name",
                        "onCreateCommand",
                        "options",
                        "postAttachCommand",
                        "postCreateCommand",
                        "postStartCommand",
                        "privileged",
                        "securityOpt",
                        "updateContentCommand",
                        "version",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Feature",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Feature>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Feature {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Feature",
                        false as usize
                            + if Option::is_none(&self.cap_add) { 0 } else { 1 }
                            + if Option::is_none(&self.container_env) { 0 } else { 1 }
                            + if Option::is_none(&self.customizations) { 0 } else { 1 }
                            + if Option::is_none(&self.deprecated) { 0 } else { 1 }
                            + if Option::is_none(&self.description) { 0 } else { 1 }
                            + if Option::is_none(&self.documentation_url) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.entrypoint) { 0 } else { 1 } + 1
                            + if Option::is_none(&self.init) { 0 } else { 1 }
                            + if Option::is_none(&self.installs_after) { 0 } else { 1 }
                            + if Option::is_none(&self.legacy_ids) { 0 } else { 1 }
                            + if Option::is_none(&self.license_url) { 0 } else { 1 }
                            + if Option::is_none(&self.mounts) { 0 } else { 1 }
                            + if Option::is_none(&self.name) { 0 } else { 1 }
                            + if Option::is_none(&self.on_create_command) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.options) { 0 } else { 1 }
                            + if Option::is_none(&self.post_attach_command) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.post_create_command) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.post_start_command) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.privileged) { 0 } else { 1 }
                            + if Option::is_none(&self.security_opt) { 0 } else { 1 }
                            + if Option::is_none(&self.update_content_command) {
                                0
                            } else {
                                1
                            } + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.cap_add) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "capAdd",
                            &self.cap_add,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "capAdd",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.container_env) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "containerEnv",
                            &self.container_env,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "containerEnv",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.customizations) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "customizations",
                            &self.customizations,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "customizations",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.deprecated) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "deprecated",
                            &self.deprecated,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "deprecated",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.description) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "description",
                            &self.description,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "description",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.documentation_url) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "documentationURL",
                            &self.documentation_url,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "documentationURL",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.entrypoint) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "entrypoint",
                            &self.entrypoint,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "entrypoint",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.init) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "init",
                            &self.init,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "init",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.installs_after) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "installsAfter",
                            &self.installs_after,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "installsAfter",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.legacy_ids) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "legacyIds",
                            &self.legacy_ids,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "legacyIds",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.license_url) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "licenseURL",
                            &self.license_url,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "licenseURL",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.mounts) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "mounts",
                            &self.mounts,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "mounts",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.name) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "name",
                            &self.name,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "name",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.on_create_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "onCreateCommand",
                            &self.on_create_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "onCreateCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.options) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "options",
                            &self.options,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "options",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.post_attach_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "postAttachCommand",
                            &self.post_attach_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "postAttachCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.post_create_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "postCreateCommand",
                            &self.post_create_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "postCreateCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.post_start_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "postStartCommand",
                            &self.post_start_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "postStartCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.privileged) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "privileged",
                            &self.privileged,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "privileged",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.security_opt) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "securityOpt",
                            &self.security_opt,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "securityOpt",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.update_content_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "updateContentCommand",
                            &self.update_content_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "updateContentCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "version",
                        &self.version,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub type FeatureOption = serde_json::Value;
        /// Mounts a volume or bind mount into the container.
        pub struct Mount {
            /// Mount source.
            pub source: String,
            /// Mount target.
            pub target: String,
            /// Type of mount. Can be 'bind' or 'volume'.
            #[serde(rename = "type")]
            pub type_: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Mount {
            #[inline]
            fn clone(&self) -> Mount {
                Mount {
                    source: ::core::clone::Clone::clone(&self.source),
                    target: ::core::clone::Clone::clone(&self.target),
                    type_: ::core::clone::Clone::clone(&self.type_),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Mount {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Mount {
            #[inline]
            fn eq(&self, other: &Mount) -> bool {
                self.source == other.source && self.target == other.target
                    && self.type_ == other.type_
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Mount {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Mount",
                    "source",
                    &self.source,
                    "target",
                    &self.target,
                    "type_",
                    &&self.type_,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Mount {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "source" => _serde::__private::Ok(__Field::__field0),
                                "target" => _serde::__private::Ok(__Field::__field1),
                                "type" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"source" => _serde::__private::Ok(__Field::__field0),
                                b"target" => _serde::__private::Ok(__Field::__field1),
                                b"type" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Mount>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Mount;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Mount",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Mount with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Mount with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Mount with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Mount {
                                source: __field0,
                                target: __field1,
                                type_: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("source"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("target"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("source") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("target") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("type") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Mount {
                                source: __field0,
                                target: __field1,
                                type_: __field2,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "source",
                        "target",
                        "type",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Mount",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Mount>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Mount {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Mount",
                        false as usize + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "source",
                        &self.source,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "target",
                        &self.target,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "type",
                        &self.type_,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub type DevContainerFeature = serde_json::Value;
    }
    mod base {
        use serde::{Deserialize, Serialize};
        pub struct Mount {
            /// Mount source.
            pub source: String,
            /// Mount target.
            pub target: String,
            /// Mount type.
            #[serde(rename = "type")]
            pub type_: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Mount {
            #[inline]
            fn clone(&self) -> Mount {
                Mount {
                    source: ::core::clone::Clone::clone(&self.source),
                    target: ::core::clone::Clone::clone(&self.target),
                    type_: ::core::clone::Clone::clone(&self.type_),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Mount {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Mount {
            #[inline]
            fn eq(&self, other: &Mount) -> bool {
                self.source == other.source && self.target == other.target
                    && self.type_ == other.type_
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Mount {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Mount",
                    "source",
                    &self.source,
                    "target",
                    &self.target,
                    "type_",
                    &&self.type_,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Mount {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "source" => _serde::__private::Ok(__Field::__field0),
                                "target" => _serde::__private::Ok(__Field::__field1),
                                "type" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"source" => _serde::__private::Ok(__Field::__field0),
                                b"target" => _serde::__private::Ok(__Field::__field1),
                                b"type" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Mount>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Mount;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Mount",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Mount with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Mount with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Mount with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Mount {
                                source: __field0,
                                target: __field1,
                                type_: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("source"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("target"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("type"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("source") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("target") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("type") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Mount {
                                source: __field0,
                                target: __field1,
                                type_: __field2,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "source",
                        "target",
                        "type",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Mount",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Mount>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Mount {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Mount",
                        false as usize + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "source",
                        &self.source,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "target",
                        &self.target,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "type",
                        &self.type_,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[serde(rename = "buildOptions")]
        pub struct BuildOptions {
            /// Build arguments.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub args: Option<::std::collections::BTreeMap<String, String>>,
            /// The image to consider as a cache. Use an array to specify multiple images.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "cacheFrom")]
            pub cache_from: Option<serde_json::Value>,
            /// Target stage in a multi-stage build.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub target: Option<String>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for BuildOptions {
            #[inline]
            fn clone(&self) -> BuildOptions {
                BuildOptions {
                    args: ::core::clone::Clone::clone(&self.args),
                    cache_from: ::core::clone::Clone::clone(&self.cache_from),
                    target: ::core::clone::Clone::clone(&self.target),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for BuildOptions {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for BuildOptions {
            #[inline]
            fn eq(&self, other: &BuildOptions) -> bool {
                self.args == other.args && self.cache_from == other.cache_from
                    && self.target == other.target
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for BuildOptions {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "BuildOptions",
                    "args",
                    &self.args,
                    "cache_from",
                    &self.cache_from,
                    "target",
                    &&self.target,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for BuildOptions {
            #[inline]
            fn default() -> BuildOptions {
                BuildOptions {
                    args: ::core::default::Default::default(),
                    cache_from: ::core::default::Default::default(),
                    target: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for BuildOptions {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "args" => _serde::__private::Ok(__Field::__field0),
                                "cacheFrom" => _serde::__private::Ok(__Field::__field1),
                                "target" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"args" => _serde::__private::Ok(__Field::__field0),
                                b"cacheFrom" => _serde::__private::Ok(__Field::__field1),
                                b"target" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<BuildOptions>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = BuildOptions;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct BuildOptions",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<::std::collections::BTreeMap<String, String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct BuildOptions with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct BuildOptions with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct BuildOptions with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(BuildOptions {
                                args: __field0,
                                cache_from: __field1,
                                target: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<::std::collections::BTreeMap<String, String>>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("args"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<::std::collections::BTreeMap<String, String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "cacheFrom",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("target"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("args") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("cacheFrom") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("target") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(BuildOptions {
                                args: __field0,
                                cache_from: __field1,
                                target: __field2,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "args",
                        "cacheFrom",
                        "target",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "buildOptions",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<BuildOptions>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for BuildOptions {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "buildOptions",
                        false as usize + if Option::is_none(&self.args) { 0 } else { 1 }
                            + if Option::is_none(&self.cache_from) { 0 } else { 1 }
                            + if Option::is_none(&self.target) { 0 } else { 1 },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.args) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "args",
                            &self.args,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "args",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.cache_from) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "cacheFrom",
                            &self.cache_from,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "cacheFrom",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.target) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "target",
                            &self.target,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "target",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[serde(rename = "composeContainer")]
        pub struct ComposeContainer {
            /// The name of the docker-compose file(s) used to start the services.
            #[serde(rename = "dockerComposeFile")]
            pub docker_compose_file: serde_json::Value,
            /// Whether to overwrite the command specified in the image. The default is false.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "overrideCommand")]
            pub override_command: Option<bool>,
            /// An array of services that should be started and stopped.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "runServices")]
            pub run_services: Option<Vec<String>>,
            /// The service you want to work on. This is considered the primary container for your dev
            /// environment which your editor will connect to.
            pub service: String,
            /// Action to take when the user disconnects from the primary container in their editor. The
            /// default is to stop all of the compose containers.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "shutdownAction")]
            pub shutdown_action: Option<String>,
            /// The path of the workspace folder inside the container. This is typically the target path of
            /// a volume mount in the docker-compose.yml.
            #[serde(rename = "workspaceFolder")]
            pub workspace_folder: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ComposeContainer {
            #[inline]
            fn clone(&self) -> ComposeContainer {
                ComposeContainer {
                    docker_compose_file: ::core::clone::Clone::clone(
                        &self.docker_compose_file,
                    ),
                    override_command: ::core::clone::Clone::clone(
                        &self.override_command,
                    ),
                    run_services: ::core::clone::Clone::clone(&self.run_services),
                    service: ::core::clone::Clone::clone(&self.service),
                    shutdown_action: ::core::clone::Clone::clone(&self.shutdown_action),
                    workspace_folder: ::core::clone::Clone::clone(&self.workspace_folder),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ComposeContainer {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ComposeContainer {
            #[inline]
            fn eq(&self, other: &ComposeContainer) -> bool {
                self.docker_compose_file == other.docker_compose_file
                    && self.override_command == other.override_command
                    && self.run_services == other.run_services
                    && self.service == other.service
                    && self.shutdown_action == other.shutdown_action
                    && self.workspace_folder == other.workspace_folder
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ComposeContainer {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "docker_compose_file",
                    "override_command",
                    "run_services",
                    "service",
                    "shutdown_action",
                    "workspace_folder",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.docker_compose_file,
                    &self.override_command,
                    &self.run_services,
                    &self.service,
                    &self.shutdown_action,
                    &&self.workspace_folder,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "ComposeContainer",
                    names,
                    values,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ComposeContainer {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "dockerComposeFile" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                "overrideCommand" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                "runServices" => _serde::__private::Ok(__Field::__field2),
                                "service" => _serde::__private::Ok(__Field::__field3),
                                "shutdownAction" => _serde::__private::Ok(__Field::__field4),
                                "workspaceFolder" => {
                                    _serde::__private::Ok(__Field::__field5)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"dockerComposeFile" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                b"overrideCommand" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                b"runServices" => _serde::__private::Ok(__Field::__field2),
                                b"service" => _serde::__private::Ok(__Field::__field3),
                                b"shutdownAction" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                b"workspaceFolder" => {
                                    _serde::__private::Ok(__Field::__field5)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ComposeContainer>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ComposeContainer;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ComposeContainer",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                serde_json::Value,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ComposeContainer with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ComposeContainer with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ComposeContainer with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct ComposeContainer with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct ComposeContainer with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct ComposeContainer with 6 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ComposeContainer {
                                docker_compose_file: __field0,
                                override_command: __field1,
                                run_services: __field2,
                                service: __field3,
                                shutdown_action: __field4,
                                workspace_folder: __field5,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                serde_json::Value,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<Vec<String>>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "dockerComposeFile",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                serde_json::Value,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "overrideCommand",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "runServices",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "service",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "shutdownAction",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "workspaceFolder",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "dockerComposeFile",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "overrideCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("runServices") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("service") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "shutdownAction",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "workspaceFolder",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(ComposeContainer {
                                docker_compose_file: __field0,
                                override_command: __field1,
                                run_services: __field2,
                                service: __field3,
                                shutdown_action: __field4,
                                workspace_folder: __field5,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "dockerComposeFile",
                        "overrideCommand",
                        "runServices",
                        "service",
                        "shutdownAction",
                        "workspaceFolder",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "composeContainer",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ComposeContainer>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ComposeContainer {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "composeContainer",
                        false as usize + 1
                            + if Option::is_none(&self.override_command) { 0 } else { 1 }
                            + if Option::is_none(&self.run_services) { 0 } else { 1 } + 1
                            + if Option::is_none(&self.shutdown_action) { 0 } else { 1 }
                            + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "dockerComposeFile",
                        &self.docker_compose_file,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.override_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "overrideCommand",
                            &self.override_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "overrideCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.run_services) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "runServices",
                            &self.run_services,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "runServices",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "service",
                        &self.service,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.shutdown_action) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "shutdownAction",
                            &self.shutdown_action,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "shutdownAction",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "workspaceFolder",
                        &self.workspace_folder,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct DevContainerCommonFeatures {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fish: Option<serde_json::Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub gradle: Option<serde_json::Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub homebrew: Option<serde_json::Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub jupyterlab: Option<serde_json::Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub maven: Option<serde_json::Value>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DevContainerCommonFeatures {
            #[inline]
            fn clone(&self) -> DevContainerCommonFeatures {
                DevContainerCommonFeatures {
                    fish: ::core::clone::Clone::clone(&self.fish),
                    gradle: ::core::clone::Clone::clone(&self.gradle),
                    homebrew: ::core::clone::Clone::clone(&self.homebrew),
                    jupyterlab: ::core::clone::Clone::clone(&self.jupyterlab),
                    maven: ::core::clone::Clone::clone(&self.maven),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DevContainerCommonFeatures {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DevContainerCommonFeatures {
            #[inline]
            fn eq(&self, other: &DevContainerCommonFeatures) -> bool {
                self.fish == other.fish && self.gradle == other.gradle
                    && self.homebrew == other.homebrew
                    && self.jupyterlab == other.jupyterlab && self.maven == other.maven
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DevContainerCommonFeatures {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "DevContainerCommonFeatures",
                    "fish",
                    &self.fish,
                    "gradle",
                    &self.gradle,
                    "homebrew",
                    &self.homebrew,
                    "jupyterlab",
                    &self.jupyterlab,
                    "maven",
                    &&self.maven,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DevContainerCommonFeatures {
            #[inline]
            fn default() -> DevContainerCommonFeatures {
                DevContainerCommonFeatures {
                    fish: ::core::default::Default::default(),
                    gradle: ::core::default::Default::default(),
                    homebrew: ::core::default::Default::default(),
                    jupyterlab: ::core::default::Default::default(),
                    maven: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for DevContainerCommonFeatures {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "fish" => _serde::__private::Ok(__Field::__field0),
                                "gradle" => _serde::__private::Ok(__Field::__field1),
                                "homebrew" => _serde::__private::Ok(__Field::__field2),
                                "jupyterlab" => _serde::__private::Ok(__Field::__field3),
                                "maven" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"fish" => _serde::__private::Ok(__Field::__field0),
                                b"gradle" => _serde::__private::Ok(__Field::__field1),
                                b"homebrew" => _serde::__private::Ok(__Field::__field2),
                                b"jupyterlab" => _serde::__private::Ok(__Field::__field3),
                                b"maven" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            DevContainerCommonFeatures,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DevContainerCommonFeatures;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DevContainerCommonFeatures",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DevContainerCommonFeatures with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DevContainerCommonFeatures with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct DevContainerCommonFeatures with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct DevContainerCommonFeatures with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct DevContainerCommonFeatures with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DevContainerCommonFeatures {
                                fish: __field0,
                                gradle: __field1,
                                homebrew: __field2,
                                jupyterlab: __field3,
                                maven: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("fish"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("gradle"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "homebrew",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "jupyterlab",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("maven"),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("fish") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("gradle") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("homebrew") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("jupyterlab") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("maven") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(DevContainerCommonFeatures {
                                fish: __field0,
                                gradle: __field1,
                                homebrew: __field2,
                                jupyterlab: __field3,
                                maven: __field4,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "fish",
                        "gradle",
                        "homebrew",
                        "jupyterlab",
                        "maven",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DevContainerCommonFeatures",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                DevContainerCommonFeatures,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DevContainerCommonFeatures {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DevContainerCommonFeatures",
                        false as usize + if Option::is_none(&self.fish) { 0 } else { 1 }
                            + if Option::is_none(&self.gradle) { 0 } else { 1 }
                            + if Option::is_none(&self.homebrew) { 0 } else { 1 }
                            + if Option::is_none(&self.jupyterlab) { 0 } else { 1 }
                            + if Option::is_none(&self.maven) { 0 } else { 1 },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.fish) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "fish",
                            &self.fish,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "fish",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.gradle) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "gradle",
                            &self.gradle,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "gradle",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.homebrew) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "homebrew",
                            &self.homebrew,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "homebrew",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.jupyterlab) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "jupyterlab",
                            &self.jupyterlab,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "jupyterlab",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.maven) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "maven",
                            &self.maven,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "maven",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub type DevContainerCommonItemForwardPortsVariant0 = i64;
        pub type DevContainerCommonItemForwardPortsVariant1 = String;
        #[serde(untagged)]
        pub enum DevContainerCommonItemForwardPorts {
            Variant0(DevContainerCommonItemForwardPortsVariant0),
            Variant1(DevContainerCommonItemForwardPortsVariant1),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DevContainerCommonItemForwardPorts {
            #[inline]
            fn clone(&self) -> DevContainerCommonItemForwardPorts {
                match self {
                    DevContainerCommonItemForwardPorts::Variant0(__self_0) => {
                        DevContainerCommonItemForwardPorts::Variant0(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    DevContainerCommonItemForwardPorts::Variant1(__self_0) => {
                        DevContainerCommonItemForwardPorts::Variant1(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DevContainerCommonItemForwardPorts {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DevContainerCommonItemForwardPorts {
            #[inline]
            fn eq(&self, other: &DevContainerCommonItemForwardPorts) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (
                            DevContainerCommonItemForwardPorts::Variant0(__self_0),
                            DevContainerCommonItemForwardPorts::Variant0(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            DevContainerCommonItemForwardPorts::Variant1(__self_0),
                            DevContainerCommonItemForwardPorts::Variant1(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DevContainerCommonItemForwardPorts {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    DevContainerCommonItemForwardPorts::Variant0(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variant0",
                            &__self_0,
                        )
                    }
                    DevContainerCommonItemForwardPorts::Variant1(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variant1",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for DevContainerCommonItemForwardPorts {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    let __content = match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                        __deserializer,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if let _serde::__private::Ok(__ok)
                        = _serde::__private::Result::map(
                            <DevContainerCommonItemForwardPortsVariant0 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<
                                    __D::Error,
                                >::new(&__content),
                            ),
                            DevContainerCommonItemForwardPorts::Variant0,
                        ) {
                        return _serde::__private::Ok(__ok);
                    }
                    if let _serde::__private::Ok(__ok)
                        = _serde::__private::Result::map(
                            <DevContainerCommonItemForwardPortsVariant1 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<
                                    __D::Error,
                                >::new(&__content),
                            ),
                            DevContainerCommonItemForwardPorts::Variant1,
                        ) {
                        return _serde::__private::Ok(__ok);
                    }
                    _serde::__private::Err(
                        _serde::de::Error::custom(
                            "data did not match any variant of untagged enum DevContainerCommonItemForwardPorts",
                        ),
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DevContainerCommonItemForwardPorts {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        DevContainerCommonItemForwardPorts::Variant0(ref __field0) => {
                            _serde::Serialize::serialize(__field0, __serializer)
                        }
                        DevContainerCommonItemForwardPorts::Variant1(ref __field0) => {
                            _serde::Serialize::serialize(__field0, __serializer)
                        }
                    }
                }
            }
        };
        pub type DevContainerCommonHostRequirementsGpuVariant0 = bool;
        pub type DevContainerCommonHostRequirementsGpuVariant1 = String;
        pub struct DevContainerCommonHostRequirementsGpuVariant2 {
            /// Number of required cores.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cores: Option<i64>,
            /// Amount of required RAM in bytes. Supports units tb, gb, mb and kb.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub memory: Option<String>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DevContainerCommonHostRequirementsGpuVariant2 {
            #[inline]
            fn clone(&self) -> DevContainerCommonHostRequirementsGpuVariant2 {
                DevContainerCommonHostRequirementsGpuVariant2 {
                    cores: ::core::clone::Clone::clone(&self.cores),
                    memory: ::core::clone::Clone::clone(&self.memory),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq
        for DevContainerCommonHostRequirementsGpuVariant2 {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DevContainerCommonHostRequirementsGpuVariant2 {
            #[inline]
            fn eq(&self, other: &DevContainerCommonHostRequirementsGpuVariant2) -> bool {
                self.cores == other.cores && self.memory == other.memory
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DevContainerCommonHostRequirementsGpuVariant2 {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "DevContainerCommonHostRequirementsGpuVariant2",
                    "cores",
                    &self.cores,
                    "memory",
                    &&self.memory,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DevContainerCommonHostRequirementsGpuVariant2 {
            #[inline]
            fn default() -> DevContainerCommonHostRequirementsGpuVariant2 {
                DevContainerCommonHostRequirementsGpuVariant2 {
                    cores: ::core::default::Default::default(),
                    memory: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de>
            for DevContainerCommonHostRequirementsGpuVariant2 {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "cores" => _serde::__private::Ok(__Field::__field0),
                                "memory" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"cores" => _serde::__private::Ok(__Field::__field0),
                                b"memory" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            DevContainerCommonHostRequirementsGpuVariant2,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DevContainerCommonHostRequirementsGpuVariant2;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DevContainerCommonHostRequirementsGpuVariant2",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<i64>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DevContainerCommonHostRequirementsGpuVariant2 with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DevContainerCommonHostRequirementsGpuVariant2 with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DevContainerCommonHostRequirementsGpuVariant2 {
                                cores: __field0,
                                memory: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Option<i64>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("cores"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<i64>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("memory"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("cores") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("memory") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(DevContainerCommonHostRequirementsGpuVariant2 {
                                cores: __field0,
                                memory: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["cores", "memory"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DevContainerCommonHostRequirementsGpuVariant2",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                DevContainerCommonHostRequirementsGpuVariant2,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DevContainerCommonHostRequirementsGpuVariant2 {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DevContainerCommonHostRequirementsGpuVariant2",
                        false as usize + if Option::is_none(&self.cores) { 0 } else { 1 }
                            + if Option::is_none(&self.memory) { 0 } else { 1 },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.cores) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "cores",
                            &self.cores,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "cores",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.memory) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "memory",
                            &self.memory,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "memory",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[serde(untagged)]
        pub enum DevContainerCommonHostRequirementsGpu {
            Variant0(DevContainerCommonHostRequirementsGpuVariant0),
            Variant1(DevContainerCommonHostRequirementsGpuVariant1),
            Variant2(DevContainerCommonHostRequirementsGpuVariant2),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DevContainerCommonHostRequirementsGpu {
            #[inline]
            fn clone(&self) -> DevContainerCommonHostRequirementsGpu {
                match self {
                    DevContainerCommonHostRequirementsGpu::Variant0(__self_0) => {
                        DevContainerCommonHostRequirementsGpu::Variant0(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    DevContainerCommonHostRequirementsGpu::Variant1(__self_0) => {
                        DevContainerCommonHostRequirementsGpu::Variant1(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    DevContainerCommonHostRequirementsGpu::Variant2(__self_0) => {
                        DevContainerCommonHostRequirementsGpu::Variant2(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq
        for DevContainerCommonHostRequirementsGpu {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DevContainerCommonHostRequirementsGpu {
            #[inline]
            fn eq(&self, other: &DevContainerCommonHostRequirementsGpu) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (
                            DevContainerCommonHostRequirementsGpu::Variant0(__self_0),
                            DevContainerCommonHostRequirementsGpu::Variant0(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            DevContainerCommonHostRequirementsGpu::Variant1(__self_0),
                            DevContainerCommonHostRequirementsGpu::Variant1(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            DevContainerCommonHostRequirementsGpu::Variant2(__self_0),
                            DevContainerCommonHostRequirementsGpu::Variant2(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DevContainerCommonHostRequirementsGpu {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    DevContainerCommonHostRequirementsGpu::Variant0(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variant0",
                            &__self_0,
                        )
                    }
                    DevContainerCommonHostRequirementsGpu::Variant1(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variant1",
                            &__self_0,
                        )
                    }
                    DevContainerCommonHostRequirementsGpu::Variant2(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variant2",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de>
            for DevContainerCommonHostRequirementsGpu {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    let __content = match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                        __deserializer,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if let _serde::__private::Ok(__ok)
                        = _serde::__private::Result::map(
                            <DevContainerCommonHostRequirementsGpuVariant0 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<
                                    __D::Error,
                                >::new(&__content),
                            ),
                            DevContainerCommonHostRequirementsGpu::Variant0,
                        ) {
                        return _serde::__private::Ok(__ok);
                    }
                    if let _serde::__private::Ok(__ok)
                        = _serde::__private::Result::map(
                            <DevContainerCommonHostRequirementsGpuVariant1 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<
                                    __D::Error,
                                >::new(&__content),
                            ),
                            DevContainerCommonHostRequirementsGpu::Variant1,
                        ) {
                        return _serde::__private::Ok(__ok);
                    }
                    if let _serde::__private::Ok(__ok)
                        = _serde::__private::Result::map(
                            <DevContainerCommonHostRequirementsGpuVariant2 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<
                                    __D::Error,
                                >::new(&__content),
                            ),
                            DevContainerCommonHostRequirementsGpu::Variant2,
                        ) {
                        return _serde::__private::Ok(__ok);
                    }
                    _serde::__private::Err(
                        _serde::de::Error::custom(
                            "data did not match any variant of untagged enum DevContainerCommonHostRequirementsGpu",
                        ),
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DevContainerCommonHostRequirementsGpu {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        DevContainerCommonHostRequirementsGpu::Variant0(ref __field0) => {
                            _serde::Serialize::serialize(__field0, __serializer)
                        }
                        DevContainerCommonHostRequirementsGpu::Variant1(ref __field0) => {
                            _serde::Serialize::serialize(__field0, __serializer)
                        }
                        DevContainerCommonHostRequirementsGpu::Variant2(ref __field0) => {
                            _serde::Serialize::serialize(__field0, __serializer)
                        }
                    }
                }
            }
        };
        pub struct DevContainerCommonHostRequirements {
            /// Number of required CPUs.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cpus: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub gpu: Option<DevContainerCommonHostRequirementsGpu>,
            /// Amount of required RAM in bytes. Supports units tb, gb, mb and kb.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub memory: Option<String>,
            /// Amount of required disk space in bytes. Supports units tb, gb, mb and kb.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub storage: Option<String>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DevContainerCommonHostRequirements {
            #[inline]
            fn clone(&self) -> DevContainerCommonHostRequirements {
                DevContainerCommonHostRequirements {
                    cpus: ::core::clone::Clone::clone(&self.cpus),
                    gpu: ::core::clone::Clone::clone(&self.gpu),
                    memory: ::core::clone::Clone::clone(&self.memory),
                    storage: ::core::clone::Clone::clone(&self.storage),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DevContainerCommonHostRequirements {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DevContainerCommonHostRequirements {
            #[inline]
            fn eq(&self, other: &DevContainerCommonHostRequirements) -> bool {
                self.cpus == other.cpus && self.gpu == other.gpu
                    && self.memory == other.memory && self.storage == other.storage
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DevContainerCommonHostRequirements {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "DevContainerCommonHostRequirements",
                    "cpus",
                    &self.cpus,
                    "gpu",
                    &self.gpu,
                    "memory",
                    &self.memory,
                    "storage",
                    &&self.storage,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DevContainerCommonHostRequirements {
            #[inline]
            fn default() -> DevContainerCommonHostRequirements {
                DevContainerCommonHostRequirements {
                    cpus: ::core::default::Default::default(),
                    gpu: ::core::default::Default::default(),
                    memory: ::core::default::Default::default(),
                    storage: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for DevContainerCommonHostRequirements {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "cpus" => _serde::__private::Ok(__Field::__field0),
                                "gpu" => _serde::__private::Ok(__Field::__field1),
                                "memory" => _serde::__private::Ok(__Field::__field2),
                                "storage" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"cpus" => _serde::__private::Ok(__Field::__field0),
                                b"gpu" => _serde::__private::Ok(__Field::__field1),
                                b"memory" => _serde::__private::Ok(__Field::__field2),
                                b"storage" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            DevContainerCommonHostRequirements,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DevContainerCommonHostRequirements;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DevContainerCommonHostRequirements",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<i64>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DevContainerCommonHostRequirements with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<DevContainerCommonHostRequirementsGpu>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DevContainerCommonHostRequirements with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct DevContainerCommonHostRequirements with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct DevContainerCommonHostRequirements with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DevContainerCommonHostRequirements {
                                cpus: __field0,
                                gpu: __field1,
                                memory: __field2,
                                storage: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Option<i64>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<DevContainerCommonHostRequirementsGpu>,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("cpus"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<i64>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("gpu"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<DevContainerCommonHostRequirementsGpu>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("memory"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "storage",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("cpus") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("gpu") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("memory") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("storage") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(DevContainerCommonHostRequirements {
                                cpus: __field0,
                                gpu: __field1,
                                memory: __field2,
                                storage: __field3,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "cpus",
                        "gpu",
                        "memory",
                        "storage",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DevContainerCommonHostRequirements",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                DevContainerCommonHostRequirements,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DevContainerCommonHostRequirements {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DevContainerCommonHostRequirements",
                        false as usize + if Option::is_none(&self.cpus) { 0 } else { 1 }
                            + if Option::is_none(&self.gpu) { 0 } else { 1 }
                            + if Option::is_none(&self.memory) { 0 } else { 1 }
                            + if Option::is_none(&self.storage) { 0 } else { 1 },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.cpus) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "cpus",
                            &self.cpus,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "cpus",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.gpu) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "gpu",
                            &self.gpu,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "gpu",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.memory) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "memory",
                            &self.memory,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "memory",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.storage) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "storage",
                            &self.storage,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "storage",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct DevContainerCommonOtherPortsAttributes {
            /// Automatically prompt for elevation (if needed) when this port is forwarded. Elevate is
            /// required if the local port is a privileged port.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "elevateIfNeeded")]
            pub elevate_if_needed: Option<bool>,
            /// Label that will be shown in the UI for this port.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub label: Option<String>,
            /// Defines the action that occurs when the port is discovered for automatic forwarding
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "onAutoForward")]
            pub on_auto_forward: Option<String>,
            /// The protocol to use when forwarding this port.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub protocol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "requireLocalPort")]
            pub require_local_port: Option<bool>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DevContainerCommonOtherPortsAttributes {
            #[inline]
            fn clone(&self) -> DevContainerCommonOtherPortsAttributes {
                DevContainerCommonOtherPortsAttributes {
                    elevate_if_needed: ::core::clone::Clone::clone(
                        &self.elevate_if_needed,
                    ),
                    label: ::core::clone::Clone::clone(&self.label),
                    on_auto_forward: ::core::clone::Clone::clone(&self.on_auto_forward),
                    protocol: ::core::clone::Clone::clone(&self.protocol),
                    require_local_port: ::core::clone::Clone::clone(
                        &self.require_local_port,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq
        for DevContainerCommonOtherPortsAttributes {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DevContainerCommonOtherPortsAttributes {
            #[inline]
            fn eq(&self, other: &DevContainerCommonOtherPortsAttributes) -> bool {
                self.elevate_if_needed == other.elevate_if_needed
                    && self.label == other.label
                    && self.on_auto_forward == other.on_auto_forward
                    && self.protocol == other.protocol
                    && self.require_local_port == other.require_local_port
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DevContainerCommonOtherPortsAttributes {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "DevContainerCommonOtherPortsAttributes",
                    "elevate_if_needed",
                    &self.elevate_if_needed,
                    "label",
                    &self.label,
                    "on_auto_forward",
                    &self.on_auto_forward,
                    "protocol",
                    &self.protocol,
                    "require_local_port",
                    &&self.require_local_port,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DevContainerCommonOtherPortsAttributes {
            #[inline]
            fn default() -> DevContainerCommonOtherPortsAttributes {
                DevContainerCommonOtherPortsAttributes {
                    elevate_if_needed: ::core::default::Default::default(),
                    label: ::core::default::Default::default(),
                    on_auto_forward: ::core::default::Default::default(),
                    protocol: ::core::default::Default::default(),
                    require_local_port: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de>
            for DevContainerCommonOtherPortsAttributes {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "elevateIfNeeded" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                "label" => _serde::__private::Ok(__Field::__field1),
                                "onAutoForward" => _serde::__private::Ok(__Field::__field2),
                                "protocol" => _serde::__private::Ok(__Field::__field3),
                                "requireLocalPort" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"elevateIfNeeded" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                b"label" => _serde::__private::Ok(__Field::__field1),
                                b"onAutoForward" => _serde::__private::Ok(__Field::__field2),
                                b"protocol" => _serde::__private::Ok(__Field::__field3),
                                b"requireLocalPort" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            DevContainerCommonOtherPortsAttributes,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DevContainerCommonOtherPortsAttributes;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DevContainerCommonOtherPortsAttributes",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DevContainerCommonOtherPortsAttributes with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DevContainerCommonOtherPortsAttributes with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct DevContainerCommonOtherPortsAttributes with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct DevContainerCommonOtherPortsAttributes with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct DevContainerCommonOtherPortsAttributes with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DevContainerCommonOtherPortsAttributes {
                                elevate_if_needed: __field0,
                                label: __field1,
                                on_auto_forward: __field2,
                                protocol: __field3,
                                require_local_port: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "elevateIfNeeded",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("label"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "onAutoForward",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "protocol",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "requireLocalPort",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "elevateIfNeeded",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("label") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "onAutoForward",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("protocol") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "requireLocalPort",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(DevContainerCommonOtherPortsAttributes {
                                elevate_if_needed: __field0,
                                label: __field1,
                                on_auto_forward: __field2,
                                protocol: __field3,
                                require_local_port: __field4,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "elevateIfNeeded",
                        "label",
                        "onAutoForward",
                        "protocol",
                        "requireLocalPort",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DevContainerCommonOtherPortsAttributes",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                DevContainerCommonOtherPortsAttributes,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DevContainerCommonOtherPortsAttributes {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DevContainerCommonOtherPortsAttributes",
                        false as usize
                            + if Option::is_none(&self.elevate_if_needed) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.label) { 0 } else { 1 }
                            + if Option::is_none(&self.on_auto_forward) { 0 } else { 1 }
                            + if Option::is_none(&self.protocol) { 0 } else { 1 }
                            + if Option::is_none(&self.require_local_port) {
                                0
                            } else {
                                1
                            },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.elevate_if_needed) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "elevateIfNeeded",
                            &self.elevate_if_needed,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "elevateIfNeeded",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.label) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "label",
                            &self.label,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "label",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.on_auto_forward) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "onAutoForward",
                            &self.on_auto_forward,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "onAutoForward",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.protocol) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "protocol",
                            &self.protocol,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "protocol",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.require_local_port) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "requireLocalPort",
                            &self.require_local_port,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "requireLocalPort",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct DevContainerCommonPortsAttributes {}
        #[automatically_derived]
        impl ::core::clone::Clone for DevContainerCommonPortsAttributes {
            #[inline]
            fn clone(&self) -> DevContainerCommonPortsAttributes {
                DevContainerCommonPortsAttributes {
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DevContainerCommonPortsAttributes {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DevContainerCommonPortsAttributes {
            #[inline]
            fn eq(&self, other: &DevContainerCommonPortsAttributes) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DevContainerCommonPortsAttributes {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "DevContainerCommonPortsAttributes")
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DevContainerCommonPortsAttributes {
            #[inline]
            fn default() -> DevContainerCommonPortsAttributes {
                DevContainerCommonPortsAttributes {
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for DevContainerCommonPortsAttributes {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            DevContainerCommonPortsAttributes,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DevContainerCommonPortsAttributes;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DevContainerCommonPortsAttributes",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            _: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            _serde::__private::Ok(DevContainerCommonPortsAttributes {
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            _serde::__private::Ok(DevContainerCommonPortsAttributes {
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DevContainerCommonPortsAttributes",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                DevContainerCommonPortsAttributes,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DevContainerCommonPortsAttributes {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DevContainerCommonPortsAttributes",
                        false as usize,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[serde(rename = "devContainerCommon")]
        pub struct DevContainerCommon {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "additionalProperties")]
            pub additional_properties: Option<
                ::std::collections::BTreeMap<String, serde_json::Value>,
            >,
            /// Passes docker capabilities to include when creating the dev container.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "capAdd")]
            pub cap_add: Option<Vec<String>>,
            /// Container environment variables.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "containerEnv")]
            pub container_env: Option<::std::collections::BTreeMap<String, String>>,
            /// The user the container will be started with. The default is the user on the Docker image.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "containerUser")]
            pub container_user: Option<String>,
            /// Tool-specific configuration. Each tool should use a JSON object subproperty with a unique
            /// name to group its customizations.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub customizations: Option<
                ::std::collections::BTreeMap<String, serde_json::Value>,
            >,
            /// Features to add to the dev container.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub features: Option<DevContainerCommonFeatures>,
            /// Ports that are forwarded from the container to the local machine. Can be an integer port
            /// number, or a string of the format "host:port_number".
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "forwardPorts")]
            pub forward_ports: Option<Vec<DevContainerCommonItemForwardPorts>>,
            /// Host hardware requirements.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "hostRequirements")]
            pub host_requirements: Option<DevContainerCommonHostRequirements>,
            /// Passes the --init flag when creating the dev container.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub init: Option<bool>,
            /// A command to run locally before anything else. This command is run before
            /// "onCreateCommand". If this is a single string, it will be run in a shell. If this is an
            /// array of strings, it will be run as a single command without shell.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "initializeCommand")]
            pub initialize_command: Option<serde_json::Value>,
            /// Mount points to set up when creating the container. See Docker's documentation for the
            /// --mount option for the supported syntax.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mounts: Option<Vec<serde_json::Value>>,
            /// A name for the dev container which can be displayed to the user.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: Option<String>,
            /// A command to run when creating the container. This command is run after "initializeCommand"
            /// and before "updateContentCommand". If this is a single string, it will be run in a shell.
            /// If this is an array of strings, it will be run as a single command without shell. If this
            /// is an object, each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "onCreateCommand")]
            pub on_create_command: Option<serde_json::Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "otherPortsAttributes")]
            pub other_ports_attributes: Option<DevContainerCommonOtherPortsAttributes>,
            /// Array consisting of the Feature id (without the semantic version) of Features in the order
            /// the user wants them to be installed.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "overrideFeatureInstallOrder")]
            pub override_feature_install_order: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "portsAttributes")]
            pub ports_attributes: Option<DevContainerCommonPortsAttributes>,
            /// A command to run when attaching to the container. This command is run after
            /// "postStartCommand". If this is a single string, it will be run in a shell. If this is an
            /// array of strings, it will be run as a single command without shell. If this is an object,
            /// each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "postAttachCommand")]
            pub post_attach_command: Option<serde_json::Value>,
            /// A command to run after creating the container. This command is run after
            /// "updateContentCommand" and before "postStartCommand". If this is a single string, it will
            /// be run in a shell. If this is an array of strings, it will be run as a single command
            /// without shell. If this is an object, each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "postCreateCommand")]
            pub post_create_command: Option<serde_json::Value>,
            /// A command to run after starting the container. This command is run after
            /// "postCreateCommand" and before "postAttachCommand". If this is a single string, it will be
            /// run in a shell. If this is an array of strings, it will be run as a single command without
            /// shell. If this is an object, each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "postStartCommand")]
            pub post_start_command: Option<serde_json::Value>,
            /// Passes the --privileged flag when creating the dev container.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub privileged: Option<bool>,
            /// Remote environment variables to set for processes spawned in the container including
            /// lifecycle scripts and any remote editor/IDE server process.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "remoteEnv")]
            pub remote_env: Option<::std::collections::BTreeMap<String, Option<String>>>,
            /// The username to use for spawning processes in the container including lifecycle scripts and
            /// any remote editor/IDE server process. The default is the same user as the container.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "remoteUser")]
            pub remote_user: Option<String>,
            /// Passes docker security options to include when creating the dev container.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "securityOpt")]
            pub security_opt: Option<Vec<String>>,
            /// A command to run when creating the container and rerun when the workspace content was
            /// updated while creating the container. This command is run after "onCreateCommand" and
            /// before "postCreateCommand". If this is a single string, it will be run in a shell. If this
            /// is an array of strings, it will be run as a single command without shell. If this is an
            /// object, each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "updateContentCommand")]
            pub update_content_command: Option<serde_json::Value>,
            /// Controls whether on Linux the container's user should be updated with the local user's UID
            /// and GID. On by default when opening from a local folder.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "updateRemoteUserUID")]
            pub update_remote_user_uid: Option<bool>,
            /// User environment probe to run. The default is "loginInteractiveShell".
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "userEnvProbe")]
            pub user_env_probe: Option<String>,
            /// The user command to wait for before continuing execution in the background while the UI is
            /// starting up. The default is "updateContentCommand".
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "waitFor")]
            pub wait_for: Option<String>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DevContainerCommon {
            #[inline]
            fn clone(&self) -> DevContainerCommon {
                DevContainerCommon {
                    additional_properties: ::core::clone::Clone::clone(
                        &self.additional_properties,
                    ),
                    cap_add: ::core::clone::Clone::clone(&self.cap_add),
                    container_env: ::core::clone::Clone::clone(&self.container_env),
                    container_user: ::core::clone::Clone::clone(&self.container_user),
                    customizations: ::core::clone::Clone::clone(&self.customizations),
                    features: ::core::clone::Clone::clone(&self.features),
                    forward_ports: ::core::clone::Clone::clone(&self.forward_ports),
                    host_requirements: ::core::clone::Clone::clone(
                        &self.host_requirements,
                    ),
                    init: ::core::clone::Clone::clone(&self.init),
                    initialize_command: ::core::clone::Clone::clone(
                        &self.initialize_command,
                    ),
                    mounts: ::core::clone::Clone::clone(&self.mounts),
                    name: ::core::clone::Clone::clone(&self.name),
                    on_create_command: ::core::clone::Clone::clone(
                        &self.on_create_command,
                    ),
                    other_ports_attributes: ::core::clone::Clone::clone(
                        &self.other_ports_attributes,
                    ),
                    override_feature_install_order: ::core::clone::Clone::clone(
                        &self.override_feature_install_order,
                    ),
                    ports_attributes: ::core::clone::Clone::clone(
                        &self.ports_attributes,
                    ),
                    post_attach_command: ::core::clone::Clone::clone(
                        &self.post_attach_command,
                    ),
                    post_create_command: ::core::clone::Clone::clone(
                        &self.post_create_command,
                    ),
                    post_start_command: ::core::clone::Clone::clone(
                        &self.post_start_command,
                    ),
                    privileged: ::core::clone::Clone::clone(&self.privileged),
                    remote_env: ::core::clone::Clone::clone(&self.remote_env),
                    remote_user: ::core::clone::Clone::clone(&self.remote_user),
                    security_opt: ::core::clone::Clone::clone(&self.security_opt),
                    update_content_command: ::core::clone::Clone::clone(
                        &self.update_content_command,
                    ),
                    update_remote_user_uid: ::core::clone::Clone::clone(
                        &self.update_remote_user_uid,
                    ),
                    user_env_probe: ::core::clone::Clone::clone(&self.user_env_probe),
                    wait_for: ::core::clone::Clone::clone(&self.wait_for),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DevContainerCommon {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DevContainerCommon {
            #[inline]
            fn eq(&self, other: &DevContainerCommon) -> bool {
                self.additional_properties == other.additional_properties
                    && self.cap_add == other.cap_add
                    && self.container_env == other.container_env
                    && self.container_user == other.container_user
                    && self.customizations == other.customizations
                    && self.features == other.features
                    && self.forward_ports == other.forward_ports
                    && self.host_requirements == other.host_requirements
                    && self.init == other.init
                    && self.initialize_command == other.initialize_command
                    && self.mounts == other.mounts && self.name == other.name
                    && self.on_create_command == other.on_create_command
                    && self.other_ports_attributes == other.other_ports_attributes
                    && self.override_feature_install_order
                        == other.override_feature_install_order
                    && self.ports_attributes == other.ports_attributes
                    && self.post_attach_command == other.post_attach_command
                    && self.post_create_command == other.post_create_command
                    && self.post_start_command == other.post_start_command
                    && self.privileged == other.privileged
                    && self.remote_env == other.remote_env
                    && self.remote_user == other.remote_user
                    && self.security_opt == other.security_opt
                    && self.update_content_command == other.update_content_command
                    && self.update_remote_user_uid == other.update_remote_user_uid
                    && self.user_env_probe == other.user_env_probe
                    && self.wait_for == other.wait_for
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DevContainerCommon {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "additional_properties",
                    "cap_add",
                    "container_env",
                    "container_user",
                    "customizations",
                    "features",
                    "forward_ports",
                    "host_requirements",
                    "init",
                    "initialize_command",
                    "mounts",
                    "name",
                    "on_create_command",
                    "other_ports_attributes",
                    "override_feature_install_order",
                    "ports_attributes",
                    "post_attach_command",
                    "post_create_command",
                    "post_start_command",
                    "privileged",
                    "remote_env",
                    "remote_user",
                    "security_opt",
                    "update_content_command",
                    "update_remote_user_uid",
                    "user_env_probe",
                    "wait_for",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.additional_properties,
                    &self.cap_add,
                    &self.container_env,
                    &self.container_user,
                    &self.customizations,
                    &self.features,
                    &self.forward_ports,
                    &self.host_requirements,
                    &self.init,
                    &self.initialize_command,
                    &self.mounts,
                    &self.name,
                    &self.on_create_command,
                    &self.other_ports_attributes,
                    &self.override_feature_install_order,
                    &self.ports_attributes,
                    &self.post_attach_command,
                    &self.post_create_command,
                    &self.post_start_command,
                    &self.privileged,
                    &self.remote_env,
                    &self.remote_user,
                    &self.security_opt,
                    &self.update_content_command,
                    &self.update_remote_user_uid,
                    &self.user_env_probe,
                    &&self.wait_for,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "DevContainerCommon",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DevContainerCommon {
            #[inline]
            fn default() -> DevContainerCommon {
                DevContainerCommon {
                    additional_properties: ::core::default::Default::default(),
                    cap_add: ::core::default::Default::default(),
                    container_env: ::core::default::Default::default(),
                    container_user: ::core::default::Default::default(),
                    customizations: ::core::default::Default::default(),
                    features: ::core::default::Default::default(),
                    forward_ports: ::core::default::Default::default(),
                    host_requirements: ::core::default::Default::default(),
                    init: ::core::default::Default::default(),
                    initialize_command: ::core::default::Default::default(),
                    mounts: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    on_create_command: ::core::default::Default::default(),
                    other_ports_attributes: ::core::default::Default::default(),
                    override_feature_install_order: ::core::default::Default::default(),
                    ports_attributes: ::core::default::Default::default(),
                    post_attach_command: ::core::default::Default::default(),
                    post_create_command: ::core::default::Default::default(),
                    post_start_command: ::core::default::Default::default(),
                    privileged: ::core::default::Default::default(),
                    remote_env: ::core::default::Default::default(),
                    remote_user: ::core::default::Default::default(),
                    security_opt: ::core::default::Default::default(),
                    update_content_command: ::core::default::Default::default(),
                    update_remote_user_uid: ::core::default::Default::default(),
                    user_env_probe: ::core::default::Default::default(),
                    wait_for: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for DevContainerCommon {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
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
                        __field12,
                        __field13,
                        __field14,
                        __field15,
                        __field16,
                        __field17,
                        __field18,
                        __field19,
                        __field20,
                        __field21,
                        __field22,
                        __field23,
                        __field24,
                        __field25,
                        __field26,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                10u64 => _serde::__private::Ok(__Field::__field10),
                                11u64 => _serde::__private::Ok(__Field::__field11),
                                12u64 => _serde::__private::Ok(__Field::__field12),
                                13u64 => _serde::__private::Ok(__Field::__field13),
                                14u64 => _serde::__private::Ok(__Field::__field14),
                                15u64 => _serde::__private::Ok(__Field::__field15),
                                16u64 => _serde::__private::Ok(__Field::__field16),
                                17u64 => _serde::__private::Ok(__Field::__field17),
                                18u64 => _serde::__private::Ok(__Field::__field18),
                                19u64 => _serde::__private::Ok(__Field::__field19),
                                20u64 => _serde::__private::Ok(__Field::__field20),
                                21u64 => _serde::__private::Ok(__Field::__field21),
                                22u64 => _serde::__private::Ok(__Field::__field22),
                                23u64 => _serde::__private::Ok(__Field::__field23),
                                24u64 => _serde::__private::Ok(__Field::__field24),
                                25u64 => _serde::__private::Ok(__Field::__field25),
                                26u64 => _serde::__private::Ok(__Field::__field26),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "additionalProperties" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                "capAdd" => _serde::__private::Ok(__Field::__field1),
                                "containerEnv" => _serde::__private::Ok(__Field::__field2),
                                "containerUser" => _serde::__private::Ok(__Field::__field3),
                                "customizations" => _serde::__private::Ok(__Field::__field4),
                                "features" => _serde::__private::Ok(__Field::__field5),
                                "forwardPorts" => _serde::__private::Ok(__Field::__field6),
                                "hostRequirements" => {
                                    _serde::__private::Ok(__Field::__field7)
                                }
                                "init" => _serde::__private::Ok(__Field::__field8),
                                "initializeCommand" => {
                                    _serde::__private::Ok(__Field::__field9)
                                }
                                "mounts" => _serde::__private::Ok(__Field::__field10),
                                "name" => _serde::__private::Ok(__Field::__field11),
                                "onCreateCommand" => {
                                    _serde::__private::Ok(__Field::__field12)
                                }
                                "otherPortsAttributes" => {
                                    _serde::__private::Ok(__Field::__field13)
                                }
                                "overrideFeatureInstallOrder" => {
                                    _serde::__private::Ok(__Field::__field14)
                                }
                                "portsAttributes" => {
                                    _serde::__private::Ok(__Field::__field15)
                                }
                                "postAttachCommand" => {
                                    _serde::__private::Ok(__Field::__field16)
                                }
                                "postCreateCommand" => {
                                    _serde::__private::Ok(__Field::__field17)
                                }
                                "postStartCommand" => {
                                    _serde::__private::Ok(__Field::__field18)
                                }
                                "privileged" => _serde::__private::Ok(__Field::__field19),
                                "remoteEnv" => _serde::__private::Ok(__Field::__field20),
                                "remoteUser" => _serde::__private::Ok(__Field::__field21),
                                "securityOpt" => _serde::__private::Ok(__Field::__field22),
                                "updateContentCommand" => {
                                    _serde::__private::Ok(__Field::__field23)
                                }
                                "updateRemoteUserUID" => {
                                    _serde::__private::Ok(__Field::__field24)
                                }
                                "userEnvProbe" => _serde::__private::Ok(__Field::__field25),
                                "waitFor" => _serde::__private::Ok(__Field::__field26),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"additionalProperties" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                b"capAdd" => _serde::__private::Ok(__Field::__field1),
                                b"containerEnv" => _serde::__private::Ok(__Field::__field2),
                                b"containerUser" => _serde::__private::Ok(__Field::__field3),
                                b"customizations" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                b"features" => _serde::__private::Ok(__Field::__field5),
                                b"forwardPorts" => _serde::__private::Ok(__Field::__field6),
                                b"hostRequirements" => {
                                    _serde::__private::Ok(__Field::__field7)
                                }
                                b"init" => _serde::__private::Ok(__Field::__field8),
                                b"initializeCommand" => {
                                    _serde::__private::Ok(__Field::__field9)
                                }
                                b"mounts" => _serde::__private::Ok(__Field::__field10),
                                b"name" => _serde::__private::Ok(__Field::__field11),
                                b"onCreateCommand" => {
                                    _serde::__private::Ok(__Field::__field12)
                                }
                                b"otherPortsAttributes" => {
                                    _serde::__private::Ok(__Field::__field13)
                                }
                                b"overrideFeatureInstallOrder" => {
                                    _serde::__private::Ok(__Field::__field14)
                                }
                                b"portsAttributes" => {
                                    _serde::__private::Ok(__Field::__field15)
                                }
                                b"postAttachCommand" => {
                                    _serde::__private::Ok(__Field::__field16)
                                }
                                b"postCreateCommand" => {
                                    _serde::__private::Ok(__Field::__field17)
                                }
                                b"postStartCommand" => {
                                    _serde::__private::Ok(__Field::__field18)
                                }
                                b"privileged" => _serde::__private::Ok(__Field::__field19),
                                b"remoteEnv" => _serde::__private::Ok(__Field::__field20),
                                b"remoteUser" => _serde::__private::Ok(__Field::__field21),
                                b"securityOpt" => _serde::__private::Ok(__Field::__field22),
                                b"updateContentCommand" => {
                                    _serde::__private::Ok(__Field::__field23)
                                }
                                b"updateRemoteUserUID" => {
                                    _serde::__private::Ok(__Field::__field24)
                                }
                                b"userEnvProbe" => _serde::__private::Ok(__Field::__field25),
                                b"waitFor" => _serde::__private::Ok(__Field::__field26),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<DevContainerCommon>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DevContainerCommon;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DevContainerCommon",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<
                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                >,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<::std::collections::BTreeMap<String, String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<
                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                >,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                Option<DevContainerCommonFeatures>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<DevContainerCommonItemForwardPorts>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<
                                Option<DevContainerCommonHostRequirements>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field10 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<serde_json::Value>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            10usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field11 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            11usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field12 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            12usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field13 = match match _serde::de::SeqAccess::next_element::<
                                Option<DevContainerCommonOtherPortsAttributes>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            13usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field14 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            14usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field15 = match match _serde::de::SeqAccess::next_element::<
                                Option<DevContainerCommonPortsAttributes>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            15usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field16 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            16usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field17 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            17usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field18 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            18usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field19 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            19usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field20 = match match _serde::de::SeqAccess::next_element::<
                                Option<::std::collections::BTreeMap<String, Option<String>>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            20usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field21 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            21usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field22 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            22usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field23 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            23usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field24 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            24usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field25 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            25usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field26 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            26usize,
                                            &"struct DevContainerCommon with 27 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DevContainerCommon {
                                additional_properties: __field0,
                                cap_add: __field1,
                                container_env: __field2,
                                container_user: __field3,
                                customizations: __field4,
                                features: __field5,
                                forward_ports: __field6,
                                host_requirements: __field7,
                                init: __field8,
                                initialize_command: __field9,
                                mounts: __field10,
                                name: __field11,
                                on_create_command: __field12,
                                other_ports_attributes: __field13,
                                override_feature_install_order: __field14,
                                ports_attributes: __field15,
                                post_attach_command: __field16,
                                post_create_command: __field17,
                                post_start_command: __field18,
                                privileged: __field19,
                                remote_env: __field20,
                                remote_user: __field21,
                                security_opt: __field22,
                                update_content_command: __field23,
                                update_remote_user_uid: __field24,
                                user_env_probe: __field25,
                                wait_for: __field26,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<
                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                >,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<Vec<String>>,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<::std::collections::BTreeMap<String, String>>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<
                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                >,
                            > = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<
                                Option<DevContainerCommonFeatures>,
                            > = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<
                                Option<Vec<DevContainerCommonItemForwardPorts>>,
                            > = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<
                                Option<DevContainerCommonHostRequirements>,
                            > = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field9: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field10: _serde::__private::Option<
                                Option<Vec<serde_json::Value>>,
                            > = _serde::__private::None;
                            let mut __field11: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field12: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field13: _serde::__private::Option<
                                Option<DevContainerCommonOtherPortsAttributes>,
                            > = _serde::__private::None;
                            let mut __field14: _serde::__private::Option<
                                Option<Vec<String>>,
                            > = _serde::__private::None;
                            let mut __field15: _serde::__private::Option<
                                Option<DevContainerCommonPortsAttributes>,
                            > = _serde::__private::None;
                            let mut __field16: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field17: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field18: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field19: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field20: _serde::__private::Option<
                                Option<::std::collections::BTreeMap<String, Option<String>>>,
                            > = _serde::__private::None;
                            let mut __field21: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field22: _serde::__private::Option<
                                Option<Vec<String>>,
                            > = _serde::__private::None;
                            let mut __field23: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field24: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field25: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field26: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "additionalProperties",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<
                                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                                >,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("capAdd"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "containerEnv",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<::std::collections::BTreeMap<String, String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "containerUser",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "customizations",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<
                                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                                >,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "features",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<DevContainerCommonFeatures>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "forwardPorts",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<DevContainerCommonItemForwardPorts>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "hostRequirements",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<DevContainerCommonHostRequirements>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("init"),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "initializeCommand",
                                                ),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field10 => {
                                        if _serde::__private::Option::is_some(&__field10) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("mounts"),
                                            );
                                        }
                                        __field10 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<serde_json::Value>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field11 => {
                                        if _serde::__private::Option::is_some(&__field11) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field11 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field12 => {
                                        if _serde::__private::Option::is_some(&__field12) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "onCreateCommand",
                                                ),
                                            );
                                        }
                                        __field12 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field13 => {
                                        if _serde::__private::Option::is_some(&__field13) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "otherPortsAttributes",
                                                ),
                                            );
                                        }
                                        __field13 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<DevContainerCommonOtherPortsAttributes>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field14 => {
                                        if _serde::__private::Option::is_some(&__field14) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "overrideFeatureInstallOrder",
                                                ),
                                            );
                                        }
                                        __field14 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field15 => {
                                        if _serde::__private::Option::is_some(&__field15) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "portsAttributes",
                                                ),
                                            );
                                        }
                                        __field15 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<DevContainerCommonPortsAttributes>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field16 => {
                                        if _serde::__private::Option::is_some(&__field16) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "postAttachCommand",
                                                ),
                                            );
                                        }
                                        __field16 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field17 => {
                                        if _serde::__private::Option::is_some(&__field17) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "postCreateCommand",
                                                ),
                                            );
                                        }
                                        __field17 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field18 => {
                                        if _serde::__private::Option::is_some(&__field18) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "postStartCommand",
                                                ),
                                            );
                                        }
                                        __field18 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field19 => {
                                        if _serde::__private::Option::is_some(&__field19) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "privileged",
                                                ),
                                            );
                                        }
                                        __field19 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field20 => {
                                        if _serde::__private::Option::is_some(&__field20) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "remoteEnv",
                                                ),
                                            );
                                        }
                                        __field20 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<::std::collections::BTreeMap<String, Option<String>>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field21 => {
                                        if _serde::__private::Option::is_some(&__field21) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "remoteUser",
                                                ),
                                            );
                                        }
                                        __field21 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field22 => {
                                        if _serde::__private::Option::is_some(&__field22) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "securityOpt",
                                                ),
                                            );
                                        }
                                        __field22 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field23 => {
                                        if _serde::__private::Option::is_some(&__field23) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "updateContentCommand",
                                                ),
                                            );
                                        }
                                        __field23 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field24 => {
                                        if _serde::__private::Option::is_some(&__field24) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "updateRemoteUserUID",
                                                ),
                                            );
                                        }
                                        __field24 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field25 => {
                                        if _serde::__private::Option::is_some(&__field25) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "userEnvProbe",
                                                ),
                                            );
                                        }
                                        __field25 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field26 => {
                                        if _serde::__private::Option::is_some(&__field26) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "waitFor",
                                                ),
                                            );
                                        }
                                        __field26 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "additionalProperties",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("capAdd") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("containerEnv") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "containerUser",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "customizations",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("features") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("forwardPorts") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "hostRequirements",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("init") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "initializeCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field10 = match __field10 {
                                _serde::__private::Some(__field10) => __field10,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("mounts") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field11 = match __field11 {
                                _serde::__private::Some(__field11) => __field11,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("name") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field12 = match __field12 {
                                _serde::__private::Some(__field12) => __field12,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "onCreateCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field13 = match __field13 {
                                _serde::__private::Some(__field13) => __field13,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "otherPortsAttributes",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field14 = match __field14 {
                                _serde::__private::Some(__field14) => __field14,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "overrideFeatureInstallOrder",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field15 = match __field15 {
                                _serde::__private::Some(__field15) => __field15,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "portsAttributes",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field16 = match __field16 {
                                _serde::__private::Some(__field16) => __field16,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "postAttachCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field17 = match __field17 {
                                _serde::__private::Some(__field17) => __field17,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "postCreateCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field18 = match __field18 {
                                _serde::__private::Some(__field18) => __field18,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "postStartCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field19 = match __field19 {
                                _serde::__private::Some(__field19) => __field19,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("privileged") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field20 = match __field20 {
                                _serde::__private::Some(__field20) => __field20,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("remoteEnv") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field21 = match __field21 {
                                _serde::__private::Some(__field21) => __field21,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("remoteUser") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field22 = match __field22 {
                                _serde::__private::Some(__field22) => __field22,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("securityOpt") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field23 = match __field23 {
                                _serde::__private::Some(__field23) => __field23,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "updateContentCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field24 = match __field24 {
                                _serde::__private::Some(__field24) => __field24,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "updateRemoteUserUID",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field25 = match __field25 {
                                _serde::__private::Some(__field25) => __field25,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("userEnvProbe") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field26 = match __field26 {
                                _serde::__private::Some(__field26) => __field26,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("waitFor") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(DevContainerCommon {
                                additional_properties: __field0,
                                cap_add: __field1,
                                container_env: __field2,
                                container_user: __field3,
                                customizations: __field4,
                                features: __field5,
                                forward_ports: __field6,
                                host_requirements: __field7,
                                init: __field8,
                                initialize_command: __field9,
                                mounts: __field10,
                                name: __field11,
                                on_create_command: __field12,
                                other_ports_attributes: __field13,
                                override_feature_install_order: __field14,
                                ports_attributes: __field15,
                                post_attach_command: __field16,
                                post_create_command: __field17,
                                post_start_command: __field18,
                                privileged: __field19,
                                remote_env: __field20,
                                remote_user: __field21,
                                security_opt: __field22,
                                update_content_command: __field23,
                                update_remote_user_uid: __field24,
                                user_env_probe: __field25,
                                wait_for: __field26,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "additionalProperties",
                        "capAdd",
                        "containerEnv",
                        "containerUser",
                        "customizations",
                        "features",
                        "forwardPorts",
                        "hostRequirements",
                        "init",
                        "initializeCommand",
                        "mounts",
                        "name",
                        "onCreateCommand",
                        "otherPortsAttributes",
                        "overrideFeatureInstallOrder",
                        "portsAttributes",
                        "postAttachCommand",
                        "postCreateCommand",
                        "postStartCommand",
                        "privileged",
                        "remoteEnv",
                        "remoteUser",
                        "securityOpt",
                        "updateContentCommand",
                        "updateRemoteUserUID",
                        "userEnvProbe",
                        "waitFor",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "devContainerCommon",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<DevContainerCommon>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DevContainerCommon {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "devContainerCommon",
                        false as usize
                            + if Option::is_none(&self.additional_properties) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.cap_add) { 0 } else { 1 }
                            + if Option::is_none(&self.container_env) { 0 } else { 1 }
                            + if Option::is_none(&self.container_user) { 0 } else { 1 }
                            + if Option::is_none(&self.customizations) { 0 } else { 1 }
                            + if Option::is_none(&self.features) { 0 } else { 1 }
                            + if Option::is_none(&self.forward_ports) { 0 } else { 1 }
                            + if Option::is_none(&self.host_requirements) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.init) { 0 } else { 1 }
                            + if Option::is_none(&self.initialize_command) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.mounts) { 0 } else { 1 }
                            + if Option::is_none(&self.name) { 0 } else { 1 }
                            + if Option::is_none(&self.on_create_command) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.other_ports_attributes) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.override_feature_install_order) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.ports_attributes) { 0 } else { 1 }
                            + if Option::is_none(&self.post_attach_command) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.post_create_command) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.post_start_command) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.privileged) { 0 } else { 1 }
                            + if Option::is_none(&self.remote_env) { 0 } else { 1 }
                            + if Option::is_none(&self.remote_user) { 0 } else { 1 }
                            + if Option::is_none(&self.security_opt) { 0 } else { 1 }
                            + if Option::is_none(&self.update_content_command) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.update_remote_user_uid) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.user_env_probe) { 0 } else { 1 }
                            + if Option::is_none(&self.wait_for) { 0 } else { 1 },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.additional_properties) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "additionalProperties",
                            &self.additional_properties,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "additionalProperties",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.cap_add) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "capAdd",
                            &self.cap_add,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "capAdd",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.container_env) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "containerEnv",
                            &self.container_env,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "containerEnv",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.container_user) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "containerUser",
                            &self.container_user,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "containerUser",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.customizations) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "customizations",
                            &self.customizations,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "customizations",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.features) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "features",
                            &self.features,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "features",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.forward_ports) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "forwardPorts",
                            &self.forward_ports,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "forwardPorts",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.host_requirements) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "hostRequirements",
                            &self.host_requirements,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "hostRequirements",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.init) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "init",
                            &self.init,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "init",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.initialize_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "initializeCommand",
                            &self.initialize_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "initializeCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.mounts) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "mounts",
                            &self.mounts,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "mounts",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.name) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "name",
                            &self.name,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "name",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.on_create_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "onCreateCommand",
                            &self.on_create_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "onCreateCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.other_ports_attributes) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "otherPortsAttributes",
                            &self.other_ports_attributes,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "otherPortsAttributes",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.override_feature_install_order) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "overrideFeatureInstallOrder",
                            &self.override_feature_install_order,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "overrideFeatureInstallOrder",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.ports_attributes) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "portsAttributes",
                            &self.ports_attributes,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "portsAttributes",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.post_attach_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "postAttachCommand",
                            &self.post_attach_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "postAttachCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.post_create_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "postCreateCommand",
                            &self.post_create_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "postCreateCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.post_start_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "postStartCommand",
                            &self.post_start_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "postStartCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.privileged) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "privileged",
                            &self.privileged,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "privileged",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.remote_env) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "remoteEnv",
                            &self.remote_env,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "remoteEnv",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.remote_user) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "remoteUser",
                            &self.remote_user,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "remoteUser",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.security_opt) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "securityOpt",
                            &self.security_opt,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "securityOpt",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.update_content_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "updateContentCommand",
                            &self.update_content_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "updateContentCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.update_remote_user_uid) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "updateRemoteUserUID",
                            &self.update_remote_user_uid,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "updateRemoteUserUID",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.user_env_probe) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "userEnvProbe",
                            &self.user_env_probe,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "userEnvProbe",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.wait_for) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "waitFor",
                            &self.wait_for,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "waitFor",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct DockerfileContainerWaitForVariant0 {
            /// Docker build-related options.
            pub build: ::std::collections::BTreeMap<String, serde_json::Value>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DockerfileContainerWaitForVariant0 {
            #[inline]
            fn clone(&self) -> DockerfileContainerWaitForVariant0 {
                DockerfileContainerWaitForVariant0 {
                    build: ::core::clone::Clone::clone(&self.build),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DockerfileContainerWaitForVariant0 {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DockerfileContainerWaitForVariant0 {
            #[inline]
            fn eq(&self, other: &DockerfileContainerWaitForVariant0) -> bool {
                self.build == other.build
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DockerfileContainerWaitForVariant0 {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "DockerfileContainerWaitForVariant0",
                    "build",
                    &&self.build,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for DockerfileContainerWaitForVariant0 {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "build" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"build" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            DockerfileContainerWaitForVariant0,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DockerfileContainerWaitForVariant0;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DockerfileContainerWaitForVariant0",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                ::std::collections::BTreeMap<String, serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DockerfileContainerWaitForVariant0 with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DockerfileContainerWaitForVariant0 {
                                build: __field0,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                ::std::collections::BTreeMap<String, serde_json::Value>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("build"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                ::std::collections::BTreeMap<String, serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("build") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(DockerfileContainerWaitForVariant0 {
                                build: __field0,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["build"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DockerfileContainerWaitForVariant0",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                DockerfileContainerWaitForVariant0,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DockerfileContainerWaitForVariant0 {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DockerfileContainerWaitForVariant0",
                        false as usize + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "build",
                        &self.build,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct DockerfileContainerWaitForVariant1 {
            /// Docker build-related options.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub build: Option<BuildOptions>,
            /// The location of the context folder for building the Docker image. The path is relative to
            /// the folder containing the `devcontainer.json` file.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub context: Option<String>,
            /// The location of the Dockerfile that defines the contents of the container. The path is
            /// relative to the folder containing the `devcontainer.json` file.
            #[serde(rename = "dockerFile")]
            pub docker_file: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DockerfileContainerWaitForVariant1 {
            #[inline]
            fn clone(&self) -> DockerfileContainerWaitForVariant1 {
                DockerfileContainerWaitForVariant1 {
                    build: ::core::clone::Clone::clone(&self.build),
                    context: ::core::clone::Clone::clone(&self.context),
                    docker_file: ::core::clone::Clone::clone(&self.docker_file),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DockerfileContainerWaitForVariant1 {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DockerfileContainerWaitForVariant1 {
            #[inline]
            fn eq(&self, other: &DockerfileContainerWaitForVariant1) -> bool {
                self.build == other.build && self.context == other.context
                    && self.docker_file == other.docker_file
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DockerfileContainerWaitForVariant1 {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "DockerfileContainerWaitForVariant1",
                    "build",
                    &self.build,
                    "context",
                    &self.context,
                    "docker_file",
                    &&self.docker_file,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for DockerfileContainerWaitForVariant1 {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "build" => _serde::__private::Ok(__Field::__field0),
                                "context" => _serde::__private::Ok(__Field::__field1),
                                "dockerFile" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"build" => _serde::__private::Ok(__Field::__field0),
                                b"context" => _serde::__private::Ok(__Field::__field1),
                                b"dockerFile" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            DockerfileContainerWaitForVariant1,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DockerfileContainerWaitForVariant1;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DockerfileContainerWaitForVariant1",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<BuildOptions>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DockerfileContainerWaitForVariant1 with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DockerfileContainerWaitForVariant1 with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct DockerfileContainerWaitForVariant1 with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DockerfileContainerWaitForVariant1 {
                                build: __field0,
                                context: __field1,
                                docker_file: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<BuildOptions>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("build"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<BuildOptions>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "context",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "dockerFile",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("build") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("context") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("dockerFile") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(DockerfileContainerWaitForVariant1 {
                                build: __field0,
                                context: __field1,
                                docker_file: __field2,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "build",
                        "context",
                        "dockerFile",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DockerfileContainerWaitForVariant1",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                DockerfileContainerWaitForVariant1,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DockerfileContainerWaitForVariant1 {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DockerfileContainerWaitForVariant1",
                        false as usize + if Option::is_none(&self.build) { 0 } else { 1 }
                            + if Option::is_none(&self.context) { 0 } else { 1 } + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.build) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "build",
                            &self.build,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "build",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.context) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "context",
                            &self.context,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "context",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "dockerFile",
                        &self.docker_file,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[serde(untagged)]
        pub enum DockerfileContainerWaitFor {
            Variant0(DockerfileContainerWaitForVariant0),
            Variant1(DockerfileContainerWaitForVariant1),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DockerfileContainerWaitFor {
            #[inline]
            fn clone(&self) -> DockerfileContainerWaitFor {
                match self {
                    DockerfileContainerWaitFor::Variant0(__self_0) => {
                        DockerfileContainerWaitFor::Variant0(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    DockerfileContainerWaitFor::Variant1(__self_0) => {
                        DockerfileContainerWaitFor::Variant1(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DockerfileContainerWaitFor {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DockerfileContainerWaitFor {
            #[inline]
            fn eq(&self, other: &DockerfileContainerWaitFor) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (
                            DockerfileContainerWaitFor::Variant0(__self_0),
                            DockerfileContainerWaitFor::Variant0(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            DockerfileContainerWaitFor::Variant1(__self_0),
                            DockerfileContainerWaitFor::Variant1(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DockerfileContainerWaitFor {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    DockerfileContainerWaitFor::Variant0(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variant0",
                            &__self_0,
                        )
                    }
                    DockerfileContainerWaitFor::Variant1(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variant1",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for DockerfileContainerWaitFor {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    let __content = match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                        __deserializer,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if let _serde::__private::Ok(__ok)
                        = _serde::__private::Result::map(
                            <DockerfileContainerWaitForVariant0 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<
                                    __D::Error,
                                >::new(&__content),
                            ),
                            DockerfileContainerWaitFor::Variant0,
                        ) {
                        return _serde::__private::Ok(__ok);
                    }
                    if let _serde::__private::Ok(__ok)
                        = _serde::__private::Result::map(
                            <DockerfileContainerWaitForVariant1 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<
                                    __D::Error,
                                >::new(&__content),
                            ),
                            DockerfileContainerWaitFor::Variant1,
                        ) {
                        return _serde::__private::Ok(__ok);
                    }
                    _serde::__private::Err(
                        _serde::de::Error::custom(
                            "data did not match any variant of untagged enum DockerfileContainerWaitFor",
                        ),
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DockerfileContainerWaitFor {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        DockerfileContainerWaitFor::Variant0(ref __field0) => {
                            _serde::Serialize::serialize(__field0, __serializer)
                        }
                        DockerfileContainerWaitFor::Variant1(ref __field0) => {
                            _serde::Serialize::serialize(__field0, __serializer)
                        }
                    }
                }
            }
        };
        pub type DockerfileContainer = DockerfileContainerWaitFor;
        #[serde(rename = "imageContainer")]
        pub struct ImageContainer {
            /// The docker image that will be used to create the container.
            pub image: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ImageContainer {
            #[inline]
            fn clone(&self) -> ImageContainer {
                ImageContainer {
                    image: ::core::clone::Clone::clone(&self.image),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ImageContainer {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ImageContainer {
            #[inline]
            fn eq(&self, other: &ImageContainer) -> bool {
                self.image == other.image
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ImageContainer {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ImageContainer",
                    "image",
                    &&self.image,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ImageContainer {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "image" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"image" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ImageContainer>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ImageContainer;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ImageContainer",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ImageContainer with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ImageContainer { image: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("image"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("image") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(ImageContainer { image: __field0 })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["image"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "imageContainer",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ImageContainer>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ImageContainer {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "imageContainer",
                        false as usize + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "image",
                        &self.image,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[serde(rename = "nonComposeBase")]
        pub struct NonComposeBase {
            /// Application ports that are exposed by the container. This can be a single port or an array
            /// of ports. Each port can be a number or a string. A number is mapped to the same port on the
            /// host. A string is passed to Docker unchanged and can be used to map ports differently, e.g.
            /// "8000:8010".
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "appPort")]
            pub app_port: Option<serde_json::Value>,
            /// Whether to overwrite the command specified in the image. The default is true.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "overrideCommand")]
            pub override_command: Option<bool>,
            /// The arguments required when starting in the container.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "runArgs")]
            pub run_args: Option<Vec<String>>,
            /// Action to take when the user disconnects from the container in their editor. The default is
            /// to stop the container.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "shutdownAction")]
            pub shutdown_action: Option<String>,
            /// The path of the workspace folder inside the container.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "workspaceFolder")]
            pub workspace_folder: Option<String>,
            /// The --mount parameter for docker run. The default is to mount the project folder at
            /// /workspaces/$project.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "workspaceMount")]
            pub workspace_mount: Option<String>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for NonComposeBase {
            #[inline]
            fn clone(&self) -> NonComposeBase {
                NonComposeBase {
                    app_port: ::core::clone::Clone::clone(&self.app_port),
                    override_command: ::core::clone::Clone::clone(
                        &self.override_command,
                    ),
                    run_args: ::core::clone::Clone::clone(&self.run_args),
                    shutdown_action: ::core::clone::Clone::clone(&self.shutdown_action),
                    workspace_folder: ::core::clone::Clone::clone(
                        &self.workspace_folder,
                    ),
                    workspace_mount: ::core::clone::Clone::clone(&self.workspace_mount),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for NonComposeBase {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for NonComposeBase {
            #[inline]
            fn eq(&self, other: &NonComposeBase) -> bool {
                self.app_port == other.app_port
                    && self.override_command == other.override_command
                    && self.run_args == other.run_args
                    && self.shutdown_action == other.shutdown_action
                    && self.workspace_folder == other.workspace_folder
                    && self.workspace_mount == other.workspace_mount
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for NonComposeBase {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "app_port",
                    "override_command",
                    "run_args",
                    "shutdown_action",
                    "workspace_folder",
                    "workspace_mount",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.app_port,
                    &self.override_command,
                    &self.run_args,
                    &self.shutdown_action,
                    &self.workspace_folder,
                    &&self.workspace_mount,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "NonComposeBase",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for NonComposeBase {
            #[inline]
            fn default() -> NonComposeBase {
                NonComposeBase {
                    app_port: ::core::default::Default::default(),
                    override_command: ::core::default::Default::default(),
                    run_args: ::core::default::Default::default(),
                    shutdown_action: ::core::default::Default::default(),
                    workspace_folder: ::core::default::Default::default(),
                    workspace_mount: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for NonComposeBase {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "appPort" => _serde::__private::Ok(__Field::__field0),
                                "overrideCommand" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                "runArgs" => _serde::__private::Ok(__Field::__field2),
                                "shutdownAction" => _serde::__private::Ok(__Field::__field3),
                                "workspaceFolder" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                "workspaceMount" => _serde::__private::Ok(__Field::__field5),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"appPort" => _serde::__private::Ok(__Field::__field0),
                                b"overrideCommand" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                b"runArgs" => _serde::__private::Ok(__Field::__field2),
                                b"shutdownAction" => {
                                    _serde::__private::Ok(__Field::__field3)
                                }
                                b"workspaceFolder" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                b"workspaceMount" => {
                                    _serde::__private::Ok(__Field::__field5)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<NonComposeBase>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = NonComposeBase;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct NonComposeBase",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct NonComposeBase with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct NonComposeBase with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct NonComposeBase with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct NonComposeBase with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct NonComposeBase with 6 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct NonComposeBase with 6 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(NonComposeBase {
                                app_port: __field0,
                                override_command: __field1,
                                run_args: __field2,
                                shutdown_action: __field3,
                                workspace_folder: __field4,
                                workspace_mount: __field5,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<Vec<String>>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "appPort",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "overrideCommand",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "runArgs",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "shutdownAction",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "workspaceFolder",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "workspaceMount",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("appPort") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "overrideCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("runArgs") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "shutdownAction",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "workspaceFolder",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "workspaceMount",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(NonComposeBase {
                                app_port: __field0,
                                override_command: __field1,
                                run_args: __field2,
                                shutdown_action: __field3,
                                workspace_folder: __field4,
                                workspace_mount: __field5,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "appPort",
                        "overrideCommand",
                        "runArgs",
                        "shutdownAction",
                        "workspaceFolder",
                        "workspaceMount",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "nonComposeBase",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<NonComposeBase>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for NonComposeBase {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "nonComposeBase",
                        false as usize
                            + if Option::is_none(&self.app_port) { 0 } else { 1 }
                            + if Option::is_none(&self.override_command) { 0 } else { 1 }
                            + if Option::is_none(&self.run_args) { 0 } else { 1 }
                            + if Option::is_none(&self.shutdown_action) { 0 } else { 1 }
                            + if Option::is_none(&self.workspace_folder) { 0 } else { 1 }
                            + if Option::is_none(&self.workspace_mount) { 0 } else { 1 },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.app_port) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "appPort",
                            &self.app_port,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "appPort",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.override_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "overrideCommand",
                            &self.override_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "overrideCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.run_args) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "runArgs",
                            &self.run_args,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "runArgs",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.shutdown_action) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "shutdownAction",
                            &self.shutdown_action,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "shutdownAction",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.workspace_folder) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "workspaceFolder",
                            &self.workspace_folder,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "workspaceFolder",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.workspace_mount) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "workspaceMount",
                            &self.workspace_mount,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "workspaceMount",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct DevContainerBaseWorkspaceMountVariant0Features {
            #[serde(skip_serializing_if = "Option::is_none")]
            pub fish: Option<serde_json::Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub gradle: Option<serde_json::Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub homebrew: Option<serde_json::Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub jupyterlab: Option<serde_json::Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub maven: Option<serde_json::Value>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DevContainerBaseWorkspaceMountVariant0Features {
            #[inline]
            fn clone(&self) -> DevContainerBaseWorkspaceMountVariant0Features {
                DevContainerBaseWorkspaceMountVariant0Features {
                    fish: ::core::clone::Clone::clone(&self.fish),
                    gradle: ::core::clone::Clone::clone(&self.gradle),
                    homebrew: ::core::clone::Clone::clone(&self.homebrew),
                    jupyterlab: ::core::clone::Clone::clone(&self.jupyterlab),
                    maven: ::core::clone::Clone::clone(&self.maven),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq
        for DevContainerBaseWorkspaceMountVariant0Features {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DevContainerBaseWorkspaceMountVariant0Features {
            #[inline]
            fn eq(
                &self,
                other: &DevContainerBaseWorkspaceMountVariant0Features,
            ) -> bool {
                self.fish == other.fish && self.gradle == other.gradle
                    && self.homebrew == other.homebrew
                    && self.jupyterlab == other.jupyterlab && self.maven == other.maven
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DevContainerBaseWorkspaceMountVariant0Features {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "DevContainerBaseWorkspaceMountVariant0Features",
                    "fish",
                    &self.fish,
                    "gradle",
                    &self.gradle,
                    "homebrew",
                    &self.homebrew,
                    "jupyterlab",
                    &self.jupyterlab,
                    "maven",
                    &&self.maven,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default
        for DevContainerBaseWorkspaceMountVariant0Features {
            #[inline]
            fn default() -> DevContainerBaseWorkspaceMountVariant0Features {
                DevContainerBaseWorkspaceMountVariant0Features {
                    fish: ::core::default::Default::default(),
                    gradle: ::core::default::Default::default(),
                    homebrew: ::core::default::Default::default(),
                    jupyterlab: ::core::default::Default::default(),
                    maven: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de>
            for DevContainerBaseWorkspaceMountVariant0Features {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "fish" => _serde::__private::Ok(__Field::__field0),
                                "gradle" => _serde::__private::Ok(__Field::__field1),
                                "homebrew" => _serde::__private::Ok(__Field::__field2),
                                "jupyterlab" => _serde::__private::Ok(__Field::__field3),
                                "maven" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"fish" => _serde::__private::Ok(__Field::__field0),
                                b"gradle" => _serde::__private::Ok(__Field::__field1),
                                b"homebrew" => _serde::__private::Ok(__Field::__field2),
                                b"jupyterlab" => _serde::__private::Ok(__Field::__field3),
                                b"maven" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            DevContainerBaseWorkspaceMountVariant0Features,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DevContainerBaseWorkspaceMountVariant0Features;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DevContainerBaseWorkspaceMountVariant0Features",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0Features with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0Features with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0Features with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0Features with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0Features with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DevContainerBaseWorkspaceMountVariant0Features {
                                fish: __field0,
                                gradle: __field1,
                                homebrew: __field2,
                                jupyterlab: __field3,
                                maven: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("fish"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("gradle"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "homebrew",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "jupyterlab",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("maven"),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("fish") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("gradle") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("homebrew") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("jupyterlab") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("maven") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(DevContainerBaseWorkspaceMountVariant0Features {
                                fish: __field0,
                                gradle: __field1,
                                homebrew: __field2,
                                jupyterlab: __field3,
                                maven: __field4,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "fish",
                        "gradle",
                        "homebrew",
                        "jupyterlab",
                        "maven",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DevContainerBaseWorkspaceMountVariant0Features",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                DevContainerBaseWorkspaceMountVariant0Features,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DevContainerBaseWorkspaceMountVariant0Features {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DevContainerBaseWorkspaceMountVariant0Features",
                        false as usize + if Option::is_none(&self.fish) { 0 } else { 1 }
                            + if Option::is_none(&self.gradle) { 0 } else { 1 }
                            + if Option::is_none(&self.homebrew) { 0 } else { 1 }
                            + if Option::is_none(&self.jupyterlab) { 0 } else { 1 }
                            + if Option::is_none(&self.maven) { 0 } else { 1 },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.fish) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "fish",
                            &self.fish,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "fish",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.gradle) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "gradle",
                            &self.gradle,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "gradle",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.homebrew) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "homebrew",
                            &self.homebrew,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "homebrew",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.jupyterlab) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "jupyterlab",
                            &self.jupyterlab,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "jupyterlab",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.maven) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "maven",
                            &self.maven,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "maven",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub type DevContainerBaseWorkspaceMountVariant0ItemForwardPortsVariant0 = i64;
        pub type DevContainerBaseWorkspaceMountVariant0ItemForwardPortsVariant1 = String;
        #[serde(untagged)]
        pub enum DevContainerBaseWorkspaceMountVariant0ItemForwardPorts {
            Variant0(DevContainerBaseWorkspaceMountVariant0ItemForwardPortsVariant0),
            Variant1(DevContainerBaseWorkspaceMountVariant0ItemForwardPortsVariant1),
        }
        #[automatically_derived]
        impl ::core::clone::Clone
        for DevContainerBaseWorkspaceMountVariant0ItemForwardPorts {
            #[inline]
            fn clone(&self) -> DevContainerBaseWorkspaceMountVariant0ItemForwardPorts {
                match self {
                    DevContainerBaseWorkspaceMountVariant0ItemForwardPorts::Variant0(
                        __self_0,
                    ) => {
                        DevContainerBaseWorkspaceMountVariant0ItemForwardPorts::Variant0(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    DevContainerBaseWorkspaceMountVariant0ItemForwardPorts::Variant1(
                        __self_0,
                    ) => {
                        DevContainerBaseWorkspaceMountVariant0ItemForwardPorts::Variant1(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq
        for DevContainerBaseWorkspaceMountVariant0ItemForwardPorts {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq
        for DevContainerBaseWorkspaceMountVariant0ItemForwardPorts {
            #[inline]
            fn eq(
                &self,
                other: &DevContainerBaseWorkspaceMountVariant0ItemForwardPorts,
            ) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (
                            DevContainerBaseWorkspaceMountVariant0ItemForwardPorts::Variant0(
                                __self_0,
                            ),
                            DevContainerBaseWorkspaceMountVariant0ItemForwardPorts::Variant0(
                                __arg1_0,
                            ),
                        ) => *__self_0 == *__arg1_0,
                        (
                            DevContainerBaseWorkspaceMountVariant0ItemForwardPorts::Variant1(
                                __self_0,
                            ),
                            DevContainerBaseWorkspaceMountVariant0ItemForwardPorts::Variant1(
                                __arg1_0,
                            ),
                        ) => *__self_0 == *__arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug
        for DevContainerBaseWorkspaceMountVariant0ItemForwardPorts {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    DevContainerBaseWorkspaceMountVariant0ItemForwardPorts::Variant0(
                        __self_0,
                    ) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variant0",
                            &__self_0,
                        )
                    }
                    DevContainerBaseWorkspaceMountVariant0ItemForwardPorts::Variant1(
                        __self_0,
                    ) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variant1",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de>
            for DevContainerBaseWorkspaceMountVariant0ItemForwardPorts {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    let __content = match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                        __deserializer,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if let _serde::__private::Ok(__ok)
                        = _serde::__private::Result::map(
                            <DevContainerBaseWorkspaceMountVariant0ItemForwardPortsVariant0 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<
                                    __D::Error,
                                >::new(&__content),
                            ),
                            DevContainerBaseWorkspaceMountVariant0ItemForwardPorts::Variant0,
                        ) {
                        return _serde::__private::Ok(__ok);
                    }
                    if let _serde::__private::Ok(__ok)
                        = _serde::__private::Result::map(
                            <DevContainerBaseWorkspaceMountVariant0ItemForwardPortsVariant1 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<
                                    __D::Error,
                                >::new(&__content),
                            ),
                            DevContainerBaseWorkspaceMountVariant0ItemForwardPorts::Variant1,
                        ) {
                        return _serde::__private::Ok(__ok);
                    }
                    _serde::__private::Err(
                        _serde::de::Error::custom(
                            "data did not match any variant of untagged enum DevContainerBaseWorkspaceMountVariant0ItemForwardPorts",
                        ),
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize
            for DevContainerBaseWorkspaceMountVariant0ItemForwardPorts {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        DevContainerBaseWorkspaceMountVariant0ItemForwardPorts::Variant0(
                            ref __field0,
                        ) => _serde::Serialize::serialize(__field0, __serializer),
                        DevContainerBaseWorkspaceMountVariant0ItemForwardPorts::Variant1(
                            ref __field0,
                        ) => _serde::Serialize::serialize(__field0, __serializer),
                    }
                }
            }
        };
        pub type DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant0 = bool;
        pub type DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant1 = String;
        pub struct DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 {
            /// Number of required cores.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cores: Option<i64>,
            /// Amount of required RAM in bytes. Supports units tb, gb, mb and kb.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub memory: Option<String>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone
        for DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 {
            #[inline]
            fn clone(
                &self,
            ) -> DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 {
                DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 {
                    cores: ::core::clone::Clone::clone(&self.cores),
                    memory: ::core::clone::Clone::clone(&self.memory),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq
        for DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq
        for DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 {
            #[inline]
            fn eq(
                &self,
                other: &DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2,
            ) -> bool {
                self.cores == other.cores && self.memory == other.memory
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug
        for DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2",
                    "cores",
                    &self.cores,
                    "memory",
                    &&self.memory,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default
        for DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 {
            #[inline]
            fn default() -> DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 {
                DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 {
                    cores: ::core::default::Default::default(),
                    memory: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de>
            for DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "cores" => _serde::__private::Ok(__Field::__field0),
                                "memory" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"cores" => _serde::__private::Ok(__Field::__field0),
                                b"memory" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<i64>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 {
                                cores: __field0,
                                memory: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Option<i64>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("cores"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<i64>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("memory"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("cores") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("memory") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 {
                                cores: __field0,
                                memory: __field1,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["cores", "memory"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize
            for DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2",
                        false as usize + if Option::is_none(&self.cores) { 0 } else { 1 }
                            + if Option::is_none(&self.memory) { 0 } else { 1 },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.cores) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "cores",
                            &self.cores,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "cores",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.memory) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "memory",
                            &self.memory,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "memory",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[serde(untagged)]
        pub enum DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu {
            Variant0(DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant0),
            Variant1(DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant1),
            Variant2(DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2),
        }
        #[automatically_derived]
        impl ::core::clone::Clone
        for DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu {
            #[inline]
            fn clone(
                &self,
            ) -> DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu {
                match self {
                    DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant0(
                        __self_0,
                    ) => {
                        DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant0(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant1(
                        __self_0,
                    ) => {
                        DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant1(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant2(
                        __self_0,
                    ) => {
                        DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant2(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq
        for DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq
        for DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu {
            #[inline]
            fn eq(
                &self,
                other: &DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu,
            ) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (
                            DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant0(
                                __self_0,
                            ),
                            DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant0(
                                __arg1_0,
                            ),
                        ) => *__self_0 == *__arg1_0,
                        (
                            DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant1(
                                __self_0,
                            ),
                            DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant1(
                                __arg1_0,
                            ),
                        ) => *__self_0 == *__arg1_0,
                        (
                            DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant2(
                                __self_0,
                            ),
                            DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant2(
                                __arg1_0,
                            ),
                        ) => *__self_0 == *__arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug
        for DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant0(
                        __self_0,
                    ) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variant0",
                            &__self_0,
                        )
                    }
                    DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant1(
                        __self_0,
                    ) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variant1",
                            &__self_0,
                        )
                    }
                    DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant2(
                        __self_0,
                    ) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variant2",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de>
            for DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    let __content = match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                        __deserializer,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if let _serde::__private::Ok(__ok)
                        = _serde::__private::Result::map(
                            <DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant0 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<
                                    __D::Error,
                                >::new(&__content),
                            ),
                            DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant0,
                        ) {
                        return _serde::__private::Ok(__ok);
                    }
                    if let _serde::__private::Ok(__ok)
                        = _serde::__private::Result::map(
                            <DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant1 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<
                                    __D::Error,
                                >::new(&__content),
                            ),
                            DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant1,
                        ) {
                        return _serde::__private::Ok(__ok);
                    }
                    if let _serde::__private::Ok(__ok)
                        = _serde::__private::Result::map(
                            <DevContainerBaseWorkspaceMountVariant0HostRequirementsGpuVariant2 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<
                                    __D::Error,
                                >::new(&__content),
                            ),
                            DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant2,
                        ) {
                        return _serde::__private::Ok(__ok);
                    }
                    _serde::__private::Err(
                        _serde::de::Error::custom(
                            "data did not match any variant of untagged enum DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu",
                        ),
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize
            for DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant0(
                            ref __field0,
                        ) => _serde::Serialize::serialize(__field0, __serializer),
                        DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant1(
                            ref __field0,
                        ) => _serde::Serialize::serialize(__field0, __serializer),
                        DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu::Variant2(
                            ref __field0,
                        ) => _serde::Serialize::serialize(__field0, __serializer),
                    }
                }
            }
        };
        pub struct DevContainerBaseWorkspaceMountVariant0HostRequirements {
            /// Number of required CPUs.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub cpus: Option<i64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub gpu: Option<DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu>,
            /// Amount of required RAM in bytes. Supports units tb, gb, mb and kb.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub memory: Option<String>,
            /// Amount of required disk space in bytes. Supports units tb, gb, mb and kb.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub storage: Option<String>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone
        for DevContainerBaseWorkspaceMountVariant0HostRequirements {
            #[inline]
            fn clone(&self) -> DevContainerBaseWorkspaceMountVariant0HostRequirements {
                DevContainerBaseWorkspaceMountVariant0HostRequirements {
                    cpus: ::core::clone::Clone::clone(&self.cpus),
                    gpu: ::core::clone::Clone::clone(&self.gpu),
                    memory: ::core::clone::Clone::clone(&self.memory),
                    storage: ::core::clone::Clone::clone(&self.storage),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq
        for DevContainerBaseWorkspaceMountVariant0HostRequirements {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq
        for DevContainerBaseWorkspaceMountVariant0HostRequirements {
            #[inline]
            fn eq(
                &self,
                other: &DevContainerBaseWorkspaceMountVariant0HostRequirements,
            ) -> bool {
                self.cpus == other.cpus && self.gpu == other.gpu
                    && self.memory == other.memory && self.storage == other.storage
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug
        for DevContainerBaseWorkspaceMountVariant0HostRequirements {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "DevContainerBaseWorkspaceMountVariant0HostRequirements",
                    "cpus",
                    &self.cpus,
                    "gpu",
                    &self.gpu,
                    "memory",
                    &self.memory,
                    "storage",
                    &&self.storage,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default
        for DevContainerBaseWorkspaceMountVariant0HostRequirements {
            #[inline]
            fn default() -> DevContainerBaseWorkspaceMountVariant0HostRequirements {
                DevContainerBaseWorkspaceMountVariant0HostRequirements {
                    cpus: ::core::default::Default::default(),
                    gpu: ::core::default::Default::default(),
                    memory: ::core::default::Default::default(),
                    storage: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de>
            for DevContainerBaseWorkspaceMountVariant0HostRequirements {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "cpus" => _serde::__private::Ok(__Field::__field0),
                                "gpu" => _serde::__private::Ok(__Field::__field1),
                                "memory" => _serde::__private::Ok(__Field::__field2),
                                "storage" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"cpus" => _serde::__private::Ok(__Field::__field0),
                                b"gpu" => _serde::__private::Ok(__Field::__field1),
                                b"memory" => _serde::__private::Ok(__Field::__field2),
                                b"storage" => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            DevContainerBaseWorkspaceMountVariant0HostRequirements,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DevContainerBaseWorkspaceMountVariant0HostRequirements;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DevContainerBaseWorkspaceMountVariant0HostRequirements",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<i64>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0HostRequirements with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<
                                    DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu,
                                >,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0HostRequirements with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0HostRequirements with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0HostRequirements with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DevContainerBaseWorkspaceMountVariant0HostRequirements {
                                cpus: __field0,
                                gpu: __field1,
                                memory: __field2,
                                storage: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Option<i64>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<
                                    DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu,
                                >,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("cpus"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<i64>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("gpu"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<
                                                    DevContainerBaseWorkspaceMountVariant0HostRequirementsGpu,
                                                >,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("memory"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "storage",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("cpus") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("gpu") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("memory") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("storage") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(DevContainerBaseWorkspaceMountVariant0HostRequirements {
                                cpus: __field0,
                                gpu: __field1,
                                memory: __field2,
                                storage: __field3,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "cpus",
                        "gpu",
                        "memory",
                        "storage",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DevContainerBaseWorkspaceMountVariant0HostRequirements",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                DevContainerBaseWorkspaceMountVariant0HostRequirements,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize
            for DevContainerBaseWorkspaceMountVariant0HostRequirements {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DevContainerBaseWorkspaceMountVariant0HostRequirements",
                        false as usize + if Option::is_none(&self.cpus) { 0 } else { 1 }
                            + if Option::is_none(&self.gpu) { 0 } else { 1 }
                            + if Option::is_none(&self.memory) { 0 } else { 1 }
                            + if Option::is_none(&self.storage) { 0 } else { 1 },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.cpus) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "cpus",
                            &self.cpus,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "cpus",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.gpu) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "gpu",
                            &self.gpu,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "gpu",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.memory) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "memory",
                            &self.memory,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "memory",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.storage) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "storage",
                            &self.storage,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "storage",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes {
            /// Automatically prompt for elevation (if needed) when this port is forwarded. Elevate is
            /// required if the local port is a privileged port.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "elevateIfNeeded")]
            pub elevate_if_needed: Option<bool>,
            /// Label that will be shown in the UI for this port.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub label: Option<String>,
            /// Defines the action that occurs when the port is discovered for automatic forwarding
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "onAutoForward")]
            pub on_auto_forward: Option<String>,
            /// The protocol to use when forwarding this port.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub protocol: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "requireLocalPort")]
            pub require_local_port: Option<bool>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone
        for DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes {
            #[inline]
            fn clone(
                &self,
            ) -> DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes {
                DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes {
                    elevate_if_needed: ::core::clone::Clone::clone(
                        &self.elevate_if_needed,
                    ),
                    label: ::core::clone::Clone::clone(&self.label),
                    on_auto_forward: ::core::clone::Clone::clone(&self.on_auto_forward),
                    protocol: ::core::clone::Clone::clone(&self.protocol),
                    require_local_port: ::core::clone::Clone::clone(
                        &self.require_local_port,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq
        for DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq
        for DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes {
            #[inline]
            fn eq(
                &self,
                other: &DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes,
            ) -> bool {
                self.elevate_if_needed == other.elevate_if_needed
                    && self.label == other.label
                    && self.on_auto_forward == other.on_auto_forward
                    && self.protocol == other.protocol
                    && self.require_local_port == other.require_local_port
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug
        for DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes",
                    "elevate_if_needed",
                    &self.elevate_if_needed,
                    "label",
                    &self.label,
                    "on_auto_forward",
                    &self.on_auto_forward,
                    "protocol",
                    &self.protocol,
                    "require_local_port",
                    &&self.require_local_port,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default
        for DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes {
            #[inline]
            fn default() -> DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes {
                DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes {
                    elevate_if_needed: ::core::default::Default::default(),
                    label: ::core::default::Default::default(),
                    on_auto_forward: ::core::default::Default::default(),
                    protocol: ::core::default::Default::default(),
                    require_local_port: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de>
            for DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "elevateIfNeeded" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                "label" => _serde::__private::Ok(__Field::__field1),
                                "onAutoForward" => _serde::__private::Ok(__Field::__field2),
                                "protocol" => _serde::__private::Ok(__Field::__field3),
                                "requireLocalPort" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"elevateIfNeeded" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                b"label" => _serde::__private::Ok(__Field::__field1),
                                b"onAutoForward" => _serde::__private::Ok(__Field::__field2),
                                b"protocol" => _serde::__private::Ok(__Field::__field3),
                                b"requireLocalPort" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes {
                                elevate_if_needed: __field0,
                                label: __field1,
                                on_auto_forward: __field2,
                                protocol: __field3,
                                require_local_port: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "elevateIfNeeded",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("label"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "onAutoForward",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "protocol",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "requireLocalPort",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "elevateIfNeeded",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("label") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "onAutoForward",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("protocol") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "requireLocalPort",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes {
                                elevate_if_needed: __field0,
                                label: __field1,
                                on_auto_forward: __field2,
                                protocol: __field3,
                                require_local_port: __field4,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "elevateIfNeeded",
                        "label",
                        "onAutoForward",
                        "protocol",
                        "requireLocalPort",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize
            for DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes",
                        false as usize
                            + if Option::is_none(&self.elevate_if_needed) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.label) { 0 } else { 1 }
                            + if Option::is_none(&self.on_auto_forward) { 0 } else { 1 }
                            + if Option::is_none(&self.protocol) { 0 } else { 1 }
                            + if Option::is_none(&self.require_local_port) {
                                0
                            } else {
                                1
                            },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.elevate_if_needed) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "elevateIfNeeded",
                            &self.elevate_if_needed,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "elevateIfNeeded",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.label) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "label",
                            &self.label,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "label",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.on_auto_forward) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "onAutoForward",
                            &self.on_auto_forward,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "onAutoForward",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.protocol) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "protocol",
                            &self.protocol,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "protocol",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.require_local_port) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "requireLocalPort",
                            &self.require_local_port,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "requireLocalPort",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct DevContainerBaseWorkspaceMountVariant0PortsAttributes {}
        #[automatically_derived]
        impl ::core::clone::Clone
        for DevContainerBaseWorkspaceMountVariant0PortsAttributes {
            #[inline]
            fn clone(&self) -> DevContainerBaseWorkspaceMountVariant0PortsAttributes {
                DevContainerBaseWorkspaceMountVariant0PortsAttributes {
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq
        for DevContainerBaseWorkspaceMountVariant0PortsAttributes {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq
        for DevContainerBaseWorkspaceMountVariant0PortsAttributes {
            #[inline]
            fn eq(
                &self,
                other: &DevContainerBaseWorkspaceMountVariant0PortsAttributes,
            ) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug
        for DevContainerBaseWorkspaceMountVariant0PortsAttributes {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    "DevContainerBaseWorkspaceMountVariant0PortsAttributes",
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default
        for DevContainerBaseWorkspaceMountVariant0PortsAttributes {
            #[inline]
            fn default() -> DevContainerBaseWorkspaceMountVariant0PortsAttributes {
                DevContainerBaseWorkspaceMountVariant0PortsAttributes {
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de>
            for DevContainerBaseWorkspaceMountVariant0PortsAttributes {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            DevContainerBaseWorkspaceMountVariant0PortsAttributes,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DevContainerBaseWorkspaceMountVariant0PortsAttributes;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DevContainerBaseWorkspaceMountVariant0PortsAttributes",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            _: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            _serde::__private::Ok(DevContainerBaseWorkspaceMountVariant0PortsAttributes {})
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            _serde::__private::Ok(DevContainerBaseWorkspaceMountVariant0PortsAttributes {})
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DevContainerBaseWorkspaceMountVariant0PortsAttributes",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                DevContainerBaseWorkspaceMountVariant0PortsAttributes,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize
            for DevContainerBaseWorkspaceMountVariant0PortsAttributes {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DevContainerBaseWorkspaceMountVariant0PortsAttributes",
                        false as usize,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct DevContainerBaseWorkspaceMountVariant0 {
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "additionalProperties")]
            pub additional_properties: Option<
                ::std::collections::BTreeMap<String, serde_json::Value>,
            >,
            /// Passes docker capabilities to include when creating the dev container.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "capAdd")]
            pub cap_add: Option<Vec<String>>,
            /// Container environment variables.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "containerEnv")]
            pub container_env: Option<::std::collections::BTreeMap<String, String>>,
            /// The user the container will be started with. The default is the user on the Docker image.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "containerUser")]
            pub container_user: Option<String>,
            /// Tool-specific configuration. Each tool should use a JSON object subproperty with a unique
            /// name to group its customizations.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub customizations: Option<
                ::std::collections::BTreeMap<String, serde_json::Value>,
            >,
            /// Features to add to the dev container.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub features: Option<DevContainerBaseWorkspaceMountVariant0Features>,
            /// Ports that are forwarded from the container to the local machine. Can be an integer port
            /// number, or a string of the format "host:port_number".
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "forwardPorts")]
            pub forward_ports: Option<
                Vec<DevContainerBaseWorkspaceMountVariant0ItemForwardPorts>,
            >,
            /// Host hardware requirements.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "hostRequirements")]
            pub host_requirements: Option<
                DevContainerBaseWorkspaceMountVariant0HostRequirements,
            >,
            /// Passes the --init flag when creating the dev container.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub init: Option<bool>,
            /// A command to run locally before anything else. This command is run before
            /// "onCreateCommand". If this is a single string, it will be run in a shell. If this is an
            /// array of strings, it will be run as a single command without shell.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "initializeCommand")]
            pub initialize_command: Option<serde_json::Value>,
            /// Mount points to set up when creating the container. See Docker's documentation for the
            /// --mount option for the supported syntax.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub mounts: Option<Vec<serde_json::Value>>,
            /// A name for the dev container which can be displayed to the user.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: Option<String>,
            /// A command to run when creating the container. This command is run after "initializeCommand"
            /// and before "updateContentCommand". If this is a single string, it will be run in a shell.
            /// If this is an array of strings, it will be run as a single command without shell. If this
            /// is an object, each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "onCreateCommand")]
            pub on_create_command: Option<serde_json::Value>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "otherPortsAttributes")]
            pub other_ports_attributes: Option<
                DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes,
            >,
            /// Array consisting of the Feature id (without the semantic version) of Features in the order
            /// the user wants them to be installed.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "overrideFeatureInstallOrder")]
            pub override_feature_install_order: Option<Vec<String>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "portsAttributes")]
            pub ports_attributes: Option<
                DevContainerBaseWorkspaceMountVariant0PortsAttributes,
            >,
            /// A command to run when attaching to the container. This command is run after
            /// "postStartCommand". If this is a single string, it will be run in a shell. If this is an
            /// array of strings, it will be run as a single command without shell. If this is an object,
            /// each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "postAttachCommand")]
            pub post_attach_command: Option<serde_json::Value>,
            /// A command to run after creating the container. This command is run after
            /// "updateContentCommand" and before "postStartCommand". If this is a single string, it will
            /// be run in a shell. If this is an array of strings, it will be run as a single command
            /// without shell. If this is an object, each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "postCreateCommand")]
            pub post_create_command: Option<serde_json::Value>,
            /// A command to run after starting the container. This command is run after
            /// "postCreateCommand" and before "postAttachCommand". If this is a single string, it will be
            /// run in a shell. If this is an array of strings, it will be run as a single command without
            /// shell. If this is an object, each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "postStartCommand")]
            pub post_start_command: Option<serde_json::Value>,
            /// Passes the --privileged flag when creating the dev container.
            #[serde(skip_serializing_if = "Option::is_none")]
            pub privileged: Option<bool>,
            /// Remote environment variables to set for processes spawned in the container including
            /// lifecycle scripts and any remote editor/IDE server process.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "remoteEnv")]
            pub remote_env: Option<::std::collections::BTreeMap<String, Option<String>>>,
            /// The username to use for spawning processes in the container including lifecycle scripts and
            /// any remote editor/IDE server process. The default is the same user as the container.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "remoteUser")]
            pub remote_user: Option<String>,
            /// Passes docker security options to include when creating the dev container.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "securityOpt")]
            pub security_opt: Option<Vec<String>>,
            /// A command to run when creating the container and rerun when the workspace content was
            /// updated while creating the container. This command is run after "onCreateCommand" and
            /// before "postCreateCommand". If this is a single string, it will be run in a shell. If this
            /// is an array of strings, it will be run as a single command without shell. If this is an
            /// object, each provided command will be run in parallel.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "updateContentCommand")]
            pub update_content_command: Option<serde_json::Value>,
            /// Controls whether on Linux the container's user should be updated with the local user's UID
            /// and GID. On by default when opening from a local folder.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "updateRemoteUserUID")]
            pub update_remote_user_uid: Option<bool>,
            /// User environment probe to run. The default is "loginInteractiveShell".
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "userEnvProbe")]
            pub user_env_probe: Option<String>,
            /// The user command to wait for before continuing execution in the background while the UI is
            /// starting up. The default is "updateContentCommand".
            #[serde(skip_serializing_if = "Option::is_none")]
            #[serde(rename = "waitFor")]
            pub wait_for: Option<String>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DevContainerBaseWorkspaceMountVariant0 {
            #[inline]
            fn clone(&self) -> DevContainerBaseWorkspaceMountVariant0 {
                DevContainerBaseWorkspaceMountVariant0 {
                    additional_properties: ::core::clone::Clone::clone(
                        &self.additional_properties,
                    ),
                    cap_add: ::core::clone::Clone::clone(&self.cap_add),
                    container_env: ::core::clone::Clone::clone(&self.container_env),
                    container_user: ::core::clone::Clone::clone(&self.container_user),
                    customizations: ::core::clone::Clone::clone(&self.customizations),
                    features: ::core::clone::Clone::clone(&self.features),
                    forward_ports: ::core::clone::Clone::clone(&self.forward_ports),
                    host_requirements: ::core::clone::Clone::clone(
                        &self.host_requirements,
                    ),
                    init: ::core::clone::Clone::clone(&self.init),
                    initialize_command: ::core::clone::Clone::clone(
                        &self.initialize_command,
                    ),
                    mounts: ::core::clone::Clone::clone(&self.mounts),
                    name: ::core::clone::Clone::clone(&self.name),
                    on_create_command: ::core::clone::Clone::clone(
                        &self.on_create_command,
                    ),
                    other_ports_attributes: ::core::clone::Clone::clone(
                        &self.other_ports_attributes,
                    ),
                    override_feature_install_order: ::core::clone::Clone::clone(
                        &self.override_feature_install_order,
                    ),
                    ports_attributes: ::core::clone::Clone::clone(
                        &self.ports_attributes,
                    ),
                    post_attach_command: ::core::clone::Clone::clone(
                        &self.post_attach_command,
                    ),
                    post_create_command: ::core::clone::Clone::clone(
                        &self.post_create_command,
                    ),
                    post_start_command: ::core::clone::Clone::clone(
                        &self.post_start_command,
                    ),
                    privileged: ::core::clone::Clone::clone(&self.privileged),
                    remote_env: ::core::clone::Clone::clone(&self.remote_env),
                    remote_user: ::core::clone::Clone::clone(&self.remote_user),
                    security_opt: ::core::clone::Clone::clone(&self.security_opt),
                    update_content_command: ::core::clone::Clone::clone(
                        &self.update_content_command,
                    ),
                    update_remote_user_uid: ::core::clone::Clone::clone(
                        &self.update_remote_user_uid,
                    ),
                    user_env_probe: ::core::clone::Clone::clone(&self.user_env_probe),
                    wait_for: ::core::clone::Clone::clone(&self.wait_for),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq
        for DevContainerBaseWorkspaceMountVariant0 {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DevContainerBaseWorkspaceMountVariant0 {
            #[inline]
            fn eq(&self, other: &DevContainerBaseWorkspaceMountVariant0) -> bool {
                self.additional_properties == other.additional_properties
                    && self.cap_add == other.cap_add
                    && self.container_env == other.container_env
                    && self.container_user == other.container_user
                    && self.customizations == other.customizations
                    && self.features == other.features
                    && self.forward_ports == other.forward_ports
                    && self.host_requirements == other.host_requirements
                    && self.init == other.init
                    && self.initialize_command == other.initialize_command
                    && self.mounts == other.mounts && self.name == other.name
                    && self.on_create_command == other.on_create_command
                    && self.other_ports_attributes == other.other_ports_attributes
                    && self.override_feature_install_order
                        == other.override_feature_install_order
                    && self.ports_attributes == other.ports_attributes
                    && self.post_attach_command == other.post_attach_command
                    && self.post_create_command == other.post_create_command
                    && self.post_start_command == other.post_start_command
                    && self.privileged == other.privileged
                    && self.remote_env == other.remote_env
                    && self.remote_user == other.remote_user
                    && self.security_opt == other.security_opt
                    && self.update_content_command == other.update_content_command
                    && self.update_remote_user_uid == other.update_remote_user_uid
                    && self.user_env_probe == other.user_env_probe
                    && self.wait_for == other.wait_for
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DevContainerBaseWorkspaceMountVariant0 {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "additional_properties",
                    "cap_add",
                    "container_env",
                    "container_user",
                    "customizations",
                    "features",
                    "forward_ports",
                    "host_requirements",
                    "init",
                    "initialize_command",
                    "mounts",
                    "name",
                    "on_create_command",
                    "other_ports_attributes",
                    "override_feature_install_order",
                    "ports_attributes",
                    "post_attach_command",
                    "post_create_command",
                    "post_start_command",
                    "privileged",
                    "remote_env",
                    "remote_user",
                    "security_opt",
                    "update_content_command",
                    "update_remote_user_uid",
                    "user_env_probe",
                    "wait_for",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.additional_properties,
                    &self.cap_add,
                    &self.container_env,
                    &self.container_user,
                    &self.customizations,
                    &self.features,
                    &self.forward_ports,
                    &self.host_requirements,
                    &self.init,
                    &self.initialize_command,
                    &self.mounts,
                    &self.name,
                    &self.on_create_command,
                    &self.other_ports_attributes,
                    &self.override_feature_install_order,
                    &self.ports_attributes,
                    &self.post_attach_command,
                    &self.post_create_command,
                    &self.post_start_command,
                    &self.privileged,
                    &self.remote_env,
                    &self.remote_user,
                    &self.security_opt,
                    &self.update_content_command,
                    &self.update_remote_user_uid,
                    &self.user_env_probe,
                    &&self.wait_for,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "DevContainerBaseWorkspaceMountVariant0",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DevContainerBaseWorkspaceMountVariant0 {
            #[inline]
            fn default() -> DevContainerBaseWorkspaceMountVariant0 {
                DevContainerBaseWorkspaceMountVariant0 {
                    additional_properties: ::core::default::Default::default(),
                    cap_add: ::core::default::Default::default(),
                    container_env: ::core::default::Default::default(),
                    container_user: ::core::default::Default::default(),
                    customizations: ::core::default::Default::default(),
                    features: ::core::default::Default::default(),
                    forward_ports: ::core::default::Default::default(),
                    host_requirements: ::core::default::Default::default(),
                    init: ::core::default::Default::default(),
                    initialize_command: ::core::default::Default::default(),
                    mounts: ::core::default::Default::default(),
                    name: ::core::default::Default::default(),
                    on_create_command: ::core::default::Default::default(),
                    other_ports_attributes: ::core::default::Default::default(),
                    override_feature_install_order: ::core::default::Default::default(),
                    ports_attributes: ::core::default::Default::default(),
                    post_attach_command: ::core::default::Default::default(),
                    post_create_command: ::core::default::Default::default(),
                    post_start_command: ::core::default::Default::default(),
                    privileged: ::core::default::Default::default(),
                    remote_env: ::core::default::Default::default(),
                    remote_user: ::core::default::Default::default(),
                    security_opt: ::core::default::Default::default(),
                    update_content_command: ::core::default::Default::default(),
                    update_remote_user_uid: ::core::default::Default::default(),
                    user_env_probe: ::core::default::Default::default(),
                    wait_for: ::core::default::Default::default(),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de>
            for DevContainerBaseWorkspaceMountVariant0 {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
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
                        __field12,
                        __field13,
                        __field14,
                        __field15,
                        __field16,
                        __field17,
                        __field18,
                        __field19,
                        __field20,
                        __field21,
                        __field22,
                        __field23,
                        __field24,
                        __field25,
                        __field26,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                10u64 => _serde::__private::Ok(__Field::__field10),
                                11u64 => _serde::__private::Ok(__Field::__field11),
                                12u64 => _serde::__private::Ok(__Field::__field12),
                                13u64 => _serde::__private::Ok(__Field::__field13),
                                14u64 => _serde::__private::Ok(__Field::__field14),
                                15u64 => _serde::__private::Ok(__Field::__field15),
                                16u64 => _serde::__private::Ok(__Field::__field16),
                                17u64 => _serde::__private::Ok(__Field::__field17),
                                18u64 => _serde::__private::Ok(__Field::__field18),
                                19u64 => _serde::__private::Ok(__Field::__field19),
                                20u64 => _serde::__private::Ok(__Field::__field20),
                                21u64 => _serde::__private::Ok(__Field::__field21),
                                22u64 => _serde::__private::Ok(__Field::__field22),
                                23u64 => _serde::__private::Ok(__Field::__field23),
                                24u64 => _serde::__private::Ok(__Field::__field24),
                                25u64 => _serde::__private::Ok(__Field::__field25),
                                26u64 => _serde::__private::Ok(__Field::__field26),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "additionalProperties" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                "capAdd" => _serde::__private::Ok(__Field::__field1),
                                "containerEnv" => _serde::__private::Ok(__Field::__field2),
                                "containerUser" => _serde::__private::Ok(__Field::__field3),
                                "customizations" => _serde::__private::Ok(__Field::__field4),
                                "features" => _serde::__private::Ok(__Field::__field5),
                                "forwardPorts" => _serde::__private::Ok(__Field::__field6),
                                "hostRequirements" => {
                                    _serde::__private::Ok(__Field::__field7)
                                }
                                "init" => _serde::__private::Ok(__Field::__field8),
                                "initializeCommand" => {
                                    _serde::__private::Ok(__Field::__field9)
                                }
                                "mounts" => _serde::__private::Ok(__Field::__field10),
                                "name" => _serde::__private::Ok(__Field::__field11),
                                "onCreateCommand" => {
                                    _serde::__private::Ok(__Field::__field12)
                                }
                                "otherPortsAttributes" => {
                                    _serde::__private::Ok(__Field::__field13)
                                }
                                "overrideFeatureInstallOrder" => {
                                    _serde::__private::Ok(__Field::__field14)
                                }
                                "portsAttributes" => {
                                    _serde::__private::Ok(__Field::__field15)
                                }
                                "postAttachCommand" => {
                                    _serde::__private::Ok(__Field::__field16)
                                }
                                "postCreateCommand" => {
                                    _serde::__private::Ok(__Field::__field17)
                                }
                                "postStartCommand" => {
                                    _serde::__private::Ok(__Field::__field18)
                                }
                                "privileged" => _serde::__private::Ok(__Field::__field19),
                                "remoteEnv" => _serde::__private::Ok(__Field::__field20),
                                "remoteUser" => _serde::__private::Ok(__Field::__field21),
                                "securityOpt" => _serde::__private::Ok(__Field::__field22),
                                "updateContentCommand" => {
                                    _serde::__private::Ok(__Field::__field23)
                                }
                                "updateRemoteUserUID" => {
                                    _serde::__private::Ok(__Field::__field24)
                                }
                                "userEnvProbe" => _serde::__private::Ok(__Field::__field25),
                                "waitFor" => _serde::__private::Ok(__Field::__field26),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"additionalProperties" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                b"capAdd" => _serde::__private::Ok(__Field::__field1),
                                b"containerEnv" => _serde::__private::Ok(__Field::__field2),
                                b"containerUser" => _serde::__private::Ok(__Field::__field3),
                                b"customizations" => {
                                    _serde::__private::Ok(__Field::__field4)
                                }
                                b"features" => _serde::__private::Ok(__Field::__field5),
                                b"forwardPorts" => _serde::__private::Ok(__Field::__field6),
                                b"hostRequirements" => {
                                    _serde::__private::Ok(__Field::__field7)
                                }
                                b"init" => _serde::__private::Ok(__Field::__field8),
                                b"initializeCommand" => {
                                    _serde::__private::Ok(__Field::__field9)
                                }
                                b"mounts" => _serde::__private::Ok(__Field::__field10),
                                b"name" => _serde::__private::Ok(__Field::__field11),
                                b"onCreateCommand" => {
                                    _serde::__private::Ok(__Field::__field12)
                                }
                                b"otherPortsAttributes" => {
                                    _serde::__private::Ok(__Field::__field13)
                                }
                                b"overrideFeatureInstallOrder" => {
                                    _serde::__private::Ok(__Field::__field14)
                                }
                                b"portsAttributes" => {
                                    _serde::__private::Ok(__Field::__field15)
                                }
                                b"postAttachCommand" => {
                                    _serde::__private::Ok(__Field::__field16)
                                }
                                b"postCreateCommand" => {
                                    _serde::__private::Ok(__Field::__field17)
                                }
                                b"postStartCommand" => {
                                    _serde::__private::Ok(__Field::__field18)
                                }
                                b"privileged" => _serde::__private::Ok(__Field::__field19),
                                b"remoteEnv" => _serde::__private::Ok(__Field::__field20),
                                b"remoteUser" => _serde::__private::Ok(__Field::__field21),
                                b"securityOpt" => _serde::__private::Ok(__Field::__field22),
                                b"updateContentCommand" => {
                                    _serde::__private::Ok(__Field::__field23)
                                }
                                b"updateRemoteUserUID" => {
                                    _serde::__private::Ok(__Field::__field24)
                                }
                                b"userEnvProbe" => _serde::__private::Ok(__Field::__field25),
                                b"waitFor" => _serde::__private::Ok(__Field::__field26),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<
                            DevContainerBaseWorkspaceMountVariant0,
                        >,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = DevContainerBaseWorkspaceMountVariant0;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct DevContainerBaseWorkspaceMountVariant0",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<
                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                >,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<::std::collections::BTreeMap<String, String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<
                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                >,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                Option<DevContainerBaseWorkspaceMountVariant0Features>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<
                                Option<
                                    Vec<DevContainerBaseWorkspaceMountVariant0ItemForwardPorts>,
                                >,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<
                                Option<
                                    DevContainerBaseWorkspaceMountVariant0HostRequirements,
                                >,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field10 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<serde_json::Value>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            10usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field11 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            11usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field12 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            12usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field13 = match match _serde::de::SeqAccess::next_element::<
                                Option<
                                    DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes,
                                >,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            13usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field14 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            14usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field15 = match match _serde::de::SeqAccess::next_element::<
                                Option<
                                    DevContainerBaseWorkspaceMountVariant0PortsAttributes,
                                >,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            15usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field16 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            16usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field17 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            17usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field18 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            18usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field19 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            19usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field20 = match match _serde::de::SeqAccess::next_element::<
                                Option<::std::collections::BTreeMap<String, Option<String>>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            20usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field21 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            21usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field22 = match match _serde::de::SeqAccess::next_element::<
                                Option<Vec<String>>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            22usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field23 = match match _serde::de::SeqAccess::next_element::<
                                Option<serde_json::Value>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            23usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field24 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            24usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field25 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            25usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            let __field26 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            26usize,
                                            &"struct DevContainerBaseWorkspaceMountVariant0 with 27 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(DevContainerBaseWorkspaceMountVariant0 {
                                additional_properties: __field0,
                                cap_add: __field1,
                                container_env: __field2,
                                container_user: __field3,
                                customizations: __field4,
                                features: __field5,
                                forward_ports: __field6,
                                host_requirements: __field7,
                                init: __field8,
                                initialize_command: __field9,
                                mounts: __field10,
                                name: __field11,
                                on_create_command: __field12,
                                other_ports_attributes: __field13,
                                override_feature_install_order: __field14,
                                ports_attributes: __field15,
                                post_attach_command: __field16,
                                post_create_command: __field17,
                                post_start_command: __field18,
                                privileged: __field19,
                                remote_env: __field20,
                                remote_user: __field21,
                                security_opt: __field22,
                                update_content_command: __field23,
                                update_remote_user_uid: __field24,
                                user_env_probe: __field25,
                                wait_for: __field26,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<
                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                >,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<
                                Option<Vec<String>>,
                            > = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<::std::collections::BTreeMap<String, String>>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<
                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                >,
                            > = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<
                                Option<DevContainerBaseWorkspaceMountVariant0Features>,
                            > = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<
                                Option<
                                    Vec<DevContainerBaseWorkspaceMountVariant0ItemForwardPorts>,
                                >,
                            > = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<
                                Option<
                                    DevContainerBaseWorkspaceMountVariant0HostRequirements,
                                >,
                            > = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field9: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field10: _serde::__private::Option<
                                Option<Vec<serde_json::Value>>,
                            > = _serde::__private::None;
                            let mut __field11: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field12: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field13: _serde::__private::Option<
                                Option<
                                    DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes,
                                >,
                            > = _serde::__private::None;
                            let mut __field14: _serde::__private::Option<
                                Option<Vec<String>>,
                            > = _serde::__private::None;
                            let mut __field15: _serde::__private::Option<
                                Option<
                                    DevContainerBaseWorkspaceMountVariant0PortsAttributes,
                                >,
                            > = _serde::__private::None;
                            let mut __field16: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field17: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field18: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field19: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field20: _serde::__private::Option<
                                Option<::std::collections::BTreeMap<String, Option<String>>>,
                            > = _serde::__private::None;
                            let mut __field21: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field22: _serde::__private::Option<
                                Option<Vec<String>>,
                            > = _serde::__private::None;
                            let mut __field23: _serde::__private::Option<
                                Option<serde_json::Value>,
                            > = _serde::__private::None;
                            let mut __field24: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field25: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field26: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "additionalProperties",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<
                                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                                >,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("capAdd"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "containerEnv",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<::std::collections::BTreeMap<String, String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "containerUser",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "customizations",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<
                                                    ::std::collections::BTreeMap<String, serde_json::Value>,
                                                >,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "features",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<DevContainerBaseWorkspaceMountVariant0Features>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "forwardPorts",
                                                ),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<
                                                    Vec<DevContainerBaseWorkspaceMountVariant0ItemForwardPorts>,
                                                >,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "hostRequirements",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<
                                                    DevContainerBaseWorkspaceMountVariant0HostRequirements,
                                                >,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("init"),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "initializeCommand",
                                                ),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field10 => {
                                        if _serde::__private::Option::is_some(&__field10) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("mounts"),
                                            );
                                        }
                                        __field10 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<serde_json::Value>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field11 => {
                                        if _serde::__private::Option::is_some(&__field11) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field11 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field12 => {
                                        if _serde::__private::Option::is_some(&__field12) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "onCreateCommand",
                                                ),
                                            );
                                        }
                                        __field12 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field13 => {
                                        if _serde::__private::Option::is_some(&__field13) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "otherPortsAttributes",
                                                ),
                                            );
                                        }
                                        __field13 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<
                                                    DevContainerBaseWorkspaceMountVariant0OtherPortsAttributes,
                                                >,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field14 => {
                                        if _serde::__private::Option::is_some(&__field14) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "overrideFeatureInstallOrder",
                                                ),
                                            );
                                        }
                                        __field14 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field15 => {
                                        if _serde::__private::Option::is_some(&__field15) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "portsAttributes",
                                                ),
                                            );
                                        }
                                        __field15 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<
                                                    DevContainerBaseWorkspaceMountVariant0PortsAttributes,
                                                >,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field16 => {
                                        if _serde::__private::Option::is_some(&__field16) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "postAttachCommand",
                                                ),
                                            );
                                        }
                                        __field16 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field17 => {
                                        if _serde::__private::Option::is_some(&__field17) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "postCreateCommand",
                                                ),
                                            );
                                        }
                                        __field17 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field18 => {
                                        if _serde::__private::Option::is_some(&__field18) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "postStartCommand",
                                                ),
                                            );
                                        }
                                        __field18 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field19 => {
                                        if _serde::__private::Option::is_some(&__field19) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "privileged",
                                                ),
                                            );
                                        }
                                        __field19 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field20 => {
                                        if _serde::__private::Option::is_some(&__field20) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "remoteEnv",
                                                ),
                                            );
                                        }
                                        __field20 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<::std::collections::BTreeMap<String, Option<String>>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field21 => {
                                        if _serde::__private::Option::is_some(&__field21) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "remoteUser",
                                                ),
                                            );
                                        }
                                        __field21 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field22 => {
                                        if _serde::__private::Option::is_some(&__field22) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "securityOpt",
                                                ),
                                            );
                                        }
                                        __field22 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<Vec<String>>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field23 => {
                                        if _serde::__private::Option::is_some(&__field23) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "updateContentCommand",
                                                ),
                                            );
                                        }
                                        __field23 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<serde_json::Value>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field24 => {
                                        if _serde::__private::Option::is_some(&__field24) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "updateRemoteUserUID",
                                                ),
                                            );
                                        }
                                        __field24 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field25 => {
                                        if _serde::__private::Option::is_some(&__field25) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "userEnvProbe",
                                                ),
                                            );
                                        }
                                        __field25 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field26 => {
                                        if _serde::__private::Option::is_some(&__field26) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "waitFor",
                                                ),
                                            );
                                        }
                                        __field26 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "additionalProperties",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("capAdd") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("containerEnv") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "containerUser",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "customizations",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("features") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("forwardPorts") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "hostRequirements",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("init") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "initializeCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field10 = match __field10 {
                                _serde::__private::Some(__field10) => __field10,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("mounts") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field11 = match __field11 {
                                _serde::__private::Some(__field11) => __field11,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("name") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field12 = match __field12 {
                                _serde::__private::Some(__field12) => __field12,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "onCreateCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field13 = match __field13 {
                                _serde::__private::Some(__field13) => __field13,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "otherPortsAttributes",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field14 = match __field14 {
                                _serde::__private::Some(__field14) => __field14,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "overrideFeatureInstallOrder",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field15 = match __field15 {
                                _serde::__private::Some(__field15) => __field15,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "portsAttributes",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field16 = match __field16 {
                                _serde::__private::Some(__field16) => __field16,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "postAttachCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field17 = match __field17 {
                                _serde::__private::Some(__field17) => __field17,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "postCreateCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field18 = match __field18 {
                                _serde::__private::Some(__field18) => __field18,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "postStartCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field19 = match __field19 {
                                _serde::__private::Some(__field19) => __field19,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("privileged") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field20 = match __field20 {
                                _serde::__private::Some(__field20) => __field20,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("remoteEnv") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field21 = match __field21 {
                                _serde::__private::Some(__field21) => __field21,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("remoteUser") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field22 = match __field22 {
                                _serde::__private::Some(__field22) => __field22,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("securityOpt") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field23 = match __field23 {
                                _serde::__private::Some(__field23) => __field23,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "updateContentCommand",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field24 = match __field24 {
                                _serde::__private::Some(__field24) => __field24,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field(
                                        "updateRemoteUserUID",
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field25 = match __field25 {
                                _serde::__private::Some(__field25) => __field25,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("userEnvProbe") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field26 = match __field26 {
                                _serde::__private::Some(__field26) => __field26,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("waitFor") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(DevContainerBaseWorkspaceMountVariant0 {
                                additional_properties: __field0,
                                cap_add: __field1,
                                container_env: __field2,
                                container_user: __field3,
                                customizations: __field4,
                                features: __field5,
                                forward_ports: __field6,
                                host_requirements: __field7,
                                init: __field8,
                                initialize_command: __field9,
                                mounts: __field10,
                                name: __field11,
                                on_create_command: __field12,
                                other_ports_attributes: __field13,
                                override_feature_install_order: __field14,
                                ports_attributes: __field15,
                                post_attach_command: __field16,
                                post_create_command: __field17,
                                post_start_command: __field18,
                                privileged: __field19,
                                remote_env: __field20,
                                remote_user: __field21,
                                security_opt: __field22,
                                update_content_command: __field23,
                                update_remote_user_uid: __field24,
                                user_env_probe: __field25,
                                wait_for: __field26,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "additionalProperties",
                        "capAdd",
                        "containerEnv",
                        "containerUser",
                        "customizations",
                        "features",
                        "forwardPorts",
                        "hostRequirements",
                        "init",
                        "initializeCommand",
                        "mounts",
                        "name",
                        "onCreateCommand",
                        "otherPortsAttributes",
                        "overrideFeatureInstallOrder",
                        "portsAttributes",
                        "postAttachCommand",
                        "postCreateCommand",
                        "postStartCommand",
                        "privileged",
                        "remoteEnv",
                        "remoteUser",
                        "securityOpt",
                        "updateContentCommand",
                        "updateRemoteUserUID",
                        "userEnvProbe",
                        "waitFor",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "DevContainerBaseWorkspaceMountVariant0",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<
                                DevContainerBaseWorkspaceMountVariant0,
                            >,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DevContainerBaseWorkspaceMountVariant0 {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "DevContainerBaseWorkspaceMountVariant0",
                        false as usize
                            + if Option::is_none(&self.additional_properties) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.cap_add) { 0 } else { 1 }
                            + if Option::is_none(&self.container_env) { 0 } else { 1 }
                            + if Option::is_none(&self.container_user) { 0 } else { 1 }
                            + if Option::is_none(&self.customizations) { 0 } else { 1 }
                            + if Option::is_none(&self.features) { 0 } else { 1 }
                            + if Option::is_none(&self.forward_ports) { 0 } else { 1 }
                            + if Option::is_none(&self.host_requirements) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.init) { 0 } else { 1 }
                            + if Option::is_none(&self.initialize_command) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.mounts) { 0 } else { 1 }
                            + if Option::is_none(&self.name) { 0 } else { 1 }
                            + if Option::is_none(&self.on_create_command) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.other_ports_attributes) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.override_feature_install_order) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.ports_attributes) { 0 } else { 1 }
                            + if Option::is_none(&self.post_attach_command) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.post_create_command) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.post_start_command) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.privileged) { 0 } else { 1 }
                            + if Option::is_none(&self.remote_env) { 0 } else { 1 }
                            + if Option::is_none(&self.remote_user) { 0 } else { 1 }
                            + if Option::is_none(&self.security_opt) { 0 } else { 1 }
                            + if Option::is_none(&self.update_content_command) {
                                0
                            } else {
                                1
                            }
                            + if Option::is_none(&self.update_remote_user_uid) {
                                0
                            } else {
                                1
                            } + if Option::is_none(&self.user_env_probe) { 0 } else { 1 }
                            + if Option::is_none(&self.wait_for) { 0 } else { 1 },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if !Option::is_none(&self.additional_properties) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "additionalProperties",
                            &self.additional_properties,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "additionalProperties",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.cap_add) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "capAdd",
                            &self.cap_add,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "capAdd",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.container_env) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "containerEnv",
                            &self.container_env,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "containerEnv",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.container_user) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "containerUser",
                            &self.container_user,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "containerUser",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.customizations) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "customizations",
                            &self.customizations,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "customizations",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.features) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "features",
                            &self.features,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "features",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.forward_ports) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "forwardPorts",
                            &self.forward_ports,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "forwardPorts",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.host_requirements) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "hostRequirements",
                            &self.host_requirements,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "hostRequirements",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.init) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "init",
                            &self.init,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "init",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.initialize_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "initializeCommand",
                            &self.initialize_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "initializeCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.mounts) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "mounts",
                            &self.mounts,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "mounts",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.name) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "name",
                            &self.name,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "name",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.on_create_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "onCreateCommand",
                            &self.on_create_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "onCreateCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.other_ports_attributes) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "otherPortsAttributes",
                            &self.other_ports_attributes,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "otherPortsAttributes",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.override_feature_install_order) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "overrideFeatureInstallOrder",
                            &self.override_feature_install_order,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "overrideFeatureInstallOrder",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.ports_attributes) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "portsAttributes",
                            &self.ports_attributes,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "portsAttributes",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.post_attach_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "postAttachCommand",
                            &self.post_attach_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "postAttachCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.post_create_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "postCreateCommand",
                            &self.post_create_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "postCreateCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.post_start_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "postStartCommand",
                            &self.post_start_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "postStartCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.privileged) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "privileged",
                            &self.privileged,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "privileged",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.remote_env) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "remoteEnv",
                            &self.remote_env,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "remoteEnv",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.remote_user) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "remoteUser",
                            &self.remote_user,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "remoteUser",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.security_opt) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "securityOpt",
                            &self.security_opt,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "securityOpt",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.update_content_command) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "updateContentCommand",
                            &self.update_content_command,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "updateContentCommand",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.update_remote_user_uid) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "updateRemoteUserUID",
                            &self.update_remote_user_uid,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "updateRemoteUserUID",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.user_env_probe) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "userEnvProbe",
                            &self.user_env_probe,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "userEnvProbe",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    if !Option::is_none(&self.wait_for) {
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "waitFor",
                            &self.wait_for,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    } else {
                        match _serde::ser::SerializeStruct::skip_field(
                            &mut __serde_state,
                            "waitFor",
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                    }
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[serde(untagged)]
        pub enum DevContainerBaseWorkspaceMount {
            Variant0(DevContainerBaseWorkspaceMountVariant0),
            Variant1(DevContainerCommon),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DevContainerBaseWorkspaceMount {
            #[inline]
            fn clone(&self) -> DevContainerBaseWorkspaceMount {
                match self {
                    DevContainerBaseWorkspaceMount::Variant0(__self_0) => {
                        DevContainerBaseWorkspaceMount::Variant0(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                    DevContainerBaseWorkspaceMount::Variant1(__self_0) => {
                        DevContainerBaseWorkspaceMount::Variant1(
                            ::core::clone::Clone::clone(__self_0),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for DevContainerBaseWorkspaceMount {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for DevContainerBaseWorkspaceMount {
            #[inline]
            fn eq(&self, other: &DevContainerBaseWorkspaceMount) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (
                            DevContainerBaseWorkspaceMount::Variant0(__self_0),
                            DevContainerBaseWorkspaceMount::Variant0(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            DevContainerBaseWorkspaceMount::Variant1(__self_0),
                            DevContainerBaseWorkspaceMount::Variant1(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DevContainerBaseWorkspaceMount {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    DevContainerBaseWorkspaceMount::Variant0(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variant0",
                            &__self_0,
                        )
                    }
                    DevContainerBaseWorkspaceMount::Variant1(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Variant1",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for DevContainerBaseWorkspaceMount {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    let __content = match <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                        __deserializer,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    if let _serde::__private::Ok(__ok)
                        = _serde::__private::Result::map(
                            <DevContainerBaseWorkspaceMountVariant0 as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<
                                    __D::Error,
                                >::new(&__content),
                            ),
                            DevContainerBaseWorkspaceMount::Variant0,
                        ) {
                        return _serde::__private::Ok(__ok);
                    }
                    if let _serde::__private::Ok(__ok)
                        = _serde::__private::Result::map(
                            <DevContainerCommon as _serde::Deserialize>::deserialize(
                                _serde::__private::de::ContentRefDeserializer::<
                                    __D::Error,
                                >::new(&__content),
                            ),
                            DevContainerBaseWorkspaceMount::Variant1,
                        ) {
                        return _serde::__private::Ok(__ok);
                    }
                    _serde::__private::Err(
                        _serde::de::Error::custom(
                            "data did not match any variant of untagged enum DevContainerBaseWorkspaceMount",
                        ),
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for DevContainerBaseWorkspaceMount {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        DevContainerBaseWorkspaceMount::Variant0(ref __field0) => {
                            _serde::Serialize::serialize(__field0, __serializer)
                        }
                        DevContainerBaseWorkspaceMount::Variant1(ref __field0) => {
                            _serde::Serialize::serialize(__field0, __serializer)
                        }
                    }
                }
            }
        };
        pub type DevContainerBase = DevContainerBaseWorkspaceMount;
    }
}
