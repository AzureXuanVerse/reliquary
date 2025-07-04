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

//! Generated file from `SceneEnterStageCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SceneEnterStageCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneEnterStageCsReq {
    // message fields
    // @@protoc_insertion_point(field:SceneEnterStageCsReq.rebattle_type)
    pub rebattle_type: ::protobuf::EnumOrUnknown<super::RebattleType::RebattleType>,
    // @@protoc_insertion_point(field:SceneEnterStageCsReq.event_id)
    pub event_id: u32,
    // @@protoc_insertion_point(field:SceneEnterStageCsReq.PMJAHILBLFL)
    pub PMJAHILBLFL: bool,
    // special fields
    // @@protoc_insertion_point(special_field:SceneEnterStageCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneEnterStageCsReq {
    fn default() -> &'a SceneEnterStageCsReq {
        <SceneEnterStageCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SceneEnterStageCsReq {
    pub fn new() -> SceneEnterStageCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rebattle_type",
            |m: &SceneEnterStageCsReq| { &m.rebattle_type },
            |m: &mut SceneEnterStageCsReq| { &mut m.rebattle_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "event_id",
            |m: &SceneEnterStageCsReq| { &m.event_id },
            |m: &mut SceneEnterStageCsReq| { &mut m.event_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PMJAHILBLFL",
            |m: &SceneEnterStageCsReq| { &m.PMJAHILBLFL },
            |m: &mut SceneEnterStageCsReq| { &mut m.PMJAHILBLFL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneEnterStageCsReq>(
            "SceneEnterStageCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneEnterStageCsReq {
    const NAME: &'static str = "SceneEnterStageCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.rebattle_type = is.read_enum_or_unknown()?;
                },
                88 => {
                    self.event_id = is.read_uint32()?;
                },
                56 => {
                    self.PMJAHILBLFL = is.read_bool()?;
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
        if self.rebattle_type != ::protobuf::EnumOrUnknown::new(super::RebattleType::RebattleType::REBATTLE_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(15, self.rebattle_type.value());
        }
        if self.event_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.event_id);
        }
        if self.PMJAHILBLFL != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.rebattle_type != ::protobuf::EnumOrUnknown::new(super::RebattleType::RebattleType::REBATTLE_TYPE_NONE) {
            os.write_enum(15, ::protobuf::EnumOrUnknown::value(&self.rebattle_type))?;
        }
        if self.event_id != 0 {
            os.write_uint32(11, self.event_id)?;
        }
        if self.PMJAHILBLFL != false {
            os.write_bool(7, self.PMJAHILBLFL)?;
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

    fn new() -> SceneEnterStageCsReq {
        SceneEnterStageCsReq::new()
    }

    fn clear(&mut self) {
        self.rebattle_type = ::protobuf::EnumOrUnknown::new(super::RebattleType::RebattleType::REBATTLE_TYPE_NONE);
        self.event_id = 0;
        self.PMJAHILBLFL = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneEnterStageCsReq {
        static instance: SceneEnterStageCsReq = SceneEnterStageCsReq {
            rebattle_type: ::protobuf::EnumOrUnknown::from_i32(0),
            event_id: 0,
            PMJAHILBLFL: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneEnterStageCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneEnterStageCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneEnterStageCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneEnterStageCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aSceneEnterStageCsReq.proto\x1a\x12RebattleType.proto\"\x87\x01\n\
    \x14SceneEnterStageCsReq\x122\n\rrebattle_type\x18\x0f\x20\x01(\x0e2\r.R\
    ebattleTypeR\x0crebattleType\x12\x19\n\x08event_id\x18\x0b\x20\x01(\rR\
    \x07eventId\x12\x20\n\x0bPMJAHILBLFL\x18\x07\x20\x01(\x08R\x0bPMJAHILBLF\
    Lb\x06proto3\
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
            deps.push(super::RebattleType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneEnterStageCsReq::generated_message_descriptor_data());
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
