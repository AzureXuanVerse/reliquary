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

//! Generated file from `StartAetherDivideSceneBattleCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:StartAetherDivideSceneBattleCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartAetherDivideSceneBattleCsReq {
    // message fields
    // @@protoc_insertion_point(field:StartAetherDivideSceneBattleCsReq.assist_monster_entity_id_list)
    pub assist_monster_entity_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:StartAetherDivideSceneBattleCsReq.assist_monster_wave_list)
    pub assist_monster_wave_list: ::std::vec::Vec<super::AssistMonsterWave::AssistMonsterWave>,
    // @@protoc_insertion_point(field:StartAetherDivideSceneBattleCsReq.caster_id)
    pub caster_id: u32,
    // @@protoc_insertion_point(field:StartAetherDivideSceneBattleCsReq.skill_index)
    pub skill_index: u32,
    // @@protoc_insertion_point(field:StartAetherDivideSceneBattleCsReq.attacked_group_id)
    pub attacked_group_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:StartAetherDivideSceneBattleCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartAetherDivideSceneBattleCsReq {
    fn default() -> &'a StartAetherDivideSceneBattleCsReq {
        <StartAetherDivideSceneBattleCsReq as ::protobuf::Message>::default_instance()
    }
}

impl StartAetherDivideSceneBattleCsReq {
    pub fn new() -> StartAetherDivideSceneBattleCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "assist_monster_entity_id_list",
            |m: &StartAetherDivideSceneBattleCsReq| { &m.assist_monster_entity_id_list },
            |m: &mut StartAetherDivideSceneBattleCsReq| { &mut m.assist_monster_entity_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "assist_monster_wave_list",
            |m: &StartAetherDivideSceneBattleCsReq| { &m.assist_monster_wave_list },
            |m: &mut StartAetherDivideSceneBattleCsReq| { &mut m.assist_monster_wave_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "caster_id",
            |m: &StartAetherDivideSceneBattleCsReq| { &m.caster_id },
            |m: &mut StartAetherDivideSceneBattleCsReq| { &mut m.caster_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "skill_index",
            |m: &StartAetherDivideSceneBattleCsReq| { &m.skill_index },
            |m: &mut StartAetherDivideSceneBattleCsReq| { &mut m.skill_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "attacked_group_id",
            |m: &StartAetherDivideSceneBattleCsReq| { &m.attacked_group_id },
            |m: &mut StartAetherDivideSceneBattleCsReq| { &mut m.attacked_group_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartAetherDivideSceneBattleCsReq>(
            "StartAetherDivideSceneBattleCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartAetherDivideSceneBattleCsReq {
    const NAME: &'static str = "StartAetherDivideSceneBattleCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.assist_monster_entity_id_list)?;
                },
                8 => {
                    self.assist_monster_entity_id_list.push(is.read_uint32()?);
                },
                114 => {
                    self.assist_monster_wave_list.push(is.read_message()?);
                },
                32 => {
                    self.caster_id = is.read_uint32()?;
                },
                80 => {
                    self.skill_index = is.read_uint32()?;
                },
                72 => {
                    self.attacked_group_id = is.read_uint32()?;
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
        my_size += ::protobuf::rt::vec_packed_uint32_size(1, &self.assist_monster_entity_id_list);
        for value in &self.assist_monster_wave_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.caster_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.caster_id);
        }
        if self.skill_index != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.skill_index);
        }
        if self.attacked_group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.attacked_group_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_uint32(1, &self.assist_monster_entity_id_list)?;
        for v in &self.assist_monster_wave_list {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        if self.caster_id != 0 {
            os.write_uint32(4, self.caster_id)?;
        }
        if self.skill_index != 0 {
            os.write_uint32(10, self.skill_index)?;
        }
        if self.attacked_group_id != 0 {
            os.write_uint32(9, self.attacked_group_id)?;
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

    fn new() -> StartAetherDivideSceneBattleCsReq {
        StartAetherDivideSceneBattleCsReq::new()
    }

    fn clear(&mut self) {
        self.assist_monster_entity_id_list.clear();
        self.assist_monster_wave_list.clear();
        self.caster_id = 0;
        self.skill_index = 0;
        self.attacked_group_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartAetherDivideSceneBattleCsReq {
        static instance: StartAetherDivideSceneBattleCsReq = StartAetherDivideSceneBattleCsReq {
            assist_monster_entity_id_list: ::std::vec::Vec::new(),
            assist_monster_wave_list: ::std::vec::Vec::new(),
            caster_id: 0,
            skill_index: 0,
            attacked_group_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartAetherDivideSceneBattleCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartAetherDivideSceneBattleCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartAetherDivideSceneBattleCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartAetherDivideSceneBattleCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'StartAetherDivideSceneBattleCsReq.proto\x1a\x17AssistMonsterWave.prot\
    o\"\x9c\x02\n!StartAetherDivideSceneBattleCsReq\x12@\n\x1dassist_monster\
    _entity_id_list\x18\x01\x20\x03(\rR\x19assistMonsterEntityIdList\x12K\n\
    \x18assist_monster_wave_list\x18\x0e\x20\x03(\x0b2\x12.AssistMonsterWave\
    R\x15assistMonsterWaveList\x12\x1b\n\tcaster_id\x18\x04\x20\x01(\rR\x08c\
    asterId\x12\x1f\n\x0bskill_index\x18\n\x20\x01(\rR\nskillIndex\x12*\n\
    \x11attacked_group_id\x18\t\x20\x01(\rR\x0fattackedGroupIdb\x06proto3\
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
            deps.push(super::AssistMonsterWave::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(StartAetherDivideSceneBattleCsReq::generated_message_descriptor_data());
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
