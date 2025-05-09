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

//! Generated file from `EBHGHGIIGOM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:EBHGHGIIGOM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EBHGHGIIGOM {
    // message fields
    // @@protoc_insertion_point(field:EBHGHGIIGOM.BILBOHBDBPN)
    pub BILBOHBDBPN: u32,
    // @@protoc_insertion_point(field:EBHGHGIIGOM.JACIGHHGCGB)
    pub JACIGHHGCGB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:EBHGHGIIGOM.KOMJMFIKBAM)
    pub KOMJMFIKBAM: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:EBHGHGIIGOM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EBHGHGIIGOM {
    fn default() -> &'a EBHGHGIIGOM {
        <EBHGHGIIGOM as ::protobuf::Message>::default_instance()
    }
}

impl EBHGHGIIGOM {
    pub fn new() -> EBHGHGIIGOM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BILBOHBDBPN",
            |m: &EBHGHGIIGOM| { &m.BILBOHBDBPN },
            |m: &mut EBHGHGIIGOM| { &mut m.BILBOHBDBPN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JACIGHHGCGB",
            |m: &EBHGHGIIGOM| { &m.JACIGHHGCGB },
            |m: &mut EBHGHGIIGOM| { &mut m.JACIGHHGCGB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KOMJMFIKBAM",
            |m: &EBHGHGIIGOM| { &m.KOMJMFIKBAM },
            |m: &mut EBHGHGIIGOM| { &mut m.KOMJMFIKBAM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EBHGHGIIGOM>(
            "EBHGHGIIGOM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EBHGHGIIGOM {
    const NAME: &'static str = "EBHGHGIIGOM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.BILBOHBDBPN = is.read_uint32()?;
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.JACIGHHGCGB)?;
                },
                32 => {
                    self.JACIGHHGCGB.push(is.read_uint32()?);
                },
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.KOMJMFIKBAM)?;
                },
                88 => {
                    self.KOMJMFIKBAM.push(is.read_uint32()?);
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
        if self.BILBOHBDBPN != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.BILBOHBDBPN);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(4, &self.JACIGHHGCGB);
        my_size += ::protobuf::rt::vec_packed_uint32_size(11, &self.KOMJMFIKBAM);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BILBOHBDBPN != 0 {
            os.write_uint32(15, self.BILBOHBDBPN)?;
        }
        os.write_repeated_packed_uint32(4, &self.JACIGHHGCGB)?;
        os.write_repeated_packed_uint32(11, &self.KOMJMFIKBAM)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> EBHGHGIIGOM {
        EBHGHGIIGOM::new()
    }

    fn clear(&mut self) {
        self.BILBOHBDBPN = 0;
        self.JACIGHHGCGB.clear();
        self.KOMJMFIKBAM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EBHGHGIIGOM {
        static instance: EBHGHGIIGOM = EBHGHGIIGOM {
            BILBOHBDBPN: 0,
            JACIGHHGCGB: ::std::vec::Vec::new(),
            KOMJMFIKBAM: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EBHGHGIIGOM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EBHGHGIIGOM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EBHGHGIIGOM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EBHGHGIIGOM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EBHGHGIIGOM.proto\"s\n\x0bEBHGHGIIGOM\x12\x20\n\x0bBILBOHBDBPN\x18\
    \x0f\x20\x01(\rR\x0bBILBOHBDBPN\x12\x20\n\x0bJACIGHHGCGB\x18\x04\x20\x03\
    (\rR\x0bJACIGHHGCGB\x12\x20\n\x0bKOMJMFIKBAM\x18\x0b\x20\x03(\rR\x0bKOMJ\
    MFIKBAMb\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EBHGHGIIGOM::generated_message_descriptor_data());
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
