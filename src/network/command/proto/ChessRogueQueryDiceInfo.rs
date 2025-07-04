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

//! Generated file from `ChessRogueQueryDiceInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ChessRogueQueryDiceInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueQueryDiceInfo {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueQueryDiceInfo.ALEHDAAOHOE)
    pub ALEHDAAOHOE: ::std::vec::Vec<super::ChessRogueDice::ChessRogueDice>,
    // @@protoc_insertion_point(field:ChessRogueQueryDiceInfo.NBCMAKNLPHG)
    pub NBCMAKNLPHG: ::std::collections::HashMap<u32, bool>,
    // @@protoc_insertion_point(field:ChessRogueQueryDiceInfo.JKMIMLBAJBL)
    pub JKMIMLBAJBL: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ChessRogueQueryDiceInfo.HDMPBPOFFDK)
    pub HDMPBPOFFDK: ::protobuf::EnumOrUnknown<super::ChessRogueNousDicePhase::ChessRogueNousDicePhase>,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueQueryDiceInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueQueryDiceInfo {
    fn default() -> &'a ChessRogueQueryDiceInfo {
        <ChessRogueQueryDiceInfo as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueQueryDiceInfo {
    pub fn new() -> ChessRogueQueryDiceInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ALEHDAAOHOE",
            |m: &ChessRogueQueryDiceInfo| { &m.ALEHDAAOHOE },
            |m: &mut ChessRogueQueryDiceInfo| { &mut m.ALEHDAAOHOE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor_new::<_, _>(
            "NBCMAKNLPHG",
            |m: &ChessRogueQueryDiceInfo| { &m.NBCMAKNLPHG },
            |m: &mut ChessRogueQueryDiceInfo| { &mut m.NBCMAKNLPHG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JKMIMLBAJBL",
            |m: &ChessRogueQueryDiceInfo| { &m.JKMIMLBAJBL },
            |m: &mut ChessRogueQueryDiceInfo| { &mut m.JKMIMLBAJBL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HDMPBPOFFDK",
            |m: &ChessRogueQueryDiceInfo| { &m.HDMPBPOFFDK },
            |m: &mut ChessRogueQueryDiceInfo| { &mut m.HDMPBPOFFDK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueQueryDiceInfo>(
            "ChessRogueQueryDiceInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueQueryDiceInfo {
    const NAME: &'static str = "ChessRogueQueryDiceInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    self.ALEHDAAOHOE.push(is.read_message()?);
                },
                18 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_bool()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.NBCMAKNLPHG.insert(key, value);
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.JKMIMLBAJBL)?;
                },
                8 => {
                    self.JKMIMLBAJBL.push(is.read_uint32()?);
                },
                40 => {
                    self.HDMPBPOFFDK = is.read_enum_or_unknown()?;
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
        for value in &self.ALEHDAAOHOE {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for (k, v) in &self.NBCMAKNLPHG {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += 1 + 1;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        my_size += ::protobuf::rt::vec_packed_uint32_size(1, &self.JKMIMLBAJBL);
        if self.HDMPBPOFFDK != ::protobuf::EnumOrUnknown::new(super::ChessRogueNousDicePhase::ChessRogueNousDicePhase::NONE) {
            my_size += ::protobuf::rt::int32_size(5, self.HDMPBPOFFDK.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.ALEHDAAOHOE {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        for (k, v) in &self.NBCMAKNLPHG {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += 1 + 1;
            os.write_raw_varint32(18)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_bool(2, *v)?;
        };
        os.write_repeated_packed_uint32(1, &self.JKMIMLBAJBL)?;
        if self.HDMPBPOFFDK != ::protobuf::EnumOrUnknown::new(super::ChessRogueNousDicePhase::ChessRogueNousDicePhase::NONE) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.HDMPBPOFFDK))?;
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

    fn new() -> ChessRogueQueryDiceInfo {
        ChessRogueQueryDiceInfo::new()
    }

    fn clear(&mut self) {
        self.ALEHDAAOHOE.clear();
        self.NBCMAKNLPHG.clear();
        self.JKMIMLBAJBL.clear();
        self.HDMPBPOFFDK = ::protobuf::EnumOrUnknown::new(super::ChessRogueNousDicePhase::ChessRogueNousDicePhase::NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueQueryDiceInfo {
        static instance: ::protobuf::rt::Lazy<ChessRogueQueryDiceInfo> = ::protobuf::rt::Lazy::new();
        instance.get(ChessRogueQueryDiceInfo::new)
    }
}

impl ::protobuf::MessageFull for ChessRogueQueryDiceInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueQueryDiceInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueQueryDiceInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueQueryDiceInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dChessRogueQueryDiceInfo.proto\x1a\x14ChessRogueDice.proto\x1a\x1dC\
    hessRogueNousDicePhase.proto\"\xb7\x02\n\x17ChessRogueQueryDiceInfo\x121\
    \n\x0bALEHDAAOHOE\x18\n\x20\x03(\x0b2\x0f.ChessRogueDiceR\x0bALEHDAAOHOE\
    \x12K\n\x0bNBCMAKNLPHG\x18\x02\x20\x03(\x0b2).ChessRogueQueryDiceInfo.NB\
    CMAKNLPHGEntryR\x0bNBCMAKNLPHG\x12\x20\n\x0bJKMIMLBAJBL\x18\x01\x20\x03(\
    \rR\x0bJKMIMLBAJBL\x12:\n\x0bHDMPBPOFFDK\x18\x05\x20\x01(\x0e2\x18.Chess\
    RogueNousDicePhaseR\x0bHDMPBPOFFDK\x1a>\n\x10NBCMAKNLPHGEntry\x12\x10\n\
    \x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\
    \x08R\x05value:\x028\x01b\x06proto3\
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
            deps.push(super::ChessRogueDice::file_descriptor().clone());
            deps.push(super::ChessRogueNousDicePhase::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueQueryDiceInfo::generated_message_descriptor_data());
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
