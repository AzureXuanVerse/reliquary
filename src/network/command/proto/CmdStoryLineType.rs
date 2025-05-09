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

//! Generated file from `CmdStoryLineType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdStoryLineType)
pub enum CmdStoryLineType {
    // @@protoc_insertion_point(enum_value:CmdStoryLineType.CmdStoryLineTypeNone)
    CmdStoryLineTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdStoryLineType.CmdChangeStoryLineFinishScNotify)
    CmdChangeStoryLineFinishScNotify = 6206,
    // @@protoc_insertion_point(enum_value:CmdStoryLineType.CmdStoryLineInfoScNotify)
    CmdStoryLineInfoScNotify = 6247,
    // @@protoc_insertion_point(enum_value:CmdStoryLineType.CmdGetStoryLineInfoCsReq)
    CmdGetStoryLineInfoCsReq = 6211,
    // @@protoc_insertion_point(enum_value:CmdStoryLineType.CmdGetStoryLineInfoScRsp)
    CmdGetStoryLineInfoScRsp = 6213,
    // @@protoc_insertion_point(enum_value:CmdStoryLineType.CmdStoryLineTrialAvatarChangeScNotify)
    CmdStoryLineTrialAvatarChangeScNotify = 6270,
}

impl ::protobuf::Enum for CmdStoryLineType {
    const NAME: &'static str = "CmdStoryLineType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdStoryLineType> {
        match value {
            0 => ::std::option::Option::Some(CmdStoryLineType::CmdStoryLineTypeNone),
            6206 => ::std::option::Option::Some(CmdStoryLineType::CmdChangeStoryLineFinishScNotify),
            6247 => ::std::option::Option::Some(CmdStoryLineType::CmdStoryLineInfoScNotify),
            6211 => ::std::option::Option::Some(CmdStoryLineType::CmdGetStoryLineInfoCsReq),
            6213 => ::std::option::Option::Some(CmdStoryLineType::CmdGetStoryLineInfoScRsp),
            6270 => ::std::option::Option::Some(CmdStoryLineType::CmdStoryLineTrialAvatarChangeScNotify),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdStoryLineType> {
        match str {
            "CmdStoryLineTypeNone" => ::std::option::Option::Some(CmdStoryLineType::CmdStoryLineTypeNone),
            "CmdChangeStoryLineFinishScNotify" => ::std::option::Option::Some(CmdStoryLineType::CmdChangeStoryLineFinishScNotify),
            "CmdStoryLineInfoScNotify" => ::std::option::Option::Some(CmdStoryLineType::CmdStoryLineInfoScNotify),
            "CmdGetStoryLineInfoCsReq" => ::std::option::Option::Some(CmdStoryLineType::CmdGetStoryLineInfoCsReq),
            "CmdGetStoryLineInfoScRsp" => ::std::option::Option::Some(CmdStoryLineType::CmdGetStoryLineInfoScRsp),
            "CmdStoryLineTrialAvatarChangeScNotify" => ::std::option::Option::Some(CmdStoryLineType::CmdStoryLineTrialAvatarChangeScNotify),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdStoryLineType] = &[
        CmdStoryLineType::CmdStoryLineTypeNone,
        CmdStoryLineType::CmdChangeStoryLineFinishScNotify,
        CmdStoryLineType::CmdStoryLineInfoScNotify,
        CmdStoryLineType::CmdGetStoryLineInfoCsReq,
        CmdStoryLineType::CmdGetStoryLineInfoScRsp,
        CmdStoryLineType::CmdStoryLineTrialAvatarChangeScNotify,
    ];
}

impl ::protobuf::EnumFull for CmdStoryLineType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdStoryLineType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdStoryLineType::CmdStoryLineTypeNone => 0,
            CmdStoryLineType::CmdChangeStoryLineFinishScNotify => 1,
            CmdStoryLineType::CmdStoryLineInfoScNotify => 2,
            CmdStoryLineType::CmdGetStoryLineInfoCsReq => 3,
            CmdStoryLineType::CmdGetStoryLineInfoScRsp => 4,
            CmdStoryLineType::CmdStoryLineTrialAvatarChangeScNotify => 5,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdStoryLineType {
    fn default() -> Self {
        CmdStoryLineType::CmdStoryLineTypeNone
    }
}

impl CmdStoryLineType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdStoryLineType>("CmdStoryLineType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16CmdStoryLineType.proto*\xdc\x01\n\x10CmdStoryLineType\x12\x18\n\
    \x14CmdStoryLineTypeNone\x10\0\x12%\n\x20CmdChangeStoryLineFinishScNotif\
    y\x10\xbe0\x12\x1d\n\x18CmdStoryLineInfoScNotify\x10\xe70\x12\x1d\n\x18C\
    mdGetStoryLineInfoCsReq\x10\xc30\x12\x1d\n\x18CmdGetStoryLineInfoScRsp\
    \x10\xc50\x12*\n%CmdStoryLineTrialAvatarChangeScNotify\x10\xfe0b\x06prot\
    o3\
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
            enums.push(CmdStoryLineType::generated_enum_descriptor_data());
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
