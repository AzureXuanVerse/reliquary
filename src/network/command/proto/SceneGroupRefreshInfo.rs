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

//! Generated file from `SceneGroupRefreshInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SceneGroupRefreshInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneGroupRefreshInfo {
    // message fields
    // @@protoc_insertion_point(field:SceneGroupRefreshInfo.group_refresh_type)
    pub group_refresh_type: ::protobuf::EnumOrUnknown<super::SceneGroupRefreshType::SceneGroupRefreshType>,
    // @@protoc_insertion_point(field:SceneGroupRefreshInfo.group_id)
    pub group_id: u32,
    // @@protoc_insertion_point(field:SceneGroupRefreshInfo.BCCGJIHNCDN)
    pub BCCGJIHNCDN: ::std::vec::Vec<super::CMGFHBHAFFB::CMGFHBHAFFB>,
    // @@protoc_insertion_point(field:SceneGroupRefreshInfo.state)
    pub state: u32,
    // @@protoc_insertion_point(field:SceneGroupRefreshInfo.refresh_entity)
    pub refresh_entity: ::std::vec::Vec<super::SceneEntityRefreshInfo::SceneEntityRefreshInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:SceneGroupRefreshInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneGroupRefreshInfo {
    fn default() -> &'a SceneGroupRefreshInfo {
        <SceneGroupRefreshInfo as ::protobuf::Message>::default_instance()
    }
}

impl SceneGroupRefreshInfo {
    pub fn new() -> SceneGroupRefreshInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "group_refresh_type",
            |m: &SceneGroupRefreshInfo| { &m.group_refresh_type },
            |m: &mut SceneGroupRefreshInfo| { &mut m.group_refresh_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "group_id",
            |m: &SceneGroupRefreshInfo| { &m.group_id },
            |m: &mut SceneGroupRefreshInfo| { &mut m.group_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BCCGJIHNCDN",
            |m: &SceneGroupRefreshInfo| { &m.BCCGJIHNCDN },
            |m: &mut SceneGroupRefreshInfo| { &mut m.BCCGJIHNCDN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "state",
            |m: &SceneGroupRefreshInfo| { &m.state },
            |m: &mut SceneGroupRefreshInfo| { &mut m.state },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "refresh_entity",
            |m: &SceneGroupRefreshInfo| { &m.refresh_entity },
            |m: &mut SceneGroupRefreshInfo| { &mut m.refresh_entity },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneGroupRefreshInfo>(
            "SceneGroupRefreshInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneGroupRefreshInfo {
    const NAME: &'static str = "SceneGroupRefreshInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.group_refresh_type = is.read_enum_or_unknown()?;
                },
                40 => {
                    self.group_id = is.read_uint32()?;
                },
                10 => {
                    self.BCCGJIHNCDN.push(is.read_message()?);
                },
                48 => {
                    self.state = is.read_uint32()?;
                },
                58 => {
                    self.refresh_entity.push(is.read_message()?);
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
        if self.group_refresh_type != ::protobuf::EnumOrUnknown::new(super::SceneGroupRefreshType::SceneGroupRefreshType::SCENE_GROUP_REFRESH_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.group_refresh_type.value());
        }
        if self.group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.group_id);
        }
        for value in &self.BCCGJIHNCDN {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.state != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.state);
        }
        for value in &self.refresh_entity {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.group_refresh_type != ::protobuf::EnumOrUnknown::new(super::SceneGroupRefreshType::SceneGroupRefreshType::SCENE_GROUP_REFRESH_TYPE_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.group_refresh_type))?;
        }
        if self.group_id != 0 {
            os.write_uint32(5, self.group_id)?;
        }
        for v in &self.BCCGJIHNCDN {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.state != 0 {
            os.write_uint32(6, self.state)?;
        }
        for v in &self.refresh_entity {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
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

    fn new() -> SceneGroupRefreshInfo {
        SceneGroupRefreshInfo::new()
    }

    fn clear(&mut self) {
        self.group_refresh_type = ::protobuf::EnumOrUnknown::new(super::SceneGroupRefreshType::SceneGroupRefreshType::SCENE_GROUP_REFRESH_TYPE_NONE);
        self.group_id = 0;
        self.BCCGJIHNCDN.clear();
        self.state = 0;
        self.refresh_entity.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneGroupRefreshInfo {
        static instance: SceneGroupRefreshInfo = SceneGroupRefreshInfo {
            group_refresh_type: ::protobuf::EnumOrUnknown::from_i32(0),
            group_id: 0,
            BCCGJIHNCDN: ::std::vec::Vec::new(),
            state: 0,
            refresh_entity: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneGroupRefreshInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneGroupRefreshInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneGroupRefreshInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneGroupRefreshInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bSceneGroupRefreshInfo.proto\x1a\x11CMGFHBHAFFB.proto\x1a\x1cSceneE\
    ntityRefreshInfo.proto\x1a\x1bSceneGroupRefreshType.proto\"\xfe\x01\n\
    \x15SceneGroupRefreshInfo\x12D\n\x12group_refresh_type\x18\x08\x20\x01(\
    \x0e2\x16.SceneGroupRefreshTypeR\x10groupRefreshType\x12\x19\n\x08group_\
    id\x18\x05\x20\x01(\rR\x07groupId\x12.\n\x0bBCCGJIHNCDN\x18\x01\x20\x03(\
    \x0b2\x0c.CMGFHBHAFFBR\x0bBCCGJIHNCDN\x12\x14\n\x05state\x18\x06\x20\x01\
    (\rR\x05state\x12>\n\x0erefresh_entity\x18\x07\x20\x03(\x0b2\x17.SceneEn\
    tityRefreshInfoR\rrefreshEntityb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::CMGFHBHAFFB::file_descriptor().clone());
            deps.push(super::SceneEntityRefreshInfo::file_descriptor().clone());
            deps.push(super::SceneGroupRefreshType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneGroupRefreshInfo::generated_message_descriptor_data());
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
