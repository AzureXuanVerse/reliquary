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

//! Generated file from `BattleMonsterWave.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:BattleMonsterWave)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BattleMonsterWave {
    // message fields
    // @@protoc_insertion_point(field:BattleMonsterWave.monster_list)
    pub monster_list: ::std::vec::Vec<super::BattleMonster::BattleMonster>,
    // @@protoc_insertion_point(field:BattleMonsterWave.monster_param)
    pub monster_param: ::protobuf::MessageField<super::BattleMonsterParam::BattleMonsterParam>,
    // @@protoc_insertion_point(field:BattleMonsterWave.battle_stage_id)
    pub battle_stage_id: u32,
    // @@protoc_insertion_point(field:BattleMonsterWave.battle_wave_id)
    pub battle_wave_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:BattleMonsterWave.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BattleMonsterWave {
    fn default() -> &'a BattleMonsterWave {
        <BattleMonsterWave as ::protobuf::Message>::default_instance()
    }
}

impl BattleMonsterWave {
    pub fn new() -> BattleMonsterWave {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "monster_list",
            |m: &BattleMonsterWave| { &m.monster_list },
            |m: &mut BattleMonsterWave| { &mut m.monster_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BattleMonsterParam::BattleMonsterParam>(
            "monster_param",
            |m: &BattleMonsterWave| { &m.monster_param },
            |m: &mut BattleMonsterWave| { &mut m.monster_param },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "battle_stage_id",
            |m: &BattleMonsterWave| { &m.battle_stage_id },
            |m: &mut BattleMonsterWave| { &mut m.battle_stage_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "battle_wave_id",
            |m: &BattleMonsterWave| { &m.battle_wave_id },
            |m: &mut BattleMonsterWave| { &mut m.battle_wave_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BattleMonsterWave>(
            "BattleMonsterWave",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BattleMonsterWave {
    const NAME: &'static str = "BattleMonsterWave";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.monster_list.push(is.read_message()?);
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.monster_param)?;
                },
                24 => {
                    self.battle_stage_id = is.read_uint32()?;
                },
                32 => {
                    self.battle_wave_id = is.read_uint32()?;
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
        for value in &self.monster_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.monster_param.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.battle_stage_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.battle_stage_id);
        }
        if self.battle_wave_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.battle_wave_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.monster_list {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if let Some(v) = self.monster_param.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.battle_stage_id != 0 {
            os.write_uint32(3, self.battle_stage_id)?;
        }
        if self.battle_wave_id != 0 {
            os.write_uint32(4, self.battle_wave_id)?;
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

    fn new() -> BattleMonsterWave {
        BattleMonsterWave::new()
    }

    fn clear(&mut self) {
        self.monster_list.clear();
        self.monster_param.clear();
        self.battle_stage_id = 0;
        self.battle_wave_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BattleMonsterWave {
        static instance: BattleMonsterWave = BattleMonsterWave {
            monster_list: ::std::vec::Vec::new(),
            monster_param: ::protobuf::MessageField::none(),
            battle_stage_id: 0,
            battle_wave_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BattleMonsterWave {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BattleMonsterWave").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BattleMonsterWave {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BattleMonsterWave {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17BattleMonsterWave.proto\x1a\x13BattleMonster.proto\x1a\x18BattleMo\
    nsterParam.proto\"\xce\x01\n\x11BattleMonsterWave\x121\n\x0cmonster_list\
    \x18\x01\x20\x03(\x0b2\x0e.BattleMonsterR\x0bmonsterList\x128\n\rmonster\
    _param\x18\x02\x20\x01(\x0b2\x13.BattleMonsterParamR\x0cmonsterParam\x12\
    &\n\x0fbattle_stage_id\x18\x03\x20\x01(\rR\rbattleStageId\x12$\n\x0ebatt\
    le_wave_id\x18\x04\x20\x01(\rR\x0cbattleWaveIdb\x06proto3\
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
            deps.push(super::BattleMonster::file_descriptor().clone());
            deps.push(super::BattleMonsterParam::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BattleMonsterWave::generated_message_descriptor_data());
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
