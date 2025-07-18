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

//! Generated file from `TarotBookGetDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:TarotBookGetDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TarotBookGetDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:TarotBookGetDataScRsp.LENPIDMPECP)
    pub LENPIDMPECP: u32,
    // @@protoc_insertion_point(field:TarotBookGetDataScRsp.HEFJEJHOJEA)
    pub HEFJEJHOJEA: ::protobuf::MessageField<super::FLOICKMNMLL::FLOICKMNMLL>,
    // @@protoc_insertion_point(field:TarotBookGetDataScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:TarotBookGetDataScRsp.NDCJJPGNFLN)
    pub NDCJJPGNFLN: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:TarotBookGetDataScRsp.energy_info)
    pub energy_info: u32,
    // @@protoc_insertion_point(field:TarotBookGetDataScRsp.IPNINOPEKBP)
    pub IPNINOPEKBP: ::protobuf::MessageField<super::OFDGOGDBHAC::OFDGOGDBHAC>,
    // @@protoc_insertion_point(field:TarotBookGetDataScRsp.JKEMDJIAMHI)
    pub JKEMDJIAMHI: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:TarotBookGetDataScRsp.FDCKFKFKHLO)
    pub FDCKFKFKHLO: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TarotBookGetDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TarotBookGetDataScRsp {
    fn default() -> &'a TarotBookGetDataScRsp {
        <TarotBookGetDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl TarotBookGetDataScRsp {
    pub fn new() -> TarotBookGetDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LENPIDMPECP",
            |m: &TarotBookGetDataScRsp| { &m.LENPIDMPECP },
            |m: &mut TarotBookGetDataScRsp| { &mut m.LENPIDMPECP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FLOICKMNMLL::FLOICKMNMLL>(
            "HEFJEJHOJEA",
            |m: &TarotBookGetDataScRsp| { &m.HEFJEJHOJEA },
            |m: &mut TarotBookGetDataScRsp| { &mut m.HEFJEJHOJEA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &TarotBookGetDataScRsp| { &m.retcode },
            |m: &mut TarotBookGetDataScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "NDCJJPGNFLN",
            |m: &TarotBookGetDataScRsp| { &m.NDCJJPGNFLN },
            |m: &mut TarotBookGetDataScRsp| { &mut m.NDCJJPGNFLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "energy_info",
            |m: &TarotBookGetDataScRsp| { &m.energy_info },
            |m: &mut TarotBookGetDataScRsp| { &mut m.energy_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OFDGOGDBHAC::OFDGOGDBHAC>(
            "IPNINOPEKBP",
            |m: &TarotBookGetDataScRsp| { &m.IPNINOPEKBP },
            |m: &mut TarotBookGetDataScRsp| { &mut m.IPNINOPEKBP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "JKEMDJIAMHI",
            |m: &TarotBookGetDataScRsp| { &m.JKEMDJIAMHI },
            |m: &mut TarotBookGetDataScRsp| { &mut m.JKEMDJIAMHI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FDCKFKFKHLO",
            |m: &TarotBookGetDataScRsp| { &m.FDCKFKFKHLO },
            |m: &mut TarotBookGetDataScRsp| { &mut m.FDCKFKFKHLO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TarotBookGetDataScRsp>(
            "TarotBookGetDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TarotBookGetDataScRsp {
    const NAME: &'static str = "TarotBookGetDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.LENPIDMPECP = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HEFJEJHOJEA)?;
                },
                112 => {
                    self.retcode = is.read_uint32()?;
                },
                50 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.NDCJJPGNFLN.insert(key, value);
                },
                80 => {
                    self.energy_info = is.read_uint32()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IPNINOPEKBP)?;
                },
                90 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.JKEMDJIAMHI.insert(key, value);
                },
                104 => {
                    self.FDCKFKFKHLO = is.read_uint32()?;
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
        if self.LENPIDMPECP != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.LENPIDMPECP);
        }
        if let Some(v) = self.HEFJEJHOJEA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.retcode);
        }
        for (k, v) in &self.NDCJJPGNFLN {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.energy_info != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.energy_info);
        }
        if let Some(v) = self.IPNINOPEKBP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for (k, v) in &self.JKEMDJIAMHI {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.FDCKFKFKHLO != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.FDCKFKFKHLO);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LENPIDMPECP != 0 {
            os.write_uint32(3, self.LENPIDMPECP)?;
        }
        if let Some(v) = self.HEFJEJHOJEA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(14, self.retcode)?;
        }
        for (k, v) in &self.NDCJJPGNFLN {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(50)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if self.energy_info != 0 {
            os.write_uint32(10, self.energy_info)?;
        }
        if let Some(v) = self.IPNINOPEKBP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        for (k, v) in &self.JKEMDJIAMHI {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(90)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if self.FDCKFKFKHLO != 0 {
            os.write_uint32(13, self.FDCKFKFKHLO)?;
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

    fn new() -> TarotBookGetDataScRsp {
        TarotBookGetDataScRsp::new()
    }

    fn clear(&mut self) {
        self.LENPIDMPECP = 0;
        self.HEFJEJHOJEA.clear();
        self.retcode = 0;
        self.NDCJJPGNFLN.clear();
        self.energy_info = 0;
        self.IPNINOPEKBP.clear();
        self.JKEMDJIAMHI.clear();
        self.FDCKFKFKHLO = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TarotBookGetDataScRsp {
        static instance: ::protobuf::rt::Lazy<TarotBookGetDataScRsp> = ::protobuf::rt::Lazy::new();
        instance.get(TarotBookGetDataScRsp::new)
    }
}

impl ::protobuf::MessageFull for TarotBookGetDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TarotBookGetDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TarotBookGetDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TarotBookGetDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bTarotBookGetDataScRsp.proto\x1a\x11FLOICKMNMLL.proto\x1a\x11OFDGOG\
    DBHAC.proto\"\x8c\x04\n\x15TarotBookGetDataScRsp\x12\x20\n\x0bLENPIDMPEC\
    P\x18\x03\x20\x01(\rR\x0bLENPIDMPECP\x12.\n\x0bHEFJEJHOJEA\x18\t\x20\x01\
    (\x0b2\x0c.FLOICKMNMLLR\x0bHEFJEJHOJEA\x12\x18\n\x07retcode\x18\x0e\x20\
    \x01(\rR\x07retcode\x12I\n\x0bNDCJJPGNFLN\x18\x06\x20\x03(\x0b2'.TarotBo\
    okGetDataScRsp.NDCJJPGNFLNEntryR\x0bNDCJJPGNFLN\x12\x1f\n\x0benergy_info\
    \x18\n\x20\x01(\rR\nenergyInfo\x12.\n\x0bIPNINOPEKBP\x18\x05\x20\x01(\
    \x0b2\x0c.OFDGOGDBHACR\x0bIPNINOPEKBP\x12I\n\x0bJKEMDJIAMHI\x18\x0b\x20\
    \x03(\x0b2'.TarotBookGetDataScRsp.JKEMDJIAMHIEntryR\x0bJKEMDJIAMHI\x12\
    \x20\n\x0bFDCKFKFKHLO\x18\r\x20\x01(\rR\x0bFDCKFKFKHLO\x1a>\n\x10NDCJJPG\
    NFLNEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\
    \x18\x02\x20\x01(\rR\x05value:\x028\x01\x1a>\n\x10JKEMDJIAMHIEntry\x12\
    \x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\
    \x01(\rR\x05value:\x028\x01b\x06proto3\
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
            deps.push(super::FLOICKMNMLL::file_descriptor().clone());
            deps.push(super::OFDGOGDBHAC::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TarotBookGetDataScRsp::generated_message_descriptor_data());
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
