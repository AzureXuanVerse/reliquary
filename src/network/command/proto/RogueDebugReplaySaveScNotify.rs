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

//! Generated file from `RogueDebugReplaySaveScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:RogueDebugReplaySaveScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueDebugReplaySaveScNotify {
    // message fields
    // @@protoc_insertion_point(field:RogueDebugReplaySaveScNotify.DFPFALBJHJH)
    pub DFPFALBJHJH: ::std::string::String,
    // @@protoc_insertion_point(field:RogueDebugReplaySaveScNotify.JDEDCKKACGO)
    pub JDEDCKKACGO: ::std::string::String,
    // @@protoc_insertion_point(field:RogueDebugReplaySaveScNotify.NEPGEEJCLAH)
    pub NEPGEEJCLAH: ::std::string::String,
    // @@protoc_insertion_point(field:RogueDebugReplaySaveScNotify.JHJGFDMKIHG)
    pub JHJGFDMKIHG: ::std::string::String,
    // @@protoc_insertion_point(field:RogueDebugReplaySaveScNotify.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:RogueDebugReplaySaveScNotify.IJPPKNKNLNL)
    pub IJPPKNKNLNL: ::std::string::String,
    // @@protoc_insertion_point(field:RogueDebugReplaySaveScNotify.KFAMACKFHPM)
    pub KFAMACKFHPM: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:RogueDebugReplaySaveScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueDebugReplaySaveScNotify {
    fn default() -> &'a RogueDebugReplaySaveScNotify {
        <RogueDebugReplaySaveScNotify as ::protobuf::Message>::default_instance()
    }
}

impl RogueDebugReplaySaveScNotify {
    pub fn new() -> RogueDebugReplaySaveScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DFPFALBJHJH",
            |m: &RogueDebugReplaySaveScNotify| { &m.DFPFALBJHJH },
            |m: &mut RogueDebugReplaySaveScNotify| { &mut m.DFPFALBJHJH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JDEDCKKACGO",
            |m: &RogueDebugReplaySaveScNotify| { &m.JDEDCKKACGO },
            |m: &mut RogueDebugReplaySaveScNotify| { &mut m.JDEDCKKACGO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NEPGEEJCLAH",
            |m: &RogueDebugReplaySaveScNotify| { &m.NEPGEEJCLAH },
            |m: &mut RogueDebugReplaySaveScNotify| { &mut m.NEPGEEJCLAH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JHJGFDMKIHG",
            |m: &RogueDebugReplaySaveScNotify| { &m.JHJGFDMKIHG },
            |m: &mut RogueDebugReplaySaveScNotify| { &mut m.JHJGFDMKIHG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &RogueDebugReplaySaveScNotify| { &m.uid },
            |m: &mut RogueDebugReplaySaveScNotify| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IJPPKNKNLNL",
            |m: &RogueDebugReplaySaveScNotify| { &m.IJPPKNKNLNL },
            |m: &mut RogueDebugReplaySaveScNotify| { &mut m.IJPPKNKNLNL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KFAMACKFHPM",
            |m: &RogueDebugReplaySaveScNotify| { &m.KFAMACKFHPM },
            |m: &mut RogueDebugReplaySaveScNotify| { &mut m.KFAMACKFHPM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueDebugReplaySaveScNotify>(
            "RogueDebugReplaySaveScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueDebugReplaySaveScNotify {
    const NAME: &'static str = "RogueDebugReplaySaveScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                58 => {
                    self.DFPFALBJHJH = is.read_string()?;
                },
                10 => {
                    self.JDEDCKKACGO = is.read_string()?;
                },
                50 => {
                    self.NEPGEEJCLAH = is.read_string()?;
                },
                98 => {
                    self.JHJGFDMKIHG = is.read_string()?;
                },
                64 => {
                    self.uid = is.read_uint32()?;
                },
                106 => {
                    self.IJPPKNKNLNL = is.read_string()?;
                },
                82 => {
                    self.KFAMACKFHPM = is.read_string()?;
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
        if !self.DFPFALBJHJH.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.DFPFALBJHJH);
        }
        if !self.JDEDCKKACGO.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.JDEDCKKACGO);
        }
        if !self.NEPGEEJCLAH.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.NEPGEEJCLAH);
        }
        if !self.JHJGFDMKIHG.is_empty() {
            my_size += ::protobuf::rt::string_size(12, &self.JHJGFDMKIHG);
        }
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.uid);
        }
        if !self.IJPPKNKNLNL.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.IJPPKNKNLNL);
        }
        if !self.KFAMACKFHPM.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.KFAMACKFHPM);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.DFPFALBJHJH.is_empty() {
            os.write_string(7, &self.DFPFALBJHJH)?;
        }
        if !self.JDEDCKKACGO.is_empty() {
            os.write_string(1, &self.JDEDCKKACGO)?;
        }
        if !self.NEPGEEJCLAH.is_empty() {
            os.write_string(6, &self.NEPGEEJCLAH)?;
        }
        if !self.JHJGFDMKIHG.is_empty() {
            os.write_string(12, &self.JHJGFDMKIHG)?;
        }
        if self.uid != 0 {
            os.write_uint32(8, self.uid)?;
        }
        if !self.IJPPKNKNLNL.is_empty() {
            os.write_string(13, &self.IJPPKNKNLNL)?;
        }
        if !self.KFAMACKFHPM.is_empty() {
            os.write_string(10, &self.KFAMACKFHPM)?;
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

    fn new() -> RogueDebugReplaySaveScNotify {
        RogueDebugReplaySaveScNotify::new()
    }

    fn clear(&mut self) {
        self.DFPFALBJHJH.clear();
        self.JDEDCKKACGO.clear();
        self.NEPGEEJCLAH.clear();
        self.JHJGFDMKIHG.clear();
        self.uid = 0;
        self.IJPPKNKNLNL.clear();
        self.KFAMACKFHPM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueDebugReplaySaveScNotify {
        static instance: RogueDebugReplaySaveScNotify = RogueDebugReplaySaveScNotify {
            DFPFALBJHJH: ::std::string::String::new(),
            JDEDCKKACGO: ::std::string::String::new(),
            NEPGEEJCLAH: ::std::string::String::new(),
            JHJGFDMKIHG: ::std::string::String::new(),
            uid: 0,
            IJPPKNKNLNL: ::std::string::String::new(),
            KFAMACKFHPM: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueDebugReplaySaveScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueDebugReplaySaveScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueDebugReplaySaveScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueDebugReplaySaveScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"RogueDebugReplaySaveScNotify.proto\"\xfc\x01\n\x1cRogueDebugReplaySa\
    veScNotify\x12\x20\n\x0bDFPFALBJHJH\x18\x07\x20\x01(\tR\x0bDFPFALBJHJH\
    \x12\x20\n\x0bJDEDCKKACGO\x18\x01\x20\x01(\tR\x0bJDEDCKKACGO\x12\x20\n\
    \x0bNEPGEEJCLAH\x18\x06\x20\x01(\tR\x0bNEPGEEJCLAH\x12\x20\n\x0bJHJGFDMK\
    IHG\x18\x0c\x20\x01(\tR\x0bJHJGFDMKIHG\x12\x10\n\x03uid\x18\x08\x20\x01(\
    \rR\x03uid\x12\x20\n\x0bIJPPKNKNLNL\x18\r\x20\x01(\tR\x0bIJPPKNKNLNL\x12\
    \x20\n\x0bKFAMACKFHPM\x18\n\x20\x01(\tR\x0bKFAMACKFHPMb\x06proto3\
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
            messages.push(RogueDebugReplaySaveScNotify::generated_message_descriptor_data());
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
