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

//! Generated file from `SocialEventServerCache.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SocialEventServerCache)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SocialEventServerCache {
    // message fields
    // @@protoc_insertion_point(field:SocialEventServerCache.add_coin)
    pub add_coin: u32,
    // @@protoc_insertion_point(field:SocialEventServerCache.id)
    pub id: u32,
    // @@protoc_insertion_point(field:SocialEventServerCache.src_uid)
    pub src_uid: u32,
    // @@protoc_insertion_point(field:SocialEventServerCache.sub_coin)
    pub sub_coin: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SocialEventServerCache.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SocialEventServerCache {
    fn default() -> &'a SocialEventServerCache {
        <SocialEventServerCache as ::protobuf::Message>::default_instance()
    }
}

impl SocialEventServerCache {
    pub fn new() -> SocialEventServerCache {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "add_coin",
            |m: &SocialEventServerCache| { &m.add_coin },
            |m: &mut SocialEventServerCache| { &mut m.add_coin },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &SocialEventServerCache| { &m.id },
            |m: &mut SocialEventServerCache| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "src_uid",
            |m: &SocialEventServerCache| { &m.src_uid },
            |m: &mut SocialEventServerCache| { &mut m.src_uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sub_coin",
            |m: &SocialEventServerCache| { &m.sub_coin },
            |m: &mut SocialEventServerCache| { &mut m.sub_coin },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SocialEventServerCache>(
            "SocialEventServerCache",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SocialEventServerCache {
    const NAME: &'static str = "SocialEventServerCache";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.add_coin = is.read_uint32()?;
                },
                48 => {
                    self.id = is.read_uint32()?;
                },
                8 => {
                    self.src_uid = is.read_uint32()?;
                },
                96 => {
                    self.sub_coin = is.read_uint32()?;
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
        if self.add_coin != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.add_coin);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.id);
        }
        if self.src_uid != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.src_uid);
        }
        if self.sub_coin != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.sub_coin);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.add_coin != 0 {
            os.write_uint32(2, self.add_coin)?;
        }
        if self.id != 0 {
            os.write_uint32(6, self.id)?;
        }
        if self.src_uid != 0 {
            os.write_uint32(1, self.src_uid)?;
        }
        if self.sub_coin != 0 {
            os.write_uint32(12, self.sub_coin)?;
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

    fn new() -> SocialEventServerCache {
        SocialEventServerCache::new()
    }

    fn clear(&mut self) {
        self.add_coin = 0;
        self.id = 0;
        self.src_uid = 0;
        self.sub_coin = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SocialEventServerCache {
        static instance: SocialEventServerCache = SocialEventServerCache {
            add_coin: 0,
            id: 0,
            src_uid: 0,
            sub_coin: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SocialEventServerCache {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SocialEventServerCache").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SocialEventServerCache {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SocialEventServerCache {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cSocialEventServerCache.proto\"w\n\x16SocialEventServerCache\x12\
    \x19\n\x08add_coin\x18\x02\x20\x01(\rR\x07addCoin\x12\x0e\n\x02id\x18\
    \x06\x20\x01(\rR\x02id\x12\x17\n\x07src_uid\x18\x01\x20\x01(\rR\x06srcUi\
    d\x12\x19\n\x08sub_coin\x18\x0c\x20\x01(\rR\x07subCoinb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SocialEventServerCache::generated_message_descriptor_data());
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
