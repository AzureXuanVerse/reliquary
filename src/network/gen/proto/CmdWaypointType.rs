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

//! Generated file from `CmdWaypointType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdWaypointType)
pub enum CmdWaypointType {
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdWaypointTypeNone)
    CmdWaypointTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdWaypointShowNewCsNotify)
    CmdWaypointShowNewCsNotify = 434,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdSetCurWaypointCsReq)
    CmdSetCurWaypointCsReq = 403,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdGetChapterScRsp)
    CmdGetChapterScRsp = 453,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdTakeChapterRewardCsReq)
    CmdTakeChapterRewardCsReq = 437,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdGetWaypointScRsp)
    CmdGetWaypointScRsp = 420,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdSetCurWaypointScRsp)
    CmdSetCurWaypointScRsp = 446,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdGetChapterCsReq)
    CmdGetChapterCsReq = 439,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdTakeChapterRewardScRsp)
    CmdTakeChapterRewardScRsp = 480,
    // @@protoc_insertion_point(enum_value:CmdWaypointType.CmdGetWaypointCsReq)
    CmdGetWaypointCsReq = 459,
}

impl ::protobuf::Enum for CmdWaypointType {
    const NAME: &'static str = "CmdWaypointType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdWaypointType> {
        match value {
            0 => ::std::option::Option::Some(CmdWaypointType::CmdWaypointTypeNone),
            434 => ::std::option::Option::Some(CmdWaypointType::CmdWaypointShowNewCsNotify),
            403 => ::std::option::Option::Some(CmdWaypointType::CmdSetCurWaypointCsReq),
            453 => ::std::option::Option::Some(CmdWaypointType::CmdGetChapterScRsp),
            437 => ::std::option::Option::Some(CmdWaypointType::CmdTakeChapterRewardCsReq),
            420 => ::std::option::Option::Some(CmdWaypointType::CmdGetWaypointScRsp),
            446 => ::std::option::Option::Some(CmdWaypointType::CmdSetCurWaypointScRsp),
            439 => ::std::option::Option::Some(CmdWaypointType::CmdGetChapterCsReq),
            480 => ::std::option::Option::Some(CmdWaypointType::CmdTakeChapterRewardScRsp),
            459 => ::std::option::Option::Some(CmdWaypointType::CmdGetWaypointCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdWaypointType> {
        match str {
            "CmdWaypointTypeNone" => ::std::option::Option::Some(CmdWaypointType::CmdWaypointTypeNone),
            "CmdWaypointShowNewCsNotify" => ::std::option::Option::Some(CmdWaypointType::CmdWaypointShowNewCsNotify),
            "CmdSetCurWaypointCsReq" => ::std::option::Option::Some(CmdWaypointType::CmdSetCurWaypointCsReq),
            "CmdGetChapterScRsp" => ::std::option::Option::Some(CmdWaypointType::CmdGetChapterScRsp),
            "CmdTakeChapterRewardCsReq" => ::std::option::Option::Some(CmdWaypointType::CmdTakeChapterRewardCsReq),
            "CmdGetWaypointScRsp" => ::std::option::Option::Some(CmdWaypointType::CmdGetWaypointScRsp),
            "CmdSetCurWaypointScRsp" => ::std::option::Option::Some(CmdWaypointType::CmdSetCurWaypointScRsp),
            "CmdGetChapterCsReq" => ::std::option::Option::Some(CmdWaypointType::CmdGetChapterCsReq),
            "CmdTakeChapterRewardScRsp" => ::std::option::Option::Some(CmdWaypointType::CmdTakeChapterRewardScRsp),
            "CmdGetWaypointCsReq" => ::std::option::Option::Some(CmdWaypointType::CmdGetWaypointCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdWaypointType] = &[
        CmdWaypointType::CmdWaypointTypeNone,
        CmdWaypointType::CmdWaypointShowNewCsNotify,
        CmdWaypointType::CmdSetCurWaypointCsReq,
        CmdWaypointType::CmdGetChapterScRsp,
        CmdWaypointType::CmdTakeChapterRewardCsReq,
        CmdWaypointType::CmdGetWaypointScRsp,
        CmdWaypointType::CmdSetCurWaypointScRsp,
        CmdWaypointType::CmdGetChapterCsReq,
        CmdWaypointType::CmdTakeChapterRewardScRsp,
        CmdWaypointType::CmdGetWaypointCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdWaypointType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdWaypointType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdWaypointType::CmdWaypointTypeNone => 0,
            CmdWaypointType::CmdWaypointShowNewCsNotify => 1,
            CmdWaypointType::CmdSetCurWaypointCsReq => 2,
            CmdWaypointType::CmdGetChapterScRsp => 3,
            CmdWaypointType::CmdTakeChapterRewardCsReq => 4,
            CmdWaypointType::CmdGetWaypointScRsp => 5,
            CmdWaypointType::CmdSetCurWaypointScRsp => 6,
            CmdWaypointType::CmdGetChapterCsReq => 7,
            CmdWaypointType::CmdTakeChapterRewardScRsp => 8,
            CmdWaypointType::CmdGetWaypointCsReq => 9,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdWaypointType {
    fn default() -> Self {
        CmdWaypointType::CmdWaypointTypeNone
    }
}

impl CmdWaypointType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdWaypointType>("CmdWaypointType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15CmdWaypointType.proto*\xab\x02\n\x0fCmdWaypointType\x12\x17\n\x13C\
    mdWaypointTypeNone\x10\0\x12\x1f\n\x1aCmdWaypointShowNewCsNotify\x10\xb2\
    \x03\x12\x1b\n\x16CmdSetCurWaypointCsReq\x10\x93\x03\x12\x17\n\x12CmdGet\
    ChapterScRsp\x10\xc5\x03\x12\x1e\n\x19CmdTakeChapterRewardCsReq\x10\xb5\
    \x03\x12\x18\n\x13CmdGetWaypointScRsp\x10\xa4\x03\x12\x1b\n\x16CmdSetCur\
    WaypointScRsp\x10\xbe\x03\x12\x17\n\x12CmdGetChapterCsReq\x10\xb7\x03\
    \x12\x1e\n\x19CmdTakeChapterRewardScRsp\x10\xe0\x03\x12\x18\n\x13CmdGetW\
    aypointCsReq\x10\xcb\x03b\x06proto3\
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
            enums.push(CmdWaypointType::generated_enum_descriptor_data());
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
