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

//! Generated file from `ExtraLineupType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:ExtraLineupType)
pub enum ExtraLineupType {
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_NONE)
    LINEUP_NONE = 0,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_CHALLENGE)
    LINEUP_CHALLENGE = 1,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_ROGUE)
    LINEUP_ROGUE = 2,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_CHALLENGE_2)
    LINEUP_CHALLENGE_2 = 3,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_CHALLENGE_3)
    LINEUP_CHALLENGE_3 = 4,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_ROGUE_CHALLENGE)
    LINEUP_ROGUE_CHALLENGE = 5,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_STAGE_TRIAL)
    LINEUP_STAGE_TRIAL = 6,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_ROGUE_TRIAL)
    LINEUP_ROGUE_TRIAL = 7,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_ACTIVITY)
    LINEUP_ACTIVITY = 8,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_BOXING_CLUB)
    LINEUP_BOXING_CLUB = 9,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_TREASURE_DUNGEON)
    LINEUP_TREASURE_DUNGEON = 11,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_CHESS_ROGUE)
    LINEUP_CHESS_ROGUE = 12,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_HELIOBUS)
    LINEUP_HELIOBUS = 13,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_TOURN_ROGUE)
    LINEUP_TOURN_ROGUE = 14,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_RELIC_ROGUE)
    LINEUP_RELIC_ROGUE = 15,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_ARCADE_ROGUE)
    LINEUP_ARCADE_ROGUE = 16,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_MAGIC_ROGUE)
    LINEUP_MAGIC_ROGUE = 17,
    // @@protoc_insertion_point(enum_value:ExtraLineupType.LINEUP_FATE)
    LINEUP_FATE = 18,
}

impl ::protobuf::Enum for ExtraLineupType {
    const NAME: &'static str = "ExtraLineupType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ExtraLineupType> {
        match value {
            0 => ::std::option::Option::Some(ExtraLineupType::LINEUP_NONE),
            1 => ::std::option::Option::Some(ExtraLineupType::LINEUP_CHALLENGE),
            2 => ::std::option::Option::Some(ExtraLineupType::LINEUP_ROGUE),
            3 => ::std::option::Option::Some(ExtraLineupType::LINEUP_CHALLENGE_2),
            4 => ::std::option::Option::Some(ExtraLineupType::LINEUP_CHALLENGE_3),
            5 => ::std::option::Option::Some(ExtraLineupType::LINEUP_ROGUE_CHALLENGE),
            6 => ::std::option::Option::Some(ExtraLineupType::LINEUP_STAGE_TRIAL),
            7 => ::std::option::Option::Some(ExtraLineupType::LINEUP_ROGUE_TRIAL),
            8 => ::std::option::Option::Some(ExtraLineupType::LINEUP_ACTIVITY),
            9 => ::std::option::Option::Some(ExtraLineupType::LINEUP_BOXING_CLUB),
            11 => ::std::option::Option::Some(ExtraLineupType::LINEUP_TREASURE_DUNGEON),
            12 => ::std::option::Option::Some(ExtraLineupType::LINEUP_CHESS_ROGUE),
            13 => ::std::option::Option::Some(ExtraLineupType::LINEUP_HELIOBUS),
            14 => ::std::option::Option::Some(ExtraLineupType::LINEUP_TOURN_ROGUE),
            15 => ::std::option::Option::Some(ExtraLineupType::LINEUP_RELIC_ROGUE),
            16 => ::std::option::Option::Some(ExtraLineupType::LINEUP_ARCADE_ROGUE),
            17 => ::std::option::Option::Some(ExtraLineupType::LINEUP_MAGIC_ROGUE),
            18 => ::std::option::Option::Some(ExtraLineupType::LINEUP_FATE),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<ExtraLineupType> {
        match str {
            "LINEUP_NONE" => ::std::option::Option::Some(ExtraLineupType::LINEUP_NONE),
            "LINEUP_CHALLENGE" => ::std::option::Option::Some(ExtraLineupType::LINEUP_CHALLENGE),
            "LINEUP_ROGUE" => ::std::option::Option::Some(ExtraLineupType::LINEUP_ROGUE),
            "LINEUP_CHALLENGE_2" => ::std::option::Option::Some(ExtraLineupType::LINEUP_CHALLENGE_2),
            "LINEUP_CHALLENGE_3" => ::std::option::Option::Some(ExtraLineupType::LINEUP_CHALLENGE_3),
            "LINEUP_ROGUE_CHALLENGE" => ::std::option::Option::Some(ExtraLineupType::LINEUP_ROGUE_CHALLENGE),
            "LINEUP_STAGE_TRIAL" => ::std::option::Option::Some(ExtraLineupType::LINEUP_STAGE_TRIAL),
            "LINEUP_ROGUE_TRIAL" => ::std::option::Option::Some(ExtraLineupType::LINEUP_ROGUE_TRIAL),
            "LINEUP_ACTIVITY" => ::std::option::Option::Some(ExtraLineupType::LINEUP_ACTIVITY),
            "LINEUP_BOXING_CLUB" => ::std::option::Option::Some(ExtraLineupType::LINEUP_BOXING_CLUB),
            "LINEUP_TREASURE_DUNGEON" => ::std::option::Option::Some(ExtraLineupType::LINEUP_TREASURE_DUNGEON),
            "LINEUP_CHESS_ROGUE" => ::std::option::Option::Some(ExtraLineupType::LINEUP_CHESS_ROGUE),
            "LINEUP_HELIOBUS" => ::std::option::Option::Some(ExtraLineupType::LINEUP_HELIOBUS),
            "LINEUP_TOURN_ROGUE" => ::std::option::Option::Some(ExtraLineupType::LINEUP_TOURN_ROGUE),
            "LINEUP_RELIC_ROGUE" => ::std::option::Option::Some(ExtraLineupType::LINEUP_RELIC_ROGUE),
            "LINEUP_ARCADE_ROGUE" => ::std::option::Option::Some(ExtraLineupType::LINEUP_ARCADE_ROGUE),
            "LINEUP_MAGIC_ROGUE" => ::std::option::Option::Some(ExtraLineupType::LINEUP_MAGIC_ROGUE),
            "LINEUP_FATE" => ::std::option::Option::Some(ExtraLineupType::LINEUP_FATE),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [ExtraLineupType] = &[
        ExtraLineupType::LINEUP_NONE,
        ExtraLineupType::LINEUP_CHALLENGE,
        ExtraLineupType::LINEUP_ROGUE,
        ExtraLineupType::LINEUP_CHALLENGE_2,
        ExtraLineupType::LINEUP_CHALLENGE_3,
        ExtraLineupType::LINEUP_ROGUE_CHALLENGE,
        ExtraLineupType::LINEUP_STAGE_TRIAL,
        ExtraLineupType::LINEUP_ROGUE_TRIAL,
        ExtraLineupType::LINEUP_ACTIVITY,
        ExtraLineupType::LINEUP_BOXING_CLUB,
        ExtraLineupType::LINEUP_TREASURE_DUNGEON,
        ExtraLineupType::LINEUP_CHESS_ROGUE,
        ExtraLineupType::LINEUP_HELIOBUS,
        ExtraLineupType::LINEUP_TOURN_ROGUE,
        ExtraLineupType::LINEUP_RELIC_ROGUE,
        ExtraLineupType::LINEUP_ARCADE_ROGUE,
        ExtraLineupType::LINEUP_MAGIC_ROGUE,
        ExtraLineupType::LINEUP_FATE,
    ];
}

impl ::protobuf::EnumFull for ExtraLineupType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("ExtraLineupType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            ExtraLineupType::LINEUP_NONE => 0,
            ExtraLineupType::LINEUP_CHALLENGE => 1,
            ExtraLineupType::LINEUP_ROGUE => 2,
            ExtraLineupType::LINEUP_CHALLENGE_2 => 3,
            ExtraLineupType::LINEUP_CHALLENGE_3 => 4,
            ExtraLineupType::LINEUP_ROGUE_CHALLENGE => 5,
            ExtraLineupType::LINEUP_STAGE_TRIAL => 6,
            ExtraLineupType::LINEUP_ROGUE_TRIAL => 7,
            ExtraLineupType::LINEUP_ACTIVITY => 8,
            ExtraLineupType::LINEUP_BOXING_CLUB => 9,
            ExtraLineupType::LINEUP_TREASURE_DUNGEON => 10,
            ExtraLineupType::LINEUP_CHESS_ROGUE => 11,
            ExtraLineupType::LINEUP_HELIOBUS => 12,
            ExtraLineupType::LINEUP_TOURN_ROGUE => 13,
            ExtraLineupType::LINEUP_RELIC_ROGUE => 14,
            ExtraLineupType::LINEUP_ARCADE_ROGUE => 15,
            ExtraLineupType::LINEUP_MAGIC_ROGUE => 16,
            ExtraLineupType::LINEUP_FATE => 17,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for ExtraLineupType {
    fn default() -> Self {
        ExtraLineupType::LINEUP_NONE
    }
}

impl ExtraLineupType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<ExtraLineupType>("ExtraLineupType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15ExtraLineupType.proto*\xaf\x03\n\x0fExtraLineupType\x12\x0f\n\x0bL\
    INEUP_NONE\x10\0\x12\x14\n\x10LINEUP_CHALLENGE\x10\x01\x12\x10\n\x0cLINE\
    UP_ROGUE\x10\x02\x12\x16\n\x12LINEUP_CHALLENGE_2\x10\x03\x12\x16\n\x12LI\
    NEUP_CHALLENGE_3\x10\x04\x12\x1a\n\x16LINEUP_ROGUE_CHALLENGE\x10\x05\x12\
    \x16\n\x12LINEUP_STAGE_TRIAL\x10\x06\x12\x16\n\x12LINEUP_ROGUE_TRIAL\x10\
    \x07\x12\x13\n\x0fLINEUP_ACTIVITY\x10\x08\x12\x16\n\x12LINEUP_BOXING_CLU\
    B\x10\t\x12\x1b\n\x17LINEUP_TREASURE_DUNGEON\x10\x0b\x12\x16\n\x12LINEUP\
    _CHESS_ROGUE\x10\x0c\x12\x13\n\x0fLINEUP_HELIOBUS\x10\r\x12\x16\n\x12LIN\
    EUP_TOURN_ROGUE\x10\x0e\x12\x16\n\x12LINEUP_RELIC_ROGUE\x10\x0f\x12\x17\
    \n\x13LINEUP_ARCADE_ROGUE\x10\x10\x12\x16\n\x12LINEUP_MAGIC_ROGUE\x10\
    \x11\x12\x0f\n\x0bLINEUP_FATE\x10\x12b\x06proto3\
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
            enums.push(ExtraLineupType::generated_enum_descriptor_data());
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
