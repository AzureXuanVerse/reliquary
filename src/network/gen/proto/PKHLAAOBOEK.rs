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

//! Generated file from `PKHLAAOBOEK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PKHLAAOBOEK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PKHLAAOBOEK {
    // message fields
    // @@protoc_insertion_point(field:PKHLAAOBOEK.CAAFCCENGBL)
    pub CAAFCCENGBL: ::std::vec::Vec<super::IHNPCEMHIMB::IHNPCEMHIMB>,
    // @@protoc_insertion_point(field:PKHLAAOBOEK.FKIACGGJLJD)
    pub FKIACGGJLJD: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PKHLAAOBOEK.BGKJHMDJDCL)
    pub BGKJHMDJDCL: ::std::vec::Vec<super::LogisticsScore::LogisticsScore>,
    // @@protoc_insertion_point(field:PKHLAAOBOEK.JKDIGGIABMB)
    pub JKDIGGIABMB: ::std::vec::Vec<super::JBJMNMFPLPH::JBJMNMFPLPH>,
    // special fields
    // @@protoc_insertion_point(special_field:PKHLAAOBOEK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PKHLAAOBOEK {
    fn default() -> &'a PKHLAAOBOEK {
        <PKHLAAOBOEK as ::protobuf::Message>::default_instance()
    }
}

impl PKHLAAOBOEK {
    pub fn new() -> PKHLAAOBOEK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CAAFCCENGBL",
            |m: &PKHLAAOBOEK| { &m.CAAFCCENGBL },
            |m: &mut PKHLAAOBOEK| { &mut m.CAAFCCENGBL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FKIACGGJLJD",
            |m: &PKHLAAOBOEK| { &m.FKIACGGJLJD },
            |m: &mut PKHLAAOBOEK| { &mut m.FKIACGGJLJD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BGKJHMDJDCL",
            |m: &PKHLAAOBOEK| { &m.BGKJHMDJDCL },
            |m: &mut PKHLAAOBOEK| { &mut m.BGKJHMDJDCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JKDIGGIABMB",
            |m: &PKHLAAOBOEK| { &m.JKDIGGIABMB },
            |m: &mut PKHLAAOBOEK| { &mut m.JKDIGGIABMB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PKHLAAOBOEK>(
            "PKHLAAOBOEK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PKHLAAOBOEK {
    const NAME: &'static str = "PKHLAAOBOEK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    self.CAAFCCENGBL.push(is.read_message()?);
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.FKIACGGJLJD)?;
                },
                16 => {
                    self.FKIACGGJLJD.push(is.read_uint32()?);
                },
                98 => {
                    self.BGKJHMDJDCL.push(is.read_message()?);
                },
                10 => {
                    self.JKDIGGIABMB.push(is.read_message()?);
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
        for value in &self.CAAFCCENGBL {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.FKIACGGJLJD {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        for value in &self.BGKJHMDJDCL {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.JKDIGGIABMB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.CAAFCCENGBL {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        for v in &self.FKIACGGJLJD {
            os.write_uint32(2, *v)?;
        };
        for v in &self.BGKJHMDJDCL {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        for v in &self.JKDIGGIABMB {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PKHLAAOBOEK {
        PKHLAAOBOEK::new()
    }

    fn clear(&mut self) {
        self.CAAFCCENGBL.clear();
        self.FKIACGGJLJD.clear();
        self.BGKJHMDJDCL.clear();
        self.JKDIGGIABMB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PKHLAAOBOEK {
        static instance: PKHLAAOBOEK = PKHLAAOBOEK {
            CAAFCCENGBL: ::std::vec::Vec::new(),
            FKIACGGJLJD: ::std::vec::Vec::new(),
            BGKJHMDJDCL: ::std::vec::Vec::new(),
            JKDIGGIABMB: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PKHLAAOBOEK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PKHLAAOBOEK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PKHLAAOBOEK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PKHLAAOBOEK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PKHLAAOBOEK.proto\x1a\x11IHNPCEMHIMB.proto\x1a\x11JBJMNMFPLPH.prot\
    o\x1a\x14LogisticsScore.proto\"\xc2\x01\n\x0bPKHLAAOBOEK\x12.\n\x0bCAAFC\
    CENGBL\x18\n\x20\x03(\x0b2\x0c.IHNPCEMHIMBR\x0bCAAFCCENGBL\x12\x20\n\x0b\
    FKIACGGJLJD\x18\x02\x20\x03(\rR\x0bFKIACGGJLJD\x121\n\x0bBGKJHMDJDCL\x18\
    \x0c\x20\x03(\x0b2\x0f.LogisticsScoreR\x0bBGKJHMDJDCL\x12.\n\x0bJKDIGGIA\
    BMB\x18\x01\x20\x03(\x0b2\x0c.JBJMNMFPLPHR\x0bJKDIGGIABMBb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::IHNPCEMHIMB::file_descriptor().clone());
            deps.push(super::JBJMNMFPLPH::file_descriptor().clone());
            deps.push(super::LogisticsScore::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PKHLAAOBOEK::generated_message_descriptor_data());
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
