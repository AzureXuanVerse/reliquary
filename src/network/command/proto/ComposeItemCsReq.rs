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

//! Generated file from `ComposeItemCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ComposeItemCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ComposeItemCsReq {
    // message fields
    // @@protoc_insertion_point(field:ComposeItemCsReq.convert_item_list)
    pub convert_item_list: ::protobuf::MessageField<super::ItemCostData::ItemCostData>,
    // @@protoc_insertion_point(field:ComposeItemCsReq.count)
    pub count: u32,
    // @@protoc_insertion_point(field:ComposeItemCsReq.compose_item_list)
    pub compose_item_list: ::protobuf::MessageField<super::ItemCostData::ItemCostData>,
    // @@protoc_insertion_point(field:ComposeItemCsReq.compose_id)
    pub compose_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ComposeItemCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ComposeItemCsReq {
    fn default() -> &'a ComposeItemCsReq {
        <ComposeItemCsReq as ::protobuf::Message>::default_instance()
    }
}

impl ComposeItemCsReq {
    pub fn new() -> ComposeItemCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemCostData::ItemCostData>(
            "convert_item_list",
            |m: &ComposeItemCsReq| { &m.convert_item_list },
            |m: &mut ComposeItemCsReq| { &mut m.convert_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "count",
            |m: &ComposeItemCsReq| { &m.count },
            |m: &mut ComposeItemCsReq| { &mut m.count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemCostData::ItemCostData>(
            "compose_item_list",
            |m: &ComposeItemCsReq| { &m.compose_item_list },
            |m: &mut ComposeItemCsReq| { &mut m.compose_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "compose_id",
            |m: &ComposeItemCsReq| { &m.compose_id },
            |m: &mut ComposeItemCsReq| { &mut m.compose_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ComposeItemCsReq>(
            "ComposeItemCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ComposeItemCsReq {
    const NAME: &'static str = "ComposeItemCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.convert_item_list)?;
                },
                104 => {
                    self.count = is.read_uint32()?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.compose_item_list)?;
                },
                88 => {
                    self.compose_id = is.read_uint32()?;
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
        if let Some(v) = self.convert_item_list.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.count != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.count);
        }
        if let Some(v) = self.compose_item_list.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.compose_id != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.compose_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.convert_item_list.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if self.count != 0 {
            os.write_uint32(13, self.count)?;
        }
        if let Some(v) = self.compose_item_list.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.compose_id != 0 {
            os.write_uint32(11, self.compose_id)?;
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

    fn new() -> ComposeItemCsReq {
        ComposeItemCsReq::new()
    }

    fn clear(&mut self) {
        self.convert_item_list.clear();
        self.count = 0;
        self.compose_item_list.clear();
        self.compose_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ComposeItemCsReq {
        static instance: ComposeItemCsReq = ComposeItemCsReq {
            convert_item_list: ::protobuf::MessageField::none(),
            count: 0,
            compose_item_list: ::protobuf::MessageField::none(),
            compose_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ComposeItemCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ComposeItemCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ComposeItemCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ComposeItemCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16ComposeItemCsReq.proto\x1a\x12ItemCostData.proto\"\xbd\x01\n\x10Co\
    mposeItemCsReq\x129\n\x11convert_item_list\x18\x0e\x20\x01(\x0b2\r.ItemC\
    ostDataR\x0fconvertItemList\x12\x14\n\x05count\x18\r\x20\x01(\rR\x05coun\
    t\x129\n\x11compose_item_list\x18\x05\x20\x01(\x0b2\r.ItemCostDataR\x0fc\
    omposeItemList\x12\x1d\n\ncompose_id\x18\x0b\x20\x01(\rR\tcomposeIdb\x06\
    proto3\
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
            deps.push(super::ItemCostData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ComposeItemCsReq::generated_message_descriptor_data());
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
