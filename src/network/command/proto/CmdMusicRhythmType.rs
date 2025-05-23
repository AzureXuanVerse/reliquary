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

//! Generated file from `CmdMusicRhythmType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdMusicRhythmType)
pub enum CmdMusicRhythmType {
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmTypeNone)
    CmdMusicRhythmTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmDataCsReq)
    CmdMusicRhythmDataCsReq = 7573,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmStartLevelScRsp)
    CmdMusicRhythmStartLevelScRsp = 7589,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmDataScRsp)
    CmdMusicRhythmDataScRsp = 7590,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmSaveSongConfigDataScRsp)
    CmdMusicRhythmSaveSongConfigDataScRsp = 7600,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmFinishLevelCsReq)
    CmdMusicRhythmFinishLevelCsReq = 7571,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmUnlockSongSfxScNotify)
    CmdMusicRhythmUnlockSongSfxScNotify = 7592,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmSaveSongConfigDataCsReq)
    CmdMusicRhythmSaveSongConfigDataCsReq = 7588,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmUnlockTrackScNotify)
    CmdMusicRhythmUnlockTrackScNotify = 7577,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmStartLevelCsReq)
    CmdMusicRhythmStartLevelCsReq = 7596,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmUnlockSongNotify)
    CmdMusicRhythmUnlockSongNotify = 7581,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmMaxDifficultyLevelsUnlockNotify)
    CmdMusicRhythmMaxDifficultyLevelsUnlockNotify = 7579,
    // @@protoc_insertion_point(enum_value:CmdMusicRhythmType.CmdMusicRhythmFinishLevelScRsp)
    CmdMusicRhythmFinishLevelScRsp = 7594,
}

impl ::protobuf::Enum for CmdMusicRhythmType {
    const NAME: &'static str = "CmdMusicRhythmType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdMusicRhythmType> {
        match value {
            0 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmTypeNone),
            7573 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmDataCsReq),
            7589 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmStartLevelScRsp),
            7590 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmDataScRsp),
            7600 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataScRsp),
            7571 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmFinishLevelCsReq),
            7592 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmUnlockSongSfxScNotify),
            7588 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataCsReq),
            7577 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmUnlockTrackScNotify),
            7596 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmStartLevelCsReq),
            7581 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmUnlockSongNotify),
            7579 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmMaxDifficultyLevelsUnlockNotify),
            7594 => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmFinishLevelScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdMusicRhythmType> {
        match str {
            "CmdMusicRhythmTypeNone" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmTypeNone),
            "CmdMusicRhythmDataCsReq" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmDataCsReq),
            "CmdMusicRhythmStartLevelScRsp" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmStartLevelScRsp),
            "CmdMusicRhythmDataScRsp" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmDataScRsp),
            "CmdMusicRhythmSaveSongConfigDataScRsp" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataScRsp),
            "CmdMusicRhythmFinishLevelCsReq" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmFinishLevelCsReq),
            "CmdMusicRhythmUnlockSongSfxScNotify" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmUnlockSongSfxScNotify),
            "CmdMusicRhythmSaveSongConfigDataCsReq" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataCsReq),
            "CmdMusicRhythmUnlockTrackScNotify" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmUnlockTrackScNotify),
            "CmdMusicRhythmStartLevelCsReq" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmStartLevelCsReq),
            "CmdMusicRhythmUnlockSongNotify" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmUnlockSongNotify),
            "CmdMusicRhythmMaxDifficultyLevelsUnlockNotify" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmMaxDifficultyLevelsUnlockNotify),
            "CmdMusicRhythmFinishLevelScRsp" => ::std::option::Option::Some(CmdMusicRhythmType::CmdMusicRhythmFinishLevelScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdMusicRhythmType] = &[
        CmdMusicRhythmType::CmdMusicRhythmTypeNone,
        CmdMusicRhythmType::CmdMusicRhythmDataCsReq,
        CmdMusicRhythmType::CmdMusicRhythmStartLevelScRsp,
        CmdMusicRhythmType::CmdMusicRhythmDataScRsp,
        CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataScRsp,
        CmdMusicRhythmType::CmdMusicRhythmFinishLevelCsReq,
        CmdMusicRhythmType::CmdMusicRhythmUnlockSongSfxScNotify,
        CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataCsReq,
        CmdMusicRhythmType::CmdMusicRhythmUnlockTrackScNotify,
        CmdMusicRhythmType::CmdMusicRhythmStartLevelCsReq,
        CmdMusicRhythmType::CmdMusicRhythmUnlockSongNotify,
        CmdMusicRhythmType::CmdMusicRhythmMaxDifficultyLevelsUnlockNotify,
        CmdMusicRhythmType::CmdMusicRhythmFinishLevelScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdMusicRhythmType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdMusicRhythmType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdMusicRhythmType::CmdMusicRhythmTypeNone => 0,
            CmdMusicRhythmType::CmdMusicRhythmDataCsReq => 1,
            CmdMusicRhythmType::CmdMusicRhythmStartLevelScRsp => 2,
            CmdMusicRhythmType::CmdMusicRhythmDataScRsp => 3,
            CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataScRsp => 4,
            CmdMusicRhythmType::CmdMusicRhythmFinishLevelCsReq => 5,
            CmdMusicRhythmType::CmdMusicRhythmUnlockSongSfxScNotify => 6,
            CmdMusicRhythmType::CmdMusicRhythmSaveSongConfigDataCsReq => 7,
            CmdMusicRhythmType::CmdMusicRhythmUnlockTrackScNotify => 8,
            CmdMusicRhythmType::CmdMusicRhythmStartLevelCsReq => 9,
            CmdMusicRhythmType::CmdMusicRhythmUnlockSongNotify => 10,
            CmdMusicRhythmType::CmdMusicRhythmMaxDifficultyLevelsUnlockNotify => 11,
            CmdMusicRhythmType::CmdMusicRhythmFinishLevelScRsp => 12,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdMusicRhythmType {
    fn default() -> Self {
        CmdMusicRhythmType::CmdMusicRhythmTypeNone
    }
}

impl CmdMusicRhythmType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdMusicRhythmType>("CmdMusicRhythmType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18CmdMusicRhythmType.proto*\x81\x04\n\x12CmdMusicRhythmType\x12\x1a\
    \n\x16CmdMusicRhythmTypeNone\x10\0\x12\x1c\n\x17CmdMusicRhythmDataCsReq\
    \x10\x95;\x12\"\n\x1dCmdMusicRhythmStartLevelScRsp\x10\xa5;\x12\x1c\n\
    \x17CmdMusicRhythmDataScRsp\x10\xa6;\x12*\n%CmdMusicRhythmSaveSongConfig\
    DataScRsp\x10\xb0;\x12#\n\x1eCmdMusicRhythmFinishLevelCsReq\x10\x93;\x12\
    (\n#CmdMusicRhythmUnlockSongSfxScNotify\x10\xa8;\x12*\n%CmdMusicRhythmSa\
    veSongConfigDataCsReq\x10\xa4;\x12&\n!CmdMusicRhythmUnlockTrackScNotify\
    \x10\x99;\x12\"\n\x1dCmdMusicRhythmStartLevelCsReq\x10\xac;\x12#\n\x1eCm\
    dMusicRhythmUnlockSongNotify\x10\x9d;\x122\n-CmdMusicRhythmMaxDifficulty\
    LevelsUnlockNotify\x10\x9b;\x12#\n\x1eCmdMusicRhythmFinishLevelScRsp\x10\
    \xaa;b\x06proto3\
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
            enums.push(CmdMusicRhythmType::generated_enum_descriptor_data());
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
