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

//! Generated file from `CmdShopType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdShopType)
pub enum CmdShopType {
    // @@protoc_insertion_point(enum_value:CmdShopType.CmdShopTypeNone)
    CmdShopTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdShopType.CmdGetShopListScRsp)
    CmdGetShopListScRsp = 1513,
    // @@protoc_insertion_point(enum_value:CmdShopType.CmdBuyGoodsScRsp)
    CmdBuyGoodsScRsp = 1509,
    // @@protoc_insertion_point(enum_value:CmdShopType.CmdTakeCityShopRewardCsReq)
    CmdTakeCityShopRewardCsReq = 1535,
    // @@protoc_insertion_point(enum_value:CmdShopType.CmdTakeCityShopRewardScRsp)
    CmdTakeCityShopRewardScRsp = 1506,
    // @@protoc_insertion_point(enum_value:CmdShopType.CmdGetShopListCsReq)
    CmdGetShopListCsReq = 1511,
    // @@protoc_insertion_point(enum_value:CmdShopType.CmdBuyGoodsCsReq)
    CmdBuyGoodsCsReq = 1547,
    // @@protoc_insertion_point(enum_value:CmdShopType.CmdCityShopInfoScNotify)
    CmdCityShopInfoScNotify = 1570,
}

impl ::protobuf::Enum for CmdShopType {
    const NAME: &'static str = "CmdShopType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdShopType> {
        match value {
            0 => ::std::option::Option::Some(CmdShopType::CmdShopTypeNone),
            1513 => ::std::option::Option::Some(CmdShopType::CmdGetShopListScRsp),
            1509 => ::std::option::Option::Some(CmdShopType::CmdBuyGoodsScRsp),
            1535 => ::std::option::Option::Some(CmdShopType::CmdTakeCityShopRewardCsReq),
            1506 => ::std::option::Option::Some(CmdShopType::CmdTakeCityShopRewardScRsp),
            1511 => ::std::option::Option::Some(CmdShopType::CmdGetShopListCsReq),
            1547 => ::std::option::Option::Some(CmdShopType::CmdBuyGoodsCsReq),
            1570 => ::std::option::Option::Some(CmdShopType::CmdCityShopInfoScNotify),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdShopType> {
        match str {
            "CmdShopTypeNone" => ::std::option::Option::Some(CmdShopType::CmdShopTypeNone),
            "CmdGetShopListScRsp" => ::std::option::Option::Some(CmdShopType::CmdGetShopListScRsp),
            "CmdBuyGoodsScRsp" => ::std::option::Option::Some(CmdShopType::CmdBuyGoodsScRsp),
            "CmdTakeCityShopRewardCsReq" => ::std::option::Option::Some(CmdShopType::CmdTakeCityShopRewardCsReq),
            "CmdTakeCityShopRewardScRsp" => ::std::option::Option::Some(CmdShopType::CmdTakeCityShopRewardScRsp),
            "CmdGetShopListCsReq" => ::std::option::Option::Some(CmdShopType::CmdGetShopListCsReq),
            "CmdBuyGoodsCsReq" => ::std::option::Option::Some(CmdShopType::CmdBuyGoodsCsReq),
            "CmdCityShopInfoScNotify" => ::std::option::Option::Some(CmdShopType::CmdCityShopInfoScNotify),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdShopType] = &[
        CmdShopType::CmdShopTypeNone,
        CmdShopType::CmdGetShopListScRsp,
        CmdShopType::CmdBuyGoodsScRsp,
        CmdShopType::CmdTakeCityShopRewardCsReq,
        CmdShopType::CmdTakeCityShopRewardScRsp,
        CmdShopType::CmdGetShopListCsReq,
        CmdShopType::CmdBuyGoodsCsReq,
        CmdShopType::CmdCityShopInfoScNotify,
    ];
}

impl ::protobuf::EnumFull for CmdShopType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdShopType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdShopType::CmdShopTypeNone => 0,
            CmdShopType::CmdGetShopListScRsp => 1,
            CmdShopType::CmdBuyGoodsScRsp => 2,
            CmdShopType::CmdTakeCityShopRewardCsReq => 3,
            CmdShopType::CmdTakeCityShopRewardScRsp => 4,
            CmdShopType::CmdGetShopListCsReq => 5,
            CmdShopType::CmdBuyGoodsCsReq => 6,
            CmdShopType::CmdCityShopInfoScNotify => 7,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdShopType {
    fn default() -> Self {
        CmdShopType::CmdShopTypeNone
    }
}

impl CmdShopType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdShopType>("CmdShopType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CmdShopType.proto*\xe4\x01\n\x0bCmdShopType\x12\x13\n\x0fCmdShopTy\
    peNone\x10\0\x12\x18\n\x13CmdGetShopListScRsp\x10\xe9\x0b\x12\x15\n\x10C\
    mdBuyGoodsScRsp\x10\xe5\x0b\x12\x1f\n\x1aCmdTakeCityShopRewardCsReq\x10\
    \xff\x0b\x12\x1f\n\x1aCmdTakeCityShopRewardScRsp\x10\xe2\x0b\x12\x18\n\
    \x13CmdGetShopListCsReq\x10\xe7\x0b\x12\x15\n\x10CmdBuyGoodsCsReq\x10\
    \x8b\x0c\x12\x1c\n\x17CmdCityShopInfoScNotify\x10\xa2\x0cb\x06proto3\
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
            enums.push(CmdShopType::generated_enum_descriptor_data());
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
