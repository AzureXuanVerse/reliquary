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

//! Generated file from `PlatformType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:PlatformType)
pub enum PlatformType {
    // @@protoc_insertion_point(enum_value:PlatformType.EDITOR)
    EDITOR = 0,
    // @@protoc_insertion_point(enum_value:PlatformType.IOS)
    IOS = 1,
    // @@protoc_insertion_point(enum_value:PlatformType.ANDROID)
    ANDROID = 2,
    // @@protoc_insertion_point(enum_value:PlatformType.PC)
    PC = 3,
    // @@protoc_insertion_point(enum_value:PlatformType.WEB)
    WEB = 4,
    // @@protoc_insertion_point(enum_value:PlatformType.WAP)
    WAP = 5,
    // @@protoc_insertion_point(enum_value:PlatformType.PS4)
    PS4 = 6,
    // @@protoc_insertion_point(enum_value:PlatformType.NINTENDO)
    NINTENDO = 7,
    // @@protoc_insertion_point(enum_value:PlatformType.CLOUD_ANDROID)
    CLOUD_ANDROID = 8,
    // @@protoc_insertion_point(enum_value:PlatformType.CLOUD_PC)
    CLOUD_PC = 9,
    // @@protoc_insertion_point(enum_value:PlatformType.CLOUD_IOS)
    CLOUD_IOS = 10,
    // @@protoc_insertion_point(enum_value:PlatformType.PS5)
    PS5 = 11,
    // @@protoc_insertion_point(enum_value:PlatformType.MAC)
    MAC = 12,
    // @@protoc_insertion_point(enum_value:PlatformType.CLOUD_MAC)
    CLOUD_MAC = 13,
    // @@protoc_insertion_point(enum_value:PlatformType.CLOUD_WEB_ANDROID)
    CLOUD_WEB_ANDROID = 20,
    // @@protoc_insertion_point(enum_value:PlatformType.CLOUD_WEB_IOS)
    CLOUD_WEB_IOS = 21,
    // @@protoc_insertion_point(enum_value:PlatformType.CLOUD_WEB_PC)
    CLOUD_WEB_PC = 22,
    // @@protoc_insertion_point(enum_value:PlatformType.CLOUD_WEB_MAC)
    CLOUD_WEB_MAC = 23,
    // @@protoc_insertion_point(enum_value:PlatformType.CLOUD_WEB_TOUCH)
    CLOUD_WEB_TOUCH = 24,
    // @@protoc_insertion_point(enum_value:PlatformType.CLOUD_WEB_KEYBOARD)
    CLOUD_WEB_KEYBOARD = 25,
    // @@protoc_insertion_point(enum_value:PlatformType.CLOUD_DOUYIN_IOS)
    CLOUD_DOUYIN_IOS = 27,
    // @@protoc_insertion_point(enum_value:PlatformType.CLOUD_DOUYIN_ANDROID)
    CLOUD_DOUYIN_ANDROID = 28,
}

impl ::protobuf::Enum for PlatformType {
    const NAME: &'static str = "PlatformType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PlatformType> {
        match value {
            0 => ::std::option::Option::Some(PlatformType::EDITOR),
            1 => ::std::option::Option::Some(PlatformType::IOS),
            2 => ::std::option::Option::Some(PlatformType::ANDROID),
            3 => ::std::option::Option::Some(PlatformType::PC),
            4 => ::std::option::Option::Some(PlatformType::WEB),
            5 => ::std::option::Option::Some(PlatformType::WAP),
            6 => ::std::option::Option::Some(PlatformType::PS4),
            7 => ::std::option::Option::Some(PlatformType::NINTENDO),
            8 => ::std::option::Option::Some(PlatformType::CLOUD_ANDROID),
            9 => ::std::option::Option::Some(PlatformType::CLOUD_PC),
            10 => ::std::option::Option::Some(PlatformType::CLOUD_IOS),
            11 => ::std::option::Option::Some(PlatformType::PS5),
            12 => ::std::option::Option::Some(PlatformType::MAC),
            13 => ::std::option::Option::Some(PlatformType::CLOUD_MAC),
            20 => ::std::option::Option::Some(PlatformType::CLOUD_WEB_ANDROID),
            21 => ::std::option::Option::Some(PlatformType::CLOUD_WEB_IOS),
            22 => ::std::option::Option::Some(PlatformType::CLOUD_WEB_PC),
            23 => ::std::option::Option::Some(PlatformType::CLOUD_WEB_MAC),
            24 => ::std::option::Option::Some(PlatformType::CLOUD_WEB_TOUCH),
            25 => ::std::option::Option::Some(PlatformType::CLOUD_WEB_KEYBOARD),
            27 => ::std::option::Option::Some(PlatformType::CLOUD_DOUYIN_IOS),
            28 => ::std::option::Option::Some(PlatformType::CLOUD_DOUYIN_ANDROID),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<PlatformType> {
        match str {
            "EDITOR" => ::std::option::Option::Some(PlatformType::EDITOR),
            "IOS" => ::std::option::Option::Some(PlatformType::IOS),
            "ANDROID" => ::std::option::Option::Some(PlatformType::ANDROID),
            "PC" => ::std::option::Option::Some(PlatformType::PC),
            "WEB" => ::std::option::Option::Some(PlatformType::WEB),
            "WAP" => ::std::option::Option::Some(PlatformType::WAP),
            "PS4" => ::std::option::Option::Some(PlatformType::PS4),
            "NINTENDO" => ::std::option::Option::Some(PlatformType::NINTENDO),
            "CLOUD_ANDROID" => ::std::option::Option::Some(PlatformType::CLOUD_ANDROID),
            "CLOUD_PC" => ::std::option::Option::Some(PlatformType::CLOUD_PC),
            "CLOUD_IOS" => ::std::option::Option::Some(PlatformType::CLOUD_IOS),
            "PS5" => ::std::option::Option::Some(PlatformType::PS5),
            "MAC" => ::std::option::Option::Some(PlatformType::MAC),
            "CLOUD_MAC" => ::std::option::Option::Some(PlatformType::CLOUD_MAC),
            "CLOUD_WEB_ANDROID" => ::std::option::Option::Some(PlatformType::CLOUD_WEB_ANDROID),
            "CLOUD_WEB_IOS" => ::std::option::Option::Some(PlatformType::CLOUD_WEB_IOS),
            "CLOUD_WEB_PC" => ::std::option::Option::Some(PlatformType::CLOUD_WEB_PC),
            "CLOUD_WEB_MAC" => ::std::option::Option::Some(PlatformType::CLOUD_WEB_MAC),
            "CLOUD_WEB_TOUCH" => ::std::option::Option::Some(PlatformType::CLOUD_WEB_TOUCH),
            "CLOUD_WEB_KEYBOARD" => ::std::option::Option::Some(PlatformType::CLOUD_WEB_KEYBOARD),
            "CLOUD_DOUYIN_IOS" => ::std::option::Option::Some(PlatformType::CLOUD_DOUYIN_IOS),
            "CLOUD_DOUYIN_ANDROID" => ::std::option::Option::Some(PlatformType::CLOUD_DOUYIN_ANDROID),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [PlatformType] = &[
        PlatformType::EDITOR,
        PlatformType::IOS,
        PlatformType::ANDROID,
        PlatformType::PC,
        PlatformType::WEB,
        PlatformType::WAP,
        PlatformType::PS4,
        PlatformType::NINTENDO,
        PlatformType::CLOUD_ANDROID,
        PlatformType::CLOUD_PC,
        PlatformType::CLOUD_IOS,
        PlatformType::PS5,
        PlatformType::MAC,
        PlatformType::CLOUD_MAC,
        PlatformType::CLOUD_WEB_ANDROID,
        PlatformType::CLOUD_WEB_IOS,
        PlatformType::CLOUD_WEB_PC,
        PlatformType::CLOUD_WEB_MAC,
        PlatformType::CLOUD_WEB_TOUCH,
        PlatformType::CLOUD_WEB_KEYBOARD,
        PlatformType::CLOUD_DOUYIN_IOS,
        PlatformType::CLOUD_DOUYIN_ANDROID,
    ];
}

impl ::protobuf::EnumFull for PlatformType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("PlatformType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            PlatformType::EDITOR => 0,
            PlatformType::IOS => 1,
            PlatformType::ANDROID => 2,
            PlatformType::PC => 3,
            PlatformType::WEB => 4,
            PlatformType::WAP => 5,
            PlatformType::PS4 => 6,
            PlatformType::NINTENDO => 7,
            PlatformType::CLOUD_ANDROID => 8,
            PlatformType::CLOUD_PC => 9,
            PlatformType::CLOUD_IOS => 10,
            PlatformType::PS5 => 11,
            PlatformType::MAC => 12,
            PlatformType::CLOUD_MAC => 13,
            PlatformType::CLOUD_WEB_ANDROID => 14,
            PlatformType::CLOUD_WEB_IOS => 15,
            PlatformType::CLOUD_WEB_PC => 16,
            PlatformType::CLOUD_WEB_MAC => 17,
            PlatformType::CLOUD_WEB_TOUCH => 18,
            PlatformType::CLOUD_WEB_KEYBOARD => 19,
            PlatformType::CLOUD_DOUYIN_IOS => 20,
            PlatformType::CLOUD_DOUYIN_ANDROID => 21,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for PlatformType {
    fn default() -> Self {
        PlatformType::EDITOR
    }
}

impl PlatformType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<PlatformType>("PlatformType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12PlatformType.proto*\xde\x02\n\x0cPlatformType\x12\n\n\x06EDITOR\
    \x10\0\x12\x07\n\x03IOS\x10\x01\x12\x0b\n\x07ANDROID\x10\x02\x12\x06\n\
    \x02PC\x10\x03\x12\x07\n\x03WEB\x10\x04\x12\x07\n\x03WAP\x10\x05\x12\x07\
    \n\x03PS4\x10\x06\x12\x0c\n\x08NINTENDO\x10\x07\x12\x11\n\rCLOUD_ANDROID\
    \x10\x08\x12\x0c\n\x08CLOUD_PC\x10\t\x12\r\n\tCLOUD_IOS\x10\n\x12\x07\n\
    \x03PS5\x10\x0b\x12\x07\n\x03MAC\x10\x0c\x12\r\n\tCLOUD_MAC\x10\r\x12\
    \x15\n\x11CLOUD_WEB_ANDROID\x10\x14\x12\x11\n\rCLOUD_WEB_IOS\x10\x15\x12\
    \x10\n\x0cCLOUD_WEB_PC\x10\x16\x12\x11\n\rCLOUD_WEB_MAC\x10\x17\x12\x13\
    \n\x0fCLOUD_WEB_TOUCH\x10\x18\x12\x16\n\x12CLOUD_WEB_KEYBOARD\x10\x19\
    \x12\x14\n\x10CLOUD_DOUYIN_IOS\x10\x1b\x12\x18\n\x14CLOUD_DOUYIN_ANDROID\
    \x10\x1cb\x06proto3\
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
            enums.push(PlatformType::generated_enum_descriptor_data());
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
