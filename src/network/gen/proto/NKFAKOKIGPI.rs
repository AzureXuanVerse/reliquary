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

//! Generated file from `NKFAKOKIGPI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NKFAKOKIGPI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NKFAKOKIGPI {
    // message fields
    // @@protoc_insertion_point(field:NKFAKOKIGPI.NLPPCEJDAED)
    pub NLPPCEJDAED: u32,
    // @@protoc_insertion_point(field:NKFAKOKIGPI.OPOGAOKJFGB)
    pub OPOGAOKJFGB: u32,
    // @@protoc_insertion_point(field:NKFAKOKIGPI.HJCGEIDBICC)
    pub HJCGEIDBICC: bool,
    // @@protoc_insertion_point(field:NKFAKOKIGPI.CJBDDDJEKJM)
    pub CJBDDDJEKJM: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:NKFAKOKIGPI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NKFAKOKIGPI {
    fn default() -> &'a NKFAKOKIGPI {
        <NKFAKOKIGPI as ::protobuf::Message>::default_instance()
    }
}

impl NKFAKOKIGPI {
    pub fn new() -> NKFAKOKIGPI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NLPPCEJDAED",
            |m: &NKFAKOKIGPI| { &m.NLPPCEJDAED },
            |m: &mut NKFAKOKIGPI| { &mut m.NLPPCEJDAED },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OPOGAOKJFGB",
            |m: &NKFAKOKIGPI| { &m.OPOGAOKJFGB },
            |m: &mut NKFAKOKIGPI| { &mut m.OPOGAOKJFGB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HJCGEIDBICC",
            |m: &NKFAKOKIGPI| { &m.HJCGEIDBICC },
            |m: &mut NKFAKOKIGPI| { &mut m.HJCGEIDBICC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CJBDDDJEKJM",
            |m: &NKFAKOKIGPI| { &m.CJBDDDJEKJM },
            |m: &mut NKFAKOKIGPI| { &mut m.CJBDDDJEKJM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NKFAKOKIGPI>(
            "NKFAKOKIGPI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NKFAKOKIGPI {
    const NAME: &'static str = "NKFAKOKIGPI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.NLPPCEJDAED = is.read_uint32()?;
                },
                80 => {
                    self.OPOGAOKJFGB = is.read_uint32()?;
                },
                40 => {
                    self.HJCGEIDBICC = is.read_bool()?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.CJBDDDJEKJM)?;
                },
                104 => {
                    self.CJBDDDJEKJM.push(is.read_uint32()?);
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
        if self.NLPPCEJDAED != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.NLPPCEJDAED);
        }
        if self.OPOGAOKJFGB != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.OPOGAOKJFGB);
        }
        if self.HJCGEIDBICC != false {
            my_size += 1 + 1;
        }
        for value in &self.CJBDDDJEKJM {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NLPPCEJDAED != 0 {
            os.write_uint32(2, self.NLPPCEJDAED)?;
        }
        if self.OPOGAOKJFGB != 0 {
            os.write_uint32(10, self.OPOGAOKJFGB)?;
        }
        if self.HJCGEIDBICC != false {
            os.write_bool(5, self.HJCGEIDBICC)?;
        }
        for v in &self.CJBDDDJEKJM {
            os.write_uint32(13, *v)?;
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

    fn new() -> NKFAKOKIGPI {
        NKFAKOKIGPI::new()
    }

    fn clear(&mut self) {
        self.NLPPCEJDAED = 0;
        self.OPOGAOKJFGB = 0;
        self.HJCGEIDBICC = false;
        self.CJBDDDJEKJM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NKFAKOKIGPI {
        static instance: NKFAKOKIGPI = NKFAKOKIGPI {
            NLPPCEJDAED: 0,
            OPOGAOKJFGB: 0,
            HJCGEIDBICC: false,
            CJBDDDJEKJM: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NKFAKOKIGPI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NKFAKOKIGPI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NKFAKOKIGPI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NKFAKOKIGPI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NKFAKOKIGPI.proto\"\x95\x01\n\x0bNKFAKOKIGPI\x12\x20\n\x0bNLPPCEJD\
    AED\x18\x02\x20\x01(\rR\x0bNLPPCEJDAED\x12\x20\n\x0bOPOGAOKJFGB\x18\n\
    \x20\x01(\rR\x0bOPOGAOKJFGB\x12\x20\n\x0bHJCGEIDBICC\x18\x05\x20\x01(\
    \x08R\x0bHJCGEIDBICC\x12\x20\n\x0bCJBDDDJEKJM\x18\r\x20\x03(\rR\x0bCJBDD\
    DJEKJMb\x06proto3\
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
            messages.push(NKFAKOKIGPI::generated_message_descriptor_data());
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
