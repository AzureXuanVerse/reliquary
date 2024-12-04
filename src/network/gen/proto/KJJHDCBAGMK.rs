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

//! Generated file from `KJJHDCBAGMK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KJJHDCBAGMK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KJJHDCBAGMK {
    // message fields
    // @@protoc_insertion_point(field:KJJHDCBAGMK.MHBCMNCKKEB)
    pub MHBCMNCKKEB: ::protobuf::MessageField<super::KCIGLNKNJLH::KCIGLNKNJLH>,
    // @@protoc_insertion_point(field:KJJHDCBAGMK.PEPHONIOECA)
    pub PEPHONIOECA: ::protobuf::EnumOrUnknown<super::HEACKDPIAEK::HEACKDPIAEK>,
    // special fields
    // @@protoc_insertion_point(special_field:KJJHDCBAGMK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KJJHDCBAGMK {
    fn default() -> &'a KJJHDCBAGMK {
        <KJJHDCBAGMK as ::protobuf::Message>::default_instance()
    }
}

impl KJJHDCBAGMK {
    pub fn new() -> KJJHDCBAGMK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KCIGLNKNJLH::KCIGLNKNJLH>(
            "MHBCMNCKKEB",
            |m: &KJJHDCBAGMK| { &m.MHBCMNCKKEB },
            |m: &mut KJJHDCBAGMK| { &mut m.MHBCMNCKKEB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PEPHONIOECA",
            |m: &KJJHDCBAGMK| { &m.PEPHONIOECA },
            |m: &mut KJJHDCBAGMK| { &mut m.PEPHONIOECA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KJJHDCBAGMK>(
            "KJJHDCBAGMK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KJJHDCBAGMK {
    const NAME: &'static str = "KJJHDCBAGMK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MHBCMNCKKEB)?;
                },
                96 => {
                    self.PEPHONIOECA = is.read_enum_or_unknown()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.MHBCMNCKKEB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.PEPHONIOECA != ::protobuf::EnumOrUnknown::new(super::HEACKDPIAEK::HEACKDPIAEK::ROGUE_COMMON_ACTION_RESULT_SOURCE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(12, self.PEPHONIOECA.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.MHBCMNCKKEB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.PEPHONIOECA != ::protobuf::EnumOrUnknown::new(super::HEACKDPIAEK::HEACKDPIAEK::ROGUE_COMMON_ACTION_RESULT_SOURCE_TYPE_NONE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.PEPHONIOECA))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> KJJHDCBAGMK {
        KJJHDCBAGMK::new()
    }

    fn clear(&mut self) {
        self.MHBCMNCKKEB.clear();
        self.PEPHONIOECA = ::protobuf::EnumOrUnknown::new(super::HEACKDPIAEK::HEACKDPIAEK::ROGUE_COMMON_ACTION_RESULT_SOURCE_TYPE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KJJHDCBAGMK {
        static instance: KJJHDCBAGMK = KJJHDCBAGMK {
            MHBCMNCKKEB: ::protobuf::MessageField::none(),
            PEPHONIOECA: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KJJHDCBAGMK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KJJHDCBAGMK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KJJHDCBAGMK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KJJHDCBAGMK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KJJHDCBAGMK.proto\x1a\x11HEACKDPIAEK.proto\x1a\x11KCIGLNKNJLH.prot\
    o\"m\n\x0bKJJHDCBAGMK\x12.\n\x0bMHBCMNCKKEB\x18\r\x20\x01(\x0b2\x0c.KCIG\
    LNKNJLHR\x0bMHBCMNCKKEB\x12.\n\x0bPEPHONIOECA\x18\x0c\x20\x01(\x0e2\x0c.\
    HEACKDPIAEKR\x0bPEPHONIOECAb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::HEACKDPIAEK::file_descriptor().clone());
            deps.push(super::KCIGLNKNJLH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(KJJHDCBAGMK::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
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
