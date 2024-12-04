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

//! Generated file from `RogueMagicBattleInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IGEFNGNCKOG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IGEFNGNCKOG {
    // message fields
    // @@protoc_insertion_point(field:IGEFNGNCKOG.OJIBOBNAIKH)
    pub OJIBOBNAIKH: u32,
    // special fields
    // @@protoc_insertion_point(special_field:IGEFNGNCKOG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IGEFNGNCKOG {
    fn default() -> &'a IGEFNGNCKOG {
        <IGEFNGNCKOG as ::protobuf::Message>::default_instance()
    }
}

impl IGEFNGNCKOG {
    pub fn new() -> IGEFNGNCKOG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OJIBOBNAIKH",
            |m: &IGEFNGNCKOG| { &m.OJIBOBNAIKH },
            |m: &mut IGEFNGNCKOG| { &mut m.OJIBOBNAIKH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IGEFNGNCKOG>(
            "IGEFNGNCKOG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IGEFNGNCKOG {
    const NAME: &'static str = "IGEFNGNCKOG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.OJIBOBNAIKH = is.read_uint32()?;
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
        if self.OJIBOBNAIKH != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.OJIBOBNAIKH);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OJIBOBNAIKH != 0 {
            os.write_uint32(1, self.OJIBOBNAIKH)?;
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

    fn new() -> IGEFNGNCKOG {
        IGEFNGNCKOG::new()
    }

    fn clear(&mut self) {
        self.OJIBOBNAIKH = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IGEFNGNCKOG {
        static instance: IGEFNGNCKOG = IGEFNGNCKOG {
            OJIBOBNAIKH: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IGEFNGNCKOG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IGEFNGNCKOG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IGEFNGNCKOG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IGEFNGNCKOG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:RogueMagicBattleInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueMagicBattleInfo {
    // message fields
    // @@protoc_insertion_point(field:RogueMagicBattleInfo.scepter)
    pub scepter: ::protobuf::MessageField<IGEFNGNCKOG>,
    // @@protoc_insertion_point(field:RogueMagicBattleInfo.player_detail_info)
    pub player_detail_info: ::protobuf::MessageField<super::RogueMagicBattleUnitInfo::RogueMagicBattleUnitInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueMagicBattleInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueMagicBattleInfo {
    fn default() -> &'a RogueMagicBattleInfo {
        <RogueMagicBattleInfo as ::protobuf::Message>::default_instance()
    }
}

impl RogueMagicBattleInfo {
    pub fn new() -> RogueMagicBattleInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, IGEFNGNCKOG>(
            "scepter",
            |m: &RogueMagicBattleInfo| { &m.scepter },
            |m: &mut RogueMagicBattleInfo| { &mut m.scepter },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueMagicBattleUnitInfo::RogueMagicBattleUnitInfo>(
            "player_detail_info",
            |m: &RogueMagicBattleInfo| { &m.player_detail_info },
            |m: &mut RogueMagicBattleInfo| { &mut m.player_detail_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueMagicBattleInfo>(
            "RogueMagicBattleInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueMagicBattleInfo {
    const NAME: &'static str = "RogueMagicBattleInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.scepter)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.player_detail_info)?;
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
        if let Some(v) = self.scepter.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.player_detail_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.scepter.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.player_detail_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> RogueMagicBattleInfo {
        RogueMagicBattleInfo::new()
    }

    fn clear(&mut self) {
        self.scepter.clear();
        self.player_detail_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueMagicBattleInfo {
        static instance: RogueMagicBattleInfo = RogueMagicBattleInfo {
            scepter: ::protobuf::MessageField::none(),
            player_detail_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueMagicBattleInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueMagicBattleInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueMagicBattleInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueMagicBattleInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aRogueMagicBattleInfo.proto\x1a\x1eRogueMagicBattleUnitInfo.proto\"\
    /\n\x0bIGEFNGNCKOG\x12\x20\n\x0bOJIBOBNAIKH\x18\x01\x20\x01(\rR\x0bOJIBO\
    BNAIKH\"\x87\x01\n\x14RogueMagicBattleInfo\x12&\n\x07scepter\x18\x01\x20\
    \x01(\x0b2\x0c.IGEFNGNCKOGR\x07scepter\x12G\n\x12player_detail_info\x18\
    \x02\x20\x01(\x0b2\x19.RogueMagicBattleUnitInfoR\x10playerDetailInfoB\
    \x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::RogueMagicBattleUnitInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(IGEFNGNCKOG::generated_message_descriptor_data());
            messages.push(RogueMagicBattleInfo::generated_message_descriptor_data());
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
