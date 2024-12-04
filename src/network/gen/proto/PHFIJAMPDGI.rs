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

//! Generated file from `PHFIJAMPDGI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:PHFIJAMPDGI)
pub enum PHFIJAMPDGI {
    // @@protoc_insertion_point(enum_value:PHFIJAMPDGI.ChangeStoryLineAction_None)
    ChangeStoryLineAction_None = 0,
    // @@protoc_insertion_point(enum_value:PHFIJAMPDGI.ChangeStoryLineAction_FinishAction)
    ChangeStoryLineAction_FinishAction = 1,
    // @@protoc_insertion_point(enum_value:PHFIJAMPDGI.ChangeStoryLineAction_Client)
    ChangeStoryLineAction_Client = 2,
    // @@protoc_insertion_point(enum_value:PHFIJAMPDGI.ChangeStoryLineAction_CustomOP)
    ChangeStoryLineAction_CustomOP = 3,
}

impl ::protobuf::Enum for PHFIJAMPDGI {
    const NAME: &'static str = "PHFIJAMPDGI";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PHFIJAMPDGI> {
        match value {
            0 => ::std::option::Option::Some(PHFIJAMPDGI::ChangeStoryLineAction_None),
            1 => ::std::option::Option::Some(PHFIJAMPDGI::ChangeStoryLineAction_FinishAction),
            2 => ::std::option::Option::Some(PHFIJAMPDGI::ChangeStoryLineAction_Client),
            3 => ::std::option::Option::Some(PHFIJAMPDGI::ChangeStoryLineAction_CustomOP),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<PHFIJAMPDGI> {
        match str {
            "ChangeStoryLineAction_None" => ::std::option::Option::Some(PHFIJAMPDGI::ChangeStoryLineAction_None),
            "ChangeStoryLineAction_FinishAction" => ::std::option::Option::Some(PHFIJAMPDGI::ChangeStoryLineAction_FinishAction),
            "ChangeStoryLineAction_Client" => ::std::option::Option::Some(PHFIJAMPDGI::ChangeStoryLineAction_Client),
            "ChangeStoryLineAction_CustomOP" => ::std::option::Option::Some(PHFIJAMPDGI::ChangeStoryLineAction_CustomOP),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [PHFIJAMPDGI] = &[
        PHFIJAMPDGI::ChangeStoryLineAction_None,
        PHFIJAMPDGI::ChangeStoryLineAction_FinishAction,
        PHFIJAMPDGI::ChangeStoryLineAction_Client,
        PHFIJAMPDGI::ChangeStoryLineAction_CustomOP,
    ];
}

impl ::protobuf::EnumFull for PHFIJAMPDGI {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("PHFIJAMPDGI").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for PHFIJAMPDGI {
    fn default() -> Self {
        PHFIJAMPDGI::ChangeStoryLineAction_None
    }
}

impl PHFIJAMPDGI {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<PHFIJAMPDGI>("PHFIJAMPDGI")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PHFIJAMPDGI.proto*\x9b\x01\n\x0bPHFIJAMPDGI\x12\x1e\n\x1aChangeSto\
    ryLineAction_None\x10\0\x12&\n\"ChangeStoryLineAction_FinishAction\x10\
    \x01\x12\x20\n\x1cChangeStoryLineAction_Client\x10\x02\x12\"\n\x1eChange\
    StoryLineAction_CustomOP\x10\x03b\x06proto3\
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
            enums.push(PHFIJAMPDGI::generated_enum_descriptor_data());
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
