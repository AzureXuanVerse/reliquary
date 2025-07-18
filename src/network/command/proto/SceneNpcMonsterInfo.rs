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

//! Generated file from `SceneNpcMonsterInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SceneNpcMonsterInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneNpcMonsterInfo {
    // message fields
    // @@protoc_insertion_point(field:SceneNpcMonsterInfo.IDPJIDNLEHH)
    pub IDPJIDNLEHH: bool,
    // @@protoc_insertion_point(field:SceneNpcMonsterInfo.world_level)
    pub world_level: u32,
    // @@protoc_insertion_point(field:SceneNpcMonsterInfo.extra_info)
    pub extra_info: ::protobuf::MessageField<super::NpcMonsterExtraInfo::NpcMonsterExtraInfo>,
    // @@protoc_insertion_point(field:SceneNpcMonsterInfo.MPFEDFBKKDF)
    pub MPFEDFBKKDF: bool,
    // @@protoc_insertion_point(field:SceneNpcMonsterInfo.monster_id)
    pub monster_id: u32,
    // @@protoc_insertion_point(field:SceneNpcMonsterInfo.event_id)
    pub event_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SceneNpcMonsterInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneNpcMonsterInfo {
    fn default() -> &'a SceneNpcMonsterInfo {
        <SceneNpcMonsterInfo as ::protobuf::Message>::default_instance()
    }
}

impl SceneNpcMonsterInfo {
    pub fn new() -> SceneNpcMonsterInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IDPJIDNLEHH",
            |m: &SceneNpcMonsterInfo| { &m.IDPJIDNLEHH },
            |m: &mut SceneNpcMonsterInfo| { &mut m.IDPJIDNLEHH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "world_level",
            |m: &SceneNpcMonsterInfo| { &m.world_level },
            |m: &mut SceneNpcMonsterInfo| { &mut m.world_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NpcMonsterExtraInfo::NpcMonsterExtraInfo>(
            "extra_info",
            |m: &SceneNpcMonsterInfo| { &m.extra_info },
            |m: &mut SceneNpcMonsterInfo| { &mut m.extra_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MPFEDFBKKDF",
            |m: &SceneNpcMonsterInfo| { &m.MPFEDFBKKDF },
            |m: &mut SceneNpcMonsterInfo| { &mut m.MPFEDFBKKDF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "monster_id",
            |m: &SceneNpcMonsterInfo| { &m.monster_id },
            |m: &mut SceneNpcMonsterInfo| { &mut m.monster_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "event_id",
            |m: &SceneNpcMonsterInfo| { &m.event_id },
            |m: &mut SceneNpcMonsterInfo| { &mut m.event_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneNpcMonsterInfo>(
            "SceneNpcMonsterInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneNpcMonsterInfo {
    const NAME: &'static str = "SceneNpcMonsterInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.IDPJIDNLEHH = is.read_bool()?;
                },
                104 => {
                    self.world_level = is.read_uint32()?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.extra_info)?;
                },
                64 => {
                    self.MPFEDFBKKDF = is.read_bool()?;
                },
                56 => {
                    self.monster_id = is.read_uint32()?;
                },
                8 => {
                    self.event_id = is.read_uint32()?;
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
        if self.IDPJIDNLEHH != false {
            my_size += 1 + 1;
        }
        if self.world_level != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.world_level);
        }
        if let Some(v) = self.extra_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.MPFEDFBKKDF != false {
            my_size += 1 + 1;
        }
        if self.monster_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.monster_id);
        }
        if self.event_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.event_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IDPJIDNLEHH != false {
            os.write_bool(2, self.IDPJIDNLEHH)?;
        }
        if self.world_level != 0 {
            os.write_uint32(13, self.world_level)?;
        }
        if let Some(v) = self.extra_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if self.MPFEDFBKKDF != false {
            os.write_bool(8, self.MPFEDFBKKDF)?;
        }
        if self.monster_id != 0 {
            os.write_uint32(7, self.monster_id)?;
        }
        if self.event_id != 0 {
            os.write_uint32(1, self.event_id)?;
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

    fn new() -> SceneNpcMonsterInfo {
        SceneNpcMonsterInfo::new()
    }

    fn clear(&mut self) {
        self.IDPJIDNLEHH = false;
        self.world_level = 0;
        self.extra_info.clear();
        self.MPFEDFBKKDF = false;
        self.monster_id = 0;
        self.event_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneNpcMonsterInfo {
        static instance: SceneNpcMonsterInfo = SceneNpcMonsterInfo {
            IDPJIDNLEHH: false,
            world_level: 0,
            extra_info: ::protobuf::MessageField::none(),
            MPFEDFBKKDF: false,
            monster_id: 0,
            event_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneNpcMonsterInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneNpcMonsterInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneNpcMonsterInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneNpcMonsterInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19SceneNpcMonsterInfo.proto\x1a\x19NpcMonsterExtraInfo.proto\"\xe9\
    \x01\n\x13SceneNpcMonsterInfo\x12\x20\n\x0bIDPJIDNLEHH\x18\x02\x20\x01(\
    \x08R\x0bIDPJIDNLEHH\x12\x1f\n\x0bworld_level\x18\r\x20\x01(\rR\nworldLe\
    vel\x123\n\nextra_info\x18\x0e\x20\x01(\x0b2\x14.NpcMonsterExtraInfoR\te\
    xtraInfo\x12\x20\n\x0bMPFEDFBKKDF\x18\x08\x20\x01(\x08R\x0bMPFEDFBKKDF\
    \x12\x1d\n\nmonster_id\x18\x07\x20\x01(\rR\tmonsterId\x12\x19\n\x08event\
    _id\x18\x01\x20\x01(\rR\x07eventIdb\x06proto3\
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
            deps.push(super::NpcMonsterExtraInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneNpcMonsterInfo::generated_message_descriptor_data());
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
