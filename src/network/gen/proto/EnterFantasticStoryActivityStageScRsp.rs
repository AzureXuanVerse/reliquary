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

//! Generated file from `EnterFantasticStoryActivityStageScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EnterFantasticStoryActivityStageScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EnterFantasticStoryActivityStageScRsp {
    // message fields
    // @@protoc_insertion_point(field:EnterFantasticStoryActivityStageScRsp.MGIEBBICLCK)
    pub MGIEBBICLCK: u32,
    // @@protoc_insertion_point(field:EnterFantasticStoryActivityStageScRsp.AIHBDNKBNMH)
    pub AIHBDNKBNMH: u32,
    // @@protoc_insertion_point(field:EnterFantasticStoryActivityStageScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:EnterFantasticStoryActivityStageScRsp.BBKGPAJCCBM)
    pub BBKGPAJCCBM: ::protobuf::MessageField<super::CHDPLFOHLCN::CHDPLFOHLCN>,
    // special fields
    // @@protoc_insertion_point(special_field:EnterFantasticStoryActivityStageScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnterFantasticStoryActivityStageScRsp {
    fn default() -> &'a EnterFantasticStoryActivityStageScRsp {
        <EnterFantasticStoryActivityStageScRsp as ::protobuf::Message>::default_instance()
    }
}

impl EnterFantasticStoryActivityStageScRsp {
    pub fn new() -> EnterFantasticStoryActivityStageScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MGIEBBICLCK",
            |m: &EnterFantasticStoryActivityStageScRsp| { &m.MGIEBBICLCK },
            |m: &mut EnterFantasticStoryActivityStageScRsp| { &mut m.MGIEBBICLCK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AIHBDNKBNMH",
            |m: &EnterFantasticStoryActivityStageScRsp| { &m.AIHBDNKBNMH },
            |m: &mut EnterFantasticStoryActivityStageScRsp| { &mut m.AIHBDNKBNMH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &EnterFantasticStoryActivityStageScRsp| { &m.ADADHIHDHJC },
            |m: &mut EnterFantasticStoryActivityStageScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CHDPLFOHLCN::CHDPLFOHLCN>(
            "BBKGPAJCCBM",
            |m: &EnterFantasticStoryActivityStageScRsp| { &m.BBKGPAJCCBM },
            |m: &mut EnterFantasticStoryActivityStageScRsp| { &mut m.BBKGPAJCCBM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnterFantasticStoryActivityStageScRsp>(
            "EnterFantasticStoryActivityStageScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnterFantasticStoryActivityStageScRsp {
    const NAME: &'static str = "EnterFantasticStoryActivityStageScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.MGIEBBICLCK = is.read_uint32()?;
                },
                24 => {
                    self.AIHBDNKBNMH = is.read_uint32()?;
                },
                80 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BBKGPAJCCBM)?;
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
        if self.MGIEBBICLCK != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.MGIEBBICLCK);
        }
        if self.AIHBDNKBNMH != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.AIHBDNKBNMH);
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.ADADHIHDHJC);
        }
        if let Some(v) = self.BBKGPAJCCBM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.MGIEBBICLCK != 0 {
            os.write_uint32(2, self.MGIEBBICLCK)?;
        }
        if self.AIHBDNKBNMH != 0 {
            os.write_uint32(3, self.AIHBDNKBNMH)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(10, self.ADADHIHDHJC)?;
        }
        if let Some(v) = self.BBKGPAJCCBM.as_ref() {
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

    fn new() -> EnterFantasticStoryActivityStageScRsp {
        EnterFantasticStoryActivityStageScRsp::new()
    }

    fn clear(&mut self) {
        self.MGIEBBICLCK = 0;
        self.AIHBDNKBNMH = 0;
        self.ADADHIHDHJC = 0;
        self.BBKGPAJCCBM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnterFantasticStoryActivityStageScRsp {
        static instance: EnterFantasticStoryActivityStageScRsp = EnterFantasticStoryActivityStageScRsp {
            MGIEBBICLCK: 0,
            AIHBDNKBNMH: 0,
            ADADHIHDHJC: 0,
            BBKGPAJCCBM: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnterFantasticStoryActivityStageScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnterFantasticStoryActivityStageScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnterFantasticStoryActivityStageScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnterFantasticStoryActivityStageScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n+EnterFantasticStoryActivityStageScRsp.proto\x1a\x11CHDPLFOHLCN.proto\
    \"\xbd\x01\n%EnterFantasticStoryActivityStageScRsp\x12\x20\n\x0bMGIEBBIC\
    LCK\x18\x02\x20\x01(\rR\x0bMGIEBBICLCK\x12\x20\n\x0bAIHBDNKBNMH\x18\x03\
    \x20\x01(\rR\x0bAIHBDNKBNMH\x12\x20\n\x0bADADHIHDHJC\x18\n\x20\x01(\rR\
    \x0bADADHIHDHJC\x12.\n\x0bBBKGPAJCCBM\x18\x07\x20\x01(\x0b2\x0c.CHDPLFOH\
    LCNR\x0bBBKGPAJCCBMb\x06proto3\
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
            deps.push(super::CHDPLFOHLCN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EnterFantasticStoryActivityStageScRsp::generated_message_descriptor_data());
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
