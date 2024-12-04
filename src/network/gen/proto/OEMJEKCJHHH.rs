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

//! Generated file from `OEMJEKCJHHH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:OEMJEKCJHHH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OEMJEKCJHHH {
    // message fields
    // @@protoc_insertion_point(field:OEMJEKCJHHH.JGMMKLOBCHO)
    pub JGMMKLOBCHO: ::std::vec::Vec<super::FCBMHIGCMHL::FCBMHIGCMHL>,
    // @@protoc_insertion_point(field:OEMJEKCJHHH.BDBHDACGOAK)
    pub BDBHDACGOAK: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:OEMJEKCJHHH.CPMDJNCBCMF)
    pub CPMDJNCBCMF: ::std::vec::Vec<super::OKBGMBEIMCB::OKBGMBEIMCB>,
    // @@protoc_insertion_point(field:OEMJEKCJHHH.CJALFGAEPKK)
    pub CJALFGAEPKK: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:OEMJEKCJHHH.LMDNPDKNDON)
    pub LMDNPDKNDON: ::std::vec::Vec<super::HLPMMJEGFMI::HLPMMJEGFMI>,
    // @@protoc_insertion_point(field:OEMJEKCJHHH.PLFOJEDCEPF)
    pub PLFOJEDCEPF: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:OEMJEKCJHHH.KAIDMEGMCDA)
    pub KAIDMEGMCDA: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:OEMJEKCJHHH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OEMJEKCJHHH {
    fn default() -> &'a OEMJEKCJHHH {
        <OEMJEKCJHHH as ::protobuf::Message>::default_instance()
    }
}

impl OEMJEKCJHHH {
    pub fn new() -> OEMJEKCJHHH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JGMMKLOBCHO",
            |m: &OEMJEKCJHHH| { &m.JGMMKLOBCHO },
            |m: &mut OEMJEKCJHHH| { &mut m.JGMMKLOBCHO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BDBHDACGOAK",
            |m: &OEMJEKCJHHH| { &m.BDBHDACGOAK },
            |m: &mut OEMJEKCJHHH| { &mut m.BDBHDACGOAK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CPMDJNCBCMF",
            |m: &OEMJEKCJHHH| { &m.CPMDJNCBCMF },
            |m: &mut OEMJEKCJHHH| { &mut m.CPMDJNCBCMF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CJALFGAEPKK",
            |m: &OEMJEKCJHHH| { &m.CJALFGAEPKK },
            |m: &mut OEMJEKCJHHH| { &mut m.CJALFGAEPKK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LMDNPDKNDON",
            |m: &OEMJEKCJHHH| { &m.LMDNPDKNDON },
            |m: &mut OEMJEKCJHHH| { &mut m.LMDNPDKNDON },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PLFOJEDCEPF",
            |m: &OEMJEKCJHHH| { &m.PLFOJEDCEPF },
            |m: &mut OEMJEKCJHHH| { &mut m.PLFOJEDCEPF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KAIDMEGMCDA",
            |m: &OEMJEKCJHHH| { &m.KAIDMEGMCDA },
            |m: &mut OEMJEKCJHHH| { &mut m.KAIDMEGMCDA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OEMJEKCJHHH>(
            "OEMJEKCJHHH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OEMJEKCJHHH {
    const NAME: &'static str = "OEMJEKCJHHH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    self.JGMMKLOBCHO.push(is.read_message()?);
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.BDBHDACGOAK)?;
                },
                64 => {
                    self.BDBHDACGOAK.push(is.read_uint32()?);
                },
                58 => {
                    self.CPMDJNCBCMF.push(is.read_message()?);
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.CJALFGAEPKK)?;
                },
                32 => {
                    self.CJALFGAEPKK.push(is.read_uint32()?);
                },
                74 => {
                    self.LMDNPDKNDON.push(is.read_message()?);
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.PLFOJEDCEPF)?;
                },
                96 => {
                    self.PLFOJEDCEPF.push(is.read_uint32()?);
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.KAIDMEGMCDA)?;
                },
                104 => {
                    self.KAIDMEGMCDA.push(is.read_uint32()?);
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
        for value in &self.JGMMKLOBCHO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.BDBHDACGOAK {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        for value in &self.CPMDJNCBCMF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.CJALFGAEPKK {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        for value in &self.LMDNPDKNDON {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.PLFOJEDCEPF {
            my_size += ::protobuf::rt::uint32_size(12, *value);
        };
        for value in &self.KAIDMEGMCDA {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.JGMMKLOBCHO {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        for v in &self.BDBHDACGOAK {
            os.write_uint32(8, *v)?;
        };
        for v in &self.CPMDJNCBCMF {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        for v in &self.CJALFGAEPKK {
            os.write_uint32(4, *v)?;
        };
        for v in &self.LMDNPDKNDON {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        for v in &self.PLFOJEDCEPF {
            os.write_uint32(12, *v)?;
        };
        for v in &self.KAIDMEGMCDA {
            os.write_uint32(13, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> OEMJEKCJHHH {
        OEMJEKCJHHH::new()
    }

    fn clear(&mut self) {
        self.JGMMKLOBCHO.clear();
        self.BDBHDACGOAK.clear();
        self.CPMDJNCBCMF.clear();
        self.CJALFGAEPKK.clear();
        self.LMDNPDKNDON.clear();
        self.PLFOJEDCEPF.clear();
        self.KAIDMEGMCDA.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OEMJEKCJHHH {
        static instance: OEMJEKCJHHH = OEMJEKCJHHH {
            JGMMKLOBCHO: ::std::vec::Vec::new(),
            BDBHDACGOAK: ::std::vec::Vec::new(),
            CPMDJNCBCMF: ::std::vec::Vec::new(),
            CJALFGAEPKK: ::std::vec::Vec::new(),
            LMDNPDKNDON: ::std::vec::Vec::new(),
            PLFOJEDCEPF: ::std::vec::Vec::new(),
            KAIDMEGMCDA: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OEMJEKCJHHH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OEMJEKCJHHH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OEMJEKCJHHH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OEMJEKCJHHH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OEMJEKCJHHH.proto\x1a\x11FCBMHIGCMHL.proto\x1a\x11HLPMMJEGFMI.prot\
    o\x1a\x11OKBGMBEIMCB.proto\"\xa5\x02\n\x0bOEMJEKCJHHH\x12.\n\x0bJGMMKLOB\
    CHO\x18\x0e\x20\x03(\x0b2\x0c.FCBMHIGCMHLR\x0bJGMMKLOBCHO\x12\x20\n\x0bB\
    DBHDACGOAK\x18\x08\x20\x03(\rR\x0bBDBHDACGOAK\x12.\n\x0bCPMDJNCBCMF\x18\
    \x07\x20\x03(\x0b2\x0c.OKBGMBEIMCBR\x0bCPMDJNCBCMF\x12\x20\n\x0bCJALFGAE\
    PKK\x18\x04\x20\x03(\rR\x0bCJALFGAEPKK\x12.\n\x0bLMDNPDKNDON\x18\t\x20\
    \x03(\x0b2\x0c.HLPMMJEGFMIR\x0bLMDNPDKNDON\x12\x20\n\x0bPLFOJEDCEPF\x18\
    \x0c\x20\x03(\rR\x0bPLFOJEDCEPF\x12\x20\n\x0bKAIDMEGMCDA\x18\r\x20\x03(\
    \rR\x0bKAIDMEGMCDAb\x06proto3\
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
            deps.push(super::FCBMHIGCMHL::file_descriptor().clone());
            deps.push(super::HLPMMJEGFMI::file_descriptor().clone());
            deps.push(super::OKBGMBEIMCB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OEMJEKCJHHH::generated_message_descriptor_data());
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
