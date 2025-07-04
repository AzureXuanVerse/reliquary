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

//! Generated file from `JBIHOCIOANH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:JBIHOCIOANH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JBIHOCIOANH {
    // message fields
    // @@protoc_insertion_point(field:JBIHOCIOANH.EEPOJGNFLPA)
    pub EEPOJGNFLPA: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:JBIHOCIOANH.FPGEFHENCCF)
    pub FPGEFHENCCF: ::std::vec::Vec<super::KOEGFFOMKIP::KOEGFFOMKIP>,
    // @@protoc_insertion_point(field:JBIHOCIOANH.PLMKMOPCDLM)
    pub PLMKMOPCDLM: ::std::vec::Vec<super::OBIPOOLIDAL::OBIPOOLIDAL>,
    // @@protoc_insertion_point(field:JBIHOCIOANH.modifier__content)
    pub modifier__content: ::protobuf::MessageField<super::INPINNPIHOB::INPINNPIHOB>,
    // special fields
    // @@protoc_insertion_point(special_field:JBIHOCIOANH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JBIHOCIOANH {
    fn default() -> &'a JBIHOCIOANH {
        <JBIHOCIOANH as ::protobuf::Message>::default_instance()
    }
}

impl JBIHOCIOANH {
    pub fn new() -> JBIHOCIOANH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "EEPOJGNFLPA",
            |m: &JBIHOCIOANH| { &m.EEPOJGNFLPA },
            |m: &mut JBIHOCIOANH| { &mut m.EEPOJGNFLPA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FPGEFHENCCF",
            |m: &JBIHOCIOANH| { &m.FPGEFHENCCF },
            |m: &mut JBIHOCIOANH| { &mut m.FPGEFHENCCF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PLMKMOPCDLM",
            |m: &JBIHOCIOANH| { &m.PLMKMOPCDLM },
            |m: &mut JBIHOCIOANH| { &mut m.PLMKMOPCDLM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::INPINNPIHOB::INPINNPIHOB>(
            "modifier__content",
            |m: &JBIHOCIOANH| { &m.modifier__content },
            |m: &mut JBIHOCIOANH| { &mut m.modifier__content },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JBIHOCIOANH>(
            "JBIHOCIOANH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JBIHOCIOANH {
    const NAME: &'static str = "JBIHOCIOANH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.EEPOJGNFLPA.insert(key, value);
                },
                42 => {
                    self.FPGEFHENCCF.push(is.read_message()?);
                },
                90 => {
                    self.PLMKMOPCDLM.push(is.read_message()?);
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.modifier__content)?;
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
        for (k, v) in &self.EEPOJGNFLPA {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.FPGEFHENCCF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.PLMKMOPCDLM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.modifier__content.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for (k, v) in &self.EEPOJGNFLPA {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(26)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        for v in &self.FPGEFHENCCF {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        for v in &self.PLMKMOPCDLM {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if let Some(v) = self.modifier__content.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> JBIHOCIOANH {
        JBIHOCIOANH::new()
    }

    fn clear(&mut self) {
        self.EEPOJGNFLPA.clear();
        self.FPGEFHENCCF.clear();
        self.PLMKMOPCDLM.clear();
        self.modifier__content.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JBIHOCIOANH {
        static instance: ::protobuf::rt::Lazy<JBIHOCIOANH> = ::protobuf::rt::Lazy::new();
        instance.get(JBIHOCIOANH::new)
    }
}

impl ::protobuf::MessageFull for JBIHOCIOANH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JBIHOCIOANH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JBIHOCIOANH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JBIHOCIOANH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JBIHOCIOANH.proto\x1a\x11INPINNPIHOB.proto\x1a\x11KOEGFFOMKIP.prot\
    o\x1a\x11OBIPOOLIDAL.proto\"\xa8\x02\n\x0bJBIHOCIOANH\x12?\n\x0bEEPOJGNF\
    LPA\x18\x03\x20\x03(\x0b2\x1d.JBIHOCIOANH.EEPOJGNFLPAEntryR\x0bEEPOJGNFL\
    PA\x12.\n\x0bFPGEFHENCCF\x18\x05\x20\x03(\x0b2\x0c.KOEGFFOMKIPR\x0bFPGEF\
    HENCCF\x12.\n\x0bPLMKMOPCDLM\x18\x0b\x20\x03(\x0b2\x0c.OBIPOOLIDALR\x0bP\
    LMKMOPCDLM\x128\n\x11modifier__content\x18\x01\x20\x01(\x0b2\x0c.INPINNP\
    IHOBR\x0fmodifierContent\x1a>\n\x10EEPOJGNFLPAEntry\x12\x10\n\x03key\x18\
    \x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05value:\
    \x028\x01b\x06proto3\
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
            deps.push(super::INPINNPIHOB::file_descriptor().clone());
            deps.push(super::KOEGFFOMKIP::file_descriptor().clone());
            deps.push(super::OBIPOOLIDAL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JBIHOCIOANH::generated_message_descriptor_data());
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
