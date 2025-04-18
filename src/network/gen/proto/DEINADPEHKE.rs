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

//! Generated file from `DEINADPEHKE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:DEINADPEHKE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DEINADPEHKE {
    // message fields
    // @@protoc_insertion_point(field:DEINADPEHKE.JPACOBGBDBG)
    pub JPACOBGBDBG: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:DEINADPEHKE.OFGFHCLDOBG)
    pub OFGFHCLDOBG: u32,
    // @@protoc_insertion_point(field:DEINADPEHKE.LFFCMNAHBDP)
    pub LFFCMNAHBDP: i64,
    // @@protoc_insertion_point(field:DEINADPEHKE.PNAKHNBDJAE)
    pub PNAKHNBDJAE: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DEINADPEHKE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DEINADPEHKE {
    fn default() -> &'a DEINADPEHKE {
        <DEINADPEHKE as ::protobuf::Message>::default_instance()
    }
}

impl DEINADPEHKE {
    pub fn new() -> DEINADPEHKE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JPACOBGBDBG",
            |m: &DEINADPEHKE| { &m.JPACOBGBDBG },
            |m: &mut DEINADPEHKE| { &mut m.JPACOBGBDBG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OFGFHCLDOBG",
            |m: &DEINADPEHKE| { &m.OFGFHCLDOBG },
            |m: &mut DEINADPEHKE| { &mut m.OFGFHCLDOBG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LFFCMNAHBDP",
            |m: &DEINADPEHKE| { &m.LFFCMNAHBDP },
            |m: &mut DEINADPEHKE| { &mut m.LFFCMNAHBDP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PNAKHNBDJAE",
            |m: &DEINADPEHKE| { &m.PNAKHNBDJAE },
            |m: &mut DEINADPEHKE| { &mut m.PNAKHNBDJAE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DEINADPEHKE>(
            "DEINADPEHKE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DEINADPEHKE {
    const NAME: &'static str = "DEINADPEHKE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.JPACOBGBDBG)?;
                },
                88 => {
                    self.JPACOBGBDBG.push(is.read_uint32()?);
                },
                112 => {
                    self.OFGFHCLDOBG = is.read_uint32()?;
                },
                120 => {
                    self.LFFCMNAHBDP = is.read_int64()?;
                },
                104 => {
                    self.PNAKHNBDJAE = is.read_uint32()?;
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
        my_size += ::protobuf::rt::vec_packed_uint32_size(11, &self.JPACOBGBDBG);
        if self.OFGFHCLDOBG != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.OFGFHCLDOBG);
        }
        if self.LFFCMNAHBDP != 0 {
            my_size += ::protobuf::rt::int64_size(15, self.LFFCMNAHBDP);
        }
        if self.PNAKHNBDJAE != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.PNAKHNBDJAE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_uint32(11, &self.JPACOBGBDBG)?;
        if self.OFGFHCLDOBG != 0 {
            os.write_uint32(14, self.OFGFHCLDOBG)?;
        }
        if self.LFFCMNAHBDP != 0 {
            os.write_int64(15, self.LFFCMNAHBDP)?;
        }
        if self.PNAKHNBDJAE != 0 {
            os.write_uint32(13, self.PNAKHNBDJAE)?;
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

    fn new() -> DEINADPEHKE {
        DEINADPEHKE::new()
    }

    fn clear(&mut self) {
        self.JPACOBGBDBG.clear();
        self.OFGFHCLDOBG = 0;
        self.LFFCMNAHBDP = 0;
        self.PNAKHNBDJAE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DEINADPEHKE {
        static instance: DEINADPEHKE = DEINADPEHKE {
            JPACOBGBDBG: ::std::vec::Vec::new(),
            OFGFHCLDOBG: 0,
            LFFCMNAHBDP: 0,
            PNAKHNBDJAE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DEINADPEHKE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DEINADPEHKE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DEINADPEHKE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DEINADPEHKE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11DEINADPEHKE.proto\"\x95\x01\n\x0bDEINADPEHKE\x12\x20\n\x0bJPACOBGB\
    DBG\x18\x0b\x20\x03(\rR\x0bJPACOBGBDBG\x12\x20\n\x0bOFGFHCLDOBG\x18\x0e\
    \x20\x01(\rR\x0bOFGFHCLDOBG\x12\x20\n\x0bLFFCMNAHBDP\x18\x0f\x20\x01(\
    \x03R\x0bLFFCMNAHBDP\x12\x20\n\x0bPNAKHNBDJAE\x18\r\x20\x01(\rR\x0bPNAKH\
    NBDJAEb\x06proto3\
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
            messages.push(DEINADPEHKE::generated_message_descriptor_data());
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
