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

//! Generated file from `ChatMessageData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ChatMessageData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChatMessageData {
    // message fields
    // @@protoc_insertion_point(field:ChatMessageData.content)
    pub content: ::std::string::String,
    // @@protoc_insertion_point(field:ChatMessageData.sender_id)
    pub sender_id: u32,
    // @@protoc_insertion_point(field:ChatMessageData.HNBEPABNBNG)
    pub HNBEPABNBNG: ::protobuf::MessageField<super::PEDLPHDBNAF::PEDLPHDBNAF>,
    // @@protoc_insertion_point(field:ChatMessageData.create_time)
    pub create_time: u64,
    // @@protoc_insertion_point(field:ChatMessageData.extra_id)
    pub extra_id: u32,
    // @@protoc_insertion_point(field:ChatMessageData.message_type)
    pub message_type: ::protobuf::EnumOrUnknown<super::MsgType::MsgType>,
    // special fields
    // @@protoc_insertion_point(special_field:ChatMessageData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChatMessageData {
    fn default() -> &'a ChatMessageData {
        <ChatMessageData as ::protobuf::Message>::default_instance()
    }
}

impl ChatMessageData {
    pub fn new() -> ChatMessageData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "content",
            |m: &ChatMessageData| { &m.content },
            |m: &mut ChatMessageData| { &mut m.content },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sender_id",
            |m: &ChatMessageData| { &m.sender_id },
            |m: &mut ChatMessageData| { &mut m.sender_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PEDLPHDBNAF::PEDLPHDBNAF>(
            "HNBEPABNBNG",
            |m: &ChatMessageData| { &m.HNBEPABNBNG },
            |m: &mut ChatMessageData| { &mut m.HNBEPABNBNG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "create_time",
            |m: &ChatMessageData| { &m.create_time },
            |m: &mut ChatMessageData| { &mut m.create_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "extra_id",
            |m: &ChatMessageData| { &m.extra_id },
            |m: &mut ChatMessageData| { &mut m.extra_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "message_type",
            |m: &ChatMessageData| { &m.message_type },
            |m: &mut ChatMessageData| { &mut m.message_type },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChatMessageData>(
            "ChatMessageData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChatMessageData {
    const NAME: &'static str = "ChatMessageData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                50 => {
                    self.content = is.read_string()?;
                },
                80 => {
                    self.sender_id = is.read_uint32()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HNBEPABNBNG)?;
                },
                24 => {
                    self.create_time = is.read_uint64()?;
                },
                96 => {
                    self.extra_id = is.read_uint32()?;
                },
                64 => {
                    self.message_type = is.read_enum_or_unknown()?;
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
        if !self.content.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.content);
        }
        if self.sender_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.sender_id);
        }
        if let Some(v) = self.HNBEPABNBNG.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.create_time != 0 {
            my_size += ::protobuf::rt::uint64_size(3, self.create_time);
        }
        if self.extra_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.extra_id);
        }
        if self.message_type != ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.message_type.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.content.is_empty() {
            os.write_string(6, &self.content)?;
        }
        if self.sender_id != 0 {
            os.write_uint32(10, self.sender_id)?;
        }
        if let Some(v) = self.HNBEPABNBNG.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.create_time != 0 {
            os.write_uint64(3, self.create_time)?;
        }
        if self.extra_id != 0 {
            os.write_uint32(12, self.extra_id)?;
        }
        if self.message_type != ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.message_type))?;
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

    fn new() -> ChatMessageData {
        ChatMessageData::new()
    }

    fn clear(&mut self) {
        self.content.clear();
        self.sender_id = 0;
        self.HNBEPABNBNG.clear();
        self.create_time = 0;
        self.extra_id = 0;
        self.message_type = ::protobuf::EnumOrUnknown::new(super::MsgType::MsgType::MSG_TYPE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChatMessageData {
        static instance: ChatMessageData = ChatMessageData {
            content: ::std::string::String::new(),
            sender_id: 0,
            HNBEPABNBNG: ::protobuf::MessageField::none(),
            create_time: 0,
            extra_id: 0,
            message_type: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChatMessageData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChatMessageData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChatMessageData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChatMessageData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15ChatMessageData.proto\x1a\rMsgType.proto\x1a\x11PEDLPHDBNAF.proto\
    \"\xe1\x01\n\x0fChatMessageData\x12\x18\n\x07content\x18\x06\x20\x01(\tR\
    \x07content\x12\x1b\n\tsender_id\x18\n\x20\x01(\rR\x08senderId\x12.\n\
    \x0bHNBEPABNBNG\x18\x02\x20\x01(\x0b2\x0c.PEDLPHDBNAFR\x0bHNBEPABNBNG\
    \x12\x1f\n\x0bcreate_time\x18\x03\x20\x01(\x04R\ncreateTime\x12\x19\n\
    \x08extra_id\x18\x0c\x20\x01(\rR\x07extraId\x12+\n\x0cmessage_type\x18\
    \x08\x20\x01(\x0e2\x08.MsgTypeR\x0bmessageTypeb\x06proto3\
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
            deps.push(super::MsgType::file_descriptor().clone());
            deps.push(super::PEDLPHDBNAF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChatMessageData::generated_message_descriptor_data());
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
