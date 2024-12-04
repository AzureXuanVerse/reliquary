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

//! Generated file from `SpaceZooTakeScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SpaceZooTakeScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SpaceZooTakeScRsp {
    // message fields
    // @@protoc_insertion_point(field:SpaceZooTakeScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:SpaceZooTakeScRsp.ELPMNKHEPKJ)
    pub ELPMNKHEPKJ: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:SpaceZooTakeScRsp.OBBNODAFPAK)
    pub OBBNODAFPAK: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SpaceZooTakeScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SpaceZooTakeScRsp {
    fn default() -> &'a SpaceZooTakeScRsp {
        <SpaceZooTakeScRsp as ::protobuf::Message>::default_instance()
    }
}

impl SpaceZooTakeScRsp {
    pub fn new() -> SpaceZooTakeScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &SpaceZooTakeScRsp| { &m.ADADHIHDHJC },
            |m: &mut SpaceZooTakeScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "ELPMNKHEPKJ",
            |m: &SpaceZooTakeScRsp| { &m.ELPMNKHEPKJ },
            |m: &mut SpaceZooTakeScRsp| { &mut m.ELPMNKHEPKJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OBBNODAFPAK",
            |m: &SpaceZooTakeScRsp| { &m.OBBNODAFPAK },
            |m: &mut SpaceZooTakeScRsp| { &mut m.OBBNODAFPAK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SpaceZooTakeScRsp>(
            "SpaceZooTakeScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SpaceZooTakeScRsp {
    const NAME: &'static str = "SpaceZooTakeScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ELPMNKHEPKJ)?;
                },
                120 => {
                    self.OBBNODAFPAK = is.read_uint32()?;
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
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.ADADHIHDHJC);
        }
        if let Some(v) = self.ELPMNKHEPKJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.OBBNODAFPAK != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.OBBNODAFPAK);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(12, self.ADADHIHDHJC)?;
        }
        if let Some(v) = self.ELPMNKHEPKJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if self.OBBNODAFPAK != 0 {
            os.write_uint32(15, self.OBBNODAFPAK)?;
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

    fn new() -> SpaceZooTakeScRsp {
        SpaceZooTakeScRsp::new()
    }

    fn clear(&mut self) {
        self.ADADHIHDHJC = 0;
        self.ELPMNKHEPKJ.clear();
        self.OBBNODAFPAK = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SpaceZooTakeScRsp {
        static instance: SpaceZooTakeScRsp = SpaceZooTakeScRsp {
            ADADHIHDHJC: 0,
            ELPMNKHEPKJ: ::protobuf::MessageField::none(),
            OBBNODAFPAK: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SpaceZooTakeScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SpaceZooTakeScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SpaceZooTakeScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SpaceZooTakeScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17SpaceZooTakeScRsp.proto\x1a\x0eItemList.proto\"\x84\x01\n\x11Space\
    ZooTakeScRsp\x12\x20\n\x0bADADHIHDHJC\x18\x0c\x20\x01(\rR\x0bADADHIHDHJC\
    \x12+\n\x0bELPMNKHEPKJ\x18\x03\x20\x01(\x0b2\t.ItemListR\x0bELPMNKHEPKJ\
    \x12\x20\n\x0bOBBNODAFPAK\x18\x0f\x20\x01(\rR\x0bOBBNODAFPAKb\x06proto3\
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
            messages.push(SpaceZooTakeScRsp::generated_message_descriptor_data());
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
