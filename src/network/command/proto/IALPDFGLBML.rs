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

//! Generated file from `IALPDFGLBML.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:IALPDFGLBML)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IALPDFGLBML {
    // message fields
    // @@protoc_insertion_point(field:IALPDFGLBML.version)
    pub version: u32,
    // @@protoc_insertion_point(field:IALPDFGLBML.logic_random_seed)
    pub logic_random_seed: u32,
    // @@protoc_insertion_point(field:IALPDFGLBML.stage_id)
    pub stage_id: u32,
    // @@protoc_insertion_point(field:IALPDFGLBML.lineup)
    pub lineup: ::protobuf::MessageField<super::DBDGPCBJEHG::DBDGPCBJEHG>,
    // @@protoc_insertion_point(field:IALPDFGLBML.op_list)
    pub op_list: ::std::vec::Vec<super::BattleOp::BattleOp>,
    // @@protoc_insertion_point(field:IALPDFGLBML.turn_snapshot_hash)
    pub turn_snapshot_hash: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:IALPDFGLBML.JONBHAEOFLI)
    pub JONBHAEOFLI: u32,
    // @@protoc_insertion_point(field:IALPDFGLBML.DJILKABPECA)
    pub DJILKABPECA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IALPDFGLBML.is_ai_consider_ultra_skill)
    pub is_ai_consider_ultra_skill: bool,
    // @@protoc_insertion_point(field:IALPDFGLBML.GPHAPAEHJFH)
    pub GPHAPAEHJFH: ::protobuf::EnumOrUnknown<super::CHEHBBADKBG::CHEHBBADKBG>,
    // @@protoc_insertion_point(field:IALPDFGLBML.BLPFEAANHDI)
    pub BLPFEAANHDI: ::protobuf::EnumOrUnknown<super::BattleModuleType::BattleModuleType>,
    // @@protoc_insertion_point(field:IALPDFGLBML.battle_event)
    pub battle_event: ::std::vec::Vec<super::BattleEventBattleInfo::BattleEventBattleInfo>,
    // @@protoc_insertion_point(field:IALPDFGLBML.rounds_limit)
    pub rounds_limit: u32,
    // @@protoc_insertion_point(field:IALPDFGLBML.config)
    pub config: ::protobuf::MessageField<super::JEAFNKHFPFB::JEAFNKHFPFB>,
    // @@protoc_insertion_point(field:IALPDFGLBML.game_core_log_encode)
    pub game_core_log_encode: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:IALPDFGLBML.client_version)
    pub client_version: u32,
    // @@protoc_insertion_point(field:IALPDFGLBML.monster_wave_length)
    pub monster_wave_length: u32,
    // @@protoc_insertion_point(field:IALPDFGLBML.GMPCFGEDHKI)
    pub GMPCFGEDHKI: ::protobuf::MessageField<super::PNDFMBJFGIM::PNDFMBJFGIM>,
    // @@protoc_insertion_point(field:IALPDFGLBML.BNMIIAHADJH)
    pub BNMIIAHADJH: ::protobuf::MessageField<super::MIAIDAILDKM::MIAIDAILDKM>,
    // special fields
    // @@protoc_insertion_point(special_field:IALPDFGLBML.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IALPDFGLBML {
    fn default() -> &'a IALPDFGLBML {
        <IALPDFGLBML as ::protobuf::Message>::default_instance()
    }
}

impl IALPDFGLBML {
    pub fn new() -> IALPDFGLBML {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(19);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "version",
            |m: &IALPDFGLBML| { &m.version },
            |m: &mut IALPDFGLBML| { &mut m.version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "logic_random_seed",
            |m: &IALPDFGLBML| { &m.logic_random_seed },
            |m: &mut IALPDFGLBML| { &mut m.logic_random_seed },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stage_id",
            |m: &IALPDFGLBML| { &m.stage_id },
            |m: &mut IALPDFGLBML| { &mut m.stage_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DBDGPCBJEHG::DBDGPCBJEHG>(
            "lineup",
            |m: &IALPDFGLBML| { &m.lineup },
            |m: &mut IALPDFGLBML| { &mut m.lineup },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "op_list",
            |m: &IALPDFGLBML| { &m.op_list },
            |m: &mut IALPDFGLBML| { &mut m.op_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "turn_snapshot_hash",
            |m: &IALPDFGLBML| { &m.turn_snapshot_hash },
            |m: &mut IALPDFGLBML| { &mut m.turn_snapshot_hash },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JONBHAEOFLI",
            |m: &IALPDFGLBML| { &m.JONBHAEOFLI },
            |m: &mut IALPDFGLBML| { &mut m.JONBHAEOFLI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DJILKABPECA",
            |m: &IALPDFGLBML| { &m.DJILKABPECA },
            |m: &mut IALPDFGLBML| { &mut m.DJILKABPECA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_ai_consider_ultra_skill",
            |m: &IALPDFGLBML| { &m.is_ai_consider_ultra_skill },
            |m: &mut IALPDFGLBML| { &mut m.is_ai_consider_ultra_skill },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GPHAPAEHJFH",
            |m: &IALPDFGLBML| { &m.GPHAPAEHJFH },
            |m: &mut IALPDFGLBML| { &mut m.GPHAPAEHJFH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BLPFEAANHDI",
            |m: &IALPDFGLBML| { &m.BLPFEAANHDI },
            |m: &mut IALPDFGLBML| { &mut m.BLPFEAANHDI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "battle_event",
            |m: &IALPDFGLBML| { &m.battle_event },
            |m: &mut IALPDFGLBML| { &mut m.battle_event },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rounds_limit",
            |m: &IALPDFGLBML| { &m.rounds_limit },
            |m: &mut IALPDFGLBML| { &mut m.rounds_limit },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JEAFNKHFPFB::JEAFNKHFPFB>(
            "config",
            |m: &IALPDFGLBML| { &m.config },
            |m: &mut IALPDFGLBML| { &mut m.config },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "game_core_log_encode",
            |m: &IALPDFGLBML| { &m.game_core_log_encode },
            |m: &mut IALPDFGLBML| { &mut m.game_core_log_encode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "client_version",
            |m: &IALPDFGLBML| { &m.client_version },
            |m: &mut IALPDFGLBML| { &mut m.client_version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "monster_wave_length",
            |m: &IALPDFGLBML| { &m.monster_wave_length },
            |m: &mut IALPDFGLBML| { &mut m.monster_wave_length },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PNDFMBJFGIM::PNDFMBJFGIM>(
            "GMPCFGEDHKI",
            |m: &IALPDFGLBML| { &m.GMPCFGEDHKI },
            |m: &mut IALPDFGLBML| { &mut m.GMPCFGEDHKI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MIAIDAILDKM::MIAIDAILDKM>(
            "BNMIIAHADJH",
            |m: &IALPDFGLBML| { &m.BNMIIAHADJH },
            |m: &mut IALPDFGLBML| { &mut m.BNMIIAHADJH },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IALPDFGLBML>(
            "IALPDFGLBML",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IALPDFGLBML {
    const NAME: &'static str = "IALPDFGLBML";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.version = is.read_uint32()?;
                },
                16 => {
                    self.logic_random_seed = is.read_uint32()?;
                },
                24 => {
                    self.stage_id = is.read_uint32()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.lineup)?;
                },
                42 => {
                    self.op_list.push(is.read_message()?);
                },
                50 => {
                    self.turn_snapshot_hash = is.read_bytes()?;
                },
                56 => {
                    self.JONBHAEOFLI = is.read_uint32()?;
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.DJILKABPECA)?;
                },
                64 => {
                    self.DJILKABPECA.push(is.read_uint32()?);
                },
                72 => {
                    self.is_ai_consider_ultra_skill = is.read_bool()?;
                },
                80 => {
                    self.GPHAPAEHJFH = is.read_enum_or_unknown()?;
                },
                88 => {
                    self.BLPFEAANHDI = is.read_enum_or_unknown()?;
                },
                98 => {
                    self.battle_event.push(is.read_message()?);
                },
                112 => {
                    self.rounds_limit = is.read_uint32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.config)?;
                },
                130 => {
                    self.game_core_log_encode = is.read_bytes()?;
                },
                136 => {
                    self.client_version = is.read_uint32()?;
                },
                144 => {
                    self.monster_wave_length = is.read_uint32()?;
                },
                154 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GMPCFGEDHKI)?;
                },
                802 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BNMIIAHADJH)?;
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
        if self.version != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.version);
        }
        if self.logic_random_seed != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.logic_random_seed);
        }
        if self.stage_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.stage_id);
        }
        if let Some(v) = self.lineup.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.op_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if !self.turn_snapshot_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.turn_snapshot_hash);
        }
        if self.JONBHAEOFLI != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.JONBHAEOFLI);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(8, &self.DJILKABPECA);
        if self.is_ai_consider_ultra_skill != false {
            my_size += 1 + 1;
        }
        if self.GPHAPAEHJFH != ::protobuf::EnumOrUnknown::new(super::CHEHBBADKBG::CHEHBBADKBG::BATTLE_CHECK_STRATEGY_IDENTICAL) {
            my_size += ::protobuf::rt::int32_size(10, self.GPHAPAEHJFH.value());
        }
        if self.BLPFEAANHDI != ::protobuf::EnumOrUnknown::new(super::BattleModuleType::BattleModuleType::BATTLE_MODULE_MAZE) {
            my_size += ::protobuf::rt::int32_size(11, self.BLPFEAANHDI.value());
        }
        for value in &self.battle_event {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.rounds_limit != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.rounds_limit);
        }
        if let Some(v) = self.config.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.game_core_log_encode.is_empty() {
            my_size += ::protobuf::rt::bytes_size(16, &self.game_core_log_encode);
        }
        if self.client_version != 0 {
            my_size += ::protobuf::rt::uint32_size(17, self.client_version);
        }
        if self.monster_wave_length != 0 {
            my_size += ::protobuf::rt::uint32_size(18, self.monster_wave_length);
        }
        if let Some(v) = self.GMPCFGEDHKI.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.BNMIIAHADJH.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.version != 0 {
            os.write_uint32(1, self.version)?;
        }
        if self.logic_random_seed != 0 {
            os.write_uint32(2, self.logic_random_seed)?;
        }
        if self.stage_id != 0 {
            os.write_uint32(3, self.stage_id)?;
        }
        if let Some(v) = self.lineup.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        for v in &self.op_list {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if !self.turn_snapshot_hash.is_empty() {
            os.write_bytes(6, &self.turn_snapshot_hash)?;
        }
        if self.JONBHAEOFLI != 0 {
            os.write_uint32(7, self.JONBHAEOFLI)?;
        }
        os.write_repeated_packed_uint32(8, &self.DJILKABPECA)?;
        if self.is_ai_consider_ultra_skill != false {
            os.write_bool(9, self.is_ai_consider_ultra_skill)?;
        }
        if self.GPHAPAEHJFH != ::protobuf::EnumOrUnknown::new(super::CHEHBBADKBG::CHEHBBADKBG::BATTLE_CHECK_STRATEGY_IDENTICAL) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.GPHAPAEHJFH))?;
        }
        if self.BLPFEAANHDI != ::protobuf::EnumOrUnknown::new(super::BattleModuleType::BattleModuleType::BATTLE_MODULE_MAZE) {
            os.write_enum(11, ::protobuf::EnumOrUnknown::value(&self.BLPFEAANHDI))?;
        }
        for v in &self.battle_event {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        if self.rounds_limit != 0 {
            os.write_uint32(14, self.rounds_limit)?;
        }
        if let Some(v) = self.config.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if !self.game_core_log_encode.is_empty() {
            os.write_bytes(16, &self.game_core_log_encode)?;
        }
        if self.client_version != 0 {
            os.write_uint32(17, self.client_version)?;
        }
        if self.monster_wave_length != 0 {
            os.write_uint32(18, self.monster_wave_length)?;
        }
        if let Some(v) = self.GMPCFGEDHKI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(19, v, os)?;
        }
        if let Some(v) = self.BNMIIAHADJH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(100, v, os)?;
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

    fn new() -> IALPDFGLBML {
        IALPDFGLBML::new()
    }

    fn clear(&mut self) {
        self.version = 0;
        self.logic_random_seed = 0;
        self.stage_id = 0;
        self.lineup.clear();
        self.op_list.clear();
        self.turn_snapshot_hash.clear();
        self.JONBHAEOFLI = 0;
        self.DJILKABPECA.clear();
        self.is_ai_consider_ultra_skill = false;
        self.GPHAPAEHJFH = ::protobuf::EnumOrUnknown::new(super::CHEHBBADKBG::CHEHBBADKBG::BATTLE_CHECK_STRATEGY_IDENTICAL);
        self.BLPFEAANHDI = ::protobuf::EnumOrUnknown::new(super::BattleModuleType::BattleModuleType::BATTLE_MODULE_MAZE);
        self.battle_event.clear();
        self.rounds_limit = 0;
        self.config.clear();
        self.game_core_log_encode.clear();
        self.client_version = 0;
        self.monster_wave_length = 0;
        self.GMPCFGEDHKI.clear();
        self.BNMIIAHADJH.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IALPDFGLBML {
        static instance: IALPDFGLBML = IALPDFGLBML {
            version: 0,
            logic_random_seed: 0,
            stage_id: 0,
            lineup: ::protobuf::MessageField::none(),
            op_list: ::std::vec::Vec::new(),
            turn_snapshot_hash: ::std::vec::Vec::new(),
            JONBHAEOFLI: 0,
            DJILKABPECA: ::std::vec::Vec::new(),
            is_ai_consider_ultra_skill: false,
            GPHAPAEHJFH: ::protobuf::EnumOrUnknown::from_i32(0),
            BLPFEAANHDI: ::protobuf::EnumOrUnknown::from_i32(0),
            battle_event: ::std::vec::Vec::new(),
            rounds_limit: 0,
            config: ::protobuf::MessageField::none(),
            game_core_log_encode: ::std::vec::Vec::new(),
            client_version: 0,
            monster_wave_length: 0,
            GMPCFGEDHKI: ::protobuf::MessageField::none(),
            BNMIIAHADJH: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IALPDFGLBML {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IALPDFGLBML").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IALPDFGLBML {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IALPDFGLBML {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IALPDFGLBML.proto\x1a\x1bBattleEventBattleInfo.proto\x1a\x16Battle\
    ModuleType.proto\x1a\x0eBattleOp.proto\x1a\x11CHEHBBADKBG.proto\x1a\x11D\
    BDGPCBJEHG.proto\x1a\x11JEAFNKHFPFB.proto\x1a\x11MIAIDAILDKM.proto\x1a\
    \x11PNDFMBJFGIM.proto\"\xb7\x06\n\x0bIALPDFGLBML\x12\x18\n\x07version\
    \x18\x01\x20\x01(\rR\x07version\x12*\n\x11logic_random_seed\x18\x02\x20\
    \x01(\rR\x0flogicRandomSeed\x12\x19\n\x08stage_id\x18\x03\x20\x01(\rR\
    \x07stageId\x12$\n\x06lineup\x18\x04\x20\x01(\x0b2\x0c.DBDGPCBJEHGR\x06l\
    ineup\x12\"\n\x07op_list\x18\x05\x20\x03(\x0b2\t.BattleOpR\x06opList\x12\
    ,\n\x12turn_snapshot_hash\x18\x06\x20\x01(\x0cR\x10turnSnapshotHash\x12\
    \x20\n\x0bJONBHAEOFLI\x18\x07\x20\x01(\rR\x0bJONBHAEOFLI\x12\x20\n\x0bDJ\
    ILKABPECA\x18\x08\x20\x03(\rR\x0bDJILKABPECA\x12:\n\x1ais_ai_consider_ul\
    tra_skill\x18\t\x20\x01(\x08R\x16isAiConsiderUltraSkill\x12.\n\x0bGPHAPA\
    EHJFH\x18\n\x20\x01(\x0e2\x0c.CHEHBBADKBGR\x0bGPHAPAEHJFH\x123\n\x0bBLPF\
    EAANHDI\x18\x0b\x20\x01(\x0e2\x11.BattleModuleTypeR\x0bBLPFEAANHDI\x129\
    \n\x0cbattle_event\x18\x0c\x20\x03(\x0b2\x16.BattleEventBattleInfoR\x0bb\
    attleEvent\x12!\n\x0crounds_limit\x18\x0e\x20\x01(\rR\x0broundsLimit\x12\
    $\n\x06config\x18\x0f\x20\x01(\x0b2\x0c.JEAFNKHFPFBR\x06config\x12/\n\
    \x14game_core_log_encode\x18\x10\x20\x01(\x0cR\x11gameCoreLogEncode\x12%\
    \n\x0eclient_version\x18\x11\x20\x01(\rR\rclientVersion\x12.\n\x13monste\
    r_wave_length\x18\x12\x20\x01(\rR\x11monsterWaveLength\x12.\n\x0bGMPCFGE\
    DHKI\x18\x13\x20\x01(\x0b2\x0c.PNDFMBJFGIMR\x0bGMPCFGEDHKI\x12.\n\x0bBNM\
    IIAHADJH\x18d\x20\x01(\x0b2\x0c.MIAIDAILDKMR\x0bBNMIIAHADJHb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(8);
            deps.push(super::BattleEventBattleInfo::file_descriptor().clone());
            deps.push(super::BattleModuleType::file_descriptor().clone());
            deps.push(super::BattleOp::file_descriptor().clone());
            deps.push(super::CHEHBBADKBG::file_descriptor().clone());
            deps.push(super::DBDGPCBJEHG::file_descriptor().clone());
            deps.push(super::JEAFNKHFPFB::file_descriptor().clone());
            deps.push(super::MIAIDAILDKM::file_descriptor().clone());
            deps.push(super::PNDFMBJFGIM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IALPDFGLBML::generated_message_descriptor_data());
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
