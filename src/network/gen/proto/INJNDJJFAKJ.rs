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

//! Generated file from `INJNDJJFAKJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:INJNDJJFAKJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct INJNDJJFAKJ {
    // message fields
    // @@protoc_insertion_point(field:INJNDJJFAKJ.AFGCICJOJKC)
    pub AFGCICJOJKC: u32,
    // @@protoc_insertion_point(field:INJNDJJFAKJ.LLDBBHOPNBI)
    pub LLDBBHOPNBI: bool,
    // @@protoc_insertion_point(field:INJNDJJFAKJ.avatar_list)
    pub avatar_list: ::std::vec::Vec<super::GachaCeilingAvatar::GachaCeilingAvatar>,
    // special fields
    // @@protoc_insertion_point(special_field:INJNDJJFAKJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a INJNDJJFAKJ {
    fn default() -> &'a INJNDJJFAKJ {
        <INJNDJJFAKJ as ::protobuf::Message>::default_instance()
    }
}

impl INJNDJJFAKJ {
    pub fn new() -> INJNDJJFAKJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AFGCICJOJKC",
            |m: &INJNDJJFAKJ| { &m.AFGCICJOJKC },
            |m: &mut INJNDJJFAKJ| { &mut m.AFGCICJOJKC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LLDBBHOPNBI",
            |m: &INJNDJJFAKJ| { &m.LLDBBHOPNBI },
            |m: &mut INJNDJJFAKJ| { &mut m.LLDBBHOPNBI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &INJNDJJFAKJ| { &m.avatar_list },
            |m: &mut INJNDJJFAKJ| { &mut m.avatar_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<INJNDJJFAKJ>(
            "INJNDJJFAKJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for INJNDJJFAKJ {
    const NAME: &'static str = "INJNDJJFAKJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.AFGCICJOJKC = is.read_uint32()?;
                },
                24 => {
                    self.LLDBBHOPNBI = is.read_bool()?;
                },
                122 => {
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
        if self.AFGCICJOJKC != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.AFGCICJOJKC);
        }
        if self.LLDBBHOPNBI != false {
            my_size += 1 + 1;
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
        if self.AFGCICJOJKC != 0 {
            os.write_uint32(9, self.AFGCICJOJKC)?;
        }
        if self.LLDBBHOPNBI != false {
            os.write_bool(3, self.LLDBBHOPNBI)?;
        }
        for v in &self.avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> INJNDJJFAKJ {
        INJNDJJFAKJ::new()
    }

    fn clear(&mut self) {
        self.AFGCICJOJKC = 0;
        self.LLDBBHOPNBI = false;
        self.avatar_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static INJNDJJFAKJ {
        static instance: INJNDJJFAKJ = INJNDJJFAKJ {
            AFGCICJOJKC: 0,
            LLDBBHOPNBI: false,
            avatar_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for INJNDJJFAKJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("INJNDJJFAKJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for INJNDJJFAKJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for INJNDJJFAKJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11INJNDJJFAKJ.proto\x1a\x18GachaCeilingAvatar.proto\"\x87\x01\n\x0bI\
    NJNDJJFAKJ\x12\x20\n\x0bAFGCICJOJKC\x18\t\x20\x01(\rR\x0bAFGCICJOJKC\x12\
    \x20\n\x0bLLDBBHOPNBI\x18\x03\x20\x01(\x08R\x0bLLDBBHOPNBI\x124\n\x0bava\
    tar_list\x18\x0f\x20\x03(\x0b2\x13.GachaCeilingAvatarR\navatarListb\x06p\
    roto3\
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
            deps.push(super::GachaCeilingAvatar::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(INJNDJJFAKJ::generated_message_descriptor_data());
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
