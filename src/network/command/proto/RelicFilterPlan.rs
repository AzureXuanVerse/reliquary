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

//! Generated file from `RelicFilterPlan.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:RelicFilterPlan)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RelicFilterPlan {
    // message fields
    // @@protoc_insertion_point(field:RelicFilterPlan.slot_index)
    pub slot_index: u32,
    // @@protoc_insertion_point(field:RelicFilterPlan.update_timestamp)
    pub update_timestamp: i64,
    // @@protoc_insertion_point(field:RelicFilterPlan.avatar_id_on_create)
    pub avatar_id_on_create: u32,
    // @@protoc_insertion_point(field:RelicFilterPlan.is_marked)
    pub is_marked: bool,
    // @@protoc_insertion_point(field:RelicFilterPlan.settings)
    pub settings: ::protobuf::MessageField<super::RelicFilterPlanSettings::RelicFilterPlanSettings>,
    // @@protoc_insertion_point(field:RelicFilterPlan.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:RelicFilterPlan.icon)
    pub icon: ::protobuf::MessageField<super::RelicFilterPlanIcon::RelicFilterPlanIcon>,
    // special fields
    // @@protoc_insertion_point(special_field:RelicFilterPlan.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RelicFilterPlan {
    fn default() -> &'a RelicFilterPlan {
        <RelicFilterPlan as ::protobuf::Message>::default_instance()
    }
}

impl RelicFilterPlan {
    pub fn new() -> RelicFilterPlan {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "slot_index",
            |m: &RelicFilterPlan| { &m.slot_index },
            |m: &mut RelicFilterPlan| { &mut m.slot_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "update_timestamp",
            |m: &RelicFilterPlan| { &m.update_timestamp },
            |m: &mut RelicFilterPlan| { &mut m.update_timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id_on_create",
            |m: &RelicFilterPlan| { &m.avatar_id_on_create },
            |m: &mut RelicFilterPlan| { &mut m.avatar_id_on_create },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_marked",
            |m: &RelicFilterPlan| { &m.is_marked },
            |m: &mut RelicFilterPlan| { &mut m.is_marked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RelicFilterPlanSettings::RelicFilterPlanSettings>(
            "settings",
            |m: &RelicFilterPlan| { &m.settings },
            |m: &mut RelicFilterPlan| { &mut m.settings },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &RelicFilterPlan| { &m.name },
            |m: &mut RelicFilterPlan| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RelicFilterPlanIcon::RelicFilterPlanIcon>(
            "icon",
            |m: &RelicFilterPlan| { &m.icon },
            |m: &mut RelicFilterPlan| { &mut m.icon },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RelicFilterPlan>(
            "RelicFilterPlan",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RelicFilterPlan {
    const NAME: &'static str = "RelicFilterPlan";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.slot_index = is.read_uint32()?;
                },
                88 => {
                    self.update_timestamp = is.read_int64()?;
                },
                48 => {
                    self.avatar_id_on_create = is.read_uint32()?;
                },
                64 => {
                    self.is_marked = is.read_bool()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.settings)?;
                },
                10 => {
                    self.name = is.read_string()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.icon)?;
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
        if self.slot_index != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.slot_index);
        }
        if self.update_timestamp != 0 {
            my_size += ::protobuf::rt::int64_size(11, self.update_timestamp);
        }
        if self.avatar_id_on_create != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.avatar_id_on_create);
        }
        if self.is_marked != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.settings.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let Some(v) = self.icon.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.slot_index != 0 {
            os.write_uint32(2, self.slot_index)?;
        }
        if self.update_timestamp != 0 {
            os.write_int64(11, self.update_timestamp)?;
        }
        if self.avatar_id_on_create != 0 {
            os.write_uint32(6, self.avatar_id_on_create)?;
        }
        if self.is_marked != false {
            os.write_bool(8, self.is_marked)?;
        }
        if let Some(v) = self.settings.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(v) = self.icon.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> RelicFilterPlan {
        RelicFilterPlan::new()
    }

    fn clear(&mut self) {
        self.slot_index = 0;
        self.update_timestamp = 0;
        self.avatar_id_on_create = 0;
        self.is_marked = false;
        self.settings.clear();
        self.name.clear();
        self.icon.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RelicFilterPlan {
        static instance: RelicFilterPlan = RelicFilterPlan {
            slot_index: 0,
            update_timestamp: 0,
            avatar_id_on_create: 0,
            is_marked: false,
            settings: ::protobuf::MessageField::none(),
            name: ::std::string::String::new(),
            icon: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RelicFilterPlan {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RelicFilterPlan").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RelicFilterPlan {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RelicFilterPlan {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15RelicFilterPlan.proto\x1a\x19RelicFilterPlanIcon.proto\x1a\x1dReli\
    cFilterPlanSettings.proto\"\x9b\x02\n\x0fRelicFilterPlan\x12\x1d\n\nslot\
    _index\x18\x02\x20\x01(\rR\tslotIndex\x12)\n\x10update_timestamp\x18\x0b\
    \x20\x01(\x03R\x0fupdateTimestamp\x12-\n\x13avatar_id_on_create\x18\x06\
    \x20\x01(\rR\x10avatarIdOnCreate\x12\x1b\n\tis_marked\x18\x08\x20\x01(\
    \x08R\x08isMarked\x124\n\x08settings\x18\r\x20\x01(\x0b2\x18.RelicFilter\
    PlanSettingsR\x08settings\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\
    \x12(\n\x04icon\x18\x03\x20\x01(\x0b2\x14.RelicFilterPlanIconR\x04iconb\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::RelicFilterPlanIcon::file_descriptor().clone());
            deps.push(super::RelicFilterPlanSettings::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RelicFilterPlan::generated_message_descriptor_data());
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
