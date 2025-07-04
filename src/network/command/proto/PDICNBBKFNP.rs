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

//! Generated file from `PDICNBBKFNP.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:PDICNBBKFNP)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PDICNBBKFNP {
    // message fields
    // @@protoc_insertion_point(field:PDICNBBKFNP.KLGHECCBHCG)
    pub KLGHECCBHCG: ::std::vec::Vec<super::CEENLALPDMK::CEENLALPDMK>,
    // @@protoc_insertion_point(field:PDICNBBKFNP.exp)
    pub exp: u32,
    // @@protoc_insertion_point(field:PDICNBBKFNP.LCJNNDGKIDP)
    pub LCJNNDGKIDP: ::std::vec::Vec<super::ECMMJLLHPMD::ECMMJLLHPMD>,
    // @@protoc_insertion_point(field:PDICNBBKFNP.FEJGMNNFFLG)
    pub FEJGMNNFFLG: ::std::vec::Vec<super::PDFHJMMDGAE::PDFHJMMDGAE>,
    // @@protoc_insertion_point(field:PDICNBBKFNP.item_value)
    pub item_value: u32,
    // @@protoc_insertion_point(field:PDICNBBKFNP.EODGCNAFIAC)
    pub EODGCNAFIAC: u32,
    // @@protoc_insertion_point(field:PDICNBBKFNP.FJOCDKIFPPC)
    pub FJOCDKIFPPC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PDICNBBKFNP.COKDNPEEMAG)
    pub COKDNPEEMAG: ::std::vec::Vec<super::IMGJIEBFGPF::IMGJIEBFGPF>,
    // @@protoc_insertion_point(field:PDICNBBKFNP.LMELJCIFBDF)
    pub LMELJCIFBDF: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:PDICNBBKFNP.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PDICNBBKFNP {
    fn default() -> &'a PDICNBBKFNP {
        <PDICNBBKFNP as ::protobuf::Message>::default_instance()
    }
}

impl PDICNBBKFNP {
    pub fn new() -> PDICNBBKFNP {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KLGHECCBHCG",
            |m: &PDICNBBKFNP| { &m.KLGHECCBHCG },
            |m: &mut PDICNBBKFNP| { &mut m.KLGHECCBHCG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &PDICNBBKFNP| { &m.exp },
            |m: &mut PDICNBBKFNP| { &mut m.exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LCJNNDGKIDP",
            |m: &PDICNBBKFNP| { &m.LCJNNDGKIDP },
            |m: &mut PDICNBBKFNP| { &mut m.LCJNNDGKIDP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FEJGMNNFFLG",
            |m: &PDICNBBKFNP| { &m.FEJGMNNFFLG },
            |m: &mut PDICNBBKFNP| { &mut m.FEJGMNNFFLG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "item_value",
            |m: &PDICNBBKFNP| { &m.item_value },
            |m: &mut PDICNBBKFNP| { &mut m.item_value },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EODGCNAFIAC",
            |m: &PDICNBBKFNP| { &m.EODGCNAFIAC },
            |m: &mut PDICNBBKFNP| { &mut m.EODGCNAFIAC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FJOCDKIFPPC",
            |m: &PDICNBBKFNP| { &m.FJOCDKIFPPC },
            |m: &mut PDICNBBKFNP| { &mut m.FJOCDKIFPPC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "COKDNPEEMAG",
            |m: &PDICNBBKFNP| { &m.COKDNPEEMAG },
            |m: &mut PDICNBBKFNP| { &mut m.COKDNPEEMAG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LMELJCIFBDF",
            |m: &PDICNBBKFNP| { &m.LMELJCIFBDF },
            |m: &mut PDICNBBKFNP| { &mut m.LMELJCIFBDF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PDICNBBKFNP>(
            "PDICNBBKFNP",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PDICNBBKFNP {
    const NAME: &'static str = "PDICNBBKFNP";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    self.KLGHECCBHCG.push(is.read_message()?);
                },
                96 => {
                    self.exp = is.read_uint32()?;
                },
                34 => {
                    self.LCJNNDGKIDP.push(is.read_message()?);
                },
                114 => {
                    self.FEJGMNNFFLG.push(is.read_message()?);
                },
                80 => {
                    self.item_value = is.read_uint32()?;
                },
                40 => {
                    self.EODGCNAFIAC = is.read_uint32()?;
                },
                50 => {
                    is.read_repeated_packed_uint32_into(&mut self.FJOCDKIFPPC)?;
                },
                48 => {
                    self.FJOCDKIFPPC.push(is.read_uint32()?);
                },
                26 => {
                    self.COKDNPEEMAG.push(is.read_message()?);
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.LMELJCIFBDF)?;
                },
                64 => {
                    self.LMELJCIFBDF.push(is.read_uint32()?);
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
        for value in &self.KLGHECCBHCG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.exp);
        }
        for value in &self.LCJNNDGKIDP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.FEJGMNNFFLG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.item_value != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.item_value);
        }
        if self.EODGCNAFIAC != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.EODGCNAFIAC);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(6, &self.FJOCDKIFPPC);
        for value in &self.COKDNPEEMAG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::vec_packed_uint32_size(8, &self.LMELJCIFBDF);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.KLGHECCBHCG {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.exp != 0 {
            os.write_uint32(12, self.exp)?;
        }
        for v in &self.LCJNNDGKIDP {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        for v in &self.FEJGMNNFFLG {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        if self.item_value != 0 {
            os.write_uint32(10, self.item_value)?;
        }
        if self.EODGCNAFIAC != 0 {
            os.write_uint32(5, self.EODGCNAFIAC)?;
        }
        os.write_repeated_packed_uint32(6, &self.FJOCDKIFPPC)?;
        for v in &self.COKDNPEEMAG {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        os.write_repeated_packed_uint32(8, &self.LMELJCIFBDF)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PDICNBBKFNP {
        PDICNBBKFNP::new()
    }

    fn clear(&mut self) {
        self.KLGHECCBHCG.clear();
        self.exp = 0;
        self.LCJNNDGKIDP.clear();
        self.FEJGMNNFFLG.clear();
        self.item_value = 0;
        self.EODGCNAFIAC = 0;
        self.FJOCDKIFPPC.clear();
        self.COKDNPEEMAG.clear();
        self.LMELJCIFBDF.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PDICNBBKFNP {
        static instance: PDICNBBKFNP = PDICNBBKFNP {
            KLGHECCBHCG: ::std::vec::Vec::new(),
            exp: 0,
            LCJNNDGKIDP: ::std::vec::Vec::new(),
            FEJGMNNFFLG: ::std::vec::Vec::new(),
            item_value: 0,
            EODGCNAFIAC: 0,
            FJOCDKIFPPC: ::std::vec::Vec::new(),
            COKDNPEEMAG: ::std::vec::Vec::new(),
            LMELJCIFBDF: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PDICNBBKFNP {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PDICNBBKFNP").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PDICNBBKFNP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PDICNBBKFNP {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PDICNBBKFNP.proto\x1a\x11CEENLALPDMK.proto\x1a\x11ECMMJLLHPMD.prot\
    o\x1a\x11IMGJIEBFGPF.proto\x1a\x11PDFHJMMDGAE.proto\"\xe4\x02\n\x0bPDICN\
    BBKFNP\x12.\n\x0bKLGHECCBHCG\x18\t\x20\x03(\x0b2\x0c.CEENLALPDMKR\x0bKLG\
    HECCBHCG\x12\x10\n\x03exp\x18\x0c\x20\x01(\rR\x03exp\x12.\n\x0bLCJNNDGKI\
    DP\x18\x04\x20\x03(\x0b2\x0c.ECMMJLLHPMDR\x0bLCJNNDGKIDP\x12.\n\x0bFEJGM\
    NNFFLG\x18\x0e\x20\x03(\x0b2\x0c.PDFHJMMDGAER\x0bFEJGMNNFFLG\x12\x1d\n\n\
    item_value\x18\n\x20\x01(\rR\titemValue\x12\x20\n\x0bEODGCNAFIAC\x18\x05\
    \x20\x01(\rR\x0bEODGCNAFIAC\x12\x20\n\x0bFJOCDKIFPPC\x18\x06\x20\x03(\rR\
    \x0bFJOCDKIFPPC\x12.\n\x0bCOKDNPEEMAG\x18\x03\x20\x03(\x0b2\x0c.IMGJIEBF\
    GPFR\x0bCOKDNPEEMAG\x12\x20\n\x0bLMELJCIFBDF\x18\x08\x20\x03(\rR\x0bLMEL\
    JCIFBDFb\x06proto3\
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
            deps.push(super::CEENLALPDMK::file_descriptor().clone());
            deps.push(super::ECMMJLLHPMD::file_descriptor().clone());
            deps.push(super::IMGJIEBFGPF::file_descriptor().clone());
            deps.push(super::PDFHJMMDGAE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PDICNBBKFNP::generated_message_descriptor_data());
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
