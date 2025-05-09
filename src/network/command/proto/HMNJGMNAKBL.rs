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

//! Generated file from `HMNJGMNAKBL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:HMNJGMNAKBL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HMNJGMNAKBL {
    // message fields
    // @@protoc_insertion_point(field:HMNJGMNAKBL.lineup_list)
    pub lineup_list: ::std::vec::Vec<super::EIOAFODNCOJ::EIOAFODNCOJ>,
    // @@protoc_insertion_point(field:HMNJGMNAKBL.CBPJJPONCNG)
    pub CBPJJPONCNG: u32,
    // @@protoc_insertion_point(field:HMNJGMNAKBL.INHDDNNPBDB)
    pub INHDDNNPBDB: u32,
    // @@protoc_insertion_point(field:HMNJGMNAKBL.level)
    pub level: u32,
    // @@protoc_insertion_point(field:HMNJGMNAKBL.FOIGOJDODMD)
    pub FOIGOJDODMD: u32,
    // @@protoc_insertion_point(field:HMNJGMNAKBL.score)
    pub score: u32,
    // special fields
    // @@protoc_insertion_point(special_field:HMNJGMNAKBL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HMNJGMNAKBL {
    fn default() -> &'a HMNJGMNAKBL {
        <HMNJGMNAKBL as ::protobuf::Message>::default_instance()
    }
}

impl HMNJGMNAKBL {
    pub fn new() -> HMNJGMNAKBL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "lineup_list",
            |m: &HMNJGMNAKBL| { &m.lineup_list },
            |m: &mut HMNJGMNAKBL| { &mut m.lineup_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CBPJJPONCNG",
            |m: &HMNJGMNAKBL| { &m.CBPJJPONCNG },
            |m: &mut HMNJGMNAKBL| { &mut m.CBPJJPONCNG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "INHDDNNPBDB",
            |m: &HMNJGMNAKBL| { &m.INHDDNNPBDB },
            |m: &mut HMNJGMNAKBL| { &mut m.INHDDNNPBDB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &HMNJGMNAKBL| { &m.level },
            |m: &mut HMNJGMNAKBL| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FOIGOJDODMD",
            |m: &HMNJGMNAKBL| { &m.FOIGOJDODMD },
            |m: &mut HMNJGMNAKBL| { &mut m.FOIGOJDODMD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score",
            |m: &HMNJGMNAKBL| { &m.score },
            |m: &mut HMNJGMNAKBL| { &mut m.score },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HMNJGMNAKBL>(
            "HMNJGMNAKBL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HMNJGMNAKBL {
    const NAME: &'static str = "HMNJGMNAKBL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    self.lineup_list.push(is.read_message()?);
                },
                32 => {
                    self.CBPJJPONCNG = is.read_uint32()?;
                },
                64 => {
                    self.INHDDNNPBDB = is.read_uint32()?;
                },
                56 => {
                    self.level = is.read_uint32()?;
                },
                8 => {
                    self.FOIGOJDODMD = is.read_uint32()?;
                },
                16 => {
                    self.score = is.read_uint32()?;
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
        for value in &self.lineup_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.CBPJJPONCNG != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.CBPJJPONCNG);
        }
        if self.INHDDNNPBDB != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.INHDDNNPBDB);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.level);
        }
        if self.FOIGOJDODMD != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.FOIGOJDODMD);
        }
        if self.score != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.score);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.lineup_list {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.CBPJJPONCNG != 0 {
            os.write_uint32(4, self.CBPJJPONCNG)?;
        }
        if self.INHDDNNPBDB != 0 {
            os.write_uint32(8, self.INHDDNNPBDB)?;
        }
        if self.level != 0 {
            os.write_uint32(7, self.level)?;
        }
        if self.FOIGOJDODMD != 0 {
            os.write_uint32(1, self.FOIGOJDODMD)?;
        }
        if self.score != 0 {
            os.write_uint32(2, self.score)?;
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

    fn new() -> HMNJGMNAKBL {
        HMNJGMNAKBL::new()
    }

    fn clear(&mut self) {
        self.lineup_list.clear();
        self.CBPJJPONCNG = 0;
        self.INHDDNNPBDB = 0;
        self.level = 0;
        self.FOIGOJDODMD = 0;
        self.score = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HMNJGMNAKBL {
        static instance: HMNJGMNAKBL = HMNJGMNAKBL {
            lineup_list: ::std::vec::Vec::new(),
            CBPJJPONCNG: 0,
            INHDDNNPBDB: 0,
            level: 0,
            FOIGOJDODMD: 0,
            score: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HMNJGMNAKBL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HMNJGMNAKBL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HMNJGMNAKBL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HMNJGMNAKBL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11HMNJGMNAKBL.proto\x1a\x11EIOAFODNCOJ.proto\"\xce\x01\n\x0bHMNJGMNA\
    KBL\x12-\n\x0blineup_list\x18\t\x20\x03(\x0b2\x0c.EIOAFODNCOJR\nlineupLi\
    st\x12\x20\n\x0bCBPJJPONCNG\x18\x04\x20\x01(\rR\x0bCBPJJPONCNG\x12\x20\n\
    \x0bINHDDNNPBDB\x18\x08\x20\x01(\rR\x0bINHDDNNPBDB\x12\x14\n\x05level\
    \x18\x07\x20\x01(\rR\x05level\x12\x20\n\x0bFOIGOJDODMD\x18\x01\x20\x01(\
    \rR\x0bFOIGOJDODMD\x12\x14\n\x05score\x18\x02\x20\x01(\rR\x05scoreb\x06p\
    roto3\
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
            deps.push(super::EIOAFODNCOJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HMNJGMNAKBL::generated_message_descriptor_data());
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
