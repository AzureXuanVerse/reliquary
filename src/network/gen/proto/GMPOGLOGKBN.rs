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

//! Generated file from `GMPOGLOGKBN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GMPOGLOGKBN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GMPOGLOGKBN {
    // message fields
    // @@protoc_insertion_point(field:GMPOGLOGKBN.POPPKLNFPPI)
    pub POPPKLNFPPI: u32,
    // @@protoc_insertion_point(field:GMPOGLOGKBN.JKOCJIMAGBN)
    pub JKOCJIMAGBN: u32,
    // @@protoc_insertion_point(field:GMPOGLOGKBN.BIODAPMJNGM)
    pub BIODAPMJNGM: u32,
    // @@protoc_insertion_point(field:GMPOGLOGKBN.EBGJHJLICDI)
    pub EBGJHJLICDI: u32,
    // @@protoc_insertion_point(field:GMPOGLOGKBN.NCJHNPAAKAC)
    pub NCJHNPAAKAC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GMPOGLOGKBN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GMPOGLOGKBN {
    fn default() -> &'a GMPOGLOGKBN {
        <GMPOGLOGKBN as ::protobuf::Message>::default_instance()
    }
}

impl GMPOGLOGKBN {
    pub fn new() -> GMPOGLOGKBN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "POPPKLNFPPI",
            |m: &GMPOGLOGKBN| { &m.POPPKLNFPPI },
            |m: &mut GMPOGLOGKBN| { &mut m.POPPKLNFPPI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JKOCJIMAGBN",
            |m: &GMPOGLOGKBN| { &m.JKOCJIMAGBN },
            |m: &mut GMPOGLOGKBN| { &mut m.JKOCJIMAGBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BIODAPMJNGM",
            |m: &GMPOGLOGKBN| { &m.BIODAPMJNGM },
            |m: &mut GMPOGLOGKBN| { &mut m.BIODAPMJNGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EBGJHJLICDI",
            |m: &GMPOGLOGKBN| { &m.EBGJHJLICDI },
            |m: &mut GMPOGLOGKBN| { &mut m.EBGJHJLICDI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NCJHNPAAKAC",
            |m: &GMPOGLOGKBN| { &m.NCJHNPAAKAC },
            |m: &mut GMPOGLOGKBN| { &mut m.NCJHNPAAKAC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GMPOGLOGKBN>(
            "GMPOGLOGKBN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GMPOGLOGKBN {
    const NAME: &'static str = "GMPOGLOGKBN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.POPPKLNFPPI = is.read_uint32()?;
                },
                64 => {
                    self.JKOCJIMAGBN = is.read_uint32()?;
                },
                40 => {
                    self.BIODAPMJNGM = is.read_uint32()?;
                },
                120 => {
                    self.EBGJHJLICDI = is.read_uint32()?;
                },
                32 => {
                    self.NCJHNPAAKAC = is.read_uint32()?;
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
        if self.POPPKLNFPPI != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.POPPKLNFPPI);
        }
        if self.JKOCJIMAGBN != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.JKOCJIMAGBN);
        }
        if self.BIODAPMJNGM != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.BIODAPMJNGM);
        }
        if self.EBGJHJLICDI != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.EBGJHJLICDI);
        }
        if self.NCJHNPAAKAC != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.NCJHNPAAKAC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.POPPKLNFPPI != 0 {
            os.write_uint32(11, self.POPPKLNFPPI)?;
        }
        if self.JKOCJIMAGBN != 0 {
            os.write_uint32(8, self.JKOCJIMAGBN)?;
        }
        if self.BIODAPMJNGM != 0 {
            os.write_uint32(5, self.BIODAPMJNGM)?;
        }
        if self.EBGJHJLICDI != 0 {
            os.write_uint32(15, self.EBGJHJLICDI)?;
        }
        if self.NCJHNPAAKAC != 0 {
            os.write_uint32(4, self.NCJHNPAAKAC)?;
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

    fn new() -> GMPOGLOGKBN {
        GMPOGLOGKBN::new()
    }

    fn clear(&mut self) {
        self.POPPKLNFPPI = 0;
        self.JKOCJIMAGBN = 0;
        self.BIODAPMJNGM = 0;
        self.EBGJHJLICDI = 0;
        self.NCJHNPAAKAC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GMPOGLOGKBN {
        static instance: GMPOGLOGKBN = GMPOGLOGKBN {
            POPPKLNFPPI: 0,
            JKOCJIMAGBN: 0,
            BIODAPMJNGM: 0,
            EBGJHJLICDI: 0,
            NCJHNPAAKAC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GMPOGLOGKBN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GMPOGLOGKBN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GMPOGLOGKBN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GMPOGLOGKBN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GMPOGLOGKBN.proto\"\xb7\x01\n\x0bGMPOGLOGKBN\x12\x20\n\x0bPOPPKLNF\
    PPI\x18\x0b\x20\x01(\rR\x0bPOPPKLNFPPI\x12\x20\n\x0bJKOCJIMAGBN\x18\x08\
    \x20\x01(\rR\x0bJKOCJIMAGBN\x12\x20\n\x0bBIODAPMJNGM\x18\x05\x20\x01(\rR\
    \x0bBIODAPMJNGM\x12\x20\n\x0bEBGJHJLICDI\x18\x0f\x20\x01(\rR\x0bEBGJHJLI\
    CDI\x12\x20\n\x0bNCJHNPAAKAC\x18\x04\x20\x01(\rR\x0bNCJHNPAAKACb\x06prot\
    o3\
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
            messages.push(GMPOGLOGKBN::generated_message_descriptor_data());
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
