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

//! Generated file from `FNKMGPNNPKC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FNKMGPNNPKC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FNKMGPNNPKC {
    // message fields
    // @@protoc_insertion_point(field:FNKMGPNNPKC.IKOOPCBPGLG)
    pub IKOOPCBPGLG: ::std::vec::Vec<super::LOAKNMELPHL::LOAKNMELPHL>,
    // @@protoc_insertion_point(field:FNKMGPNNPKC.JCEOEGMBFFL)
    pub JCEOEGMBFFL: u32,
    // @@protoc_insertion_point(field:FNKMGPNNPKC.IFHJLGPJCDK)
    pub IFHJLGPJCDK: u32,
    // @@protoc_insertion_point(field:FNKMGPNNPKC.NHPFGPFPDOP)
    pub NHPFGPFPDOP: u32,
    // @@protoc_insertion_point(field:FNKMGPNNPKC.EEKGDKFJKPM)
    pub EEKGDKFJKPM: u32,
    // @@protoc_insertion_point(field:FNKMGPNNPKC.PEBMOKCPDNF)
    pub PEBMOKCPDNF: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:FNKMGPNNPKC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FNKMGPNNPKC {
    fn default() -> &'a FNKMGPNNPKC {
        <FNKMGPNNPKC as ::protobuf::Message>::default_instance()
    }
}

impl FNKMGPNNPKC {
    pub fn new() -> FNKMGPNNPKC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "IKOOPCBPGLG",
            |m: &FNKMGPNNPKC| { &m.IKOOPCBPGLG },
            |m: &mut FNKMGPNNPKC| { &mut m.IKOOPCBPGLG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JCEOEGMBFFL",
            |m: &FNKMGPNNPKC| { &m.JCEOEGMBFFL },
            |m: &mut FNKMGPNNPKC| { &mut m.JCEOEGMBFFL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IFHJLGPJCDK",
            |m: &FNKMGPNNPKC| { &m.IFHJLGPJCDK },
            |m: &mut FNKMGPNNPKC| { &mut m.IFHJLGPJCDK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NHPFGPFPDOP",
            |m: &FNKMGPNNPKC| { &m.NHPFGPFPDOP },
            |m: &mut FNKMGPNNPKC| { &mut m.NHPFGPFPDOP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EEKGDKFJKPM",
            |m: &FNKMGPNNPKC| { &m.EEKGDKFJKPM },
            |m: &mut FNKMGPNNPKC| { &mut m.EEKGDKFJKPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PEBMOKCPDNF",
            |m: &FNKMGPNNPKC| { &m.PEBMOKCPDNF },
            |m: &mut FNKMGPNNPKC| { &mut m.PEBMOKCPDNF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FNKMGPNNPKC>(
            "FNKMGPNNPKC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FNKMGPNNPKC {
    const NAME: &'static str = "FNKMGPNNPKC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    self.IKOOPCBPGLG.push(is.read_message()?);
                },
                64 => {
                    self.JCEOEGMBFFL = is.read_uint32()?;
                },
                16 => {
                    self.IFHJLGPJCDK = is.read_uint32()?;
                },
                104 => {
                    self.NHPFGPFPDOP = is.read_uint32()?;
                },
                40 => {
                    self.EEKGDKFJKPM = is.read_uint32()?;
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.PEBMOKCPDNF)?;
                },
                72 => {
                    self.PEBMOKCPDNF.push(is.read_uint32()?);
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
        for value in &self.IKOOPCBPGLG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.JCEOEGMBFFL != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.JCEOEGMBFFL);
        }
        if self.IFHJLGPJCDK != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.IFHJLGPJCDK);
        }
        if self.NHPFGPFPDOP != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.NHPFGPFPDOP);
        }
        if self.EEKGDKFJKPM != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.EEKGDKFJKPM);
        }
        for value in &self.PEBMOKCPDNF {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.IKOOPCBPGLG {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        if self.JCEOEGMBFFL != 0 {
            os.write_uint32(8, self.JCEOEGMBFFL)?;
        }
        if self.IFHJLGPJCDK != 0 {
            os.write_uint32(2, self.IFHJLGPJCDK)?;
        }
        if self.NHPFGPFPDOP != 0 {
            os.write_uint32(13, self.NHPFGPFPDOP)?;
        }
        if self.EEKGDKFJKPM != 0 {
            os.write_uint32(5, self.EEKGDKFJKPM)?;
        }
        for v in &self.PEBMOKCPDNF {
            os.write_uint32(9, *v)?;
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

    fn new() -> FNKMGPNNPKC {
        FNKMGPNNPKC::new()
    }

    fn clear(&mut self) {
        self.IKOOPCBPGLG.clear();
        self.JCEOEGMBFFL = 0;
        self.IFHJLGPJCDK = 0;
        self.NHPFGPFPDOP = 0;
        self.EEKGDKFJKPM = 0;
        self.PEBMOKCPDNF.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FNKMGPNNPKC {
        static instance: FNKMGPNNPKC = FNKMGPNNPKC {
            IKOOPCBPGLG: ::std::vec::Vec::new(),
            JCEOEGMBFFL: 0,
            IFHJLGPJCDK: 0,
            NHPFGPFPDOP: 0,
            EEKGDKFJKPM: 0,
            PEBMOKCPDNF: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FNKMGPNNPKC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FNKMGPNNPKC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FNKMGPNNPKC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FNKMGPNNPKC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FNKMGPNNPKC.proto\x1a\x11LOAKNMELPHL.proto\"\xe7\x01\n\x0bFNKMGPNN\
    PKC\x12.\n\x0bIKOOPCBPGLG\x18\x0e\x20\x03(\x0b2\x0c.LOAKNMELPHLR\x0bIKOO\
    PCBPGLG\x12\x20\n\x0bJCEOEGMBFFL\x18\x08\x20\x01(\rR\x0bJCEOEGMBFFL\x12\
    \x20\n\x0bIFHJLGPJCDK\x18\x02\x20\x01(\rR\x0bIFHJLGPJCDK\x12\x20\n\x0bNH\
    PFGPFPDOP\x18\r\x20\x01(\rR\x0bNHPFGPFPDOP\x12\x20\n\x0bEEKGDKFJKPM\x18\
    \x05\x20\x01(\rR\x0bEEKGDKFJKPM\x12\x20\n\x0bPEBMOKCPDNF\x18\t\x20\x03(\
    \rR\x0bPEBMOKCPDNFb\x06proto3\
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
            deps.push(super::LOAKNMELPHL::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FNKMGPNNPKC::generated_message_descriptor_data());
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
