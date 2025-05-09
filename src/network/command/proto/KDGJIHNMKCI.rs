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

//! Generated file from `KDGJIHNMKCI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:KDGJIHNMKCI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KDGJIHNMKCI {
    // message fields
    // @@protoc_insertion_point(field:KDGJIHNMKCI.cur_hp)
    pub cur_hp: u32,
    // @@protoc_insertion_point(field:KDGJIHNMKCI.max_hp)
    pub max_hp: u32,
    // @@protoc_insertion_point(field:KDGJIHNMKCI.monster_id)
    pub monster_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:KDGJIHNMKCI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KDGJIHNMKCI {
    fn default() -> &'a KDGJIHNMKCI {
        <KDGJIHNMKCI as ::protobuf::Message>::default_instance()
    }
}

impl KDGJIHNMKCI {
    pub fn new() -> KDGJIHNMKCI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_hp",
            |m: &KDGJIHNMKCI| { &m.cur_hp },
            |m: &mut KDGJIHNMKCI| { &mut m.cur_hp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_hp",
            |m: &KDGJIHNMKCI| { &m.max_hp },
            |m: &mut KDGJIHNMKCI| { &mut m.max_hp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "monster_id",
            |m: &KDGJIHNMKCI| { &m.monster_id },
            |m: &mut KDGJIHNMKCI| { &mut m.monster_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KDGJIHNMKCI>(
            "KDGJIHNMKCI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KDGJIHNMKCI {
    const NAME: &'static str = "KDGJIHNMKCI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.cur_hp = is.read_uint32()?;
                },
                104 => {
                    self.max_hp = is.read_uint32()?;
                },
                64 => {
                    self.monster_id = is.read_uint32()?;
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
        if self.cur_hp != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.cur_hp);
        }
        if self.max_hp != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.max_hp);
        }
        if self.monster_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.monster_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.cur_hp != 0 {
            os.write_uint32(14, self.cur_hp)?;
        }
        if self.max_hp != 0 {
            os.write_uint32(13, self.max_hp)?;
        }
        if self.monster_id != 0 {
            os.write_uint32(8, self.monster_id)?;
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

    fn new() -> KDGJIHNMKCI {
        KDGJIHNMKCI::new()
    }

    fn clear(&mut self) {
        self.cur_hp = 0;
        self.max_hp = 0;
        self.monster_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KDGJIHNMKCI {
        static instance: KDGJIHNMKCI = KDGJIHNMKCI {
            cur_hp: 0,
            max_hp: 0,
            monster_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KDGJIHNMKCI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KDGJIHNMKCI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KDGJIHNMKCI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KDGJIHNMKCI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KDGJIHNMKCI.proto\"Z\n\x0bKDGJIHNMKCI\x12\x15\n\x06cur_hp\x18\x0e\
    \x20\x01(\rR\x05curHp\x12\x15\n\x06max_hp\x18\r\x20\x01(\rR\x05maxHp\x12\
    \x1d\n\nmonster_id\x18\x08\x20\x01(\rR\tmonsterIdb\x06proto3\
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
            messages.push(KDGJIHNMKCI::generated_message_descriptor_data());
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
