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

//! Generated file from `CIOMOMHALAL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:CIOMOMHALAL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CIOMOMHALAL {
    // message fields
    // @@protoc_insertion_point(field:CIOMOMHALAL.NJDMFPFKKIH)
    pub NJDMFPFKKIH: ::protobuf::EnumOrUnknown<super::ExtraLineupType::ExtraLineupType>,
    // @@protoc_insertion_point(field:CIOMOMHALAL.DENBNLILGIJ)
    pub DENBNLILGIJ: u32,
    // @@protoc_insertion_point(field:CIOMOMHALAL.FNOOEMHNIHI)
    pub FNOOEMHNIHI: ::std::vec::Vec<super::DMEBMEAGBOK::DMEBMEAGBOK>,
    // @@protoc_insertion_point(field:CIOMOMHALAL.PBLFLJNHMIL)
    pub PBLFLJNHMIL: ::protobuf::EnumOrUnknown<super::IJCLGJIMHME::IJCLGJIMHME>,
    // @@protoc_insertion_point(field:CIOMOMHALAL.DPJPNNCPNPJ)
    pub DPJPNNCPNPJ: u32,
    // @@protoc_insertion_point(field:CIOMOMHALAL.CFAAFJJAADP)
    pub CFAAFJJAADP: u32,
    // @@protoc_insertion_point(field:CIOMOMHALAL.PIMBLBKEECJ)
    pub PIMBLBKEECJ: u32,
    // @@protoc_insertion_point(field:CIOMOMHALAL.DNPHCJEBIKB)
    pub DNPHCJEBIKB: u32,
    // @@protoc_insertion_point(field:CIOMOMHALAL.MIFOLPKEOOO)
    pub MIFOLPKEOOO: ::protobuf::MessageField<super::MIOFFPLNBEH::MIOFFPLNBEH>,
    // special fields
    // @@protoc_insertion_point(special_field:CIOMOMHALAL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CIOMOMHALAL {
    fn default() -> &'a CIOMOMHALAL {
        <CIOMOMHALAL as ::protobuf::Message>::default_instance()
    }
}

impl CIOMOMHALAL {
    pub fn new() -> CIOMOMHALAL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NJDMFPFKKIH",
            |m: &CIOMOMHALAL| { &m.NJDMFPFKKIH },
            |m: &mut CIOMOMHALAL| { &mut m.NJDMFPFKKIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DENBNLILGIJ",
            |m: &CIOMOMHALAL| { &m.DENBNLILGIJ },
            |m: &mut CIOMOMHALAL| { &mut m.DENBNLILGIJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FNOOEMHNIHI",
            |m: &CIOMOMHALAL| { &m.FNOOEMHNIHI },
            |m: &mut CIOMOMHALAL| { &mut m.FNOOEMHNIHI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PBLFLJNHMIL",
            |m: &CIOMOMHALAL| { &m.PBLFLJNHMIL },
            |m: &mut CIOMOMHALAL| { &mut m.PBLFLJNHMIL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DPJPNNCPNPJ",
            |m: &CIOMOMHALAL| { &m.DPJPNNCPNPJ },
            |m: &mut CIOMOMHALAL| { &mut m.DPJPNNCPNPJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CFAAFJJAADP",
            |m: &CIOMOMHALAL| { &m.CFAAFJJAADP },
            |m: &mut CIOMOMHALAL| { &mut m.CFAAFJJAADP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PIMBLBKEECJ",
            |m: &CIOMOMHALAL| { &m.PIMBLBKEECJ },
            |m: &mut CIOMOMHALAL| { &mut m.PIMBLBKEECJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DNPHCJEBIKB",
            |m: &CIOMOMHALAL| { &m.DNPHCJEBIKB },
            |m: &mut CIOMOMHALAL| { &mut m.DNPHCJEBIKB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MIOFFPLNBEH::MIOFFPLNBEH>(
            "MIFOLPKEOOO",
            |m: &CIOMOMHALAL| { &m.MIFOLPKEOOO },
            |m: &mut CIOMOMHALAL| { &mut m.MIFOLPKEOOO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CIOMOMHALAL>(
            "CIOMOMHALAL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CIOMOMHALAL {
    const NAME: &'static str = "CIOMOMHALAL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.NJDMFPFKKIH = is.read_enum_or_unknown()?;
                },
                24 => {
                    self.DENBNLILGIJ = is.read_uint32()?;
                },
                74 => {
                    self.FNOOEMHNIHI.push(is.read_message()?);
                },
                96 => {
                    self.PBLFLJNHMIL = is.read_enum_or_unknown()?;
                },
                32 => {
                    self.DPJPNNCPNPJ = is.read_uint32()?;
                },
                88 => {
                    self.CFAAFJJAADP = is.read_uint32()?;
                },
                16 => {
                    self.PIMBLBKEECJ = is.read_uint32()?;
                },
                8 => {
                    self.DNPHCJEBIKB = is.read_uint32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MIFOLPKEOOO)?;
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
        if self.NJDMFPFKKIH != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.NJDMFPFKKIH.value());
        }
        if self.DENBNLILGIJ != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.DENBNLILGIJ);
        }
        for value in &self.FNOOEMHNIHI {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.PBLFLJNHMIL != ::protobuf::EnumOrUnknown::new(super::IJCLGJIMHME::IJCLGJIMHME::CHALLENGE_UNKNOWN) {
            my_size += ::protobuf::rt::int32_size(12, self.PBLFLJNHMIL.value());
        }
        if self.DPJPNNCPNPJ != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.DPJPNNCPNPJ);
        }
        if self.CFAAFJJAADP != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.CFAAFJJAADP);
        }
        if self.PIMBLBKEECJ != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.PIMBLBKEECJ);
        }
        if self.DNPHCJEBIKB != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.DNPHCJEBIKB);
        }
        if let Some(v) = self.MIFOLPKEOOO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NJDMFPFKKIH != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.NJDMFPFKKIH))?;
        }
        if self.DENBNLILGIJ != 0 {
            os.write_uint32(3, self.DENBNLILGIJ)?;
        }
        for v in &self.FNOOEMHNIHI {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.PBLFLJNHMIL != ::protobuf::EnumOrUnknown::new(super::IJCLGJIMHME::IJCLGJIMHME::CHALLENGE_UNKNOWN) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.PBLFLJNHMIL))?;
        }
        if self.DPJPNNCPNPJ != 0 {
            os.write_uint32(4, self.DPJPNNCPNPJ)?;
        }
        if self.CFAAFJJAADP != 0 {
            os.write_uint32(11, self.CFAAFJJAADP)?;
        }
        if self.PIMBLBKEECJ != 0 {
            os.write_uint32(2, self.PIMBLBKEECJ)?;
        }
        if self.DNPHCJEBIKB != 0 {
            os.write_uint32(1, self.DNPHCJEBIKB)?;
        }
        if let Some(v) = self.MIFOLPKEOOO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> CIOMOMHALAL {
        CIOMOMHALAL::new()
    }

    fn clear(&mut self) {
        self.NJDMFPFKKIH = ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE);
        self.DENBNLILGIJ = 0;
        self.FNOOEMHNIHI.clear();
        self.PBLFLJNHMIL = ::protobuf::EnumOrUnknown::new(super::IJCLGJIMHME::IJCLGJIMHME::CHALLENGE_UNKNOWN);
        self.DPJPNNCPNPJ = 0;
        self.CFAAFJJAADP = 0;
        self.PIMBLBKEECJ = 0;
        self.DNPHCJEBIKB = 0;
        self.MIFOLPKEOOO.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CIOMOMHALAL {
        static instance: CIOMOMHALAL = CIOMOMHALAL {
            NJDMFPFKKIH: ::protobuf::EnumOrUnknown::from_i32(0),
            DENBNLILGIJ: 0,
            FNOOEMHNIHI: ::std::vec::Vec::new(),
            PBLFLJNHMIL: ::protobuf::EnumOrUnknown::from_i32(0),
            DPJPNNCPNPJ: 0,
            CFAAFJJAADP: 0,
            PIMBLBKEECJ: 0,
            DNPHCJEBIKB: 0,
            MIFOLPKEOOO: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CIOMOMHALAL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CIOMOMHALAL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CIOMOMHALAL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CIOMOMHALAL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CIOMOMHALAL.proto\x1a\x11DMEBMEAGBOK.proto\x1a\x15ExtraLineupType.\
    proto\x1a\x11IJCLGJIMHME.proto\x1a\x11MIOFFPLNBEH.proto\"\xfb\x02\n\x0bC\
    IOMOMHALAL\x122\n\x0bNJDMFPFKKIH\x18\x08\x20\x01(\x0e2\x10.ExtraLineupTy\
    peR\x0bNJDMFPFKKIH\x12\x20\n\x0bDENBNLILGIJ\x18\x03\x20\x01(\rR\x0bDENBN\
    LILGIJ\x12.\n\x0bFNOOEMHNIHI\x18\t\x20\x03(\x0b2\x0c.DMEBMEAGBOKR\x0bFNO\
    OEMHNIHI\x12.\n\x0bPBLFLJNHMIL\x18\x0c\x20\x01(\x0e2\x0c.IJCLGJIMHMER\
    \x0bPBLFLJNHMIL\x12\x20\n\x0bDPJPNNCPNPJ\x18\x04\x20\x01(\rR\x0bDPJPNNCP\
    NPJ\x12\x20\n\x0bCFAAFJJAADP\x18\x0b\x20\x01(\rR\x0bCFAAFJJAADP\x12\x20\
    \n\x0bPIMBLBKEECJ\x18\x02\x20\x01(\rR\x0bPIMBLBKEECJ\x12\x20\n\x0bDNPHCJ\
    EBIKB\x18\x01\x20\x01(\rR\x0bDNPHCJEBIKB\x12.\n\x0bMIFOLPKEOOO\x18\x0f\
    \x20\x01(\x0b2\x0c.MIOFFPLNBEHR\x0bMIFOLPKEOOOb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::DMEBMEAGBOK::file_descriptor().clone());
            deps.push(super::ExtraLineupType::file_descriptor().clone());
            deps.push(super::IJCLGJIMHME::file_descriptor().clone());
            deps.push(super::MIOFFPLNBEH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CIOMOMHALAL::generated_message_descriptor_data());
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
