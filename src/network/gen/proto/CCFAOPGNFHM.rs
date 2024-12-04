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

//! Generated file from `CCFAOPGNFHM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CCFAOPGNFHM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CCFAOPGNFHM {
    // message fields
    // @@protoc_insertion_point(field:CCFAOPGNFHM.MAMKEEKMJHA)
    pub MAMKEEKMJHA: u32,
    // @@protoc_insertion_point(field:CCFAOPGNFHM.EBGJHJLICDI)
    pub EBGJHJLICDI: u32,
    // @@protoc_insertion_point(field:CCFAOPGNFHM.POPPKLNFPPI)
    pub POPPKLNFPPI: u32,
    // @@protoc_insertion_point(field:CCFAOPGNFHM.MMFAALHNABF)
    pub MMFAALHNABF: u32,
    // @@protoc_insertion_point(field:CCFAOPGNFHM.PPJHIOODPHD)
    pub PPJHIOODPHD: u32,
    // @@protoc_insertion_point(field:CCFAOPGNFHM.JKOCJIMAGBN)
    pub JKOCJIMAGBN: u32,
    // @@protoc_insertion_point(field:CCFAOPGNFHM.JBPHGNACHJP)
    pub JBPHGNACHJP: ::std::vec::Vec<super::CIJIDBKMNGG::CIJIDBKMNGG>,
    // @@protoc_insertion_point(field:CCFAOPGNFHM.HJHHOIDBECJ)
    pub HJHHOIDBECJ: bool,
    // @@protoc_insertion_point(field:CCFAOPGNFHM.BNPLKKFIKME)
    pub BNPLKKFIKME: bool,
    // special fields
    // @@protoc_insertion_point(special_field:CCFAOPGNFHM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CCFAOPGNFHM {
    fn default() -> &'a CCFAOPGNFHM {
        <CCFAOPGNFHM as ::protobuf::Message>::default_instance()
    }
}

impl CCFAOPGNFHM {
    pub fn new() -> CCFAOPGNFHM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MAMKEEKMJHA",
            |m: &CCFAOPGNFHM| { &m.MAMKEEKMJHA },
            |m: &mut CCFAOPGNFHM| { &mut m.MAMKEEKMJHA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EBGJHJLICDI",
            |m: &CCFAOPGNFHM| { &m.EBGJHJLICDI },
            |m: &mut CCFAOPGNFHM| { &mut m.EBGJHJLICDI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "POPPKLNFPPI",
            |m: &CCFAOPGNFHM| { &m.POPPKLNFPPI },
            |m: &mut CCFAOPGNFHM| { &mut m.POPPKLNFPPI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MMFAALHNABF",
            |m: &CCFAOPGNFHM| { &m.MMFAALHNABF },
            |m: &mut CCFAOPGNFHM| { &mut m.MMFAALHNABF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PPJHIOODPHD",
            |m: &CCFAOPGNFHM| { &m.PPJHIOODPHD },
            |m: &mut CCFAOPGNFHM| { &mut m.PPJHIOODPHD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JKOCJIMAGBN",
            |m: &CCFAOPGNFHM| { &m.JKOCJIMAGBN },
            |m: &mut CCFAOPGNFHM| { &mut m.JKOCJIMAGBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JBPHGNACHJP",
            |m: &CCFAOPGNFHM| { &m.JBPHGNACHJP },
            |m: &mut CCFAOPGNFHM| { &mut m.JBPHGNACHJP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HJHHOIDBECJ",
            |m: &CCFAOPGNFHM| { &m.HJHHOIDBECJ },
            |m: &mut CCFAOPGNFHM| { &mut m.HJHHOIDBECJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BNPLKKFIKME",
            |m: &CCFAOPGNFHM| { &m.BNPLKKFIKME },
            |m: &mut CCFAOPGNFHM| { &mut m.BNPLKKFIKME },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CCFAOPGNFHM>(
            "CCFAOPGNFHM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CCFAOPGNFHM {
    const NAME: &'static str = "CCFAOPGNFHM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.MAMKEEKMJHA = is.read_uint32()?;
                },
                16 => {
                    self.EBGJHJLICDI = is.read_uint32()?;
                },
                56 => {
                    self.POPPKLNFPPI = is.read_uint32()?;
                },
                88 => {
                    self.MMFAALHNABF = is.read_uint32()?;
                },
                112 => {
                    self.PPJHIOODPHD = is.read_uint32()?;
                },
                96 => {
                    self.JKOCJIMAGBN = is.read_uint32()?;
                },
                106 => {
                    self.JBPHGNACHJP.push(is.read_message()?);
                },
                72 => {
                    self.HJHHOIDBECJ = is.read_bool()?;
                },
                64 => {
                    self.BNPLKKFIKME = is.read_bool()?;
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
        if self.MAMKEEKMJHA != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.MAMKEEKMJHA);
        }
        if self.EBGJHJLICDI != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.EBGJHJLICDI);
        }
        if self.POPPKLNFPPI != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.POPPKLNFPPI);
        }
        if self.MMFAALHNABF != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.MMFAALHNABF);
        }
        if self.PPJHIOODPHD != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.PPJHIOODPHD);
        }
        if self.JKOCJIMAGBN != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.JKOCJIMAGBN);
        }
        for value in &self.JBPHGNACHJP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.HJHHOIDBECJ != false {
            my_size += 1 + 1;
        }
        if self.BNPLKKFIKME != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.MAMKEEKMJHA != 0 {
            os.write_uint32(4, self.MAMKEEKMJHA)?;
        }
        if self.EBGJHJLICDI != 0 {
            os.write_uint32(2, self.EBGJHJLICDI)?;
        }
        if self.POPPKLNFPPI != 0 {
            os.write_uint32(7, self.POPPKLNFPPI)?;
        }
        if self.MMFAALHNABF != 0 {
            os.write_uint32(11, self.MMFAALHNABF)?;
        }
        if self.PPJHIOODPHD != 0 {
            os.write_uint32(14, self.PPJHIOODPHD)?;
        }
        if self.JKOCJIMAGBN != 0 {
            os.write_uint32(12, self.JKOCJIMAGBN)?;
        }
        for v in &self.JBPHGNACHJP {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.HJHHOIDBECJ != false {
            os.write_bool(9, self.HJHHOIDBECJ)?;
        }
        if self.BNPLKKFIKME != false {
            os.write_bool(8, self.BNPLKKFIKME)?;
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

    fn new() -> CCFAOPGNFHM {
        CCFAOPGNFHM::new()
    }

    fn clear(&mut self) {
        self.MAMKEEKMJHA = 0;
        self.EBGJHJLICDI = 0;
        self.POPPKLNFPPI = 0;
        self.MMFAALHNABF = 0;
        self.PPJHIOODPHD = 0;
        self.JKOCJIMAGBN = 0;
        self.JBPHGNACHJP.clear();
        self.HJHHOIDBECJ = false;
        self.BNPLKKFIKME = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CCFAOPGNFHM {
        static instance: CCFAOPGNFHM = CCFAOPGNFHM {
            MAMKEEKMJHA: 0,
            EBGJHJLICDI: 0,
            POPPKLNFPPI: 0,
            MMFAALHNABF: 0,
            PPJHIOODPHD: 0,
            JKOCJIMAGBN: 0,
            JBPHGNACHJP: ::std::vec::Vec::new(),
            HJHHOIDBECJ: false,
            BNPLKKFIKME: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CCFAOPGNFHM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CCFAOPGNFHM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CCFAOPGNFHM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCFAOPGNFHM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CCFAOPGNFHM.proto\x1a\x11CIJIDBKMNGG.proto\"\xcd\x02\n\x0bCCFAOPGN\
    FHM\x12\x20\n\x0bMAMKEEKMJHA\x18\x04\x20\x01(\rR\x0bMAMKEEKMJHA\x12\x20\
    \n\x0bEBGJHJLICDI\x18\x02\x20\x01(\rR\x0bEBGJHJLICDI\x12\x20\n\x0bPOPPKL\
    NFPPI\x18\x07\x20\x01(\rR\x0bPOPPKLNFPPI\x12\x20\n\x0bMMFAALHNABF\x18\
    \x0b\x20\x01(\rR\x0bMMFAALHNABF\x12\x20\n\x0bPPJHIOODPHD\x18\x0e\x20\x01\
    (\rR\x0bPPJHIOODPHD\x12\x20\n\x0bJKOCJIMAGBN\x18\x0c\x20\x01(\rR\x0bJKOC\
    JIMAGBN\x12.\n\x0bJBPHGNACHJP\x18\r\x20\x03(\x0b2\x0c.CIJIDBKMNGGR\x0bJB\
    PHGNACHJP\x12\x20\n\x0bHJHHOIDBECJ\x18\t\x20\x01(\x08R\x0bHJHHOIDBECJ\
    \x12\x20\n\x0bBNPLKKFIKME\x18\x08\x20\x01(\x08R\x0bBNPLKKFIKMEb\x06proto\
    3\
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
            deps.push(super::CIJIDBKMNGG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CCFAOPGNFHM::generated_message_descriptor_data());
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
