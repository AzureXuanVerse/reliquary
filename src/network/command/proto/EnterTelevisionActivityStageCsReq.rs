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

//! Generated file from `EnterTelevisionActivityStageCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:EnterTelevisionActivityStageCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EnterTelevisionActivityStageCsReq {
    // message fields
    // @@protoc_insertion_point(field:EnterTelevisionActivityStageCsReq.buff_list)
    pub buff_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:EnterTelevisionActivityStageCsReq.NINLFBGLBLL)
    pub NINLFBGLBLL: u32,
    // @@protoc_insertion_point(field:EnterTelevisionActivityStageCsReq.avatar_list)
    pub avatar_list: ::std::vec::Vec<super::DEPEAHJNKGJ::DEPEAHJNKGJ>,
    // special fields
    // @@protoc_insertion_point(special_field:EnterTelevisionActivityStageCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnterTelevisionActivityStageCsReq {
    fn default() -> &'a EnterTelevisionActivityStageCsReq {
        <EnterTelevisionActivityStageCsReq as ::protobuf::Message>::default_instance()
    }
}

impl EnterTelevisionActivityStageCsReq {
    pub fn new() -> EnterTelevisionActivityStageCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "buff_list",
            |m: &EnterTelevisionActivityStageCsReq| { &m.buff_list },
            |m: &mut EnterTelevisionActivityStageCsReq| { &mut m.buff_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NINLFBGLBLL",
            |m: &EnterTelevisionActivityStageCsReq| { &m.NINLFBGLBLL },
            |m: &mut EnterTelevisionActivityStageCsReq| { &mut m.NINLFBGLBLL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &EnterTelevisionActivityStageCsReq| { &m.avatar_list },
            |m: &mut EnterTelevisionActivityStageCsReq| { &mut m.avatar_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnterTelevisionActivityStageCsReq>(
            "EnterTelevisionActivityStageCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnterTelevisionActivityStageCsReq {
    const NAME: &'static str = "EnterTelevisionActivityStageCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.buff_list)?;
                },
                120 => {
                    self.buff_list.push(is.read_uint32()?);
                },
                32 => {
                    self.NINLFBGLBLL = is.read_uint32()?;
                },
                42 => {
                    self.avatar_list.push(is.read_message()?);
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
        my_size += ::protobuf::rt::vec_packed_uint32_size(15, &self.buff_list);
        if self.NINLFBGLBLL != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.NINLFBGLBLL);
        }
        for value in &self.avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_uint32(15, &self.buff_list)?;
        if self.NINLFBGLBLL != 0 {
            os.write_uint32(4, self.NINLFBGLBLL)?;
        }
        for v in &self.avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> EnterTelevisionActivityStageCsReq {
        EnterTelevisionActivityStageCsReq::new()
    }

    fn clear(&mut self) {
        self.buff_list.clear();
        self.NINLFBGLBLL = 0;
        self.avatar_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnterTelevisionActivityStageCsReq {
        static instance: EnterTelevisionActivityStageCsReq = EnterTelevisionActivityStageCsReq {
            buff_list: ::std::vec::Vec::new(),
            NINLFBGLBLL: 0,
            avatar_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnterTelevisionActivityStageCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnterTelevisionActivityStageCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnterTelevisionActivityStageCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnterTelevisionActivityStageCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'EnterTelevisionActivityStageCsReq.proto\x1a\x11DEPEAHJNKGJ.proto\"\
    \x91\x01\n!EnterTelevisionActivityStageCsReq\x12\x1b\n\tbuff_list\x18\
    \x0f\x20\x03(\rR\x08buffList\x12\x20\n\x0bNINLFBGLBLL\x18\x04\x20\x01(\r\
    R\x0bNINLFBGLBLL\x12-\n\x0bavatar_list\x18\x05\x20\x03(\x0b2\x0c.DEPEAHJ\
    NKGJR\navatarListb\x06proto3\
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
            deps.push(super::DEPEAHJNKGJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EnterTelevisionActivityStageCsReq::generated_message_descriptor_data());
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
