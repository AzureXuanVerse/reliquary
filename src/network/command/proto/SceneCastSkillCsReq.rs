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

//! Generated file from `SceneCastSkillCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SceneCastSkillCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneCastSkillCsReq {
    // message fields
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.target_motion)
    pub target_motion: ::protobuf::MessageField<super::MotionInfo::MotionInfo>,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.assist_monster_entity_id_list)
    pub assist_monster_entity_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.caster_id)
    pub caster_id: u32,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.skill_index)
    pub skill_index: u32,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.hit_target_entity_id_list)
    pub hit_target_entity_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.HCHDHLJCIJE)
    pub HCHDHLJCIJE: u32,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.dynamic_values)
    pub dynamic_values: ::std::vec::Vec<super::JNHLELNABBD::JNHLELNABBD>,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.assist_monster_wave_list)
    pub assist_monster_wave_list: ::std::vec::Vec<super::AssistMonsterWave::AssistMonsterWave>,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.attacked_group_id)
    pub attacked_group_id: u32,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.ability_name)
    pub ability_name: ::std::string::String,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.casted_skill_type_list)
    pub casted_skill_type_list: ::std::vec::Vec<::protobuf::EnumOrUnknown<super::SceneCastSkillType::SceneCastSkillType>>,
    // special fields
    // @@protoc_insertion_point(special_field:SceneCastSkillCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneCastSkillCsReq {
    fn default() -> &'a SceneCastSkillCsReq {
        <SceneCastSkillCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SceneCastSkillCsReq {
    pub fn new() -> SceneCastSkillCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(11);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MotionInfo::MotionInfo>(
            "target_motion",
            |m: &SceneCastSkillCsReq| { &m.target_motion },
            |m: &mut SceneCastSkillCsReq| { &mut m.target_motion },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "assist_monster_entity_id_list",
            |m: &SceneCastSkillCsReq| { &m.assist_monster_entity_id_list },
            |m: &mut SceneCastSkillCsReq| { &mut m.assist_monster_entity_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "caster_id",
            |m: &SceneCastSkillCsReq| { &m.caster_id },
            |m: &mut SceneCastSkillCsReq| { &mut m.caster_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "skill_index",
            |m: &SceneCastSkillCsReq| { &m.skill_index },
            |m: &mut SceneCastSkillCsReq| { &mut m.skill_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "hit_target_entity_id_list",
            |m: &SceneCastSkillCsReq| { &m.hit_target_entity_id_list },
            |m: &mut SceneCastSkillCsReq| { &mut m.hit_target_entity_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HCHDHLJCIJE",
            |m: &SceneCastSkillCsReq| { &m.HCHDHLJCIJE },
            |m: &mut SceneCastSkillCsReq| { &mut m.HCHDHLJCIJE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "dynamic_values",
            |m: &SceneCastSkillCsReq| { &m.dynamic_values },
            |m: &mut SceneCastSkillCsReq| { &mut m.dynamic_values },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "assist_monster_wave_list",
            |m: &SceneCastSkillCsReq| { &m.assist_monster_wave_list },
            |m: &mut SceneCastSkillCsReq| { &mut m.assist_monster_wave_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "attacked_group_id",
            |m: &SceneCastSkillCsReq| { &m.attacked_group_id },
            |m: &mut SceneCastSkillCsReq| { &mut m.attacked_group_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ability_name",
            |m: &SceneCastSkillCsReq| { &m.ability_name },
            |m: &mut SceneCastSkillCsReq| { &mut m.ability_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "casted_skill_type_list",
            |m: &SceneCastSkillCsReq| { &m.casted_skill_type_list },
            |m: &mut SceneCastSkillCsReq| { &mut m.casted_skill_type_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneCastSkillCsReq>(
            "SceneCastSkillCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneCastSkillCsReq {
    const NAME: &'static str = "SceneCastSkillCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.target_motion)?;
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.assist_monster_entity_id_list)?;
                },
                96 => {
                    self.assist_monster_entity_id_list.push(is.read_uint32()?);
                },
                56 => {
                    self.caster_id = is.read_uint32()?;
                },
                112 => {
                    self.skill_index = is.read_uint32()?;
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.hit_target_entity_id_list)?;
                },
                64 => {
                    self.hit_target_entity_id_list.push(is.read_uint32()?);
                },
                88 => {
                    self.HCHDHLJCIJE = is.read_uint32()?;
                },
                18 => {
                    self.dynamic_values.push(is.read_message()?);
                },
                34 => {
                    self.assist_monster_wave_list.push(is.read_message()?);
                },
                48 => {
                    self.attacked_group_id = is.read_uint32()?;
                },
                10 => {
                    self.ability_name = is.read_string()?;
                },
                120 => {
                    self.casted_skill_type_list.push(is.read_enum_or_unknown()?);
                },
                122 => {
                    ::protobuf::rt::read_repeated_packed_enum_or_unknown_into(is, &mut self.casted_skill_type_list)?
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
        if let Some(v) = self.target_motion.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(12, &self.assist_monster_entity_id_list);
        if self.caster_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.caster_id);
        }
        if self.skill_index != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.skill_index);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(8, &self.hit_target_entity_id_list);
        if self.HCHDHLJCIJE != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.HCHDHLJCIJE);
        }
        for value in &self.dynamic_values {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.assist_monster_wave_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.attacked_group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.attacked_group_id);
        }
        if !self.ability_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.ability_name);
        }
        my_size += ::protobuf::rt::vec_packed_enum_or_unknown_size(15, &self.casted_skill_type_list);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.target_motion.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        os.write_repeated_packed_uint32(12, &self.assist_monster_entity_id_list)?;
        if self.caster_id != 0 {
            os.write_uint32(7, self.caster_id)?;
        }
        if self.skill_index != 0 {
            os.write_uint32(14, self.skill_index)?;
        }
        os.write_repeated_packed_uint32(8, &self.hit_target_entity_id_list)?;
        if self.HCHDHLJCIJE != 0 {
            os.write_uint32(11, self.HCHDHLJCIJE)?;
        }
        for v in &self.dynamic_values {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        for v in &self.assist_monster_wave_list {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.attacked_group_id != 0 {
            os.write_uint32(6, self.attacked_group_id)?;
        }
        if !self.ability_name.is_empty() {
            os.write_string(1, &self.ability_name)?;
        }
        os.write_repeated_packed_enum_or_unknown(15, &self.casted_skill_type_list)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SceneCastSkillCsReq {
        SceneCastSkillCsReq::new()
    }

    fn clear(&mut self) {
        self.target_motion.clear();
        self.assist_monster_entity_id_list.clear();
        self.caster_id = 0;
        self.skill_index = 0;
        self.hit_target_entity_id_list.clear();
        self.HCHDHLJCIJE = 0;
        self.dynamic_values.clear();
        self.assist_monster_wave_list.clear();
        self.attacked_group_id = 0;
        self.ability_name.clear();
        self.casted_skill_type_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneCastSkillCsReq {
        static instance: SceneCastSkillCsReq = SceneCastSkillCsReq {
            target_motion: ::protobuf::MessageField::none(),
            assist_monster_entity_id_list: ::std::vec::Vec::new(),
            caster_id: 0,
            skill_index: 0,
            hit_target_entity_id_list: ::std::vec::Vec::new(),
            HCHDHLJCIJE: 0,
            dynamic_values: ::std::vec::Vec::new(),
            assist_monster_wave_list: ::std::vec::Vec::new(),
            attacked_group_id: 0,
            ability_name: ::std::string::String::new(),
            casted_skill_type_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneCastSkillCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneCastSkillCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneCastSkillCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneCastSkillCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19SceneCastSkillCsReq.proto\x1a\x17AssistMonsterWave.proto\x1a\x11JN\
    HLELNABBD.proto\x1a\x10MotionInfo.proto\x1a\x18SceneCastSkillType.proto\
    \"\xbe\x04\n\x13SceneCastSkillCsReq\x120\n\rtarget_motion\x18\r\x20\x01(\
    \x0b2\x0b.MotionInfoR\x0ctargetMotion\x12@\n\x1dassist_monster_entity_id\
    _list\x18\x0c\x20\x03(\rR\x19assistMonsterEntityIdList\x12\x1b\n\tcaster\
    _id\x18\x07\x20\x01(\rR\x08casterId\x12\x1f\n\x0bskill_index\x18\x0e\x20\
    \x01(\rR\nskillIndex\x128\n\x19hit_target_entity_id_list\x18\x08\x20\x03\
    (\rR\x15hitTargetEntityIdList\x12\x20\n\x0bHCHDHLJCIJE\x18\x0b\x20\x01(\
    \rR\x0bHCHDHLJCIJE\x123\n\x0edynamic_values\x18\x02\x20\x03(\x0b2\x0c.JN\
    HLELNABBDR\rdynamicValues\x12K\n\x18assist_monster_wave_list\x18\x04\x20\
    \x03(\x0b2\x12.AssistMonsterWaveR\x15assistMonsterWaveList\x12*\n\x11att\
    acked_group_id\x18\x06\x20\x01(\rR\x0fattackedGroupId\x12!\n\x0cability_\
    name\x18\x01\x20\x01(\tR\x0babilityName\x12H\n\x16casted_skill_type_list\
    \x18\x0f\x20\x03(\x0e2\x13.SceneCastSkillTypeR\x13castedSkillTypeListb\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::AssistMonsterWave::file_descriptor().clone());
            deps.push(super::JNHLELNABBD::file_descriptor().clone());
            deps.push(super::MotionInfo::file_descriptor().clone());
            deps.push(super::SceneCastSkillType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneCastSkillCsReq::generated_message_descriptor_data());
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
