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

//! Generated file from `AGFNMLEBEJD.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AGFNMLEBEJD)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AGFNMLEBEJD {
    // message fields
    // @@protoc_insertion_point(field:AGFNMLEBEJD.NEDIBLKMFAO)
    pub NEDIBLKMFAO: ::protobuf::MessageField<super::FKGKOEMFGEO::FKGKOEMFGEO>,
    // @@protoc_insertion_point(field:AGFNMLEBEJD.JAEPIJOFHEC)
    pub JAEPIJOFHEC: i32,
    // @@protoc_insertion_point(field:AGFNMLEBEJD.JHBNOOKJIOA)
    pub JHBNOOKJIOA: ::protobuf::MessageField<super::FOCKFLEPFCF::FOCKFLEPFCF>,
    // @@protoc_insertion_point(field:AGFNMLEBEJD.IDFCEEDKDGN)
    pub IDFCEEDKDGN: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:AGFNMLEBEJD.HKPLKKPGPII)
    pub HKPLKKPGPII: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AGFNMLEBEJD.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AGFNMLEBEJD {
    fn default() -> &'a AGFNMLEBEJD {
        <AGFNMLEBEJD as ::protobuf::Message>::default_instance()
    }
}

impl AGFNMLEBEJD {
    pub fn new() -> AGFNMLEBEJD {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FKGKOEMFGEO::FKGKOEMFGEO>(
            "NEDIBLKMFAO",
            |m: &AGFNMLEBEJD| { &m.NEDIBLKMFAO },
            |m: &mut AGFNMLEBEJD| { &mut m.NEDIBLKMFAO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JAEPIJOFHEC",
            |m: &AGFNMLEBEJD| { &m.JAEPIJOFHEC },
            |m: &mut AGFNMLEBEJD| { &mut m.JAEPIJOFHEC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FOCKFLEPFCF::FOCKFLEPFCF>(
            "JHBNOOKJIOA",
            |m: &AGFNMLEBEJD| { &m.JHBNOOKJIOA },
            |m: &mut AGFNMLEBEJD| { &mut m.JHBNOOKJIOA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "IDFCEEDKDGN",
            |m: &AGFNMLEBEJD| { &m.IDFCEEDKDGN },
            |m: &mut AGFNMLEBEJD| { &mut m.IDFCEEDKDGN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HKPLKKPGPII",
            |m: &AGFNMLEBEJD| { &m.HKPLKKPGPII },
            |m: &mut AGFNMLEBEJD| { &mut m.HKPLKKPGPII },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AGFNMLEBEJD>(
            "AGFNMLEBEJD",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AGFNMLEBEJD {
    const NAME: &'static str = "AGFNMLEBEJD";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NEDIBLKMFAO)?;
                },
                80 => {
                    self.JAEPIJOFHEC = is.read_int32()?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.JHBNOOKJIOA)?;
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.IDFCEEDKDGN)?;
                },
                96 => {
                    self.IDFCEEDKDGN.push(is.read_uint32()?);
                },
                16 => {
                    self.HKPLKKPGPII = is.read_uint32()?;
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
        if let Some(v) = self.NEDIBLKMFAO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.JAEPIJOFHEC != 0 {
            my_size += ::protobuf::rt::int32_size(10, self.JAEPIJOFHEC);
        }
        if let Some(v) = self.JHBNOOKJIOA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.IDFCEEDKDGN {
            my_size += ::protobuf::rt::uint32_size(12, *value);
        };
        if self.HKPLKKPGPII != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.HKPLKKPGPII);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.NEDIBLKMFAO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.JAEPIJOFHEC != 0 {
            os.write_int32(10, self.JAEPIJOFHEC)?;
        }
        if let Some(v) = self.JHBNOOKJIOA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        for v in &self.IDFCEEDKDGN {
            os.write_uint32(12, *v)?;
        };
        if self.HKPLKKPGPII != 0 {
            os.write_uint32(2, self.HKPLKKPGPII)?;
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

    fn new() -> AGFNMLEBEJD {
        AGFNMLEBEJD::new()
    }

    fn clear(&mut self) {
        self.NEDIBLKMFAO.clear();
        self.JAEPIJOFHEC = 0;
        self.JHBNOOKJIOA.clear();
        self.IDFCEEDKDGN.clear();
        self.HKPLKKPGPII = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AGFNMLEBEJD {
        static instance: AGFNMLEBEJD = AGFNMLEBEJD {
            NEDIBLKMFAO: ::protobuf::MessageField::none(),
            JAEPIJOFHEC: 0,
            JHBNOOKJIOA: ::protobuf::MessageField::none(),
            IDFCEEDKDGN: ::std::vec::Vec::new(),
            HKPLKKPGPII: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AGFNMLEBEJD {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AGFNMLEBEJD").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AGFNMLEBEJD {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AGFNMLEBEJD {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AGFNMLEBEJD.proto\x1a\x11FKGKOEMFGEO.proto\x1a\x11FOCKFLEPFCF.prot\
    o\"\xd3\x01\n\x0bAGFNMLEBEJD\x12.\n\x0bNEDIBLKMFAO\x18\t\x20\x01(\x0b2\
    \x0c.FKGKOEMFGEOR\x0bNEDIBLKMFAO\x12\x20\n\x0bJAEPIJOFHEC\x18\n\x20\x01(\
    \x05R\x0bJAEPIJOFHEC\x12.\n\x0bJHBNOOKJIOA\x18\x06\x20\x01(\x0b2\x0c.FOC\
    KFLEPFCFR\x0bJHBNOOKJIOA\x12\x20\n\x0bIDFCEEDKDGN\x18\x0c\x20\x03(\rR\
    \x0bIDFCEEDKDGN\x12\x20\n\x0bHKPLKKPGPII\x18\x02\x20\x01(\rR\x0bHKPLKKPG\
    PIIb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::FKGKOEMFGEO::file_descriptor().clone());
            deps.push(super::FOCKFLEPFCF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AGFNMLEBEJD::generated_message_descriptor_data());
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
