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

//! Generated file from `CmdContentPackageType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdContentPackageType)
pub enum CmdContentPackageType {
    // @@protoc_insertion_point(enum_value:CmdContentPackageType.CmdContentPackageTypeNone)
    CmdContentPackageTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdContentPackageType.CmdContentPackageGetDataScRsp)
    CmdContentPackageGetDataScRsp = 7518,
    // @@protoc_insertion_point(enum_value:CmdContentPackageType.CmdContentPackageUnlockCsReq)
    CmdContentPackageUnlockCsReq = 7506,
    // @@protoc_insertion_point(enum_value:CmdContentPackageType.CmdContentPackageUnlockScRsp)
    CmdContentPackageUnlockScRsp = 7541,
    // @@protoc_insertion_point(enum_value:CmdContentPackageType.CmdContentPackageTransferScNotify)
    CmdContentPackageTransferScNotify = 7509,
    // @@protoc_insertion_point(enum_value:CmdContentPackageType.CmdContentPackageSyncDataScNotify)
    CmdContentPackageSyncDataScNotify = 7514,
    // @@protoc_insertion_point(enum_value:CmdContentPackageType.CmdContentPackageGetDataCsReq)
    CmdContentPackageGetDataCsReq = 7542,
}

impl ::protobuf::Enum for CmdContentPackageType {
    const NAME: &'static str = "CmdContentPackageType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdContentPackageType> {
        match value {
            0 => ::std::option::Option::Some(CmdContentPackageType::CmdContentPackageTypeNone),
            7518 => ::std::option::Option::Some(CmdContentPackageType::CmdContentPackageGetDataScRsp),
            7506 => ::std::option::Option::Some(CmdContentPackageType::CmdContentPackageUnlockCsReq),
            7541 => ::std::option::Option::Some(CmdContentPackageType::CmdContentPackageUnlockScRsp),
            7509 => ::std::option::Option::Some(CmdContentPackageType::CmdContentPackageTransferScNotify),
            7514 => ::std::option::Option::Some(CmdContentPackageType::CmdContentPackageSyncDataScNotify),
            7542 => ::std::option::Option::Some(CmdContentPackageType::CmdContentPackageGetDataCsReq),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdContentPackageType> {
        match str {
            "CmdContentPackageTypeNone" => ::std::option::Option::Some(CmdContentPackageType::CmdContentPackageTypeNone),
            "CmdContentPackageGetDataScRsp" => ::std::option::Option::Some(CmdContentPackageType::CmdContentPackageGetDataScRsp),
            "CmdContentPackageUnlockCsReq" => ::std::option::Option::Some(CmdContentPackageType::CmdContentPackageUnlockCsReq),
            "CmdContentPackageUnlockScRsp" => ::std::option::Option::Some(CmdContentPackageType::CmdContentPackageUnlockScRsp),
            "CmdContentPackageTransferScNotify" => ::std::option::Option::Some(CmdContentPackageType::CmdContentPackageTransferScNotify),
            "CmdContentPackageSyncDataScNotify" => ::std::option::Option::Some(CmdContentPackageType::CmdContentPackageSyncDataScNotify),
            "CmdContentPackageGetDataCsReq" => ::std::option::Option::Some(CmdContentPackageType::CmdContentPackageGetDataCsReq),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdContentPackageType] = &[
        CmdContentPackageType::CmdContentPackageTypeNone,
        CmdContentPackageType::CmdContentPackageGetDataScRsp,
        CmdContentPackageType::CmdContentPackageUnlockCsReq,
        CmdContentPackageType::CmdContentPackageUnlockScRsp,
        CmdContentPackageType::CmdContentPackageTransferScNotify,
        CmdContentPackageType::CmdContentPackageSyncDataScNotify,
        CmdContentPackageType::CmdContentPackageGetDataCsReq,
    ];
}

impl ::protobuf::EnumFull for CmdContentPackageType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdContentPackageType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdContentPackageType::CmdContentPackageTypeNone => 0,
            CmdContentPackageType::CmdContentPackageGetDataScRsp => 1,
            CmdContentPackageType::CmdContentPackageUnlockCsReq => 2,
            CmdContentPackageType::CmdContentPackageUnlockScRsp => 3,
            CmdContentPackageType::CmdContentPackageTransferScNotify => 4,
            CmdContentPackageType::CmdContentPackageSyncDataScNotify => 5,
            CmdContentPackageType::CmdContentPackageGetDataCsReq => 6,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdContentPackageType {
    fn default() -> Self {
        CmdContentPackageType::CmdContentPackageTypeNone
    }
}

impl CmdContentPackageType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdContentPackageType>("CmdContentPackageType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bCmdContentPackageType.proto*\x94\x02\n\x15CmdContentPackageType\
    \x12\x1d\n\x19CmdContentPackageTypeNone\x10\0\x12\"\n\x1dCmdContentPacka\
    geGetDataScRsp\x10\xde:\x12!\n\x1cCmdContentPackageUnlockCsReq\x10\xd2:\
    \x12!\n\x1cCmdContentPackageUnlockScRsp\x10\xf5:\x12&\n!CmdContentPackag\
    eTransferScNotify\x10\xd5:\x12&\n!CmdContentPackageSyncDataScNotify\x10\
    \xda:\x12\"\n\x1dCmdContentPackageGetDataCsReq\x10\xf6:b\x06proto3\
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
            enums.push(CmdContentPackageType::generated_enum_descriptor_data());
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
