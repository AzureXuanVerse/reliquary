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

//! Generated file from `NBNJJABBCAN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NBNJJABBCAN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NBNJJABBCAN {
    // message fields
    // @@protoc_insertion_point(field:NBNJJABBCAN.OJBAILGKLBM)
    pub OJBAILGKLBM: ::protobuf::EnumOrUnknown<super::RogueTalentStatus::RogueTalentStatus>,
    // @@protoc_insertion_point(field:NBNJJABBCAN.FJEBNELLODE)
    pub FJEBNELLODE: u32,
    // @@protoc_insertion_point(field:NBNJJABBCAN.NPJJILJJMFA)
    pub NPJJILJJMFA: ::std::vec::Vec<super::RogueUnlockProgress::RogueUnlockProgress>,
    // special fields
    // @@protoc_insertion_point(special_field:NBNJJABBCAN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NBNJJABBCAN {
    fn default() -> &'a NBNJJABBCAN {
        <NBNJJABBCAN as ::protobuf::Message>::default_instance()
    }
}

impl NBNJJABBCAN {
    pub fn new() -> NBNJJABBCAN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OJBAILGKLBM",
            |m: &NBNJJABBCAN| { &m.OJBAILGKLBM },
            |m: &mut NBNJJABBCAN| { &mut m.OJBAILGKLBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJEBNELLODE",
            |m: &NBNJJABBCAN| { &m.FJEBNELLODE },
            |m: &mut NBNJJABBCAN| { &mut m.FJEBNELLODE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NPJJILJJMFA",
            |m: &NBNJJABBCAN| { &m.NPJJILJJMFA },
            |m: &mut NBNJJABBCAN| { &mut m.NPJJILJJMFA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NBNJJABBCAN>(
            "NBNJJABBCAN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NBNJJABBCAN {
    const NAME: &'static str = "NBNJJABBCAN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.OJBAILGKLBM = is.read_enum_or_unknown()?;
                },
                88 => {
                    self.FJEBNELLODE = is.read_uint32()?;
                },
                122 => {
                    self.NPJJILJJMFA.push(is.read_message()?);
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
        if self.OJBAILGKLBM != ::protobuf::EnumOrUnknown::new(super::RogueTalentStatus::RogueTalentStatus::ROGUE_TALENT_STATUS_LOCK) {
            my_size += ::protobuf::rt::int32_size(8, self.OJBAILGKLBM.value());
        }
        if self.FJEBNELLODE != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.FJEBNELLODE);
        }
        for value in &self.NPJJILJJMFA {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OJBAILGKLBM != ::protobuf::EnumOrUnknown::new(super::RogueTalentStatus::RogueTalentStatus::ROGUE_TALENT_STATUS_LOCK) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.OJBAILGKLBM))?;
        }
        if self.FJEBNELLODE != 0 {
            os.write_uint32(11, self.FJEBNELLODE)?;
        }
        for v in &self.NPJJILJJMFA {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> NBNJJABBCAN {
        NBNJJABBCAN::new()
    }

    fn clear(&mut self) {
        self.OJBAILGKLBM = ::protobuf::EnumOrUnknown::new(super::RogueTalentStatus::RogueTalentStatus::ROGUE_TALENT_STATUS_LOCK);
        self.FJEBNELLODE = 0;
        self.NPJJILJJMFA.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NBNJJABBCAN {
        static instance: NBNJJABBCAN = NBNJJABBCAN {
            OJBAILGKLBM: ::protobuf::EnumOrUnknown::from_i32(0),
            FJEBNELLODE: 0,
            NPJJILJJMFA: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for NBNJJABBCAN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NBNJJABBCAN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NBNJJABBCAN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NBNJJABBCAN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NBNJJABBCAN.proto\x1a\x17RogueTalentStatus.proto\x1a\x19RogueUnloc\
    kProgress.proto\"\x9d\x01\n\x0bNBNJJABBCAN\x124\n\x0bOJBAILGKLBM\x18\x08\
    \x20\x01(\x0e2\x12.RogueTalentStatusR\x0bOJBAILGKLBM\x12\x20\n\x0bFJEBNE\
    LLODE\x18\x0b\x20\x01(\rR\x0bFJEBNELLODE\x126\n\x0bNPJJILJJMFA\x18\x0f\
    \x20\x03(\x0b2\x14.RogueUnlockProgressR\x0bNPJJILJJMFAb\x06proto3\
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
            deps.push(super::RogueTalentStatus::file_descriptor().clone());
            deps.push(super::RogueUnlockProgress::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(NBNJJABBCAN::generated_message_descriptor_data());
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
