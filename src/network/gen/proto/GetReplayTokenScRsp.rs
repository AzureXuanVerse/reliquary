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

//! Generated file from `GetReplayTokenScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetReplayTokenScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetReplayTokenScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetReplayTokenScRsp.LOGEBIDMFJD)
    pub LOGEBIDMFJD: ::protobuf::EnumOrUnknown<super::JAOPMDDBNHF::JAOPMDDBNHF>,
    // @@protoc_insertion_point(field:GetReplayTokenScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:GetReplayTokenScRsp.KAGEGBLHJDJ)
    pub KAGEGBLHJDJ: u32,
    // @@protoc_insertion_point(field:GetReplayTokenScRsp.BMIEOLBAOBG)
    pub BMIEOLBAOBG: ::std::string::String,
    // @@protoc_insertion_point(field:GetReplayTokenScRsp.FJCKEOKNBJL)
    pub FJCKEOKNBJL: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:GetReplayTokenScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetReplayTokenScRsp {
    fn default() -> &'a GetReplayTokenScRsp {
        <GetReplayTokenScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetReplayTokenScRsp {
    pub fn new() -> GetReplayTokenScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LOGEBIDMFJD",
            |m: &GetReplayTokenScRsp| { &m.LOGEBIDMFJD },
            |m: &mut GetReplayTokenScRsp| { &mut m.LOGEBIDMFJD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &GetReplayTokenScRsp| { &m.ADADHIHDHJC },
            |m: &mut GetReplayTokenScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KAGEGBLHJDJ",
            |m: &GetReplayTokenScRsp| { &m.KAGEGBLHJDJ },
            |m: &mut GetReplayTokenScRsp| { &mut m.KAGEGBLHJDJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BMIEOLBAOBG",
            |m: &GetReplayTokenScRsp| { &m.BMIEOLBAOBG },
            |m: &mut GetReplayTokenScRsp| { &mut m.BMIEOLBAOBG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJCKEOKNBJL",
            |m: &GetReplayTokenScRsp| { &m.FJCKEOKNBJL },
            |m: &mut GetReplayTokenScRsp| { &mut m.FJCKEOKNBJL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetReplayTokenScRsp>(
            "GetReplayTokenScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetReplayTokenScRsp {
    const NAME: &'static str = "GetReplayTokenScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.LOGEBIDMFJD = is.read_enum_or_unknown()?;
                },
                8 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                112 => {
                    self.KAGEGBLHJDJ = is.read_uint32()?;
                },
                58 => {
                    self.BMIEOLBAOBG = is.read_string()?;
                },
                34 => {
                    self.FJCKEOKNBJL = is.read_string()?;
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
        if self.LOGEBIDMFJD != ::protobuf::EnumOrUnknown::new(super::JAOPMDDBNHF::JAOPMDDBNHF::REPLAY_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.LOGEBIDMFJD.value());
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.ADADHIHDHJC);
        }
        if self.KAGEGBLHJDJ != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.KAGEGBLHJDJ);
        }
        if !self.BMIEOLBAOBG.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.BMIEOLBAOBG);
        }
        if !self.FJCKEOKNBJL.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.FJCKEOKNBJL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LOGEBIDMFJD != ::protobuf::EnumOrUnknown::new(super::JAOPMDDBNHF::JAOPMDDBNHF::REPLAY_TYPE_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.LOGEBIDMFJD))?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(1, self.ADADHIHDHJC)?;
        }
        if self.KAGEGBLHJDJ != 0 {
            os.write_uint32(14, self.KAGEGBLHJDJ)?;
        }
        if !self.BMIEOLBAOBG.is_empty() {
            os.write_string(7, &self.BMIEOLBAOBG)?;
        }
        if !self.FJCKEOKNBJL.is_empty() {
            os.write_string(4, &self.FJCKEOKNBJL)?;
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

    fn new() -> GetReplayTokenScRsp {
        GetReplayTokenScRsp::new()
    }

    fn clear(&mut self) {
        self.LOGEBIDMFJD = ::protobuf::EnumOrUnknown::new(super::JAOPMDDBNHF::JAOPMDDBNHF::REPLAY_TYPE_NONE);
        self.ADADHIHDHJC = 0;
        self.KAGEGBLHJDJ = 0;
        self.BMIEOLBAOBG.clear();
        self.FJCKEOKNBJL.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetReplayTokenScRsp {
        static instance: GetReplayTokenScRsp = GetReplayTokenScRsp {
            LOGEBIDMFJD: ::protobuf::EnumOrUnknown::from_i32(0),
            ADADHIHDHJC: 0,
            KAGEGBLHJDJ: 0,
            BMIEOLBAOBG: ::std::string::String::new(),
            FJCKEOKNBJL: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetReplayTokenScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetReplayTokenScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetReplayTokenScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetReplayTokenScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19GetReplayTokenScRsp.proto\x1a\x11JAOPMDDBNHF.proto\"\xcd\x01\n\x13\
    GetReplayTokenScRsp\x12.\n\x0bLOGEBIDMFJD\x18\x08\x20\x01(\x0e2\x0c.JAOP\
    MDDBNHFR\x0bLOGEBIDMFJD\x12\x20\n\x0bADADHIHDHJC\x18\x01\x20\x01(\rR\x0b\
    ADADHIHDHJC\x12\x20\n\x0bKAGEGBLHJDJ\x18\x0e\x20\x01(\rR\x0bKAGEGBLHJDJ\
    \x12\x20\n\x0bBMIEOLBAOBG\x18\x07\x20\x01(\tR\x0bBMIEOLBAOBG\x12\x20\n\
    \x0bFJCKEOKNBJL\x18\x04\x20\x01(\tR\x0bFJCKEOKNBJLb\x06proto3\
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
            deps.push(super::JAOPMDDBNHF::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetReplayTokenScRsp::generated_message_descriptor_data());
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
