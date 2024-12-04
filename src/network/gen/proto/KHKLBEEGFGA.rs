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

//! Generated file from `KHKLBEEGFGA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:KHKLBEEGFGA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KHKLBEEGFGA {
    // message fields
    // @@protoc_insertion_point(field:KHKLBEEGFGA.BNKPPBPIGAO)
    pub BNKPPBPIGAO: u32,
    // @@protoc_insertion_point(field:KHKLBEEGFGA.CPBFCDAMDPA)
    pub CPBFCDAMDPA: ::std::vec::Vec<super::KFLMNJFICNK::KFLMNJFICNK>,
    // @@protoc_insertion_point(field:KHKLBEEGFGA.BJGNEPEEDLN)
    pub BJGNEPEEDLN: ::protobuf::EnumOrUnknown<super::RogueCommonBuffSelectSourceType::RogueCommonBuffSelectSourceType>,
    // @@protoc_insertion_point(field:KHKLBEEGFGA.LGOAHPCLIFI)
    pub LGOAHPCLIFI: u32,
    // @@protoc_insertion_point(field:KHKLBEEGFGA.LIBHJLCEPPP)
    pub LIBHJLCEPPP: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:KHKLBEEGFGA.DLGEFPHNGEJ)
    pub DLGEFPHNGEJ: u32,
    // @@protoc_insertion_point(field:KHKLBEEGFGA.IKOJACBMGBP)
    pub IKOJACBMGBP: ::protobuf::MessageField<super::GCPAMIEDFIP::GCPAMIEDFIP>,
    // @@protoc_insertion_point(field:KHKLBEEGFGA.KELFINIJCHP)
    pub KELFINIJCHP: bool,
    // @@protoc_insertion_point(field:KHKLBEEGFGA.LLBICIFCPPM)
    pub LLBICIFCPPM: u32,
    // @@protoc_insertion_point(field:KHKLBEEGFGA.DOBPBBDGDEB)
    pub DOBPBBDGDEB: u32,
    // @@protoc_insertion_point(field:KHKLBEEGFGA.FMIIGIKMFIC)
    pub FMIIGIKMFIC: u32,
    // @@protoc_insertion_point(field:KHKLBEEGFGA.BOIEEFIPNEG)
    pub BOIEEFIPNEG: u32,
    // @@protoc_insertion_point(field:KHKLBEEGFGA.NDNOIECIINJ)
    pub NDNOIECIINJ: u32,
    // special fields
    // @@protoc_insertion_point(special_field:KHKLBEEGFGA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KHKLBEEGFGA {
    fn default() -> &'a KHKLBEEGFGA {
        <KHKLBEEGFGA as ::protobuf::Message>::default_instance()
    }
}

impl KHKLBEEGFGA {
    pub fn new() -> KHKLBEEGFGA {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BNKPPBPIGAO",
            |m: &KHKLBEEGFGA| { &m.BNKPPBPIGAO },
            |m: &mut KHKLBEEGFGA| { &mut m.BNKPPBPIGAO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CPBFCDAMDPA",
            |m: &KHKLBEEGFGA| { &m.CPBFCDAMDPA },
            |m: &mut KHKLBEEGFGA| { &mut m.CPBFCDAMDPA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BJGNEPEEDLN",
            |m: &KHKLBEEGFGA| { &m.BJGNEPEEDLN },
            |m: &mut KHKLBEEGFGA| { &mut m.BJGNEPEEDLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LGOAHPCLIFI",
            |m: &KHKLBEEGFGA| { &m.LGOAHPCLIFI },
            |m: &mut KHKLBEEGFGA| { &mut m.LGOAHPCLIFI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LIBHJLCEPPP",
            |m: &KHKLBEEGFGA| { &m.LIBHJLCEPPP },
            |m: &mut KHKLBEEGFGA| { &mut m.LIBHJLCEPPP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DLGEFPHNGEJ",
            |m: &KHKLBEEGFGA| { &m.DLGEFPHNGEJ },
            |m: &mut KHKLBEEGFGA| { &mut m.DLGEFPHNGEJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GCPAMIEDFIP::GCPAMIEDFIP>(
            "IKOJACBMGBP",
            |m: &KHKLBEEGFGA| { &m.IKOJACBMGBP },
            |m: &mut KHKLBEEGFGA| { &mut m.IKOJACBMGBP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KELFINIJCHP",
            |m: &KHKLBEEGFGA| { &m.KELFINIJCHP },
            |m: &mut KHKLBEEGFGA| { &mut m.KELFINIJCHP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LLBICIFCPPM",
            |m: &KHKLBEEGFGA| { &m.LLBICIFCPPM },
            |m: &mut KHKLBEEGFGA| { &mut m.LLBICIFCPPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DOBPBBDGDEB",
            |m: &KHKLBEEGFGA| { &m.DOBPBBDGDEB },
            |m: &mut KHKLBEEGFGA| { &mut m.DOBPBBDGDEB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FMIIGIKMFIC",
            |m: &KHKLBEEGFGA| { &m.FMIIGIKMFIC },
            |m: &mut KHKLBEEGFGA| { &mut m.FMIIGIKMFIC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BOIEEFIPNEG",
            |m: &KHKLBEEGFGA| { &m.BOIEEFIPNEG },
            |m: &mut KHKLBEEGFGA| { &mut m.BOIEEFIPNEG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NDNOIECIINJ",
            |m: &KHKLBEEGFGA| { &m.NDNOIECIINJ },
            |m: &mut KHKLBEEGFGA| { &mut m.NDNOIECIINJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KHKLBEEGFGA>(
            "KHKLBEEGFGA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KHKLBEEGFGA {
    const NAME: &'static str = "KHKLBEEGFGA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.BNKPPBPIGAO = is.read_uint32()?;
                },
                74 => {
                    self.CPBFCDAMDPA.push(is.read_message()?);
                },
                16 => {
                    self.BJGNEPEEDLN = is.read_enum_or_unknown()?;
                },
                120 => {
                    self.LGOAHPCLIFI = is.read_uint32()?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.LIBHJLCEPPP)?;
                },
                104 => {
                    self.LIBHJLCEPPP.push(is.read_uint32()?);
                },
                80 => {
                    self.DLGEFPHNGEJ = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IKOJACBMGBP)?;
                },
                88 => {
                    self.KELFINIJCHP = is.read_bool()?;
                },
                40 => {
                    self.LLBICIFCPPM = is.read_uint32()?;
                },
                48 => {
                    self.DOBPBBDGDEB = is.read_uint32()?;
                },
                32 => {
                    self.FMIIGIKMFIC = is.read_uint32()?;
                },
                8 => {
                    self.BOIEEFIPNEG = is.read_uint32()?;
                },
                24 => {
                    self.NDNOIECIINJ = is.read_uint32()?;
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
        if self.BNKPPBPIGAO != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.BNKPPBPIGAO);
        }
        for value in &self.CPBFCDAMDPA {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.BJGNEPEEDLN != ::protobuf::EnumOrUnknown::new(super::RogueCommonBuffSelectSourceType::RogueCommonBuffSelectSourceType::ROGUE_COMMON_BUFF_SELECT_SOURCE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(2, self.BJGNEPEEDLN.value());
        }
        if self.LGOAHPCLIFI != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.LGOAHPCLIFI);
        }
        for value in &self.LIBHJLCEPPP {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        if self.DLGEFPHNGEJ != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.DLGEFPHNGEJ);
        }
        if let Some(v) = self.IKOJACBMGBP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.KELFINIJCHP != false {
            my_size += 1 + 1;
        }
        if self.LLBICIFCPPM != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.LLBICIFCPPM);
        }
        if self.DOBPBBDGDEB != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.DOBPBBDGDEB);
        }
        if self.FMIIGIKMFIC != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.FMIIGIKMFIC);
        }
        if self.BOIEEFIPNEG != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.BOIEEFIPNEG);
        }
        if self.NDNOIECIINJ != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.NDNOIECIINJ);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BNKPPBPIGAO != 0 {
            os.write_uint32(12, self.BNKPPBPIGAO)?;
        }
        for v in &self.CPBFCDAMDPA {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.BJGNEPEEDLN != ::protobuf::EnumOrUnknown::new(super::RogueCommonBuffSelectSourceType::RogueCommonBuffSelectSourceType::ROGUE_COMMON_BUFF_SELECT_SOURCE_TYPE_NONE) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.BJGNEPEEDLN))?;
        }
        if self.LGOAHPCLIFI != 0 {
            os.write_uint32(15, self.LGOAHPCLIFI)?;
        }
        for v in &self.LIBHJLCEPPP {
            os.write_uint32(13, *v)?;
        };
        if self.DLGEFPHNGEJ != 0 {
            os.write_uint32(10, self.DLGEFPHNGEJ)?;
        }
        if let Some(v) = self.IKOJACBMGBP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.KELFINIJCHP != false {
            os.write_bool(11, self.KELFINIJCHP)?;
        }
        if self.LLBICIFCPPM != 0 {
            os.write_uint32(5, self.LLBICIFCPPM)?;
        }
        if self.DOBPBBDGDEB != 0 {
            os.write_uint32(6, self.DOBPBBDGDEB)?;
        }
        if self.FMIIGIKMFIC != 0 {
            os.write_uint32(4, self.FMIIGIKMFIC)?;
        }
        if self.BOIEEFIPNEG != 0 {
            os.write_uint32(1, self.BOIEEFIPNEG)?;
        }
        if self.NDNOIECIINJ != 0 {
            os.write_uint32(3, self.NDNOIECIINJ)?;
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

    fn new() -> KHKLBEEGFGA {
        KHKLBEEGFGA::new()
    }

    fn clear(&mut self) {
        self.BNKPPBPIGAO = 0;
        self.CPBFCDAMDPA.clear();
        self.BJGNEPEEDLN = ::protobuf::EnumOrUnknown::new(super::RogueCommonBuffSelectSourceType::RogueCommonBuffSelectSourceType::ROGUE_COMMON_BUFF_SELECT_SOURCE_TYPE_NONE);
        self.LGOAHPCLIFI = 0;
        self.LIBHJLCEPPP.clear();
        self.DLGEFPHNGEJ = 0;
        self.IKOJACBMGBP.clear();
        self.KELFINIJCHP = false;
        self.LLBICIFCPPM = 0;
        self.DOBPBBDGDEB = 0;
        self.FMIIGIKMFIC = 0;
        self.BOIEEFIPNEG = 0;
        self.NDNOIECIINJ = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KHKLBEEGFGA {
        static instance: KHKLBEEGFGA = KHKLBEEGFGA {
            BNKPPBPIGAO: 0,
            CPBFCDAMDPA: ::std::vec::Vec::new(),
            BJGNEPEEDLN: ::protobuf::EnumOrUnknown::from_i32(0),
            LGOAHPCLIFI: 0,
            LIBHJLCEPPP: ::std::vec::Vec::new(),
            DLGEFPHNGEJ: 0,
            IKOJACBMGBP: ::protobuf::MessageField::none(),
            KELFINIJCHP: false,
            LLBICIFCPPM: 0,
            DOBPBBDGDEB: 0,
            FMIIGIKMFIC: 0,
            BOIEEFIPNEG: 0,
            NDNOIECIINJ: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KHKLBEEGFGA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KHKLBEEGFGA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KHKLBEEGFGA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KHKLBEEGFGA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KHKLBEEGFGA.proto\x1a\x11GCPAMIEDFIP.proto\x1a\x11KFLMNJFICNK.prot\
    o\x1a%RogueCommonBuffSelectSourceType.proto\"\x85\x04\n\x0bKHKLBEEGFGA\
    \x12\x20\n\x0bBNKPPBPIGAO\x18\x0c\x20\x01(\rR\x0bBNKPPBPIGAO\x12.\n\x0bC\
    PBFCDAMDPA\x18\t\x20\x03(\x0b2\x0c.KFLMNJFICNKR\x0bCPBFCDAMDPA\x12B\n\
    \x0bBJGNEPEEDLN\x18\x02\x20\x01(\x0e2\x20.RogueCommonBuffSelectSourceTyp\
    eR\x0bBJGNEPEEDLN\x12\x20\n\x0bLGOAHPCLIFI\x18\x0f\x20\x01(\rR\x0bLGOAHP\
    CLIFI\x12\x20\n\x0bLIBHJLCEPPP\x18\r\x20\x03(\rR\x0bLIBHJLCEPPP\x12\x20\
    \n\x0bDLGEFPHNGEJ\x18\n\x20\x01(\rR\x0bDLGEFPHNGEJ\x12.\n\x0bIKOJACBMGBP\
    \x18\x07\x20\x01(\x0b2\x0c.GCPAMIEDFIPR\x0bIKOJACBMGBP\x12\x20\n\x0bKELF\
    INIJCHP\x18\x0b\x20\x01(\x08R\x0bKELFINIJCHP\x12\x20\n\x0bLLBICIFCPPM\
    \x18\x05\x20\x01(\rR\x0bLLBICIFCPPM\x12\x20\n\x0bDOBPBBDGDEB\x18\x06\x20\
    \x01(\rR\x0bDOBPBBDGDEB\x12\x20\n\x0bFMIIGIKMFIC\x18\x04\x20\x01(\rR\x0b\
    FMIIGIKMFIC\x12\x20\n\x0bBOIEEFIPNEG\x18\x01\x20\x01(\rR\x0bBOIEEFIPNEG\
    \x12\x20\n\x0bNDNOIECIINJ\x18\x03\x20\x01(\rR\x0bNDNOIECIINJb\x06proto3\
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
            deps.push(super::GCPAMIEDFIP::file_descriptor().clone());
            deps.push(super::KFLMNJFICNK::file_descriptor().clone());
            deps.push(super::RogueCommonBuffSelectSourceType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(KHKLBEEGFGA::generated_message_descriptor_data());
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
