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

//! Generated file from `GetPrivateChatHistoryScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetPrivateChatHistoryScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetPrivateChatHistoryScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetPrivateChatHistoryScRsp.to_uid)
    pub to_uid: u32,
    // @@protoc_insertion_point(field:GetPrivateChatHistoryScRsp.from_uid)
    pub from_uid: u32,
    // @@protoc_insertion_point(field:GetPrivateChatHistoryScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetPrivateChatHistoryScRsp.chat_list)
    pub chat_list: ::std::vec::Vec<super::Chat::Chat>,
    // special fields
    // @@protoc_insertion_point(special_field:GetPrivateChatHistoryScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetPrivateChatHistoryScRsp {
    fn default() -> &'a GetPrivateChatHistoryScRsp {
        <GetPrivateChatHistoryScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetPrivateChatHistoryScRsp {
    pub fn new() -> GetPrivateChatHistoryScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "to_uid",
            |m: &GetPrivateChatHistoryScRsp| { &m.to_uid },
            |m: &mut GetPrivateChatHistoryScRsp| { &mut m.to_uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "from_uid",
            |m: &GetPrivateChatHistoryScRsp| { &m.from_uid },
            |m: &mut GetPrivateChatHistoryScRsp| { &mut m.from_uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetPrivateChatHistoryScRsp| { &m.retcode },
            |m: &mut GetPrivateChatHistoryScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "chat_list",
            |m: &GetPrivateChatHistoryScRsp| { &m.chat_list },
            |m: &mut GetPrivateChatHistoryScRsp| { &mut m.chat_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetPrivateChatHistoryScRsp>(
            "GetPrivateChatHistoryScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetPrivateChatHistoryScRsp {
    const NAME: &'static str = "GetPrivateChatHistoryScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.to_uid = is.read_uint32()?;
                },
                104 => {
                    self.from_uid = is.read_uint32()?;
                },
                40 => {
                    self.retcode = is.read_uint32()?;
                },
                66 => {
                    self.chat_list.push(is.read_message()?);
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
        if self.to_uid != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.to_uid);
        }
        if self.from_uid != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.from_uid);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.retcode);
        }
        for value in &self.chat_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.to_uid != 0 {
            os.write_uint32(2, self.to_uid)?;
        }
        if self.from_uid != 0 {
            os.write_uint32(13, self.from_uid)?;
        }
        if self.retcode != 0 {
            os.write_uint32(5, self.retcode)?;
        }
        for v in &self.chat_list {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
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

    fn new() -> GetPrivateChatHistoryScRsp {
        GetPrivateChatHistoryScRsp::new()
    }

    fn clear(&mut self) {
        self.to_uid = 0;
        self.from_uid = 0;
        self.retcode = 0;
        self.chat_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetPrivateChatHistoryScRsp {
        static instance: GetPrivateChatHistoryScRsp = GetPrivateChatHistoryScRsp {
            to_uid: 0,
            from_uid: 0,
            retcode: 0,
            chat_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetPrivateChatHistoryScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetPrivateChatHistoryScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetPrivateChatHistoryScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetPrivateChatHistoryScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20GetPrivateChatHistoryScRsp.proto\x1a\nChat.proto\"\x8c\x01\n\x1aGe\
    tPrivateChatHistoryScRsp\x12\x15\n\x06to_uid\x18\x02\x20\x01(\rR\x05toUi\
    d\x12\x19\n\x08from_uid\x18\r\x20\x01(\rR\x07fromUid\x12\x18\n\x07retcod\
    e\x18\x05\x20\x01(\rR\x07retcode\x12\"\n\tchat_list\x18\x08\x20\x03(\x0b\
    2\x05.ChatR\x08chatListb\x06proto3\
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
            deps.push(super::Chat::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetPrivateChatHistoryScRsp::generated_message_descriptor_data());
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
