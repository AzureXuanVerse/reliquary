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

//! Generated file from `CmdMapRotationType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdMapRotationType)
pub enum CmdMapRotationType {
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdMapRotationTypeNone)
    CmdMapRotationTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdInteractChargerScRsp)
    CmdInteractChargerScRsp = 6839,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdLeaveMapRotationRegionScNotify)
    CmdLeaveMapRotationRegionScNotify = 6873,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdResetMapRotationRegionCsReq)
    CmdResetMapRotationRegionCsReq = 6805,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdRemoveRotaterScRsp)
    CmdRemoveRotaterScRsp = 6852,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdEnterMapRotationRegionCsReq)
    CmdEnterMapRotationRegionCsReq = 6820,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdUpdateRotaterScNotify)
    CmdUpdateRotaterScNotify = 6868,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdGetMapRotationDataCsReq)
    CmdGetMapRotationDataCsReq = 6880,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdGetMapRotationDataScRsp)
    CmdGetMapRotationDataScRsp = 6802,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdInteractChargerCsReq)
    CmdInteractChargerCsReq = 6867,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdResetMapRotationRegionScRsp)
    CmdResetMapRotationRegionScRsp = 6874,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdDeployRotaterCsReq)
    CmdDeployRotaterCsReq = 6827,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdRemoveRotaterCsReq)
    CmdRemoveRotaterCsReq = 6838,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdUpdateEnergyScNotify)
    CmdUpdateEnergyScNotify = 6898,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdUpdateMapRotationDataScNotify)
    CmdUpdateMapRotationDataScNotify = 6885,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdLeaveMapRotationRegionCsReq)
    CmdLeaveMapRotationRegionCsReq = 6854,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdLeaveMapRotationRegionScRsp)
    CmdLeaveMapRotationRegionScRsp = 6877,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdEnterMapRotationRegionScRsp)
    CmdEnterMapRotationRegionScRsp = 6891,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdRotateMapCsReq)
    CmdRotateMapCsReq = 6870,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdRotateMapScRsp)
    CmdRotateMapScRsp = 6859,
    // @@protoc_insertion_point(enum_value:CmdMapRotationType.CmdDeployRotaterScRsp)
    CmdDeployRotaterScRsp = 6821,
}

impl ::protobuf::Enum for CmdMapRotationType {
    const NAME: &'static str = "CmdMapRotationType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdMapRotationType> {
        match value {
            0 => ::std::option::Option::Some(CmdMapRotationType::CmdMapRotationTypeNone),
            6839 => ::std::option::Option::Some(CmdMapRotationType::CmdInteractChargerScRsp),
            6873 => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionScNotify),
            6805 => ::std::option::Option::Some(CmdMapRotationType::CmdResetMapRotationRegionCsReq),
            6852 => ::std::option::Option::Some(CmdMapRotationType::CmdRemoveRotaterScRsp),
            6820 => ::std::option::Option::Some(CmdMapRotationType::CmdEnterMapRotationRegionCsReq),
            6868 => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateRotaterScNotify),
            6880 => ::std::option::Option::Some(CmdMapRotationType::CmdGetMapRotationDataCsReq),
            6802 => ::std::option::Option::Some(CmdMapRotationType::CmdGetMapRotationDataScRsp),
            6867 => ::std::option::Option::Some(CmdMapRotationType::CmdInteractChargerCsReq),
            6874 => ::std::option::Option::Some(CmdMapRotationType::CmdResetMapRotationRegionScRsp),
            6827 => ::std::option::Option::Some(CmdMapRotationType::CmdDeployRotaterCsReq),
            6838 => ::std::option::Option::Some(CmdMapRotationType::CmdRemoveRotaterCsReq),
            6898 => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateEnergyScNotify),
            6885 => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateMapRotationDataScNotify),
            6854 => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionCsReq),
            6877 => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionScRsp),
            6891 => ::std::option::Option::Some(CmdMapRotationType::CmdEnterMapRotationRegionScRsp),
            6870 => ::std::option::Option::Some(CmdMapRotationType::CmdRotateMapCsReq),
            6859 => ::std::option::Option::Some(CmdMapRotationType::CmdRotateMapScRsp),
            6821 => ::std::option::Option::Some(CmdMapRotationType::CmdDeployRotaterScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdMapRotationType> {
        match str {
            "CmdMapRotationTypeNone" => ::std::option::Option::Some(CmdMapRotationType::CmdMapRotationTypeNone),
            "CmdInteractChargerScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdInteractChargerScRsp),
            "CmdLeaveMapRotationRegionScNotify" => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionScNotify),
            "CmdResetMapRotationRegionCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdResetMapRotationRegionCsReq),
            "CmdRemoveRotaterScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdRemoveRotaterScRsp),
            "CmdEnterMapRotationRegionCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdEnterMapRotationRegionCsReq),
            "CmdUpdateRotaterScNotify" => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateRotaterScNotify),
            "CmdGetMapRotationDataCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdGetMapRotationDataCsReq),
            "CmdGetMapRotationDataScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdGetMapRotationDataScRsp),
            "CmdInteractChargerCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdInteractChargerCsReq),
            "CmdResetMapRotationRegionScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdResetMapRotationRegionScRsp),
            "CmdDeployRotaterCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdDeployRotaterCsReq),
            "CmdRemoveRotaterCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdRemoveRotaterCsReq),
            "CmdUpdateEnergyScNotify" => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateEnergyScNotify),
            "CmdUpdateMapRotationDataScNotify" => ::std::option::Option::Some(CmdMapRotationType::CmdUpdateMapRotationDataScNotify),
            "CmdLeaveMapRotationRegionCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionCsReq),
            "CmdLeaveMapRotationRegionScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdLeaveMapRotationRegionScRsp),
            "CmdEnterMapRotationRegionScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdEnterMapRotationRegionScRsp),
            "CmdRotateMapCsReq" => ::std::option::Option::Some(CmdMapRotationType::CmdRotateMapCsReq),
            "CmdRotateMapScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdRotateMapScRsp),
            "CmdDeployRotaterScRsp" => ::std::option::Option::Some(CmdMapRotationType::CmdDeployRotaterScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdMapRotationType] = &[
        CmdMapRotationType::CmdMapRotationTypeNone,
        CmdMapRotationType::CmdInteractChargerScRsp,
        CmdMapRotationType::CmdLeaveMapRotationRegionScNotify,
        CmdMapRotationType::CmdResetMapRotationRegionCsReq,
        CmdMapRotationType::CmdRemoveRotaterScRsp,
        CmdMapRotationType::CmdEnterMapRotationRegionCsReq,
        CmdMapRotationType::CmdUpdateRotaterScNotify,
        CmdMapRotationType::CmdGetMapRotationDataCsReq,
        CmdMapRotationType::CmdGetMapRotationDataScRsp,
        CmdMapRotationType::CmdInteractChargerCsReq,
        CmdMapRotationType::CmdResetMapRotationRegionScRsp,
        CmdMapRotationType::CmdDeployRotaterCsReq,
        CmdMapRotationType::CmdRemoveRotaterCsReq,
        CmdMapRotationType::CmdUpdateEnergyScNotify,
        CmdMapRotationType::CmdUpdateMapRotationDataScNotify,
        CmdMapRotationType::CmdLeaveMapRotationRegionCsReq,
        CmdMapRotationType::CmdLeaveMapRotationRegionScRsp,
        CmdMapRotationType::CmdEnterMapRotationRegionScRsp,
        CmdMapRotationType::CmdRotateMapCsReq,
        CmdMapRotationType::CmdRotateMapScRsp,
        CmdMapRotationType::CmdDeployRotaterScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdMapRotationType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdMapRotationType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdMapRotationType::CmdMapRotationTypeNone => 0,
            CmdMapRotationType::CmdInteractChargerScRsp => 1,
            CmdMapRotationType::CmdLeaveMapRotationRegionScNotify => 2,
            CmdMapRotationType::CmdResetMapRotationRegionCsReq => 3,
            CmdMapRotationType::CmdRemoveRotaterScRsp => 4,
            CmdMapRotationType::CmdEnterMapRotationRegionCsReq => 5,
            CmdMapRotationType::CmdUpdateRotaterScNotify => 6,
            CmdMapRotationType::CmdGetMapRotationDataCsReq => 7,
            CmdMapRotationType::CmdGetMapRotationDataScRsp => 8,
            CmdMapRotationType::CmdInteractChargerCsReq => 9,
            CmdMapRotationType::CmdResetMapRotationRegionScRsp => 10,
            CmdMapRotationType::CmdDeployRotaterCsReq => 11,
            CmdMapRotationType::CmdRemoveRotaterCsReq => 12,
            CmdMapRotationType::CmdUpdateEnergyScNotify => 13,
            CmdMapRotationType::CmdUpdateMapRotationDataScNotify => 14,
            CmdMapRotationType::CmdLeaveMapRotationRegionCsReq => 15,
            CmdMapRotationType::CmdLeaveMapRotationRegionScRsp => 16,
            CmdMapRotationType::CmdEnterMapRotationRegionScRsp => 17,
            CmdMapRotationType::CmdRotateMapCsReq => 18,
            CmdMapRotationType::CmdRotateMapScRsp => 19,
            CmdMapRotationType::CmdDeployRotaterScRsp => 20,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdMapRotationType {
    fn default() -> Self {
        CmdMapRotationType::CmdMapRotationTypeNone
    }
}

impl CmdMapRotationType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdMapRotationType>("CmdMapRotationType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18CmdMapRotationType.proto*\xb8\x05\n\x12CmdMapRotationType\x12\x1a\
    \n\x16CmdMapRotationTypeNone\x10\0\x12\x1c\n\x17CmdInteractChargerScRsp\
    \x10\xb75\x12&\n!CmdLeaveMapRotationRegionScNotify\x10\xd95\x12#\n\x1eCm\
    dResetMapRotationRegionCsReq\x10\x955\x12\x1a\n\x15CmdRemoveRotaterScRsp\
    \x10\xc45\x12#\n\x1eCmdEnterMapRotationRegionCsReq\x10\xa45\x12\x1d\n\
    \x18CmdUpdateRotaterScNotify\x10\xd45\x12\x1f\n\x1aCmdGetMapRotationData\
    CsReq\x10\xe05\x12\x1f\n\x1aCmdGetMapRotationDataScRsp\x10\x925\x12\x1c\
    \n\x17CmdInteractChargerCsReq\x10\xd35\x12#\n\x1eCmdResetMapRotationRegi\
    onScRsp\x10\xda5\x12\x1a\n\x15CmdDeployRotaterCsReq\x10\xab5\x12\x1a\n\
    \x15CmdRemoveRotaterCsReq\x10\xb65\x12\x1c\n\x17CmdUpdateEnergyScNotify\
    \x10\xf25\x12%\n\x20CmdUpdateMapRotationDataScNotify\x10\xe55\x12#\n\x1e\
    CmdLeaveMapRotationRegionCsReq\x10\xc65\x12#\n\x1eCmdLeaveMapRotationReg\
    ionScRsp\x10\xdd5\x12#\n\x1eCmdEnterMapRotationRegionScRsp\x10\xeb5\x12\
    \x16\n\x11CmdRotateMapCsReq\x10\xd65\x12\x16\n\x11CmdRotateMapScRsp\x10\
    \xcb5\x12\x1a\n\x15CmdDeployRotaterScRsp\x10\xa55b\x06proto3\
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
            enums.push(CmdMapRotationType::generated_enum_descriptor_data());
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
