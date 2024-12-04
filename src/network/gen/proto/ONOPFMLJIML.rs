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

//! Generated file from `ONOPFMLJIML.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ONOPFMLJIML)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ONOPFMLJIML {
    // message fields
    // @@protoc_insertion_point(field:ONOPFMLJIML.JNKPPJHENCH)
    pub JNKPPJHENCH: bool,
    // @@protoc_insertion_point(field:ONOPFMLJIML.OEKBKACHMAB)
    pub OEKBKACHMAB: u32,
    // @@protoc_insertion_point(field:ONOPFMLJIML.FMLKGHHLNDK)
    pub FMLKGHHLNDK: u32,
    // @@protoc_insertion_point(field:ONOPFMLJIML.ILIFHHJFMIH)
    pub ILIFHHJFMIH: u32,
    // @@protoc_insertion_point(field:ONOPFMLJIML.PLCCICHPEDE)
    pub PLCCICHPEDE: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ONOPFMLJIML.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ONOPFMLJIML {
    fn default() -> &'a ONOPFMLJIML {
        <ONOPFMLJIML as ::protobuf::Message>::default_instance()
    }
}

impl ONOPFMLJIML {
    pub fn new() -> ONOPFMLJIML {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JNKPPJHENCH",
            |m: &ONOPFMLJIML| { &m.JNKPPJHENCH },
            |m: &mut ONOPFMLJIML| { &mut m.JNKPPJHENCH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OEKBKACHMAB",
            |m: &ONOPFMLJIML| { &m.OEKBKACHMAB },
            |m: &mut ONOPFMLJIML| { &mut m.OEKBKACHMAB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FMLKGHHLNDK",
            |m: &ONOPFMLJIML| { &m.FMLKGHHLNDK },
            |m: &mut ONOPFMLJIML| { &mut m.FMLKGHHLNDK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ILIFHHJFMIH",
            |m: &ONOPFMLJIML| { &m.ILIFHHJFMIH },
            |m: &mut ONOPFMLJIML| { &mut m.ILIFHHJFMIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PLCCICHPEDE",
            |m: &ONOPFMLJIML| { &m.PLCCICHPEDE },
            |m: &mut ONOPFMLJIML| { &mut m.PLCCICHPEDE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ONOPFMLJIML>(
            "ONOPFMLJIML",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ONOPFMLJIML {
    const NAME: &'static str = "ONOPFMLJIML";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.JNKPPJHENCH = is.read_bool()?;
                },
                104 => {
                    self.OEKBKACHMAB = is.read_uint32()?;
                },
                120 => {
                    self.FMLKGHHLNDK = is.read_uint32()?;
                },
                40 => {
                    self.ILIFHHJFMIH = is.read_uint32()?;
                },
                56 => {
                    self.PLCCICHPEDE = is.read_uint32()?;
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
        if self.JNKPPJHENCH != false {
            my_size += 1 + 1;
        }
        if self.OEKBKACHMAB != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.OEKBKACHMAB);
        }
        if self.FMLKGHHLNDK != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.FMLKGHHLNDK);
        }
        if self.ILIFHHJFMIH != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.ILIFHHJFMIH);
        }
        if self.PLCCICHPEDE != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.PLCCICHPEDE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JNKPPJHENCH != false {
            os.write_bool(3, self.JNKPPJHENCH)?;
        }
        if self.OEKBKACHMAB != 0 {
            os.write_uint32(13, self.OEKBKACHMAB)?;
        }
        if self.FMLKGHHLNDK != 0 {
            os.write_uint32(15, self.FMLKGHHLNDK)?;
        }
        if self.ILIFHHJFMIH != 0 {
            os.write_uint32(5, self.ILIFHHJFMIH)?;
        }
        if self.PLCCICHPEDE != 0 {
            os.write_uint32(7, self.PLCCICHPEDE)?;
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

    fn new() -> ONOPFMLJIML {
        ONOPFMLJIML::new()
    }

    fn clear(&mut self) {
        self.JNKPPJHENCH = false;
        self.OEKBKACHMAB = 0;
        self.FMLKGHHLNDK = 0;
        self.ILIFHHJFMIH = 0;
        self.PLCCICHPEDE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ONOPFMLJIML {
        static instance: ONOPFMLJIML = ONOPFMLJIML {
            JNKPPJHENCH: false,
            OEKBKACHMAB: 0,
            FMLKGHHLNDK: 0,
            ILIFHHJFMIH: 0,
            PLCCICHPEDE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ONOPFMLJIML {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ONOPFMLJIML").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ONOPFMLJIML {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ONOPFMLJIML {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ONOPFMLJIML.proto\"\xb7\x01\n\x0bONOPFMLJIML\x12\x20\n\x0bJNKPPJHE\
    NCH\x18\x03\x20\x01(\x08R\x0bJNKPPJHENCH\x12\x20\n\x0bOEKBKACHMAB\x18\r\
    \x20\x01(\rR\x0bOEKBKACHMAB\x12\x20\n\x0bFMLKGHHLNDK\x18\x0f\x20\x01(\rR\
    \x0bFMLKGHHLNDK\x12\x20\n\x0bILIFHHJFMIH\x18\x05\x20\x01(\rR\x0bILIFHHJF\
    MIH\x12\x20\n\x0bPLCCICHPEDE\x18\x07\x20\x01(\rR\x0bPLCCICHPEDEb\x06prot\
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
            messages.push(ONOPFMLJIML::generated_message_descriptor_data());
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
