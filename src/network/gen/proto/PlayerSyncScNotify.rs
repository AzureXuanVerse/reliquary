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

//! Generated file from `PlayerSyncScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlayerSyncScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerSyncScNotify {
    // message fields
    // @@protoc_insertion_point(field:PlayerSyncScNotify.material_list)
    pub material_list: ::std::vec::Vec<super::Material::Material>,
    // @@protoc_insertion_point(field:PlayerSyncScNotify.del_relic_list)
    pub del_relic_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PlayerSyncScNotify.total_achievement_exp)
    pub total_achievement_exp: u32,
    // @@protoc_insertion_point(field:PlayerSyncScNotify.LLMKCDEDIFP)
    pub LLMKCDEDIFP: ::std::vec::Vec<super::Material::Material>,
    // @@protoc_insertion_point(field:PlayerSyncScNotify.basic_info)
    pub basic_info: ::protobuf::MessageField<super::PlayerBasicInfo::PlayerBasicInfo>,
    // @@protoc_insertion_point(field:PlayerSyncScNotify.unk)
    pub unk: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PlayerSyncScNotify.equipment_list)
    pub equipment_list: ::std::vec::Vec<super::Equipment::Equipment>,
    // @@protoc_insertion_point(field:PlayerSyncScNotify.avatar_sync)
    pub avatar_sync: ::protobuf::MessageField<super::AvatarSync::AvatarSync>,
    // @@protoc_insertion_point(field:PlayerSyncScNotify.EDFCAFLFLFH)
    pub EDFCAFLFLFH: ::protobuf::MessageField<super::MissionSync::MissionSync>,
    // @@protoc_insertion_point(field:PlayerSyncScNotify.board_data_sync)
    pub board_data_sync: ::protobuf::MessageField<super::BoardDataSync::BoardDataSync>,
    // @@protoc_insertion_point(field:PlayerSyncScNotify.del_equipment_list)
    pub del_equipment_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:PlayerSyncScNotify.relic_list)
    pub relic_list: ::std::vec::Vec<super::Relic::Relic>,
    // @@protoc_insertion_point(field:PlayerSyncScNotify.multi_path_avatar_type_info_list)
    pub multi_path_avatar_type_info_list: ::std::vec::Vec<super::MultiPathAvatarTypeInfo::MultiPathAvatarTypeInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerSyncScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerSyncScNotify {
    fn default() -> &'a PlayerSyncScNotify {
        <PlayerSyncScNotify as ::protobuf::Message>::default_instance()
    }
}

impl PlayerSyncScNotify {
    pub fn new() -> PlayerSyncScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "material_list",
            |m: &PlayerSyncScNotify| { &m.material_list },
            |m: &mut PlayerSyncScNotify| { &mut m.material_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "del_relic_list",
            |m: &PlayerSyncScNotify| { &m.del_relic_list },
            |m: &mut PlayerSyncScNotify| { &mut m.del_relic_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "total_achievement_exp",
            |m: &PlayerSyncScNotify| { &m.total_achievement_exp },
            |m: &mut PlayerSyncScNotify| { &mut m.total_achievement_exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LLMKCDEDIFP",
            |m: &PlayerSyncScNotify| { &m.LLMKCDEDIFP },
            |m: &mut PlayerSyncScNotify| { &mut m.LLMKCDEDIFP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PlayerBasicInfo::PlayerBasicInfo>(
            "basic_info",
            |m: &PlayerSyncScNotify| { &m.basic_info },
            |m: &mut PlayerSyncScNotify| { &mut m.basic_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unk",
            |m: &PlayerSyncScNotify| { &m.unk },
            |m: &mut PlayerSyncScNotify| { &mut m.unk },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "equipment_list",
            |m: &PlayerSyncScNotify| { &m.equipment_list },
            |m: &mut PlayerSyncScNotify| { &mut m.equipment_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AvatarSync::AvatarSync>(
            "avatar_sync",
            |m: &PlayerSyncScNotify| { &m.avatar_sync },
            |m: &mut PlayerSyncScNotify| { &mut m.avatar_sync },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MissionSync::MissionSync>(
            "EDFCAFLFLFH",
            |m: &PlayerSyncScNotify| { &m.EDFCAFLFLFH },
            |m: &mut PlayerSyncScNotify| { &mut m.EDFCAFLFLFH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BoardDataSync::BoardDataSync>(
            "board_data_sync",
            |m: &PlayerSyncScNotify| { &m.board_data_sync },
            |m: &mut PlayerSyncScNotify| { &mut m.board_data_sync },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "del_equipment_list",
            |m: &PlayerSyncScNotify| { &m.del_equipment_list },
            |m: &mut PlayerSyncScNotify| { &mut m.del_equipment_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "relic_list",
            |m: &PlayerSyncScNotify| { &m.relic_list },
            |m: &mut PlayerSyncScNotify| { &mut m.relic_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "multi_path_avatar_type_info_list",
            |m: &PlayerSyncScNotify| { &m.multi_path_avatar_type_info_list },
            |m: &mut PlayerSyncScNotify| { &mut m.multi_path_avatar_type_info_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerSyncScNotify>(
            "PlayerSyncScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerSyncScNotify {
    const NAME: &'static str = "PlayerSyncScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    self.material_list.push(is.read_message()?);
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.del_relic_list)?;
                },
                104 => {
                    self.del_relic_list.push(is.read_uint32()?);
                },
                6008 => {
                    self.total_achievement_exp = is.read_uint32()?;
                },
                7890 => {
                    self.LLMKCDEDIFP.push(is.read_message()?);
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.basic_info)?;
                },
                6938 => {
                    is.read_repeated_packed_uint32_into(&mut self.unk)?;
                },
                6936 => {
                    self.unk.push(is.read_uint32()?);
                },
                42 => {
                    self.equipment_list.push(is.read_message()?);
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.avatar_sync)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EDFCAFLFLFH)?;
                },
                15778 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.board_data_sync)?;
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.del_equipment_list)?;
                },
                72 => {
                    self.del_equipment_list.push(is.read_uint32()?);
                },
                90 => {
                    self.relic_list.push(is.read_message()?);
                },
                12010 => {
                    self.multi_path_avatar_type_info_list.push(is.read_message()?);
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
        for value in &self.material_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.del_relic_list {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        if self.total_achievement_exp != 0 {
            my_size += ::protobuf::rt::uint32_size(751, self.total_achievement_exp);
        }
        for value in &self.LLMKCDEDIFP {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.basic_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.unk {
            my_size += ::protobuf::rt::uint32_size(867, *value);
        };
        for value in &self.equipment_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.avatar_sync.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EDFCAFLFLFH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.board_data_sync.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.del_equipment_list {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        for value in &self.relic_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.multi_path_avatar_type_info_list {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.material_list {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        for v in &self.del_relic_list {
            os.write_uint32(13, *v)?;
        };
        if self.total_achievement_exp != 0 {
            os.write_uint32(751, self.total_achievement_exp)?;
        }
        for v in &self.LLMKCDEDIFP {
            ::protobuf::rt::write_message_field_with_cached_size(986, v, os)?;
        };
        if let Some(v) = self.basic_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        for v in &self.unk {
            os.write_uint32(867, *v)?;
        };
        for v in &self.equipment_list {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if let Some(v) = self.avatar_sync.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.EDFCAFLFLFH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if let Some(v) = self.board_data_sync.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1972, v, os)?;
        }
        for v in &self.del_equipment_list {
            os.write_uint32(9, *v)?;
        };
        for v in &self.relic_list {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        for v in &self.multi_path_avatar_type_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(1501, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PlayerSyncScNotify {
        PlayerSyncScNotify::new()
    }

    fn clear(&mut self) {
        self.material_list.clear();
        self.del_relic_list.clear();
        self.total_achievement_exp = 0;
        self.LLMKCDEDIFP.clear();
        self.basic_info.clear();
        self.unk.clear();
        self.equipment_list.clear();
        self.avatar_sync.clear();
        self.EDFCAFLFLFH.clear();
        self.board_data_sync.clear();
        self.del_equipment_list.clear();
        self.relic_list.clear();
        self.multi_path_avatar_type_info_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerSyncScNotify {
        static instance: PlayerSyncScNotify = PlayerSyncScNotify {
            material_list: ::std::vec::Vec::new(),
            del_relic_list: ::std::vec::Vec::new(),
            total_achievement_exp: 0,
            LLMKCDEDIFP: ::std::vec::Vec::new(),
            basic_info: ::protobuf::MessageField::none(),
            unk: ::std::vec::Vec::new(),
            equipment_list: ::std::vec::Vec::new(),
            avatar_sync: ::protobuf::MessageField::none(),
            EDFCAFLFLFH: ::protobuf::MessageField::none(),
            board_data_sync: ::protobuf::MessageField::none(),
            del_equipment_list: ::std::vec::Vec::new(),
            relic_list: ::std::vec::Vec::new(),
            multi_path_avatar_type_info_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerSyncScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerSyncScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerSyncScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerSyncScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18PlayerSyncScNotify.proto\x1a\x0bRelic.proto\x1a\x0fEquipment.proto\
    \x1a\x15PlayerBasicInfo.proto\x1a\x13BoardDataSync.proto\x1a\x0eMaterial\
    .proto\x1a\x10AvatarSync.proto\x1a\x11MissionSync.proto\x1a\x1dMultiPath\
    AvatarTypeInfo.proto\"\x92\x05\n\x12PlayerSyncScNotify\x12.\n\rmaterial_\
    list\x18\x02\x20\x03(\x0b2\t.MaterialR\x0cmaterialList\x12$\n\x0edel_rel\
    ic_list\x18\r\x20\x03(\rR\x0cdelRelicList\x123\n\x15total_achievement_ex\
    p\x18\xef\x05\x20\x01(\rR\x13totalAchievementExp\x12,\n\x0bLLMKCDEDIFP\
    \x18\xda\x07\x20\x03(\x0b2\t.MaterialR\x0bLLMKCDEDIFP\x12/\n\nbasic_info\
    \x18\n\x20\x01(\x0b2\x10.PlayerBasicInfoR\tbasicInfo\x12\x11\n\x03unk\
    \x18\xe3\x06\x20\x03(\rR\x03unk\x121\n\x0eequipment_list\x18\x05\x20\x03\
    (\x0b2\n.EquipmentR\requipmentList\x12,\n\x0bavatar_sync\x18\x04\x20\x01\
    (\x0b2\x0b.AvatarSyncR\navatarSync\x12.\n\x0bEDFCAFLFLFH\x18\x07\x20\x01\
    (\x0b2\x0c.MissionSyncR\x0bEDFCAFLFLFH\x127\n\x0fboard_data_sync\x18\xb4\
    \x0f\x20\x01(\x0b2\x0e.BoardDataSyncR\rboardDataSync\x12,\n\x12del_equip\
    ment_list\x18\t\x20\x03(\rR\x10delEquipmentList\x12%\n\nrelic_list\x18\
    \x0b\x20\x03(\x0b2\x06.RelicR\trelicList\x12`\n\x20multi_path_avatar_typ\
    e_info_list\x18\xdd\x0b\x20\x03(\x0b2\x18.MultiPathAvatarTypeInfoR\x1bmu\
    ltiPathAvatarTypeInfoListB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::Relic::file_descriptor().clone());
            deps.push(super::Equipment::file_descriptor().clone());
            deps.push(super::PlayerBasicInfo::file_descriptor().clone());
            deps.push(super::BoardDataSync::file_descriptor().clone());
            deps.push(super::Material::file_descriptor().clone());
            deps.push(super::AvatarSync::file_descriptor().clone());
            deps.push(super::MissionSync::file_descriptor().clone());
            deps.push(super::MultiPathAvatarTypeInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlayerSyncScNotify::generated_message_descriptor_data());
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
