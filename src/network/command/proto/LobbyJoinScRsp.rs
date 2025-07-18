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

//! Generated file from `LobbyJoinScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:LobbyJoinScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LobbyJoinScRsp {
    // message fields
    // @@protoc_insertion_point(field:LobbyJoinScRsp.NEPODDOJJFE)
    pub NEPODDOJJFE: u32,
    // @@protoc_insertion_point(field:LobbyJoinScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:LobbyJoinScRsp.LIPJDJPMOKB)
    pub LIPJDJPMOKB: ::std::vec::Vec<super::CBBDIOMIFHD::CBBDIOMIFHD>,
    // @@protoc_insertion_point(field:LobbyJoinScRsp.NBDLPGBIDLC)
    pub NBDLPGBIDLC: ::protobuf::EnumOrUnknown<super::FightGameMode::FightGameMode>,
    // @@protoc_insertion_point(field:LobbyJoinScRsp.room_id)
    pub room_id: u64,
    // special fields
    // @@protoc_insertion_point(special_field:LobbyJoinScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LobbyJoinScRsp {
    fn default() -> &'a LobbyJoinScRsp {
        <LobbyJoinScRsp as ::protobuf::Message>::default_instance()
    }
}

impl LobbyJoinScRsp {
    pub fn new() -> LobbyJoinScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NEPODDOJJFE",
            |m: &LobbyJoinScRsp| { &m.NEPODDOJJFE },
            |m: &mut LobbyJoinScRsp| { &mut m.NEPODDOJJFE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &LobbyJoinScRsp| { &m.retcode },
            |m: &mut LobbyJoinScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LIPJDJPMOKB",
            |m: &LobbyJoinScRsp| { &m.LIPJDJPMOKB },
            |m: &mut LobbyJoinScRsp| { &mut m.LIPJDJPMOKB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NBDLPGBIDLC",
            |m: &LobbyJoinScRsp| { &m.NBDLPGBIDLC },
            |m: &mut LobbyJoinScRsp| { &mut m.NBDLPGBIDLC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "room_id",
            |m: &LobbyJoinScRsp| { &m.room_id },
            |m: &mut LobbyJoinScRsp| { &mut m.room_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LobbyJoinScRsp>(
            "LobbyJoinScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LobbyJoinScRsp {
    const NAME: &'static str = "LobbyJoinScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.NEPODDOJJFE = is.read_uint32()?;
                },
                32 => {
                    self.retcode = is.read_uint32()?;
                },
                90 => {
                    self.LIPJDJPMOKB.push(is.read_message()?);
                },
                24 => {
                    self.NBDLPGBIDLC = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.room_id = is.read_uint64()?;
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
        if self.NEPODDOJJFE != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.NEPODDOJJFE);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.retcode);
        }
        for value in &self.LIPJDJPMOKB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.NBDLPGBIDLC != ::protobuf::EnumOrUnknown::new(super::FightGameMode::FightGameMode::FIGHT_GAME_MODE_NONE) {
            my_size += ::protobuf::rt::int32_size(3, self.NBDLPGBIDLC.value());
        }
        if self.room_id != 0 {
            my_size += ::protobuf::rt::uint64_size(2, self.room_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NEPODDOJJFE != 0 {
            os.write_uint32(7, self.NEPODDOJJFE)?;
        }
        if self.retcode != 0 {
            os.write_uint32(4, self.retcode)?;
        }
        for v in &self.LIPJDJPMOKB {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if self.NBDLPGBIDLC != ::protobuf::EnumOrUnknown::new(super::FightGameMode::FightGameMode::FIGHT_GAME_MODE_NONE) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.NBDLPGBIDLC))?;
        }
        if self.room_id != 0 {
            os.write_uint64(2, self.room_id)?;
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

    fn new() -> LobbyJoinScRsp {
        LobbyJoinScRsp::new()
    }

    fn clear(&mut self) {
        self.NEPODDOJJFE = 0;
        self.retcode = 0;
        self.LIPJDJPMOKB.clear();
        self.NBDLPGBIDLC = ::protobuf::EnumOrUnknown::new(super::FightGameMode::FightGameMode::FIGHT_GAME_MODE_NONE);
        self.room_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LobbyJoinScRsp {
        static instance: LobbyJoinScRsp = LobbyJoinScRsp {
            NEPODDOJJFE: 0,
            retcode: 0,
            LIPJDJPMOKB: ::std::vec::Vec::new(),
            NBDLPGBIDLC: ::protobuf::EnumOrUnknown::from_i32(0),
            room_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LobbyJoinScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LobbyJoinScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LobbyJoinScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LobbyJoinScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14LobbyJoinScRsp.proto\x1a\x11CBBDIOMIFHD.proto\x1a\x13FightGameMode\
    .proto\"\xc7\x01\n\x0eLobbyJoinScRsp\x12\x20\n\x0bNEPODDOJJFE\x18\x07\
    \x20\x01(\rR\x0bNEPODDOJJFE\x12\x18\n\x07retcode\x18\x04\x20\x01(\rR\x07\
    retcode\x12.\n\x0bLIPJDJPMOKB\x18\x0b\x20\x03(\x0b2\x0c.CBBDIOMIFHDR\x0b\
    LIPJDJPMOKB\x120\n\x0bNBDLPGBIDLC\x18\x03\x20\x01(\x0e2\x0e.FightGameMod\
    eR\x0bNBDLPGBIDLC\x12\x17\n\x07room_id\x18\x02\x20\x01(\x04R\x06roomIdb\
    \x06proto3\
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
            deps.push(super::CBBDIOMIFHD::file_descriptor().clone());
            deps.push(super::FightGameMode::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LobbyJoinScRsp::generated_message_descriptor_data());
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
