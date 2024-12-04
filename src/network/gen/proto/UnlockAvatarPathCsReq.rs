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

//! Generated file from `UnlockAvatarPathCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:UnlockAvatarPathCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UnlockAvatarPathCsReq {
    // message fields
    // @@protoc_insertion_point(field:UnlockAvatarPathCsReq.DHPDHGHAEOP)
    pub DHPDHGHAEOP: ::protobuf::EnumOrUnknown<super::MultiPathAvatarType::MultiPathAvatarType>,
    // special fields
    // @@protoc_insertion_point(special_field:UnlockAvatarPathCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UnlockAvatarPathCsReq {
    fn default() -> &'a UnlockAvatarPathCsReq {
        <UnlockAvatarPathCsReq as ::protobuf::Message>::default_instance()
    }
}

impl UnlockAvatarPathCsReq {
    pub fn new() -> UnlockAvatarPathCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DHPDHGHAEOP",
            |m: &UnlockAvatarPathCsReq| { &m.DHPDHGHAEOP },
            |m: &mut UnlockAvatarPathCsReq| { &mut m.DHPDHGHAEOP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UnlockAvatarPathCsReq>(
            "UnlockAvatarPathCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UnlockAvatarPathCsReq {
    const NAME: &'static str = "UnlockAvatarPathCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.DHPDHGHAEOP = is.read_enum_or_unknown()?;
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
        if self.DHPDHGHAEOP != ::protobuf::EnumOrUnknown::new(super::MultiPathAvatarType::MultiPathAvatarType::MultiPathAvatarTypeNone) {
            my_size += ::protobuf::rt::int32_size(10, self.DHPDHGHAEOP.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DHPDHGHAEOP != ::protobuf::EnumOrUnknown::new(super::MultiPathAvatarType::MultiPathAvatarType::MultiPathAvatarTypeNone) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.DHPDHGHAEOP))?;
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

    fn new() -> UnlockAvatarPathCsReq {
        UnlockAvatarPathCsReq::new()
    }

    fn clear(&mut self) {
        self.DHPDHGHAEOP = ::protobuf::EnumOrUnknown::new(super::MultiPathAvatarType::MultiPathAvatarType::MultiPathAvatarTypeNone);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UnlockAvatarPathCsReq {
        static instance: UnlockAvatarPathCsReq = UnlockAvatarPathCsReq {
            DHPDHGHAEOP: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UnlockAvatarPathCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UnlockAvatarPathCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UnlockAvatarPathCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnlockAvatarPathCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bUnlockAvatarPathCsReq.proto\x1a\x19MultiPathAvatarType.proto\"O\n\
    \x15UnlockAvatarPathCsReq\x126\n\x0bDHPDHGHAEOP\x18\n\x20\x01(\x0e2\x14.\
    MultiPathAvatarTypeR\x0bDHPDHGHAEOPb\x06proto3\
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
            deps.push(super::MultiPathAvatarType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(UnlockAvatarPathCsReq::generated_message_descriptor_data());
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
