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

//! Generated file from `CmdPlayerReturnType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdPlayerReturnType)
pub enum CmdPlayerReturnType {
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnTypeNone)
    CmdPlayerReturnTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnTakePointRewardScRsp)
    CmdPlayerReturnTakePointRewardScRsp = 4548,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnTakeRewardScRsp)
    CmdPlayerReturnTakeRewardScRsp = 4552,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnForceFinishScNotify)
    CmdPlayerReturnForceFinishScNotify = 4572,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnTakeRewardCsReq)
    CmdPlayerReturnTakeRewardCsReq = 4571,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnInfoQueryCsReq)
    CmdPlayerReturnInfoQueryCsReq = 4522,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnTakeRelicCsReq)
    CmdPlayerReturnTakeRelicCsReq = 4585,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnSignCsReq)
    CmdPlayerReturnSignCsReq = 4532,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnTakeRelicScRsp)
    CmdPlayerReturnTakeRelicScRsp = 4516,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnPointChangeScNotify)
    CmdPlayerReturnPointChangeScNotify = 4540,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnStartScNotify)
    CmdPlayerReturnStartScNotify = 4595,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnTakePointRewardCsReq)
    CmdPlayerReturnTakePointRewardCsReq = 4576,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnSignScRsp)
    CmdPlayerReturnSignScRsp = 4531,
    // @@protoc_insertion_point(enum_value:CmdPlayerReturnType.CmdPlayerReturnInfoQueryScRsp)
    CmdPlayerReturnInfoQueryScRsp = 4556,
}

impl ::protobuf::Enum for CmdPlayerReturnType {
    const NAME: &'static str = "CmdPlayerReturnType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdPlayerReturnType> {
        match value {
            0 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTypeNone),
            4548 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakePointRewardScRsp),
            4552 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakeRewardScRsp),
            4572 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnForceFinishScNotify),
            4571 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakeRewardCsReq),
            4522 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnInfoQueryCsReq),
            4585 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakeRelicCsReq),
            4532 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnSignCsReq),
            4516 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakeRelicScRsp),
            4540 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnPointChangeScNotify),
            4595 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnStartScNotify),
            4576 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakePointRewardCsReq),
            4531 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnSignScRsp),
            4556 => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnInfoQueryScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdPlayerReturnType> {
        match str {
            "CmdPlayerReturnTypeNone" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTypeNone),
            "CmdPlayerReturnTakePointRewardScRsp" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakePointRewardScRsp),
            "CmdPlayerReturnTakeRewardScRsp" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakeRewardScRsp),
            "CmdPlayerReturnForceFinishScNotify" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnForceFinishScNotify),
            "CmdPlayerReturnTakeRewardCsReq" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakeRewardCsReq),
            "CmdPlayerReturnInfoQueryCsReq" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnInfoQueryCsReq),
            "CmdPlayerReturnTakeRelicCsReq" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakeRelicCsReq),
            "CmdPlayerReturnSignCsReq" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnSignCsReq),
            "CmdPlayerReturnTakeRelicScRsp" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakeRelicScRsp),
            "CmdPlayerReturnPointChangeScNotify" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnPointChangeScNotify),
            "CmdPlayerReturnStartScNotify" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnStartScNotify),
            "CmdPlayerReturnTakePointRewardCsReq" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnTakePointRewardCsReq),
            "CmdPlayerReturnSignScRsp" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnSignScRsp),
            "CmdPlayerReturnInfoQueryScRsp" => ::std::option::Option::Some(CmdPlayerReturnType::CmdPlayerReturnInfoQueryScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdPlayerReturnType] = &[
        CmdPlayerReturnType::CmdPlayerReturnTypeNone,
        CmdPlayerReturnType::CmdPlayerReturnTakePointRewardScRsp,
        CmdPlayerReturnType::CmdPlayerReturnTakeRewardScRsp,
        CmdPlayerReturnType::CmdPlayerReturnForceFinishScNotify,
        CmdPlayerReturnType::CmdPlayerReturnTakeRewardCsReq,
        CmdPlayerReturnType::CmdPlayerReturnInfoQueryCsReq,
        CmdPlayerReturnType::CmdPlayerReturnTakeRelicCsReq,
        CmdPlayerReturnType::CmdPlayerReturnSignCsReq,
        CmdPlayerReturnType::CmdPlayerReturnTakeRelicScRsp,
        CmdPlayerReturnType::CmdPlayerReturnPointChangeScNotify,
        CmdPlayerReturnType::CmdPlayerReturnStartScNotify,
        CmdPlayerReturnType::CmdPlayerReturnTakePointRewardCsReq,
        CmdPlayerReturnType::CmdPlayerReturnSignScRsp,
        CmdPlayerReturnType::CmdPlayerReturnInfoQueryScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdPlayerReturnType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdPlayerReturnType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdPlayerReturnType::CmdPlayerReturnTypeNone => 0,
            CmdPlayerReturnType::CmdPlayerReturnTakePointRewardScRsp => 1,
            CmdPlayerReturnType::CmdPlayerReturnTakeRewardScRsp => 2,
            CmdPlayerReturnType::CmdPlayerReturnForceFinishScNotify => 3,
            CmdPlayerReturnType::CmdPlayerReturnTakeRewardCsReq => 4,
            CmdPlayerReturnType::CmdPlayerReturnInfoQueryCsReq => 5,
            CmdPlayerReturnType::CmdPlayerReturnTakeRelicCsReq => 6,
            CmdPlayerReturnType::CmdPlayerReturnSignCsReq => 7,
            CmdPlayerReturnType::CmdPlayerReturnTakeRelicScRsp => 8,
            CmdPlayerReturnType::CmdPlayerReturnPointChangeScNotify => 9,
            CmdPlayerReturnType::CmdPlayerReturnStartScNotify => 10,
            CmdPlayerReturnType::CmdPlayerReturnTakePointRewardCsReq => 11,
            CmdPlayerReturnType::CmdPlayerReturnSignScRsp => 12,
            CmdPlayerReturnType::CmdPlayerReturnInfoQueryScRsp => 13,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdPlayerReturnType {
    fn default() -> Self {
        CmdPlayerReturnType::CmdPlayerReturnTypeNone
    }
}

impl CmdPlayerReturnType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdPlayerReturnType>("CmdPlayerReturnType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19CmdPlayerReturnType.proto*\x93\x04\n\x13CmdPlayerReturnType\x12\
    \x1b\n\x17CmdPlayerReturnTypeNone\x10\0\x12(\n#CmdPlayerReturnTakePointR\
    ewardScRsp\x10\xc4#\x12#\n\x1eCmdPlayerReturnTakeRewardScRsp\x10\xc8#\
    \x12'\n\"CmdPlayerReturnForceFinishScNotify\x10\xdc#\x12#\n\x1eCmdPlayer\
    ReturnTakeRewardCsReq\x10\xdb#\x12\"\n\x1dCmdPlayerReturnInfoQueryCsReq\
    \x10\xaa#\x12\"\n\x1dCmdPlayerReturnTakeRelicCsReq\x10\xe9#\x12\x1d\n\
    \x18CmdPlayerReturnSignCsReq\x10\xb4#\x12\"\n\x1dCmdPlayerReturnTakeReli\
    cScRsp\x10\xa4#\x12'\n\"CmdPlayerReturnPointChangeScNotify\x10\xbc#\x12!\
    \n\x1cCmdPlayerReturnStartScNotify\x10\xf3#\x12(\n#CmdPlayerReturnTakePo\
    intRewardCsReq\x10\xe0#\x12\x1d\n\x18CmdPlayerReturnSignScRsp\x10\xb3#\
    \x12\"\n\x1dCmdPlayerReturnInfoQueryScRsp\x10\xcc#b\x06proto3\
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
            enums.push(CmdPlayerReturnType::generated_enum_descriptor_data());
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
