// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `SceneGroupRefreshType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:SceneGroupRefreshType)
pub enum SceneGroupRefreshType {
    // @@protoc_insertion_point(enum_value:SceneGroupRefreshType.SCENE_GROUP_REFRESH_TYPE_NONE)
    SCENE_GROUP_REFRESH_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:SceneGroupRefreshType.SCENE_GROUP_REFRESH_TYPE_LOADED)
    SCENE_GROUP_REFRESH_TYPE_LOADED = 1,
    // @@protoc_insertion_point(enum_value:SceneGroupRefreshType.SCENE_GROUP_REFRESH_TYPE_UNLOAD)
    SCENE_GROUP_REFRESH_TYPE_UNLOAD = 2,
}

impl ::protobuf::Enum for SceneGroupRefreshType {
    const NAME: &'static str = "SceneGroupRefreshType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SceneGroupRefreshType> {
        match value {
            0 => ::std::option::Option::Some(SceneGroupRefreshType::SCENE_GROUP_REFRESH_TYPE_NONE),
            1 => ::std::option::Option::Some(SceneGroupRefreshType::SCENE_GROUP_REFRESH_TYPE_LOADED),
            2 => ::std::option::Option::Some(SceneGroupRefreshType::SCENE_GROUP_REFRESH_TYPE_UNLOAD),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<SceneGroupRefreshType> {
        match str {
            "SCENE_GROUP_REFRESH_TYPE_NONE" => ::std::option::Option::Some(SceneGroupRefreshType::SCENE_GROUP_REFRESH_TYPE_NONE),
            "SCENE_GROUP_REFRESH_TYPE_LOADED" => ::std::option::Option::Some(SceneGroupRefreshType::SCENE_GROUP_REFRESH_TYPE_LOADED),
            "SCENE_GROUP_REFRESH_TYPE_UNLOAD" => ::std::option::Option::Some(SceneGroupRefreshType::SCENE_GROUP_REFRESH_TYPE_UNLOAD),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [SceneGroupRefreshType] = &[
        SceneGroupRefreshType::SCENE_GROUP_REFRESH_TYPE_NONE,
        SceneGroupRefreshType::SCENE_GROUP_REFRESH_TYPE_LOADED,
        SceneGroupRefreshType::SCENE_GROUP_REFRESH_TYPE_UNLOAD,
    ];
}

impl ::protobuf::EnumFull for SceneGroupRefreshType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("SceneGroupRefreshType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for SceneGroupRefreshType {
    fn default() -> Self {
        SceneGroupRefreshType::SCENE_GROUP_REFRESH_TYPE_NONE
    }
}

impl SceneGroupRefreshType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<SceneGroupRefreshType>("SceneGroupRefreshType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bSceneGroupRefreshType.proto*\x84\x01\n\x15SceneGroupRefreshType\
    \x12!\n\x1dSCENE_GROUP_REFRESH_TYPE_NONE\x10\0\x12#\n\x1fSCENE_GROUP_REF\
    RESH_TYPE_LOADED\x10\x01\x12#\n\x1fSCENE_GROUP_REFRESH_TYPE_UNLOAD\x10\
    \x02b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(SceneGroupRefreshType::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
