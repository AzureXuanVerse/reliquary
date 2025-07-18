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

//! Generated file from `EvolveBuildShopAbilityResetScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:EvolveBuildShopAbilityResetScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EvolveBuildShopAbilityResetScRsp {
    // message fields
    // @@protoc_insertion_point(field:EvolveBuildShopAbilityResetScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:EvolveBuildShopAbilityResetScRsp.MDCJFOAFDJK)
    pub MDCJFOAFDJK: ::protobuf::EnumOrUnknown<super::KLNIPNJCNMJ::KLNIPNJCNMJ>,
    // @@protoc_insertion_point(field:EvolveBuildShopAbilityResetScRsp.item_value)
    pub item_value: u32,
    // @@protoc_insertion_point(field:EvolveBuildShopAbilityResetScRsp.COKDNPEEMAG)
    pub COKDNPEEMAG: ::std::vec::Vec<super::IMGJIEBFGPF::IMGJIEBFGPF>,
    // special fields
    // @@protoc_insertion_point(special_field:EvolveBuildShopAbilityResetScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EvolveBuildShopAbilityResetScRsp {
    fn default() -> &'a EvolveBuildShopAbilityResetScRsp {
        <EvolveBuildShopAbilityResetScRsp as ::protobuf::Message>::default_instance()
    }
}

impl EvolveBuildShopAbilityResetScRsp {
    pub fn new() -> EvolveBuildShopAbilityResetScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &EvolveBuildShopAbilityResetScRsp| { &m.retcode },
            |m: &mut EvolveBuildShopAbilityResetScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MDCJFOAFDJK",
            |m: &EvolveBuildShopAbilityResetScRsp| { &m.MDCJFOAFDJK },
            |m: &mut EvolveBuildShopAbilityResetScRsp| { &mut m.MDCJFOAFDJK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "item_value",
            |m: &EvolveBuildShopAbilityResetScRsp| { &m.item_value },
            |m: &mut EvolveBuildShopAbilityResetScRsp| { &mut m.item_value },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "COKDNPEEMAG",
            |m: &EvolveBuildShopAbilityResetScRsp| { &m.COKDNPEEMAG },
            |m: &mut EvolveBuildShopAbilityResetScRsp| { &mut m.COKDNPEEMAG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EvolveBuildShopAbilityResetScRsp>(
            "EvolveBuildShopAbilityResetScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EvolveBuildShopAbilityResetScRsp {
    const NAME: &'static str = "EvolveBuildShopAbilityResetScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.retcode = is.read_uint32()?;
                },
                24 => {
                    self.MDCJFOAFDJK = is.read_enum_or_unknown()?;
                },
                120 => {
                    self.item_value = is.read_uint32()?;
                },
                74 => {
                    self.COKDNPEEMAG.push(is.read_message()?);
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.retcode);
        }
        if self.MDCJFOAFDJK != ::protobuf::EnumOrUnknown::new(super::KLNIPNJCNMJ::KLNIPNJCNMJ::EVOLVE_BUILD_SEASON_NONE) {
            my_size += ::protobuf::rt::int32_size(3, self.MDCJFOAFDJK.value());
        }
        if self.item_value != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.item_value);
        }
        for value in &self.COKDNPEEMAG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(12, self.retcode)?;
        }
        if self.MDCJFOAFDJK != ::protobuf::EnumOrUnknown::new(super::KLNIPNJCNMJ::KLNIPNJCNMJ::EVOLVE_BUILD_SEASON_NONE) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.MDCJFOAFDJK))?;
        }
        if self.item_value != 0 {
            os.write_uint32(15, self.item_value)?;
        }
        for v in &self.COKDNPEEMAG {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
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

    fn new() -> EvolveBuildShopAbilityResetScRsp {
        EvolveBuildShopAbilityResetScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.MDCJFOAFDJK = ::protobuf::EnumOrUnknown::new(super::KLNIPNJCNMJ::KLNIPNJCNMJ::EVOLVE_BUILD_SEASON_NONE);
        self.item_value = 0;
        self.COKDNPEEMAG.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EvolveBuildShopAbilityResetScRsp {
        static instance: EvolveBuildShopAbilityResetScRsp = EvolveBuildShopAbilityResetScRsp {
            retcode: 0,
            MDCJFOAFDJK: ::protobuf::EnumOrUnknown::from_i32(0),
            item_value: 0,
            COKDNPEEMAG: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EvolveBuildShopAbilityResetScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EvolveBuildShopAbilityResetScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EvolveBuildShopAbilityResetScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EvolveBuildShopAbilityResetScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&EvolveBuildShopAbilityResetScRsp.proto\x1a\x11IMGJIEBFGPF.proto\x1a\
    \x11KLNIPNJCNMJ.proto\"\xbb\x01\n\x20EvolveBuildShopAbilityResetScRsp\
    \x12\x18\n\x07retcode\x18\x0c\x20\x01(\rR\x07retcode\x12.\n\x0bMDCJFOAFD\
    JK\x18\x03\x20\x01(\x0e2\x0c.KLNIPNJCNMJR\x0bMDCJFOAFDJK\x12\x1d\n\nitem\
    _value\x18\x0f\x20\x01(\rR\titemValue\x12.\n\x0bCOKDNPEEMAG\x18\t\x20\
    \x03(\x0b2\x0c.IMGJIEBFGPFR\x0bCOKDNPEEMAGb\x06proto3\
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
            deps.push(super::IMGJIEBFGPF::file_descriptor().clone());
            deps.push(super::KLNIPNJCNMJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EvolveBuildShopAbilityResetScRsp::generated_message_descriptor_data());
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
