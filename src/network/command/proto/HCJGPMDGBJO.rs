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

//! Generated file from `HCJGPMDGBJO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HCJGPMDGBJO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HCJGPMDGBJO {
    // message fields
    // @@protoc_insertion_point(field:HCJGPMDGBJO.KLMGAEBEAGK)
    pub KLMGAEBEAGK: ::std::vec::Vec<super::INPINNPIHOB::INPINNPIHOB>,
    // @@protoc_insertion_point(field:HCJGPMDGBJO.MNKCJFELCNG)
    pub MNKCJFELCNG: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:HCJGPMDGBJO.BLBFDCGCEDA)
    pub BLBFDCGCEDA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:HCJGPMDGBJO.EPCKCOOKCLJ)
    pub EPCKCOOKCLJ: ::std::vec::Vec<super::OBIPOOLIDAL::OBIPOOLIDAL>,
    // special fields
    // @@protoc_insertion_point(special_field:HCJGPMDGBJO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HCJGPMDGBJO {
    fn default() -> &'a HCJGPMDGBJO {
        <HCJGPMDGBJO as ::protobuf::Message>::default_instance()
    }
}

impl HCJGPMDGBJO {
    pub fn new() -> HCJGPMDGBJO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KLMGAEBEAGK",
            |m: &HCJGPMDGBJO| { &m.KLMGAEBEAGK },
            |m: &mut HCJGPMDGBJO| { &mut m.KLMGAEBEAGK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MNKCJFELCNG",
            |m: &HCJGPMDGBJO| { &m.MNKCJFELCNG },
            |m: &mut HCJGPMDGBJO| { &mut m.MNKCJFELCNG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BLBFDCGCEDA",
            |m: &HCJGPMDGBJO| { &m.BLBFDCGCEDA },
            |m: &mut HCJGPMDGBJO| { &mut m.BLBFDCGCEDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EPCKCOOKCLJ",
            |m: &HCJGPMDGBJO| { &m.EPCKCOOKCLJ },
            |m: &mut HCJGPMDGBJO| { &mut m.EPCKCOOKCLJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HCJGPMDGBJO>(
            "HCJGPMDGBJO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HCJGPMDGBJO {
    const NAME: &'static str = "HCJGPMDGBJO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    self.KLMGAEBEAGK.push(is.read_message()?);
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.MNKCJFELCNG)?;
                },
                32 => {
                    self.MNKCJFELCNG.push(is.read_uint32()?);
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.BLBFDCGCEDA)?;
                },
                80 => {
                    self.BLBFDCGCEDA.push(is.read_uint32()?);
                },
                10 => {
                    self.EPCKCOOKCLJ.push(is.read_message()?);
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
        for value in &self.KLMGAEBEAGK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::vec_packed_uint32_size(4, &self.MNKCJFELCNG);
        my_size += ::protobuf::rt::vec_packed_uint32_size(10, &self.BLBFDCGCEDA);
        for value in &self.EPCKCOOKCLJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.KLMGAEBEAGK {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        os.write_repeated_packed_uint32(4, &self.MNKCJFELCNG)?;
        os.write_repeated_packed_uint32(10, &self.BLBFDCGCEDA)?;
        for v in &self.EPCKCOOKCLJ {
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

    fn new() -> HCJGPMDGBJO {
        HCJGPMDGBJO::new()
    }

    fn clear(&mut self) {
        self.KLMGAEBEAGK.clear();
        self.MNKCJFELCNG.clear();
        self.BLBFDCGCEDA.clear();
        self.EPCKCOOKCLJ.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HCJGPMDGBJO {
        static instance: HCJGPMDGBJO = HCJGPMDGBJO {
            KLMGAEBEAGK: ::std::vec::Vec::new(),
            MNKCJFELCNG: ::std::vec::Vec::new(),
            BLBFDCGCEDA: ::std::vec::Vec::new(),
            EPCKCOOKCLJ: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HCJGPMDGBJO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HCJGPMDGBJO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HCJGPMDGBJO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HCJGPMDGBJO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HCJGPMDGBJO.proto\x1a\x11INPINNPIHOB.proto\x1a\x11OBIPOOLIDAL.prot\
    o\"\xb1\x01\n\x0bHCJGPMDGBJO\x12.\n\x0bKLMGAEBEAGK\x18\x0f\x20\x03(\x0b2\
    \x0c.INPINNPIHOBR\x0bKLMGAEBEAGK\x12\x20\n\x0bMNKCJFELCNG\x18\x04\x20\
    \x03(\rR\x0bMNKCJFELCNG\x12\x20\n\x0bBLBFDCGCEDA\x18\n\x20\x03(\rR\x0bBL\
    BFDCGCEDA\x12.\n\x0bEPCKCOOKCLJ\x18\x01\x20\x03(\x0b2\x0c.OBIPOOLIDALR\
    \x0bEPCKCOOKCLJb\x06proto3\
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
            deps.push(super::INPINNPIHOB::file_descriptor().clone());
            deps.push(super::OBIPOOLIDAL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HCJGPMDGBJO::generated_message_descriptor_data());
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
