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

//! Generated file from `TakeActivityExpeditionRewardScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:TakeActivityExpeditionRewardScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TakeActivityExpeditionRewardScRsp {
    // message fields
    // @@protoc_insertion_point(field:TakeActivityExpeditionRewardScRsp.PEHCDFAEEFK)
    pub PEHCDFAEEFK: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:TakeActivityExpeditionRewardScRsp.reward)
    pub reward: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:TakeActivityExpeditionRewardScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:TakeActivityExpeditionRewardScRsp.MPGEMLGLHBH)
    pub MPGEMLGLHBH: u32,
    // @@protoc_insertion_point(field:TakeActivityExpeditionRewardScRsp.score)
    pub score: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TakeActivityExpeditionRewardScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TakeActivityExpeditionRewardScRsp {
    fn default() -> &'a TakeActivityExpeditionRewardScRsp {
        <TakeActivityExpeditionRewardScRsp as ::protobuf::Message>::default_instance()
    }
}

impl TakeActivityExpeditionRewardScRsp {
    pub fn new() -> TakeActivityExpeditionRewardScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "PEHCDFAEEFK",
            |m: &TakeActivityExpeditionRewardScRsp| { &m.PEHCDFAEEFK },
            |m: &mut TakeActivityExpeditionRewardScRsp| { &mut m.PEHCDFAEEFK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "reward",
            |m: &TakeActivityExpeditionRewardScRsp| { &m.reward },
            |m: &mut TakeActivityExpeditionRewardScRsp| { &mut m.reward },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &TakeActivityExpeditionRewardScRsp| { &m.retcode },
            |m: &mut TakeActivityExpeditionRewardScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MPGEMLGLHBH",
            |m: &TakeActivityExpeditionRewardScRsp| { &m.MPGEMLGLHBH },
            |m: &mut TakeActivityExpeditionRewardScRsp| { &mut m.MPGEMLGLHBH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score",
            |m: &TakeActivityExpeditionRewardScRsp| { &m.score },
            |m: &mut TakeActivityExpeditionRewardScRsp| { &mut m.score },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TakeActivityExpeditionRewardScRsp>(
            "TakeActivityExpeditionRewardScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TakeActivityExpeditionRewardScRsp {
    const NAME: &'static str = "TakeActivityExpeditionRewardScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.PEHCDFAEEFK)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.reward)?;
                },
                96 => {
                    self.retcode = is.read_uint32()?;
                },
                8 => {
                    self.MPGEMLGLHBH = is.read_uint32()?;
                },
                24 => {
                    self.score = is.read_uint32()?;
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
        if let Some(v) = self.PEHCDFAEEFK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.reward.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.retcode);
        }
        if self.MPGEMLGLHBH != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.MPGEMLGLHBH);
        }
        if self.score != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.score);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.PEHCDFAEEFK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.reward.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.retcode != 0 {
            os.write_uint32(12, self.retcode)?;
        }
        if self.MPGEMLGLHBH != 0 {
            os.write_uint32(1, self.MPGEMLGLHBH)?;
        }
        if self.score != 0 {
            os.write_uint32(3, self.score)?;
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

    fn new() -> TakeActivityExpeditionRewardScRsp {
        TakeActivityExpeditionRewardScRsp::new()
    }

    fn clear(&mut self) {
        self.PEHCDFAEEFK.clear();
        self.reward.clear();
        self.retcode = 0;
        self.MPGEMLGLHBH = 0;
        self.score = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TakeActivityExpeditionRewardScRsp {
        static instance: TakeActivityExpeditionRewardScRsp = TakeActivityExpeditionRewardScRsp {
            PEHCDFAEEFK: ::protobuf::MessageField::none(),
            reward: ::protobuf::MessageField::none(),
            retcode: 0,
            MPGEMLGLHBH: 0,
            score: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TakeActivityExpeditionRewardScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TakeActivityExpeditionRewardScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TakeActivityExpeditionRewardScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TakeActivityExpeditionRewardScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'TakeActivityExpeditionRewardScRsp.proto\x1a\x0eItemList.proto\"\xc5\
    \x01\n!TakeActivityExpeditionRewardScRsp\x12+\n\x0bPEHCDFAEEFK\x18\n\x20\
    \x01(\x0b2\t.ItemListR\x0bPEHCDFAEEFK\x12!\n\x06reward\x18\x05\x20\x01(\
    \x0b2\t.ItemListR\x06reward\x12\x18\n\x07retcode\x18\x0c\x20\x01(\rR\x07\
    retcode\x12\x20\n\x0bMPGEMLGLHBH\x18\x01\x20\x01(\rR\x0bMPGEMLGLHBH\x12\
    \x14\n\x05score\x18\x03\x20\x01(\rR\x05scoreb\x06proto3\
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
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TakeActivityExpeditionRewardScRsp::generated_message_descriptor_data());
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
