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

//! Generated file from `LineupSlotData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LineupSlotData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LineupSlotData {
    // message fields
    // @@protoc_insertion_point(field:LineupSlotData.avatar_type)
    pub avatar_type: ::protobuf::EnumOrUnknown<super::AvatarType::AvatarType>,
    // @@protoc_insertion_point(field:LineupSlotData.id)
    pub id: u32,
    // @@protoc_insertion_point(field:LineupSlotData.slot)
    pub slot: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LineupSlotData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LineupSlotData {
    fn default() -> &'a LineupSlotData {
        <LineupSlotData as ::protobuf::Message>::default_instance()
    }
}

impl LineupSlotData {
    pub fn new() -> LineupSlotData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_type",
            |m: &LineupSlotData| { &m.avatar_type },
            |m: &mut LineupSlotData| { &mut m.avatar_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &LineupSlotData| { &m.id },
            |m: &mut LineupSlotData| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "slot",
            |m: &LineupSlotData| { &m.slot },
            |m: &mut LineupSlotData| { &mut m.slot },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LineupSlotData>(
            "LineupSlotData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LineupSlotData {
    const NAME: &'static str = "LineupSlotData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.avatar_type = is.read_enum_or_unknown()?;
                },
                112 => {
                    self.id = is.read_uint32()?;
                },
                24 => {
                    self.slot = is.read_uint32()?;
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
        if self.avatar_type != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(6, self.avatar_type.value());
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.id);
        }
        if self.slot != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.slot);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.avatar_type != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            os.write_enum(6, ::protobuf::EnumOrUnknown::value(&self.avatar_type))?;
        }
        if self.id != 0 {
            os.write_uint32(14, self.id)?;
        }
        if self.slot != 0 {
            os.write_uint32(3, self.slot)?;
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

    fn new() -> LineupSlotData {
        LineupSlotData::new()
    }

    fn clear(&mut self) {
        self.avatar_type = ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE);
        self.id = 0;
        self.slot = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LineupSlotData {
        static instance: LineupSlotData = LineupSlotData {
            avatar_type: ::protobuf::EnumOrUnknown::from_i32(0),
            id: 0,
            slot: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LineupSlotData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LineupSlotData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LineupSlotData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LineupSlotData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14LineupSlotData.proto\x1a\x10AvatarType.proto\"b\n\x0eLineupSlotDat\
    a\x12,\n\x0bavatar_type\x18\x06\x20\x01(\x0e2\x0b.AvatarTypeR\navatarTyp\
    e\x12\x0e\n\x02id\x18\x0e\x20\x01(\rR\x02id\x12\x12\n\x04slot\x18\x03\
    \x20\x01(\rR\x04slotB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::AvatarType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LineupSlotData::generated_message_descriptor_data());
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
