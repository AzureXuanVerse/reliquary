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

//! Generated file from `HEHPIOGGIEG.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HEHPIOGGIEG)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HEHPIOGGIEG {
    // message fields
    // @@protoc_insertion_point(field:HEHPIOGGIEG.uuid)
    pub uuid: ::std::string::String,
    // @@protoc_insertion_point(field:HEHPIOGGIEG.BDNFLICNFHN)
    pub BDNFLICNFHN: bool,
    // @@protoc_insertion_point(field:HEHPIOGGIEG.HEFJEJHOJEA)
    pub HEFJEJHOJEA: ::protobuf::MessageField<super::HFPFFJIMCKM::HFPFFJIMCKM>,
    // @@protoc_insertion_point(field:HEHPIOGGIEG.OPCLDNLLNFH)
    pub OPCLDNLLNFH: ::std::vec::Vec<super::Vector::Vector>,
    // @@protoc_insertion_point(field:HEHPIOGGIEG.id)
    pub id: u32,
    // @@protoc_insertion_point(field:HEHPIOGGIEG.LKJMLIDAODM)
    pub LKJMLIDAODM: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HEHPIOGGIEG.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HEHPIOGGIEG {
    fn default() -> &'a HEHPIOGGIEG {
        <HEHPIOGGIEG as ::protobuf::Message>::default_instance()
    }
}

impl HEHPIOGGIEG {
    pub fn new() -> HEHPIOGGIEG {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uuid",
            |m: &HEHPIOGGIEG| { &m.uuid },
            |m: &mut HEHPIOGGIEG| { &mut m.uuid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BDNFLICNFHN",
            |m: &HEHPIOGGIEG| { &m.BDNFLICNFHN },
            |m: &mut HEHPIOGGIEG| { &mut m.BDNFLICNFHN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::HFPFFJIMCKM::HFPFFJIMCKM>(
            "HEFJEJHOJEA",
            |m: &HEHPIOGGIEG| { &m.HEFJEJHOJEA },
            |m: &mut HEHPIOGGIEG| { &mut m.HEFJEJHOJEA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OPCLDNLLNFH",
            |m: &HEHPIOGGIEG| { &m.OPCLDNLLNFH },
            |m: &mut HEHPIOGGIEG| { &mut m.OPCLDNLLNFH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &HEHPIOGGIEG| { &m.id },
            |m: &mut HEHPIOGGIEG| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LKJMLIDAODM",
            |m: &HEHPIOGGIEG| { &m.LKJMLIDAODM },
            |m: &mut HEHPIOGGIEG| { &mut m.LKJMLIDAODM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HEHPIOGGIEG>(
            "HEHPIOGGIEG",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HEHPIOGGIEG {
    const NAME: &'static str = "HEHPIOGGIEG";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    self.uuid = is.read_string()?;
                },
                112 => {
                    self.BDNFLICNFHN = is.read_bool()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.HEFJEJHOJEA)?;
                },
                74 => {
                    self.OPCLDNLLNFH.push(is.read_message()?);
                },
                96 => {
                    self.id = is.read_uint32()?;
                },
                40 => {
                    self.LKJMLIDAODM = is.read_uint32()?;
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
        if !self.uuid.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.uuid);
        }
        if self.BDNFLICNFHN != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.HEFJEJHOJEA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.OPCLDNLLNFH {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.id);
        }
        if self.LKJMLIDAODM != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.LKJMLIDAODM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.uuid.is_empty() {
            os.write_string(11, &self.uuid)?;
        }
        if self.BDNFLICNFHN != false {
            os.write_bool(14, self.BDNFLICNFHN)?;
        }
        if let Some(v) = self.HEFJEJHOJEA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        for v in &self.OPCLDNLLNFH {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.id != 0 {
            os.write_uint32(12, self.id)?;
        }
        if self.LKJMLIDAODM != 0 {
            os.write_uint32(5, self.LKJMLIDAODM)?;
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

    fn new() -> HEHPIOGGIEG {
        HEHPIOGGIEG::new()
    }

    fn clear(&mut self) {
        self.uuid.clear();
        self.BDNFLICNFHN = false;
        self.HEFJEJHOJEA.clear();
        self.OPCLDNLLNFH.clear();
        self.id = 0;
        self.LKJMLIDAODM = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HEHPIOGGIEG {
        static instance: HEHPIOGGIEG = HEHPIOGGIEG {
            uuid: ::std::string::String::new(),
            BDNFLICNFHN: false,
            HEFJEJHOJEA: ::protobuf::MessageField::none(),
            OPCLDNLLNFH: ::std::vec::Vec::new(),
            id: 0,
            LKJMLIDAODM: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HEHPIOGGIEG {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HEHPIOGGIEG").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HEHPIOGGIEG {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HEHPIOGGIEG {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HEHPIOGGIEG.proto\x1a\x11HFPFFJIMCKM.proto\x1a\x0cVector.proto\"\
    \xd0\x01\n\x0bHEHPIOGGIEG\x12\x12\n\x04uuid\x18\x0b\x20\x01(\tR\x04uuid\
    \x12\x20\n\x0bBDNFLICNFHN\x18\x0e\x20\x01(\x08R\x0bBDNFLICNFHN\x12.\n\
    \x0bHEFJEJHOJEA\x18\x07\x20\x01(\x0b2\x0c.HFPFFJIMCKMR\x0bHEFJEJHOJEA\
    \x12)\n\x0bOPCLDNLLNFH\x18\t\x20\x03(\x0b2\x07.VectorR\x0bOPCLDNLLNFH\
    \x12\x0e\n\x02id\x18\x0c\x20\x01(\rR\x02id\x12\x20\n\x0bLKJMLIDAODM\x18\
    \x05\x20\x01(\rR\x0bLKJMLIDAODMb\x06proto3\
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
            deps.push(super::HFPFFJIMCKM::file_descriptor().clone());
            deps.push(super::Vector::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HEHPIOGGIEG::generated_message_descriptor_data());
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
