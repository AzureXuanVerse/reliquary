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

//! Generated file from `DPNDLHGEMEI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:DPNDLHGEMEI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DPNDLHGEMEI {
    // message fields
    // @@protoc_insertion_point(field:DPNDLHGEMEI.IMCPKLDFDOG)
    pub IMCPKLDFDOG: u32,
    // @@protoc_insertion_point(field:DPNDLHGEMEI.AJGEOFIIDDH)
    pub AJGEOFIIDDH: u32,
    // @@protoc_insertion_point(field:DPNDLHGEMEI.OMOENBAKMHJ)
    pub OMOENBAKMHJ: f64,
    // @@protoc_insertion_point(field:DPNDLHGEMEI.wave)
    pub wave: u32,
    // @@protoc_insertion_point(field:DPNDLHGEMEI.GMLFMPJPEGG)
    pub GMLFMPJPEGG: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DPNDLHGEMEI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DPNDLHGEMEI {
    fn default() -> &'a DPNDLHGEMEI {
        <DPNDLHGEMEI as ::protobuf::Message>::default_instance()
    }
}

impl DPNDLHGEMEI {
    pub fn new() -> DPNDLHGEMEI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IMCPKLDFDOG",
            |m: &DPNDLHGEMEI| { &m.IMCPKLDFDOG },
            |m: &mut DPNDLHGEMEI| { &mut m.IMCPKLDFDOG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AJGEOFIIDDH",
            |m: &DPNDLHGEMEI| { &m.AJGEOFIIDDH },
            |m: &mut DPNDLHGEMEI| { &mut m.AJGEOFIIDDH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OMOENBAKMHJ",
            |m: &DPNDLHGEMEI| { &m.OMOENBAKMHJ },
            |m: &mut DPNDLHGEMEI| { &mut m.OMOENBAKMHJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "wave",
            |m: &DPNDLHGEMEI| { &m.wave },
            |m: &mut DPNDLHGEMEI| { &mut m.wave },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GMLFMPJPEGG",
            |m: &DPNDLHGEMEI| { &m.GMLFMPJPEGG },
            |m: &mut DPNDLHGEMEI| { &mut m.GMLFMPJPEGG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DPNDLHGEMEI>(
            "DPNDLHGEMEI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DPNDLHGEMEI {
    const NAME: &'static str = "DPNDLHGEMEI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.IMCPKLDFDOG = is.read_uint32()?;
                },
                16 => {
                    self.AJGEOFIIDDH = is.read_uint32()?;
                },
                25 => {
                    self.OMOENBAKMHJ = is.read_double()?;
                },
                32 => {
                    self.wave = is.read_uint32()?;
                },
                40 => {
                    self.GMLFMPJPEGG = is.read_uint32()?;
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
        if self.IMCPKLDFDOG != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.IMCPKLDFDOG);
        }
        if self.AJGEOFIIDDH != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.AJGEOFIIDDH);
        }
        if self.OMOENBAKMHJ != 0. {
            my_size += 1 + 8;
        }
        if self.wave != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.wave);
        }
        if self.GMLFMPJPEGG != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.GMLFMPJPEGG);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IMCPKLDFDOG != 0 {
            os.write_uint32(1, self.IMCPKLDFDOG)?;
        }
        if self.AJGEOFIIDDH != 0 {
            os.write_uint32(2, self.AJGEOFIIDDH)?;
        }
        if self.OMOENBAKMHJ != 0. {
            os.write_double(3, self.OMOENBAKMHJ)?;
        }
        if self.wave != 0 {
            os.write_uint32(4, self.wave)?;
        }
        if self.GMLFMPJPEGG != 0 {
            os.write_uint32(5, self.GMLFMPJPEGG)?;
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

    fn new() -> DPNDLHGEMEI {
        DPNDLHGEMEI::new()
    }

    fn clear(&mut self) {
        self.IMCPKLDFDOG = 0;
        self.AJGEOFIIDDH = 0;
        self.OMOENBAKMHJ = 0.;
        self.wave = 0;
        self.GMLFMPJPEGG = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DPNDLHGEMEI {
        static instance: DPNDLHGEMEI = DPNDLHGEMEI {
            IMCPKLDFDOG: 0,
            AJGEOFIIDDH: 0,
            OMOENBAKMHJ: 0.,
            wave: 0,
            GMLFMPJPEGG: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DPNDLHGEMEI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DPNDLHGEMEI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DPNDLHGEMEI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DPNDLHGEMEI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DPNDLHGEMEI.proto\"\xa9\x01\n\x0bDPNDLHGEMEI\x12\x20\n\x0bIMCPKLDF\
    DOG\x18\x01\x20\x01(\rR\x0bIMCPKLDFDOG\x12\x20\n\x0bAJGEOFIIDDH\x18\x02\
    \x20\x01(\rR\x0bAJGEOFIIDDH\x12\x20\n\x0bOMOENBAKMHJ\x18\x03\x20\x01(\
    \x01R\x0bOMOENBAKMHJ\x12\x12\n\x04wave\x18\x04\x20\x01(\rR\x04wave\x12\
    \x20\n\x0bGMLFMPJPEGG\x18\x05\x20\x01(\rR\x0bGMLFMPJPEGGb\x06proto3\
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
            messages.push(DPNDLHGEMEI::generated_message_descriptor_data());
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
