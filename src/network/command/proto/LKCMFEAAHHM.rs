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

//! Generated file from `LKCMFEAAHHM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:LKCMFEAAHHM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LKCMFEAAHHM {
    // message fields
    // @@protoc_insertion_point(field:LKCMFEAAHHM.buff_list)
    pub buff_list: ::std::vec::Vec<super::GGGCOCPGBBH::GGGCOCPGBBH>,
    // @@protoc_insertion_point(field:LKCMFEAAHHM.locked)
    pub locked: bool,
    // @@protoc_insertion_point(field:LKCMFEAAHHM.LIMMILEAPJM)
    pub LIMMILEAPJM: bool,
    // @@protoc_insertion_point(field:LKCMFEAAHHM.HFNHLCFNHKD)
    pub HFNHLCFNHKD: u32,
    // @@protoc_insertion_point(field:LKCMFEAAHHM.KNLFELDECAL)
    pub KNLFELDECAL: bool,
    // @@protoc_insertion_point(field:LKCMFEAAHHM.OLLHOBHDDEN)
    pub OLLHOBHDDEN: u32,
    // @@protoc_insertion_point(field:LKCMFEAAHHM.DEMNCGLLJCP)
    pub DEMNCGLLJCP: bool,
    // @@protoc_insertion_point(field:LKCMFEAAHHM.MONHIBBPKEE)
    pub MONHIBBPKEE: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LKCMFEAAHHM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LKCMFEAAHHM {
    fn default() -> &'a LKCMFEAAHHM {
        <LKCMFEAAHHM as ::protobuf::Message>::default_instance()
    }
}

impl LKCMFEAAHHM {
    pub fn new() -> LKCMFEAAHHM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "buff_list",
            |m: &LKCMFEAAHHM| { &m.buff_list },
            |m: &mut LKCMFEAAHHM| { &mut m.buff_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "locked",
            |m: &LKCMFEAAHHM| { &m.locked },
            |m: &mut LKCMFEAAHHM| { &mut m.locked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LIMMILEAPJM",
            |m: &LKCMFEAAHHM| { &m.LIMMILEAPJM },
            |m: &mut LKCMFEAAHHM| { &mut m.LIMMILEAPJM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HFNHLCFNHKD",
            |m: &LKCMFEAAHHM| { &m.HFNHLCFNHKD },
            |m: &mut LKCMFEAAHHM| { &mut m.HFNHLCFNHKD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KNLFELDECAL",
            |m: &LKCMFEAAHHM| { &m.KNLFELDECAL },
            |m: &mut LKCMFEAAHHM| { &mut m.KNLFELDECAL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OLLHOBHDDEN",
            |m: &LKCMFEAAHHM| { &m.OLLHOBHDDEN },
            |m: &mut LKCMFEAAHHM| { &mut m.OLLHOBHDDEN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DEMNCGLLJCP",
            |m: &LKCMFEAAHHM| { &m.DEMNCGLLJCP },
            |m: &mut LKCMFEAAHHM| { &mut m.DEMNCGLLJCP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MONHIBBPKEE",
            |m: &LKCMFEAAHHM| { &m.MONHIBBPKEE },
            |m: &mut LKCMFEAAHHM| { &mut m.MONHIBBPKEE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LKCMFEAAHHM>(
            "LKCMFEAAHHM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LKCMFEAAHHM {
    const NAME: &'static str = "LKCMFEAAHHM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                3034 => {
                    self.buff_list.push(is.read_message()?);
                },
                120 => {
                    self.locked = is.read_bool()?;
                },
                112 => {
                    self.LIMMILEAPJM = is.read_bool()?;
                },
                56 => {
                    self.HFNHLCFNHKD = is.read_uint32()?;
                },
                88 => {
                    self.KNLFELDECAL = is.read_bool()?;
                },
                16 => {
                    self.OLLHOBHDDEN = is.read_uint32()?;
                },
                64 => {
                    self.DEMNCGLLJCP = is.read_bool()?;
                },
                96 => {
                    self.MONHIBBPKEE = is.read_uint32()?;
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
        for value in &self.buff_list {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.locked != false {
            my_size += 1 + 1;
        }
        if self.LIMMILEAPJM != false {
            my_size += 1 + 1;
        }
        if self.HFNHLCFNHKD != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.HFNHLCFNHKD);
        }
        if self.KNLFELDECAL != false {
            my_size += 1 + 1;
        }
        if self.OLLHOBHDDEN != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.OLLHOBHDDEN);
        }
        if self.DEMNCGLLJCP != false {
            my_size += 1 + 1;
        }
        if self.MONHIBBPKEE != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.MONHIBBPKEE);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.buff_list {
            ::protobuf::rt::write_message_field_with_cached_size(379, v, os)?;
        };
        if self.locked != false {
            os.write_bool(15, self.locked)?;
        }
        if self.LIMMILEAPJM != false {
            os.write_bool(14, self.LIMMILEAPJM)?;
        }
        if self.HFNHLCFNHKD != 0 {
            os.write_uint32(7, self.HFNHLCFNHKD)?;
        }
        if self.KNLFELDECAL != false {
            os.write_bool(11, self.KNLFELDECAL)?;
        }
        if self.OLLHOBHDDEN != 0 {
            os.write_uint32(2, self.OLLHOBHDDEN)?;
        }
        if self.DEMNCGLLJCP != false {
            os.write_bool(8, self.DEMNCGLLJCP)?;
        }
        if self.MONHIBBPKEE != 0 {
            os.write_uint32(12, self.MONHIBBPKEE)?;
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

    fn new() -> LKCMFEAAHHM {
        LKCMFEAAHHM::new()
    }

    fn clear(&mut self) {
        self.buff_list.clear();
        self.locked = false;
        self.LIMMILEAPJM = false;
        self.HFNHLCFNHKD = 0;
        self.KNLFELDECAL = false;
        self.OLLHOBHDDEN = 0;
        self.DEMNCGLLJCP = false;
        self.MONHIBBPKEE = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LKCMFEAAHHM {
        static instance: LKCMFEAAHHM = LKCMFEAAHHM {
            buff_list: ::std::vec::Vec::new(),
            locked: false,
            LIMMILEAPJM: false,
            HFNHLCFNHKD: 0,
            KNLFELDECAL: false,
            OLLHOBHDDEN: 0,
            DEMNCGLLJCP: false,
            MONHIBBPKEE: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LKCMFEAAHHM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LKCMFEAAHHM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LKCMFEAAHHM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LKCMFEAAHHM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11LKCMFEAAHHM.proto\x1a\x11GGGCOCPGBBH.proto\"\x9d\x02\n\x0bLKCMFEAA\
    HHM\x12*\n\tbuff_list\x18\xfb\x02\x20\x03(\x0b2\x0c.GGGCOCPGBBHR\x08buff\
    List\x12\x16\n\x06locked\x18\x0f\x20\x01(\x08R\x06locked\x12\x20\n\x0bLI\
    MMILEAPJM\x18\x0e\x20\x01(\x08R\x0bLIMMILEAPJM\x12\x20\n\x0bHFNHLCFNHKD\
    \x18\x07\x20\x01(\rR\x0bHFNHLCFNHKD\x12\x20\n\x0bKNLFELDECAL\x18\x0b\x20\
    \x01(\x08R\x0bKNLFELDECAL\x12\x20\n\x0bOLLHOBHDDEN\x18\x02\x20\x01(\rR\
    \x0bOLLHOBHDDEN\x12\x20\n\x0bDEMNCGLLJCP\x18\x08\x20\x01(\x08R\x0bDEMNCG\
    LLJCP\x12\x20\n\x0bMONHIBBPKEE\x18\x0c\x20\x01(\rR\x0bMONHIBBPKEEb\x06pr\
    oto3\
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
            deps.push(super::GGGCOCPGBBH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LKCMFEAAHHM::generated_message_descriptor_data());
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
