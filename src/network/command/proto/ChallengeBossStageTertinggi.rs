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

//! Generated file from `ChallengeBossStageTertinggi.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ChallengeBossStageTertinggi)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChallengeBossStageTertinggi {
    // message fields
    // @@protoc_insertion_point(field:ChallengeBossStageTertinggi.buff_one)
    pub buff_one: u32,
    // @@protoc_insertion_point(field:ChallengeBossStageTertinggi.lineup_list)
    pub lineup_list: ::std::vec::Vec<super::ChallengeLineupList::ChallengeLineupList>,
    // @@protoc_insertion_point(field:ChallengeBossStageTertinggi.level)
    pub level: u32,
    // @@protoc_insertion_point(field:ChallengeBossStageTertinggi.score_id)
    pub score_id: u32,
    // @@protoc_insertion_point(field:ChallengeBossStageTertinggi.buff_two)
    pub buff_two: u32,
    // @@protoc_insertion_point(field:ChallengeBossStageTertinggi.INHDDNNPBDB)
    pub INHDDNNPBDB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ChallengeBossStageTertinggi.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChallengeBossStageTertinggi {
    fn default() -> &'a ChallengeBossStageTertinggi {
        <ChallengeBossStageTertinggi as ::protobuf::Message>::default_instance()
    }
}

impl ChallengeBossStageTertinggi {
    pub fn new() -> ChallengeBossStageTertinggi {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "buff_one",
            |m: &ChallengeBossStageTertinggi| { &m.buff_one },
            |m: &mut ChallengeBossStageTertinggi| { &mut m.buff_one },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "lineup_list",
            |m: &ChallengeBossStageTertinggi| { &m.lineup_list },
            |m: &mut ChallengeBossStageTertinggi| { &mut m.lineup_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &ChallengeBossStageTertinggi| { &m.level },
            |m: &mut ChallengeBossStageTertinggi| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score_id",
            |m: &ChallengeBossStageTertinggi| { &m.score_id },
            |m: &mut ChallengeBossStageTertinggi| { &mut m.score_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "buff_two",
            |m: &ChallengeBossStageTertinggi| { &m.buff_two },
            |m: &mut ChallengeBossStageTertinggi| { &mut m.buff_two },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "INHDDNNPBDB",
            |m: &ChallengeBossStageTertinggi| { &m.INHDDNNPBDB },
            |m: &mut ChallengeBossStageTertinggi| { &mut m.INHDDNNPBDB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChallengeBossStageTertinggi>(
            "ChallengeBossStageTertinggi",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChallengeBossStageTertinggi {
    const NAME: &'static str = "ChallengeBossStageTertinggi";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.buff_one = is.read_uint32()?;
                },
                106 => {
                    self.lineup_list.push(is.read_message()?);
                },
                120 => {
                    self.level = is.read_uint32()?;
                },
                32 => {
                    self.score_id = is.read_uint32()?;
                },
                72 => {
                    self.buff_two = is.read_uint32()?;
                },
                24 => {
                    self.INHDDNNPBDB = is.read_uint32()?;
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
        if self.buff_one != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.buff_one);
        }
        for value in &self.lineup_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.level);
        }
        if self.score_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.score_id);
        }
        if self.buff_two != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.buff_two);
        }
        if self.INHDDNNPBDB != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.INHDDNNPBDB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.buff_one != 0 {
            os.write_uint32(1, self.buff_one)?;
        }
        for v in &self.lineup_list {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.level != 0 {
            os.write_uint32(15, self.level)?;
        }
        if self.score_id != 0 {
            os.write_uint32(4, self.score_id)?;
        }
        if self.buff_two != 0 {
            os.write_uint32(9, self.buff_two)?;
        }
        if self.INHDDNNPBDB != 0 {
            os.write_uint32(3, self.INHDDNNPBDB)?;
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

    fn new() -> ChallengeBossStageTertinggi {
        ChallengeBossStageTertinggi::new()
    }

    fn clear(&mut self) {
        self.buff_one = 0;
        self.lineup_list.clear();
        self.level = 0;
        self.score_id = 0;
        self.buff_two = 0;
        self.INHDDNNPBDB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChallengeBossStageTertinggi {
        static instance: ChallengeBossStageTertinggi = ChallengeBossStageTertinggi {
            buff_one: 0,
            lineup_list: ::std::vec::Vec::new(),
            level: 0,
            score_id: 0,
            buff_two: 0,
            INHDDNNPBDB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChallengeBossStageTertinggi {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChallengeBossStageTertinggi").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChallengeBossStageTertinggi {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChallengeBossStageTertinggi {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!ChallengeBossStageTertinggi.proto\x1a\x19ChallengeLineupList.proto\"\
    \xdd\x01\n\x1bChallengeBossStageTertinggi\x12\x19\n\x08buff_one\x18\x01\
    \x20\x01(\rR\x07buffOne\x125\n\x0blineup_list\x18\r\x20\x03(\x0b2\x14.Ch\
    allengeLineupListR\nlineupList\x12\x14\n\x05level\x18\x0f\x20\x01(\rR\
    \x05level\x12\x19\n\x08score_id\x18\x04\x20\x01(\rR\x07scoreId\x12\x19\n\
    \x08buff_two\x18\t\x20\x01(\rR\x07buffTwo\x12\x20\n\x0bINHDDNNPBDB\x18\
    \x03\x20\x01(\rR\x0bINHDDNNPBDBb\x06proto3\
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
            deps.push(super::ChallengeLineupList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChallengeBossStageTertinggi::generated_message_descriptor_data());
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
