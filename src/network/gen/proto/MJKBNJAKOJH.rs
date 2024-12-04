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

//! Generated file from `MJKBNJAKOJH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MJKBNJAKOJH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MJKBNJAKOJH {
    // message fields
    // @@protoc_insertion_point(field:MJKBNJAKOJH.ONGDCBBDNGB)
    pub ONGDCBBDNGB: ::protobuf::MessageField<super::HCAEMAPAEGN::HCAEMAPAEGN>,
    // @@protoc_insertion_point(field:MJKBNJAKOJH.ILIOLNDJHCN)
    pub ILIOLNDJHCN: ::protobuf::MessageField<super::OPFGFMECLBE::OPFGFMECLBE>,
    // @@protoc_insertion_point(field:MJKBNJAKOJH.KNPFNNGBAIC)
    pub KNPFNNGBAIC: u32,
    // @@protoc_insertion_point(field:MJKBNJAKOJH.POAFBAKKLJI)
    pub POAFBAKKLJI: ::protobuf::MessageField<super::KIBHPLLKFAM::KIBHPLLKFAM>,
    // @@protoc_insertion_point(field:MJKBNJAKOJH.FEOIPHDDMDH)
    pub FEOIPHDDMDH: ::protobuf::MessageField<super::EJJBPHADAOC::EJJBPHADAOC>,
    // special fields
    // @@protoc_insertion_point(special_field:MJKBNJAKOJH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MJKBNJAKOJH {
    fn default() -> &'a MJKBNJAKOJH {
        <MJKBNJAKOJH as ::protobuf::Message>::default_instance()
    }
}

impl MJKBNJAKOJH {
    pub fn new() -> MJKBNJAKOJH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HCAEMAPAEGN::HCAEMAPAEGN>(
            "ONGDCBBDNGB",
            |m: &MJKBNJAKOJH| { &m.ONGDCBBDNGB },
            |m: &mut MJKBNJAKOJH| { &mut m.ONGDCBBDNGB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OPFGFMECLBE::OPFGFMECLBE>(
            "ILIOLNDJHCN",
            |m: &MJKBNJAKOJH| { &m.ILIOLNDJHCN },
            |m: &mut MJKBNJAKOJH| { &mut m.ILIOLNDJHCN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KNPFNNGBAIC",
            |m: &MJKBNJAKOJH| { &m.KNPFNNGBAIC },
            |m: &mut MJKBNJAKOJH| { &mut m.KNPFNNGBAIC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::KIBHPLLKFAM::KIBHPLLKFAM>(
            "POAFBAKKLJI",
            |m: &MJKBNJAKOJH| { &m.POAFBAKKLJI },
            |m: &mut MJKBNJAKOJH| { &mut m.POAFBAKKLJI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EJJBPHADAOC::EJJBPHADAOC>(
            "FEOIPHDDMDH",
            |m: &MJKBNJAKOJH| { &m.FEOIPHDDMDH },
            |m: &mut MJKBNJAKOJH| { &mut m.FEOIPHDDMDH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MJKBNJAKOJH>(
            "MJKBNJAKOJH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MJKBNJAKOJH {
    const NAME: &'static str = "MJKBNJAKOJH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ONGDCBBDNGB)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ILIOLNDJHCN)?;
                },
                40 => {
                    self.KNPFNNGBAIC = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.POAFBAKKLJI)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.FEOIPHDDMDH)?;
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
        if let Some(v) = self.ONGDCBBDNGB.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.ILIOLNDJHCN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.KNPFNNGBAIC != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.KNPFNNGBAIC);
        }
        if let Some(v) = self.POAFBAKKLJI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.FEOIPHDDMDH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.ONGDCBBDNGB.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.ILIOLNDJHCN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if self.KNPFNNGBAIC != 0 {
            os.write_uint32(5, self.KNPFNNGBAIC)?;
        }
        if let Some(v) = self.POAFBAKKLJI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if let Some(v) = self.FEOIPHDDMDH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> MJKBNJAKOJH {
        MJKBNJAKOJH::new()
    }

    fn clear(&mut self) {
        self.ONGDCBBDNGB.clear();
        self.ILIOLNDJHCN.clear();
        self.KNPFNNGBAIC = 0;
        self.POAFBAKKLJI.clear();
        self.FEOIPHDDMDH.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MJKBNJAKOJH {
        static instance: MJKBNJAKOJH = MJKBNJAKOJH {
            ONGDCBBDNGB: ::protobuf::MessageField::none(),
            ILIOLNDJHCN: ::protobuf::MessageField::none(),
            KNPFNNGBAIC: 0,
            POAFBAKKLJI: ::protobuf::MessageField::none(),
            FEOIPHDDMDH: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MJKBNJAKOJH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MJKBNJAKOJH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MJKBNJAKOJH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MJKBNJAKOJH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MJKBNJAKOJH.proto\x1a\x11EJJBPHADAOC.proto\x1a\x11HCAEMAPAEGN.prot\
    o\x1a\x11KIBHPLLKFAM.proto\x1a\x11OPFGFMECLBE.proto\"\xef\x01\n\x0bMJKBN\
    JAKOJH\x12.\n\x0bONGDCBBDNGB\x18\x0f\x20\x01(\x0b2\x0c.HCAEMAPAEGNR\x0bO\
    NGDCBBDNGB\x12.\n\x0bILIOLNDJHCN\x18\n\x20\x01(\x0b2\x0c.OPFGFMECLBER\
    \x0bILIOLNDJHCN\x12\x20\n\x0bKNPFNNGBAIC\x18\x05\x20\x01(\rR\x0bKNPFNNGB\
    AIC\x12.\n\x0bPOAFBAKKLJI\x18\x07\x20\x01(\x0b2\x0c.KIBHPLLKFAMR\x0bPOAF\
    BAKKLJI\x12.\n\x0bFEOIPHDDMDH\x18\x04\x20\x01(\x0b2\x0c.EJJBPHADAOCR\x0b\
    FEOIPHDDMDHb\x06proto3\
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
            deps.push(super::EJJBPHADAOC::file_descriptor().clone());
            deps.push(super::HCAEMAPAEGN::file_descriptor().clone());
            deps.push(super::KIBHPLLKFAM::file_descriptor().clone());
            deps.push(super::OPFGFMECLBE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MJKBNJAKOJH::generated_message_descriptor_data());
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
