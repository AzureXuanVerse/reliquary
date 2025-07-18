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

//! Generated file from `FightFestUpdateChallengeRecordNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:FightFestUpdateChallengeRecordNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FightFestUpdateChallengeRecordNotify {
    // message fields
    // @@protoc_insertion_point(field:FightFestUpdateChallengeRecordNotify.HOEHIOBIIEJ)
    pub HOEHIOBIIEJ: u32,
    // @@protoc_insertion_point(field:FightFestUpdateChallengeRecordNotify.FKPEPBMJHKN)
    pub FKPEPBMJHKN: ::protobuf::EnumOrUnknown<super::HGDAPJPKFFB::HGDAPJPKFFB>,
    // @@protoc_insertion_point(field:FightFestUpdateChallengeRecordNotify.JBOLAAFDKAN)
    pub JBOLAAFDKAN: u32,
    // @@protoc_insertion_point(field:FightFestUpdateChallengeRecordNotify.rank)
    pub rank: ::protobuf::EnumOrUnknown<super::HGDAPJPKFFB::HGDAPJPKFFB>,
    // @@protoc_insertion_point(field:FightFestUpdateChallengeRecordNotify.challenge_id)
    pub challenge_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FightFestUpdateChallengeRecordNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FightFestUpdateChallengeRecordNotify {
    fn default() -> &'a FightFestUpdateChallengeRecordNotify {
        <FightFestUpdateChallengeRecordNotify as ::protobuf::Message>::default_instance()
    }
}

impl FightFestUpdateChallengeRecordNotify {
    pub fn new() -> FightFestUpdateChallengeRecordNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HOEHIOBIIEJ",
            |m: &FightFestUpdateChallengeRecordNotify| { &m.HOEHIOBIIEJ },
            |m: &mut FightFestUpdateChallengeRecordNotify| { &mut m.HOEHIOBIIEJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FKPEPBMJHKN",
            |m: &FightFestUpdateChallengeRecordNotify| { &m.FKPEPBMJHKN },
            |m: &mut FightFestUpdateChallengeRecordNotify| { &mut m.FKPEPBMJHKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JBOLAAFDKAN",
            |m: &FightFestUpdateChallengeRecordNotify| { &m.JBOLAAFDKAN },
            |m: &mut FightFestUpdateChallengeRecordNotify| { &mut m.JBOLAAFDKAN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rank",
            |m: &FightFestUpdateChallengeRecordNotify| { &m.rank },
            |m: &mut FightFestUpdateChallengeRecordNotify| { &mut m.rank },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "challenge_id",
            |m: &FightFestUpdateChallengeRecordNotify| { &m.challenge_id },
            |m: &mut FightFestUpdateChallengeRecordNotify| { &mut m.challenge_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FightFestUpdateChallengeRecordNotify>(
            "FightFestUpdateChallengeRecordNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FightFestUpdateChallengeRecordNotify {
    const NAME: &'static str = "FightFestUpdateChallengeRecordNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.HOEHIOBIIEJ = is.read_uint32()?;
                },
                72 => {
                    self.FKPEPBMJHKN = is.read_enum_or_unknown()?;
                },
                48 => {
                    self.JBOLAAFDKAN = is.read_uint32()?;
                },
                112 => {
                    self.rank = is.read_enum_or_unknown()?;
                },
                40 => {
                    self.challenge_id = is.read_uint32()?;
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
        if self.HOEHIOBIIEJ != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.HOEHIOBIIEJ);
        }
        if self.FKPEPBMJHKN != ::protobuf::EnumOrUnknown::new(super::HGDAPJPKFFB::HGDAPJPKFFB::FIGHT_FEST_BATTLE_RANK_C) {
            my_size += ::protobuf::rt::int32_size(9, self.FKPEPBMJHKN.value());
        }
        if self.JBOLAAFDKAN != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.JBOLAAFDKAN);
        }
        if self.rank != ::protobuf::EnumOrUnknown::new(super::HGDAPJPKFFB::HGDAPJPKFFB::FIGHT_FEST_BATTLE_RANK_C) {
            my_size += ::protobuf::rt::int32_size(14, self.rank.value());
        }
        if self.challenge_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.challenge_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HOEHIOBIIEJ != 0 {
            os.write_uint32(3, self.HOEHIOBIIEJ)?;
        }
        if self.FKPEPBMJHKN != ::protobuf::EnumOrUnknown::new(super::HGDAPJPKFFB::HGDAPJPKFFB::FIGHT_FEST_BATTLE_RANK_C) {
            os.write_enum(9, ::protobuf::EnumOrUnknown::value(&self.FKPEPBMJHKN))?;
        }
        if self.JBOLAAFDKAN != 0 {
            os.write_uint32(6, self.JBOLAAFDKAN)?;
        }
        if self.rank != ::protobuf::EnumOrUnknown::new(super::HGDAPJPKFFB::HGDAPJPKFFB::FIGHT_FEST_BATTLE_RANK_C) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.rank))?;
        }
        if self.challenge_id != 0 {
            os.write_uint32(5, self.challenge_id)?;
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

    fn new() -> FightFestUpdateChallengeRecordNotify {
        FightFestUpdateChallengeRecordNotify::new()
    }

    fn clear(&mut self) {
        self.HOEHIOBIIEJ = 0;
        self.FKPEPBMJHKN = ::protobuf::EnumOrUnknown::new(super::HGDAPJPKFFB::HGDAPJPKFFB::FIGHT_FEST_BATTLE_RANK_C);
        self.JBOLAAFDKAN = 0;
        self.rank = ::protobuf::EnumOrUnknown::new(super::HGDAPJPKFFB::HGDAPJPKFFB::FIGHT_FEST_BATTLE_RANK_C);
        self.challenge_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FightFestUpdateChallengeRecordNotify {
        static instance: FightFestUpdateChallengeRecordNotify = FightFestUpdateChallengeRecordNotify {
            HOEHIOBIIEJ: 0,
            FKPEPBMJHKN: ::protobuf::EnumOrUnknown::from_i32(0),
            JBOLAAFDKAN: 0,
            rank: ::protobuf::EnumOrUnknown::from_i32(0),
            challenge_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FightFestUpdateChallengeRecordNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FightFestUpdateChallengeRecordNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FightFestUpdateChallengeRecordNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FightFestUpdateChallengeRecordNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n*FightFestUpdateChallengeRecordNotify.proto\x1a\x11HGDAPJPKFFB.proto\"\
    \xdf\x01\n$FightFestUpdateChallengeRecordNotify\x12\x20\n\x0bHOEHIOBIIEJ\
    \x18\x03\x20\x01(\rR\x0bHOEHIOBIIEJ\x12.\n\x0bFKPEPBMJHKN\x18\t\x20\x01(\
    \x0e2\x0c.HGDAPJPKFFBR\x0bFKPEPBMJHKN\x12\x20\n\x0bJBOLAAFDKAN\x18\x06\
    \x20\x01(\rR\x0bJBOLAAFDKAN\x12\x20\n\x04rank\x18\x0e\x20\x01(\x0e2\x0c.\
    HGDAPJPKFFBR\x04rank\x12!\n\x0cchallenge_id\x18\x05\x20\x01(\rR\x0bchall\
    engeIdb\x06proto3\
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
            deps.push(super::HGDAPJPKFFB::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FightFestUpdateChallengeRecordNotify::generated_message_descriptor_data());
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
