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

//! Generated file from `MLKHNKBJEFB.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MLKHNKBJEFB)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MLKHNKBJEFB {
    // message fields
    // @@protoc_insertion_point(field:MLKHNKBJEFB.GIHOBANKDJB)
    pub GIHOBANKDJB: u32,
    // @@protoc_insertion_point(field:MLKHNKBJEFB.GBGOPMEEHIG)
    pub GBGOPMEEHIG: bool,
    // special fields
    // @@protoc_insertion_point(special_field:MLKHNKBJEFB.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MLKHNKBJEFB {
    fn default() -> &'a MLKHNKBJEFB {
        <MLKHNKBJEFB as ::protobuf::Message>::default_instance()
    }
}

impl MLKHNKBJEFB {
    pub fn new() -> MLKHNKBJEFB {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GIHOBANKDJB",
            |m: &MLKHNKBJEFB| { &m.GIHOBANKDJB },
            |m: &mut MLKHNKBJEFB| { &mut m.GIHOBANKDJB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GBGOPMEEHIG",
            |m: &MLKHNKBJEFB| { &m.GBGOPMEEHIG },
            |m: &mut MLKHNKBJEFB| { &mut m.GBGOPMEEHIG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MLKHNKBJEFB>(
            "MLKHNKBJEFB",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MLKHNKBJEFB {
    const NAME: &'static str = "MLKHNKBJEFB";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.GIHOBANKDJB = is.read_uint32()?;
                },
                24 => {
                    self.GBGOPMEEHIG = is.read_bool()?;
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
        if self.GIHOBANKDJB != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.GIHOBANKDJB);
        }
        if self.GBGOPMEEHIG != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GIHOBANKDJB != 0 {
            os.write_uint32(1, self.GIHOBANKDJB)?;
        }
        if self.GBGOPMEEHIG != false {
            os.write_bool(3, self.GBGOPMEEHIG)?;
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

    fn new() -> MLKHNKBJEFB {
        MLKHNKBJEFB::new()
    }

    fn clear(&mut self) {
        self.GIHOBANKDJB = 0;
        self.GBGOPMEEHIG = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MLKHNKBJEFB {
        static instance: MLKHNKBJEFB = MLKHNKBJEFB {
            GIHOBANKDJB: 0,
            GBGOPMEEHIG: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MLKHNKBJEFB {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MLKHNKBJEFB").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MLKHNKBJEFB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MLKHNKBJEFB {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MLKHNKBJEFB.proto\"Q\n\x0bMLKHNKBJEFB\x12\x20\n\x0bGIHOBANKDJB\x18\
    \x01\x20\x01(\rR\x0bGIHOBANKDJB\x12\x20\n\x0bGBGOPMEEHIG\x18\x03\x20\x01\
    (\x08R\x0bGBGOPMEEHIGb\x06proto3\
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
            messages.push(MLKHNKBJEFB::generated_message_descriptor_data());
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
