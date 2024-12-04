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

//! Generated file from `GMELPKNHKIO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GMELPKNHKIO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GMELPKNHKIO {
    // message fields
    // @@protoc_insertion_point(field:GMELPKNHKIO.NHLHMBLGMHB)
    pub NHLHMBLGMHB: u32,
    // @@protoc_insertion_point(field:GMELPKNHKIO.COAOFGPMPEH)
    pub COAOFGPMPEH: u32,
    // @@protoc_insertion_point(field:GMELPKNHKIO.IPEALHJLCHH)
    pub IPEALHJLCHH: ::protobuf::EnumOrUnknown<super::RogueRoomStatus::RogueRoomStatus>,
    // @@protoc_insertion_point(field:GMELPKNHKIO.HGMFEGGJEFB)
    pub HGMFEGGJEFB: u32,
    // @@protoc_insertion_point(field:GMELPKNHKIO.POMBBDDANNJ)
    pub POMBBDDANNJ: ::protobuf::EnumOrUnknown<super::RogueRoomStatus::RogueRoomStatus>,
    // special fields
    // @@protoc_insertion_point(special_field:GMELPKNHKIO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GMELPKNHKIO {
    fn default() -> &'a GMELPKNHKIO {
        <GMELPKNHKIO as ::protobuf::Message>::default_instance()
    }
}

impl GMELPKNHKIO {
    pub fn new() -> GMELPKNHKIO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NHLHMBLGMHB",
            |m: &GMELPKNHKIO| { &m.NHLHMBLGMHB },
            |m: &mut GMELPKNHKIO| { &mut m.NHLHMBLGMHB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "COAOFGPMPEH",
            |m: &GMELPKNHKIO| { &m.COAOFGPMPEH },
            |m: &mut GMELPKNHKIO| { &mut m.COAOFGPMPEH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IPEALHJLCHH",
            |m: &GMELPKNHKIO| { &m.IPEALHJLCHH },
            |m: &mut GMELPKNHKIO| { &mut m.IPEALHJLCHH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HGMFEGGJEFB",
            |m: &GMELPKNHKIO| { &m.HGMFEGGJEFB },
            |m: &mut GMELPKNHKIO| { &mut m.HGMFEGGJEFB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "POMBBDDANNJ",
            |m: &GMELPKNHKIO| { &m.POMBBDDANNJ },
            |m: &mut GMELPKNHKIO| { &mut m.POMBBDDANNJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GMELPKNHKIO>(
            "GMELPKNHKIO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GMELPKNHKIO {
    const NAME: &'static str = "GMELPKNHKIO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.NHLHMBLGMHB = is.read_uint32()?;
                },
                32 => {
                    self.COAOFGPMPEH = is.read_uint32()?;
                },
                80 => {
                    self.IPEALHJLCHH = is.read_enum_or_unknown()?;
                },
                112 => {
                    self.HGMFEGGJEFB = is.read_uint32()?;
                },
                56 => {
                    self.POMBBDDANNJ = is.read_enum_or_unknown()?;
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
        if self.NHLHMBLGMHB != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.NHLHMBLGMHB);
        }
        if self.COAOFGPMPEH != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.COAOFGPMPEH);
        }
        if self.IPEALHJLCHH != ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(10, self.IPEALHJLCHH.value());
        }
        if self.HGMFEGGJEFB != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.HGMFEGGJEFB);
        }
        if self.POMBBDDANNJ != ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(7, self.POMBBDDANNJ.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NHLHMBLGMHB != 0 {
            os.write_uint32(12, self.NHLHMBLGMHB)?;
        }
        if self.COAOFGPMPEH != 0 {
            os.write_uint32(4, self.COAOFGPMPEH)?;
        }
        if self.IPEALHJLCHH != ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.IPEALHJLCHH))?;
        }
        if self.HGMFEGGJEFB != 0 {
            os.write_uint32(14, self.HGMFEGGJEFB)?;
        }
        if self.POMBBDDANNJ != ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.POMBBDDANNJ))?;
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

    fn new() -> GMELPKNHKIO {
        GMELPKNHKIO::new()
    }

    fn clear(&mut self) {
        self.NHLHMBLGMHB = 0;
        self.COAOFGPMPEH = 0;
        self.IPEALHJLCHH = ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE);
        self.HGMFEGGJEFB = 0;
        self.POMBBDDANNJ = ::protobuf::EnumOrUnknown::new(super::RogueRoomStatus::RogueRoomStatus::ROGUE_ROOM_STATUS_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GMELPKNHKIO {
        static instance: GMELPKNHKIO = GMELPKNHKIO {
            NHLHMBLGMHB: 0,
            COAOFGPMPEH: 0,
            IPEALHJLCHH: ::protobuf::EnumOrUnknown::from_i32(0),
            HGMFEGGJEFB: 0,
            POMBBDDANNJ: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GMELPKNHKIO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GMELPKNHKIO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GMELPKNHKIO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GMELPKNHKIO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GMELPKNHKIO.proto\x1a\x15RogueRoomStatus.proto\"\xdb\x01\n\x0bGMEL\
    PKNHKIO\x12\x20\n\x0bNHLHMBLGMHB\x18\x0c\x20\x01(\rR\x0bNHLHMBLGMHB\x12\
    \x20\n\x0bCOAOFGPMPEH\x18\x04\x20\x01(\rR\x0bCOAOFGPMPEH\x122\n\x0bIPEAL\
    HJLCHH\x18\n\x20\x01(\x0e2\x10.RogueRoomStatusR\x0bIPEALHJLCHH\x12\x20\n\
    \x0bHGMFEGGJEFB\x18\x0e\x20\x01(\rR\x0bHGMFEGGJEFB\x122\n\x0bPOMBBDDANNJ\
    \x18\x07\x20\x01(\x0e2\x10.RogueRoomStatusR\x0bPOMBBDDANNJb\x06proto3\
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
            deps.push(super::RogueRoomStatus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GMELPKNHKIO::generated_message_descriptor_data());
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
