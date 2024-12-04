// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `FriendApplySource.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:FriendApplySource)
pub enum FriendApplySource {
    // @@protoc_insertion_point(enum_value:FriendApplySource.FRIEND_APPLY_SOURCE_NONE)
    FRIEND_APPLY_SOURCE_NONE = 0,
    // @@protoc_insertion_point(enum_value:FriendApplySource.FRIEND_APPLY_SOURCE_SEARCH)
    FRIEND_APPLY_SOURCE_SEARCH = 1,
    // @@protoc_insertion_point(enum_value:FriendApplySource.FRIEND_APPLY_SOURCE_RECOMMEND)
    FRIEND_APPLY_SOURCE_RECOMMEND = 2,
    // @@protoc_insertion_point(enum_value:FriendApplySource.FRIEND_APPLY_SOURCE_ASSIST)
    FRIEND_APPLY_SOURCE_ASSIST = 3,
    // @@protoc_insertion_point(enum_value:FriendApplySource.FRIEND_APPLY_SOURCE_RECOMMEND_ASSIST)
    FRIEND_APPLY_SOURCE_RECOMMEND_ASSIST = 4,
    // @@protoc_insertion_point(enum_value:FriendApplySource.FRIEND_APPLY_SOURCE_PSN_FRIEND)
    FRIEND_APPLY_SOURCE_PSN_FRIEND = 5,
}

impl ::protobuf::Enum for FriendApplySource {
    const NAME: &'static str = "FriendApplySource";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FriendApplySource> {
        match value {
            0 => ::std::option::Option::Some(FriendApplySource::FRIEND_APPLY_SOURCE_NONE),
            1 => ::std::option::Option::Some(FriendApplySource::FRIEND_APPLY_SOURCE_SEARCH),
            2 => ::std::option::Option::Some(FriendApplySource::FRIEND_APPLY_SOURCE_RECOMMEND),
            3 => ::std::option::Option::Some(FriendApplySource::FRIEND_APPLY_SOURCE_ASSIST),
            4 => ::std::option::Option::Some(FriendApplySource::FRIEND_APPLY_SOURCE_RECOMMEND_ASSIST),
            5 => ::std::option::Option::Some(FriendApplySource::FRIEND_APPLY_SOURCE_PSN_FRIEND),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<FriendApplySource> {
        match str {
            "FRIEND_APPLY_SOURCE_NONE" => ::std::option::Option::Some(FriendApplySource::FRIEND_APPLY_SOURCE_NONE),
            "FRIEND_APPLY_SOURCE_SEARCH" => ::std::option::Option::Some(FriendApplySource::FRIEND_APPLY_SOURCE_SEARCH),
            "FRIEND_APPLY_SOURCE_RECOMMEND" => ::std::option::Option::Some(FriendApplySource::FRIEND_APPLY_SOURCE_RECOMMEND),
            "FRIEND_APPLY_SOURCE_ASSIST" => ::std::option::Option::Some(FriendApplySource::FRIEND_APPLY_SOURCE_ASSIST),
            "FRIEND_APPLY_SOURCE_RECOMMEND_ASSIST" => ::std::option::Option::Some(FriendApplySource::FRIEND_APPLY_SOURCE_RECOMMEND_ASSIST),
            "FRIEND_APPLY_SOURCE_PSN_FRIEND" => ::std::option::Option::Some(FriendApplySource::FRIEND_APPLY_SOURCE_PSN_FRIEND),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [FriendApplySource] = &[
        FriendApplySource::FRIEND_APPLY_SOURCE_NONE,
        FriendApplySource::FRIEND_APPLY_SOURCE_SEARCH,
        FriendApplySource::FRIEND_APPLY_SOURCE_RECOMMEND,
        FriendApplySource::FRIEND_APPLY_SOURCE_ASSIST,
        FriendApplySource::FRIEND_APPLY_SOURCE_RECOMMEND_ASSIST,
        FriendApplySource::FRIEND_APPLY_SOURCE_PSN_FRIEND,
    ];
}

impl ::protobuf::EnumFull for FriendApplySource {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("FriendApplySource").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for FriendApplySource {
    fn default() -> Self {
        FriendApplySource::FRIEND_APPLY_SOURCE_NONE
    }
}

impl FriendApplySource {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<FriendApplySource>("FriendApplySource")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17FriendApplySource.proto*\xe2\x01\n\x11FriendApplySource\x12\x1c\n\
    \x18FRIEND_APPLY_SOURCE_NONE\x10\0\x12\x1e\n\x1aFRIEND_APPLY_SOURCE_SEAR\
    CH\x10\x01\x12!\n\x1dFRIEND_APPLY_SOURCE_RECOMMEND\x10\x02\x12\x1e\n\x1a\
    FRIEND_APPLY_SOURCE_ASSIST\x10\x03\x12(\n$FRIEND_APPLY_SOURCE_RECOMMEND_\
    ASSIST\x10\x04\x12\"\n\x1eFRIEND_APPLY_SOURCE_PSN_FRIEND\x10\x05B\x15\n\
    \x13emu.lunarcore.protob\x06proto3\
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
            enums.push(FriendApplySource::generated_enum_descriptor_data());
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
