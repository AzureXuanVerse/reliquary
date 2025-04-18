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

//! Generated file from `StartPartialChallengeScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:StartPartialChallengeScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StartPartialChallengeScRsp {
    // message fields
    // @@protoc_insertion_point(field:StartPartialChallengeScRsp.CPBNMACLBEH)
    pub CPBNMACLBEH: ::protobuf::MessageField<super::EIMJEAMDFKJ::EIMJEAMDFKJ>,
    // @@protoc_insertion_point(field:StartPartialChallengeScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:StartPartialChallengeScRsp.MACBLMFJOOO)
    pub MACBLMFJOOO: ::protobuf::MessageField<super::CIOMOMHALAL::CIOMOMHALAL>,
    // @@protoc_insertion_point(field:StartPartialChallengeScRsp.IDOMKBKKKKL)
    pub IDOMKBKKKKL: ::protobuf::MessageField<super::PPJCDCOAFDK::PPJCDCOAFDK>,
    // special fields
    // @@protoc_insertion_point(special_field:StartPartialChallengeScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StartPartialChallengeScRsp {
    fn default() -> &'a StartPartialChallengeScRsp {
        <StartPartialChallengeScRsp as ::protobuf::Message>::default_instance()
    }
}

impl StartPartialChallengeScRsp {
    pub fn new() -> StartPartialChallengeScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EIMJEAMDFKJ::EIMJEAMDFKJ>(
            "CPBNMACLBEH",
            |m: &StartPartialChallengeScRsp| { &m.CPBNMACLBEH },
            |m: &mut StartPartialChallengeScRsp| { &mut m.CPBNMACLBEH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &StartPartialChallengeScRsp| { &m.retcode },
            |m: &mut StartPartialChallengeScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CIOMOMHALAL::CIOMOMHALAL>(
            "MACBLMFJOOO",
            |m: &StartPartialChallengeScRsp| { &m.MACBLMFJOOO },
            |m: &mut StartPartialChallengeScRsp| { &mut m.MACBLMFJOOO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PPJCDCOAFDK::PPJCDCOAFDK>(
            "IDOMKBKKKKL",
            |m: &StartPartialChallengeScRsp| { &m.IDOMKBKKKKL },
            |m: &mut StartPartialChallengeScRsp| { &mut m.IDOMKBKKKKL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StartPartialChallengeScRsp>(
            "StartPartialChallengeScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StartPartialChallengeScRsp {
    const NAME: &'static str = "StartPartialChallengeScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CPBNMACLBEH)?;
                },
                72 => {
                    self.retcode = is.read_uint32()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.MACBLMFJOOO)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.IDOMKBKKKKL)?;
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
        if let Some(v) = self.CPBNMACLBEH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.retcode);
        }
        if let Some(v) = self.MACBLMFJOOO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.IDOMKBKKKKL.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.CPBNMACLBEH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(9, self.retcode)?;
        }
        if let Some(v) = self.MACBLMFJOOO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.IDOMKBKKKKL.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
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

    fn new() -> StartPartialChallengeScRsp {
        StartPartialChallengeScRsp::new()
    }

    fn clear(&mut self) {
        self.CPBNMACLBEH.clear();
        self.retcode = 0;
        self.MACBLMFJOOO.clear();
        self.IDOMKBKKKKL.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StartPartialChallengeScRsp {
        static instance: StartPartialChallengeScRsp = StartPartialChallengeScRsp {
            CPBNMACLBEH: ::protobuf::MessageField::none(),
            retcode: 0,
            MACBLMFJOOO: ::protobuf::MessageField::none(),
            IDOMKBKKKKL: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StartPartialChallengeScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StartPartialChallengeScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StartPartialChallengeScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartPartialChallengeScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20StartPartialChallengeScRsp.proto\x1a\x11CIOMOMHALAL.proto\x1a\x11E\
    IMJEAMDFKJ.proto\x1a\x11PPJCDCOAFDK.proto\"\xc6\x01\n\x1aStartPartialCha\
    llengeScRsp\x12.\n\x0bCPBNMACLBEH\x18\x08\x20\x01(\x0b2\x0c.EIMJEAMDFKJR\
    \x0bCPBNMACLBEH\x12\x18\n\x07retcode\x18\t\x20\x01(\rR\x07retcode\x12.\n\
    \x0bMACBLMFJOOO\x18\x04\x20\x01(\x0b2\x0c.CIOMOMHALALR\x0bMACBLMFJOOO\
    \x12.\n\x0bIDOMKBKKKKL\x18\x07\x20\x01(\x0b2\x0c.PPJCDCOAFDKR\x0bIDOMKBK\
    KKKLb\x06proto3\
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
            deps.push(super::CIOMOMHALAL::file_descriptor().clone());
            deps.push(super::EIMJEAMDFKJ::file_descriptor().clone());
            deps.push(super::PPJCDCOAFDK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(StartPartialChallengeScRsp::generated_message_descriptor_data());
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
