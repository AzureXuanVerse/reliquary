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

//! Generated file from `IOIJEPGMJAF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:IOIJEPGMJAF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IOIJEPGMJAF {
    // message fields
    // @@protoc_insertion_point(field:IOIJEPGMJAF.HCBBHCKJNJI)
    pub HCBBHCKJNJI: i32,
    // @@protoc_insertion_point(field:IOIJEPGMJAF.JGIBHFJMOBE)
    pub JGIBHFJMOBE: ::protobuf::EnumOrUnknown<super::KJJLDIMBGMM::KJJLDIMBGMM>,
    // @@protoc_insertion_point(field:IOIJEPGMJAF.HLOACJFDNGN)
    pub HLOACJFDNGN: u32,
    // @@protoc_insertion_point(field:IOIJEPGMJAF.CPKPINCCEIP)
    pub CPKPINCCEIP: ::std::vec::Vec<super::BFDLHIDCNMF::BFDLHIDCNMF>,
    // @@protoc_insertion_point(field:IOIJEPGMJAF.MCOKHHFPBPJ)
    pub MCOKHHFPBPJ: u64,
    // @@protoc_insertion_point(field:IOIJEPGMJAF.MEJDMJIKIMO)
    pub MEJDMJIKIMO: ::protobuf::MessageField<super::MDOHAFBEEPK::MDOHAFBEEPK>,
    // @@protoc_insertion_point(field:IOIJEPGMJAF.EBKPMEOBNPO)
    pub EBKPMEOBNPO: u32,
    // special fields
    // @@protoc_insertion_point(special_field:IOIJEPGMJAF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IOIJEPGMJAF {
    fn default() -> &'a IOIJEPGMJAF {
        <IOIJEPGMJAF as ::protobuf::Message>::default_instance()
    }
}

impl IOIJEPGMJAF {
    pub fn new() -> IOIJEPGMJAF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HCBBHCKJNJI",
            |m: &IOIJEPGMJAF| { &m.HCBBHCKJNJI },
            |m: &mut IOIJEPGMJAF| { &mut m.HCBBHCKJNJI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JGIBHFJMOBE",
            |m: &IOIJEPGMJAF| { &m.JGIBHFJMOBE },
            |m: &mut IOIJEPGMJAF| { &mut m.JGIBHFJMOBE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HLOACJFDNGN",
            |m: &IOIJEPGMJAF| { &m.HLOACJFDNGN },
            |m: &mut IOIJEPGMJAF| { &mut m.HLOACJFDNGN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CPKPINCCEIP",
            |m: &IOIJEPGMJAF| { &m.CPKPINCCEIP },
            |m: &mut IOIJEPGMJAF| { &mut m.CPKPINCCEIP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MCOKHHFPBPJ",
            |m: &IOIJEPGMJAF| { &m.MCOKHHFPBPJ },
            |m: &mut IOIJEPGMJAF| { &mut m.MCOKHHFPBPJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MDOHAFBEEPK::MDOHAFBEEPK>(
            "MEJDMJIKIMO",
            |m: &IOIJEPGMJAF| { &m.MEJDMJIKIMO },
            |m: &mut IOIJEPGMJAF| { &mut m.MEJDMJIKIMO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EBKPMEOBNPO",
            |m: &IOIJEPGMJAF| { &m.EBKPMEOBNPO },
            |m: &mut IOIJEPGMJAF| { &mut m.EBKPMEOBNPO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IOIJEPGMJAF>(
            "IOIJEPGMJAF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IOIJEPGMJAF {
    const NAME: &'static str = "IOIJEPGMJAF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.HCBBHCKJNJI = is.read_int32()?;
                },
                72 => {
                    self.JGIBHFJMOBE = is.read_enum_or_unknown()?;
                },
                112 => {
                    self.HLOACJFDNGN = is.read_uint32()?;
                },
                82 => {
                    self.CPKPINCCEIP.push(is.read_message()?);
                },
                120 => {
                    self.MCOKHHFPBPJ = is.read_uint64()?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MEJDMJIKIMO)?;
                },
                48 => {
                    self.EBKPMEOBNPO = is.read_uint32()?;
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
        if self.HCBBHCKJNJI != 0 {
            my_size += ::protobuf::rt::int32_size(13, self.HCBBHCKJNJI);
        }
        if self.JGIBHFJMOBE != ::protobuf::EnumOrUnknown::new(super::KJJLDIMBGMM::KJJLDIMBGMM::MATCH3_SOLO_STATE_IDLE) {
            my_size += ::protobuf::rt::int32_size(9, self.JGIBHFJMOBE.value());
        }
        if self.HLOACJFDNGN != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.HLOACJFDNGN);
        }
        for value in &self.CPKPINCCEIP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.MCOKHHFPBPJ != 0 {
            my_size += ::protobuf::rt::uint64_size(15, self.MCOKHHFPBPJ);
        }
        if let Some(v) = self.MEJDMJIKIMO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.EBKPMEOBNPO != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.EBKPMEOBNPO);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HCBBHCKJNJI != 0 {
            os.write_int32(13, self.HCBBHCKJNJI)?;
        }
        if self.JGIBHFJMOBE != ::protobuf::EnumOrUnknown::new(super::KJJLDIMBGMM::KJJLDIMBGMM::MATCH3_SOLO_STATE_IDLE) {
            os.write_enum(9, ::protobuf::EnumOrUnknown::value(&self.JGIBHFJMOBE))?;
        }
        if self.HLOACJFDNGN != 0 {
            os.write_uint32(14, self.HLOACJFDNGN)?;
        }
        for v in &self.CPKPINCCEIP {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if self.MCOKHHFPBPJ != 0 {
            os.write_uint64(15, self.MCOKHHFPBPJ)?;
        }
        if let Some(v) = self.MEJDMJIKIMO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.EBKPMEOBNPO != 0 {
            os.write_uint32(6, self.EBKPMEOBNPO)?;
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

    fn new() -> IOIJEPGMJAF {
        IOIJEPGMJAF::new()
    }

    fn clear(&mut self) {
        self.HCBBHCKJNJI = 0;
        self.JGIBHFJMOBE = ::protobuf::EnumOrUnknown::new(super::KJJLDIMBGMM::KJJLDIMBGMM::MATCH3_SOLO_STATE_IDLE);
        self.HLOACJFDNGN = 0;
        self.CPKPINCCEIP.clear();
        self.MCOKHHFPBPJ = 0;
        self.MEJDMJIKIMO.clear();
        self.EBKPMEOBNPO = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IOIJEPGMJAF {
        static instance: IOIJEPGMJAF = IOIJEPGMJAF {
            HCBBHCKJNJI: 0,
            JGIBHFJMOBE: ::protobuf::EnumOrUnknown::from_i32(0),
            HLOACJFDNGN: 0,
            CPKPINCCEIP: ::std::vec::Vec::new(),
            MCOKHHFPBPJ: 0,
            MEJDMJIKIMO: ::protobuf::MessageField::none(),
            EBKPMEOBNPO: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IOIJEPGMJAF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IOIJEPGMJAF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IOIJEPGMJAF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IOIJEPGMJAF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IOIJEPGMJAF.proto\x1a\x11BFDLHIDCNMF.proto\x1a\x11KJJLDIMBGMM.prot\
    o\x1a\x11MDOHAFBEEPK.proto\"\xa5\x02\n\x0bIOIJEPGMJAF\x12\x20\n\x0bHCBBH\
    CKJNJI\x18\r\x20\x01(\x05R\x0bHCBBHCKJNJI\x12.\n\x0bJGIBHFJMOBE\x18\t\
    \x20\x01(\x0e2\x0c.KJJLDIMBGMMR\x0bJGIBHFJMOBE\x12\x20\n\x0bHLOACJFDNGN\
    \x18\x0e\x20\x01(\rR\x0bHLOACJFDNGN\x12.\n\x0bCPKPINCCEIP\x18\n\x20\x03(\
    \x0b2\x0c.BFDLHIDCNMFR\x0bCPKPINCCEIP\x12\x20\n\x0bMCOKHHFPBPJ\x18\x0f\
    \x20\x01(\x04R\x0bMCOKHHFPBPJ\x12.\n\x0bMEJDMJIKIMO\x18\x01\x20\x01(\x0b\
    2\x0c.MDOHAFBEEPKR\x0bMEJDMJIKIMO\x12\x20\n\x0bEBKPMEOBNPO\x18\x06\x20\
    \x01(\rR\x0bEBKPMEOBNPOb\x06proto3\
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
            deps.push(super::BFDLHIDCNMF::file_descriptor().clone());
            deps.push(super::KJJLDIMBGMM::file_descriptor().clone());
            deps.push(super::MDOHAFBEEPK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IOIJEPGMJAF::generated_message_descriptor_data());
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
