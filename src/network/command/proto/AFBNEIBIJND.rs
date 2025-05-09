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

//! Generated file from `AFBNEIBIJND.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:AFBNEIBIJND)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AFBNEIBIJND {
    // message fields
    // @@protoc_insertion_point(field:AFBNEIBIJND.ILCFOJCDNHI)
    pub ILCFOJCDNHI: u32,
    // @@protoc_insertion_point(field:AFBNEIBIJND.MAMHOJMFJOF)
    pub MAMHOJMFJOF: u32,
    // @@protoc_insertion_point(field:AFBNEIBIJND.COIFHFPEGPH)
    pub COIFHFPEGPH: i64,
    // @@protoc_insertion_point(field:AFBNEIBIJND.NMKLEGOMEPJ)
    pub NMKLEGOMEPJ: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:AFBNEIBIJND.world_level)
    pub world_level: u32,
    // @@protoc_insertion_point(field:AFBNEIBIJND.CAIMHCAACFG)
    pub CAIMHCAACFG: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:AFBNEIBIJND.BGAFGHIPOJE)
    pub BGAFGHIPOJE: u32,
    // @@protoc_insertion_point(field:AFBNEIBIJND.status)
    pub status: ::protobuf::EnumOrUnknown<super::NOBPMMNFENJ::NOBPMMNFENJ>,
    // @@protoc_insertion_point(field:AFBNEIBIJND.KGJHJMDCAOC)
    pub KGJHJMDCAOC: bool,
    // @@protoc_insertion_point(field:AFBNEIBIJND.AHNFMDNEJNL)
    pub AHNFMDNEJNL: u32,
    // @@protoc_insertion_point(field:AFBNEIBIJND.FJNDPCFNFLO)
    pub FJNDPCFNFLO: bool,
    // @@protoc_insertion_point(field:AFBNEIBIJND.FHFOECINOAA)
    pub FHFOECINOAA: i64,
    // special fields
    // @@protoc_insertion_point(special_field:AFBNEIBIJND.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AFBNEIBIJND {
    fn default() -> &'a AFBNEIBIJND {
        <AFBNEIBIJND as ::protobuf::Message>::default_instance()
    }
}

impl AFBNEIBIJND {
    pub fn new() -> AFBNEIBIJND {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(12);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ILCFOJCDNHI",
            |m: &AFBNEIBIJND| { &m.ILCFOJCDNHI },
            |m: &mut AFBNEIBIJND| { &mut m.ILCFOJCDNHI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MAMHOJMFJOF",
            |m: &AFBNEIBIJND| { &m.MAMHOJMFJOF },
            |m: &mut AFBNEIBIJND| { &mut m.MAMHOJMFJOF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "COIFHFPEGPH",
            |m: &AFBNEIBIJND| { &m.COIFHFPEGPH },
            |m: &mut AFBNEIBIJND| { &mut m.COIFHFPEGPH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NMKLEGOMEPJ",
            |m: &AFBNEIBIJND| { &m.NMKLEGOMEPJ },
            |m: &mut AFBNEIBIJND| { &mut m.NMKLEGOMEPJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "world_level",
            |m: &AFBNEIBIJND| { &m.world_level },
            |m: &mut AFBNEIBIJND| { &mut m.world_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CAIMHCAACFG",
            |m: &AFBNEIBIJND| { &m.CAIMHCAACFG },
            |m: &mut AFBNEIBIJND| { &mut m.CAIMHCAACFG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BGAFGHIPOJE",
            |m: &AFBNEIBIJND| { &m.BGAFGHIPOJE },
            |m: &mut AFBNEIBIJND| { &mut m.BGAFGHIPOJE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &AFBNEIBIJND| { &m.status },
            |m: &mut AFBNEIBIJND| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGJHJMDCAOC",
            |m: &AFBNEIBIJND| { &m.KGJHJMDCAOC },
            |m: &mut AFBNEIBIJND| { &mut m.KGJHJMDCAOC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AHNFMDNEJNL",
            |m: &AFBNEIBIJND| { &m.AHNFMDNEJNL },
            |m: &mut AFBNEIBIJND| { &mut m.AHNFMDNEJNL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJNDPCFNFLO",
            |m: &AFBNEIBIJND| { &m.FJNDPCFNFLO },
            |m: &mut AFBNEIBIJND| { &mut m.FJNDPCFNFLO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FHFOECINOAA",
            |m: &AFBNEIBIJND| { &m.FHFOECINOAA },
            |m: &mut AFBNEIBIJND| { &mut m.FHFOECINOAA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AFBNEIBIJND>(
            "AFBNEIBIJND",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AFBNEIBIJND {
    const NAME: &'static str = "AFBNEIBIJND";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.ILCFOJCDNHI = is.read_uint32()?;
                },
                8 => {
                    self.MAMHOJMFJOF = is.read_uint32()?;
                },
                120 => {
                    self.COIFHFPEGPH = is.read_int64()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.NMKLEGOMEPJ)?;
                },
                16 => {
                    self.NMKLEGOMEPJ.push(is.read_uint32()?);
                },
                96 => {
                    self.world_level = is.read_uint32()?;
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.CAIMHCAACFG)?;
                },
                32 => {
                    self.CAIMHCAACFG.push(is.read_uint32()?);
                },
                64 => {
                    self.BGAFGHIPOJE = is.read_uint32()?;
                },
                24 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                112 => {
                    self.KGJHJMDCAOC = is.read_bool()?;
                },
                88 => {
                    self.AHNFMDNEJNL = is.read_uint32()?;
                },
                80 => {
                    self.FJNDPCFNFLO = is.read_bool()?;
                },
                40 => {
                    self.FHFOECINOAA = is.read_int64()?;
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
        if self.ILCFOJCDNHI != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.ILCFOJCDNHI);
        }
        if self.MAMHOJMFJOF != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.MAMHOJMFJOF);
        }
        if self.COIFHFPEGPH != 0 {
            my_size += ::protobuf::rt::int64_size(15, self.COIFHFPEGPH);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(2, &self.NMKLEGOMEPJ);
        if self.world_level != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.world_level);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(4, &self.CAIMHCAACFG);
        if self.BGAFGHIPOJE != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.BGAFGHIPOJE);
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::NOBPMMNFENJ::NOBPMMNFENJ::PLAYER_RETURN_NONE) {
            my_size += ::protobuf::rt::int32_size(3, self.status.value());
        }
        if self.KGJHJMDCAOC != false {
            my_size += 1 + 1;
        }
        if self.AHNFMDNEJNL != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.AHNFMDNEJNL);
        }
        if self.FJNDPCFNFLO != false {
            my_size += 1 + 1;
        }
        if self.FHFOECINOAA != 0 {
            my_size += ::protobuf::rt::int64_size(5, self.FHFOECINOAA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ILCFOJCDNHI != 0 {
            os.write_uint32(9, self.ILCFOJCDNHI)?;
        }
        if self.MAMHOJMFJOF != 0 {
            os.write_uint32(1, self.MAMHOJMFJOF)?;
        }
        if self.COIFHFPEGPH != 0 {
            os.write_int64(15, self.COIFHFPEGPH)?;
        }
        os.write_repeated_packed_uint32(2, &self.NMKLEGOMEPJ)?;
        if self.world_level != 0 {
            os.write_uint32(12, self.world_level)?;
        }
        os.write_repeated_packed_uint32(4, &self.CAIMHCAACFG)?;
        if self.BGAFGHIPOJE != 0 {
            os.write_uint32(8, self.BGAFGHIPOJE)?;
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::NOBPMMNFENJ::NOBPMMNFENJ::PLAYER_RETURN_NONE) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        if self.KGJHJMDCAOC != false {
            os.write_bool(14, self.KGJHJMDCAOC)?;
        }
        if self.AHNFMDNEJNL != 0 {
            os.write_uint32(11, self.AHNFMDNEJNL)?;
        }
        if self.FJNDPCFNFLO != false {
            os.write_bool(10, self.FJNDPCFNFLO)?;
        }
        if self.FHFOECINOAA != 0 {
            os.write_int64(5, self.FHFOECINOAA)?;
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

    fn new() -> AFBNEIBIJND {
        AFBNEIBIJND::new()
    }

    fn clear(&mut self) {
        self.ILCFOJCDNHI = 0;
        self.MAMHOJMFJOF = 0;
        self.COIFHFPEGPH = 0;
        self.NMKLEGOMEPJ.clear();
        self.world_level = 0;
        self.CAIMHCAACFG.clear();
        self.BGAFGHIPOJE = 0;
        self.status = ::protobuf::EnumOrUnknown::new(super::NOBPMMNFENJ::NOBPMMNFENJ::PLAYER_RETURN_NONE);
        self.KGJHJMDCAOC = false;
        self.AHNFMDNEJNL = 0;
        self.FJNDPCFNFLO = false;
        self.FHFOECINOAA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AFBNEIBIJND {
        static instance: AFBNEIBIJND = AFBNEIBIJND {
            ILCFOJCDNHI: 0,
            MAMHOJMFJOF: 0,
            COIFHFPEGPH: 0,
            NMKLEGOMEPJ: ::std::vec::Vec::new(),
            world_level: 0,
            CAIMHCAACFG: ::std::vec::Vec::new(),
            BGAFGHIPOJE: 0,
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            KGJHJMDCAOC: false,
            AHNFMDNEJNL: 0,
            FJNDPCFNFLO: false,
            FHFOECINOAA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AFBNEIBIJND {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AFBNEIBIJND").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AFBNEIBIJND {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AFBNEIBIJND {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AFBNEIBIJND.proto\x1a\x11NOBPMMNFENJ.proto\"\xa8\x03\n\x0bAFBNEIBI\
    JND\x12\x20\n\x0bILCFOJCDNHI\x18\t\x20\x01(\rR\x0bILCFOJCDNHI\x12\x20\n\
    \x0bMAMHOJMFJOF\x18\x01\x20\x01(\rR\x0bMAMHOJMFJOF\x12\x20\n\x0bCOIFHFPE\
    GPH\x18\x0f\x20\x01(\x03R\x0bCOIFHFPEGPH\x12\x20\n\x0bNMKLEGOMEPJ\x18\
    \x02\x20\x03(\rR\x0bNMKLEGOMEPJ\x12\x1f\n\x0bworld_level\x18\x0c\x20\x01\
    (\rR\nworldLevel\x12\x20\n\x0bCAIMHCAACFG\x18\x04\x20\x03(\rR\x0bCAIMHCA\
    ACFG\x12\x20\n\x0bBGAFGHIPOJE\x18\x08\x20\x01(\rR\x0bBGAFGHIPOJE\x12$\n\
    \x06status\x18\x03\x20\x01(\x0e2\x0c.NOBPMMNFENJR\x06status\x12\x20\n\
    \x0bKGJHJMDCAOC\x18\x0e\x20\x01(\x08R\x0bKGJHJMDCAOC\x12\x20\n\x0bAHNFMD\
    NEJNL\x18\x0b\x20\x01(\rR\x0bAHNFMDNEJNL\x12\x20\n\x0bFJNDPCFNFLO\x18\n\
    \x20\x01(\x08R\x0bFJNDPCFNFLO\x12\x20\n\x0bFHFOECINOAA\x18\x05\x20\x01(\
    \x03R\x0bFHFOECINOAAb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::NOBPMMNFENJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AFBNEIBIJND::generated_message_descriptor_data());
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
