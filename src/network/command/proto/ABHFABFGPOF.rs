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

//! Generated file from `ABHFABFGPOF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ABHFABFGPOF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ABHFABFGPOF {
    // message fields
    // @@protoc_insertion_point(field:ABHFABFGPOF.CLOONOIFEFO)
    pub CLOONOIFEFO: bool,
    // @@protoc_insertion_point(field:ABHFABFGPOF.FCJECKCICKB)
    pub FCJECKCICKB: u32,
    // @@protoc_insertion_point(field:ABHFABFGPOF.NLMDEMOHBOO)
    pub NLMDEMOHBOO: u32,
    // @@protoc_insertion_point(field:ABHFABFGPOF.NKHKDJKEGDH)
    pub NKHKDJKEGDH: u32,
    // @@protoc_insertion_point(field:ABHFABFGPOF.PIKAPDJHGND)
    pub PIKAPDJHGND: u32,
    // @@protoc_insertion_point(field:ABHFABFGPOF.JAFNPNMOHCM)
    pub JAFNPNMOHCM: u32,
    // @@protoc_insertion_point(field:ABHFABFGPOF.BIINNCNDPCG)
    pub BIINNCNDPCG: bool,
    // special fields
    // @@protoc_insertion_point(special_field:ABHFABFGPOF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ABHFABFGPOF {
    fn default() -> &'a ABHFABFGPOF {
        <ABHFABFGPOF as ::protobuf::Message>::default_instance()
    }
}

impl ABHFABFGPOF {
    pub fn new() -> ABHFABFGPOF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CLOONOIFEFO",
            |m: &ABHFABFGPOF| { &m.CLOONOIFEFO },
            |m: &mut ABHFABFGPOF| { &mut m.CLOONOIFEFO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FCJECKCICKB",
            |m: &ABHFABFGPOF| { &m.FCJECKCICKB },
            |m: &mut ABHFABFGPOF| { &mut m.FCJECKCICKB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NLMDEMOHBOO",
            |m: &ABHFABFGPOF| { &m.NLMDEMOHBOO },
            |m: &mut ABHFABFGPOF| { &mut m.NLMDEMOHBOO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NKHKDJKEGDH",
            |m: &ABHFABFGPOF| { &m.NKHKDJKEGDH },
            |m: &mut ABHFABFGPOF| { &mut m.NKHKDJKEGDH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PIKAPDJHGND",
            |m: &ABHFABFGPOF| { &m.PIKAPDJHGND },
            |m: &mut ABHFABFGPOF| { &mut m.PIKAPDJHGND },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JAFNPNMOHCM",
            |m: &ABHFABFGPOF| { &m.JAFNPNMOHCM },
            |m: &mut ABHFABFGPOF| { &mut m.JAFNPNMOHCM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BIINNCNDPCG",
            |m: &ABHFABFGPOF| { &m.BIINNCNDPCG },
            |m: &mut ABHFABFGPOF| { &mut m.BIINNCNDPCG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ABHFABFGPOF>(
            "ABHFABFGPOF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ABHFABFGPOF {
    const NAME: &'static str = "ABHFABFGPOF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.CLOONOIFEFO = is.read_bool()?;
                },
                64 => {
                    self.FCJECKCICKB = is.read_uint32()?;
                },
                40 => {
                    self.NLMDEMOHBOO = is.read_uint32()?;
                },
                80 => {
                    self.NKHKDJKEGDH = is.read_uint32()?;
                },
                112 => {
                    self.PIKAPDJHGND = is.read_uint32()?;
                },
                104 => {
                    self.JAFNPNMOHCM = is.read_uint32()?;
                },
                24 => {
                    self.BIINNCNDPCG = is.read_bool()?;
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
        if self.CLOONOIFEFO != false {
            my_size += 1 + 1;
        }
        if self.FCJECKCICKB != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.FCJECKCICKB);
        }
        if self.NLMDEMOHBOO != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.NLMDEMOHBOO);
        }
        if self.NKHKDJKEGDH != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.NKHKDJKEGDH);
        }
        if self.PIKAPDJHGND != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.PIKAPDJHGND);
        }
        if self.JAFNPNMOHCM != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.JAFNPNMOHCM);
        }
        if self.BIINNCNDPCG != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.CLOONOIFEFO != false {
            os.write_bool(2, self.CLOONOIFEFO)?;
        }
        if self.FCJECKCICKB != 0 {
            os.write_uint32(8, self.FCJECKCICKB)?;
        }
        if self.NLMDEMOHBOO != 0 {
            os.write_uint32(5, self.NLMDEMOHBOO)?;
        }
        if self.NKHKDJKEGDH != 0 {
            os.write_uint32(10, self.NKHKDJKEGDH)?;
        }
        if self.PIKAPDJHGND != 0 {
            os.write_uint32(14, self.PIKAPDJHGND)?;
        }
        if self.JAFNPNMOHCM != 0 {
            os.write_uint32(13, self.JAFNPNMOHCM)?;
        }
        if self.BIINNCNDPCG != false {
            os.write_bool(3, self.BIINNCNDPCG)?;
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

    fn new() -> ABHFABFGPOF {
        ABHFABFGPOF::new()
    }

    fn clear(&mut self) {
        self.CLOONOIFEFO = false;
        self.FCJECKCICKB = 0;
        self.NLMDEMOHBOO = 0;
        self.NKHKDJKEGDH = 0;
        self.PIKAPDJHGND = 0;
        self.JAFNPNMOHCM = 0;
        self.BIINNCNDPCG = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ABHFABFGPOF {
        static instance: ABHFABFGPOF = ABHFABFGPOF {
            CLOONOIFEFO: false,
            FCJECKCICKB: 0,
            NLMDEMOHBOO: 0,
            NKHKDJKEGDH: 0,
            PIKAPDJHGND: 0,
            JAFNPNMOHCM: 0,
            BIINNCNDPCG: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ABHFABFGPOF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ABHFABFGPOF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ABHFABFGPOF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ABHFABFGPOF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11ABHFABFGPOF.proto\"\xfb\x01\n\x0bABHFABFGPOF\x12\x20\n\x0bCLOONOIF\
    EFO\x18\x02\x20\x01(\x08R\x0bCLOONOIFEFO\x12\x20\n\x0bFCJECKCICKB\x18\
    \x08\x20\x01(\rR\x0bFCJECKCICKB\x12\x20\n\x0bNLMDEMOHBOO\x18\x05\x20\x01\
    (\rR\x0bNLMDEMOHBOO\x12\x20\n\x0bNKHKDJKEGDH\x18\n\x20\x01(\rR\x0bNKHKDJ\
    KEGDH\x12\x20\n\x0bPIKAPDJHGND\x18\x0e\x20\x01(\rR\x0bPIKAPDJHGND\x12\
    \x20\n\x0bJAFNPNMOHCM\x18\r\x20\x01(\rR\x0bJAFNPNMOHCM\x12\x20\n\x0bBIIN\
    NCNDPCG\x18\x03\x20\x01(\x08R\x0bBIINNCNDPCGb\x06proto3\
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
            messages.push(ABHFABFGPOF::generated_message_descriptor_data());
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
