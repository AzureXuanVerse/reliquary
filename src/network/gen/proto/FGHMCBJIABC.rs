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

//! Generated file from `FGHMCBJIABC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FGHMCBJIABC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FGHMCBJIABC {
    // message fields
    // @@protoc_insertion_point(field:FGHMCBJIABC.HCBLBJJAKHJ)
    pub HCBLBJJAKHJ: f64,
    // @@protoc_insertion_point(field:FGHMCBJIABC.TURN_FOOD_SWITCH_ATTACK)
    pub TURN_FOOD_SWITCH_ATTACK: f64,
    // @@protoc_insertion_point(field:FGHMCBJIABC.HJPOELOJIIG)
    pub HJPOELOJIIG: f64,
    // @@protoc_insertion_point(field:FGHMCBJIABC.JANJNGLGAFB)
    pub JANJNGLGAFB: f64,
    // @@protoc_insertion_point(field:FGHMCBJIABC.DJJIBIEPNAK)
    pub DJJIBIEPNAK: f64,
    // @@protoc_insertion_point(field:FGHMCBJIABC.FMOLMBHJOJP)
    pub FMOLMBHJOJP: f64,
    // @@protoc_insertion_point(field:FGHMCBJIABC.HLMOECPIDHF)
    pub HLMOECPIDHF: f64,
    // special fields
    // @@protoc_insertion_point(special_field:FGHMCBJIABC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FGHMCBJIABC {
    fn default() -> &'a FGHMCBJIABC {
        <FGHMCBJIABC as ::protobuf::Message>::default_instance()
    }
}

impl FGHMCBJIABC {
    pub fn new() -> FGHMCBJIABC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HCBLBJJAKHJ",
            |m: &FGHMCBJIABC| { &m.HCBLBJJAKHJ },
            |m: &mut FGHMCBJIABC| { &mut m.HCBLBJJAKHJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "TURN_FOOD_SWITCH_ATTACK",
            |m: &FGHMCBJIABC| { &m.TURN_FOOD_SWITCH_ATTACK },
            |m: &mut FGHMCBJIABC| { &mut m.TURN_FOOD_SWITCH_ATTACK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HJPOELOJIIG",
            |m: &FGHMCBJIABC| { &m.HJPOELOJIIG },
            |m: &mut FGHMCBJIABC| { &mut m.HJPOELOJIIG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JANJNGLGAFB",
            |m: &FGHMCBJIABC| { &m.JANJNGLGAFB },
            |m: &mut FGHMCBJIABC| { &mut m.JANJNGLGAFB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DJJIBIEPNAK",
            |m: &FGHMCBJIABC| { &m.DJJIBIEPNAK },
            |m: &mut FGHMCBJIABC| { &mut m.DJJIBIEPNAK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FMOLMBHJOJP",
            |m: &FGHMCBJIABC| { &m.FMOLMBHJOJP },
            |m: &mut FGHMCBJIABC| { &mut m.FMOLMBHJOJP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HLMOECPIDHF",
            |m: &FGHMCBJIABC| { &m.HLMOECPIDHF },
            |m: &mut FGHMCBJIABC| { &mut m.HLMOECPIDHF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FGHMCBJIABC>(
            "FGHMCBJIABC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FGHMCBJIABC {
    const NAME: &'static str = "FGHMCBJIABC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                9 => {
                    self.HCBLBJJAKHJ = is.read_double()?;
                },
                17 => {
                    self.TURN_FOOD_SWITCH_ATTACK = is.read_double()?;
                },
                25 => {
                    self.HJPOELOJIIG = is.read_double()?;
                },
                33 => {
                    self.JANJNGLGAFB = is.read_double()?;
                },
                41 => {
                    self.DJJIBIEPNAK = is.read_double()?;
                },
                49 => {
                    self.FMOLMBHJOJP = is.read_double()?;
                },
                57 => {
                    self.HLMOECPIDHF = is.read_double()?;
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
        if self.HCBLBJJAKHJ != 0. {
            my_size += 1 + 8;
        }
        if self.TURN_FOOD_SWITCH_ATTACK != 0. {
            my_size += 1 + 8;
        }
        if self.HJPOELOJIIG != 0. {
            my_size += 1 + 8;
        }
        if self.JANJNGLGAFB != 0. {
            my_size += 1 + 8;
        }
        if self.DJJIBIEPNAK != 0. {
            my_size += 1 + 8;
        }
        if self.FMOLMBHJOJP != 0. {
            my_size += 1 + 8;
        }
        if self.HLMOECPIDHF != 0. {
            my_size += 1 + 8;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.HCBLBJJAKHJ != 0. {
            os.write_double(1, self.HCBLBJJAKHJ)?;
        }
        if self.TURN_FOOD_SWITCH_ATTACK != 0. {
            os.write_double(2, self.TURN_FOOD_SWITCH_ATTACK)?;
        }
        if self.HJPOELOJIIG != 0. {
            os.write_double(3, self.HJPOELOJIIG)?;
        }
        if self.JANJNGLGAFB != 0. {
            os.write_double(4, self.JANJNGLGAFB)?;
        }
        if self.DJJIBIEPNAK != 0. {
            os.write_double(5, self.DJJIBIEPNAK)?;
        }
        if self.FMOLMBHJOJP != 0. {
            os.write_double(6, self.FMOLMBHJOJP)?;
        }
        if self.HLMOECPIDHF != 0. {
            os.write_double(7, self.HLMOECPIDHF)?;
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

    fn new() -> FGHMCBJIABC {
        FGHMCBJIABC::new()
    }

    fn clear(&mut self) {
        self.HCBLBJJAKHJ = 0.;
        self.TURN_FOOD_SWITCH_ATTACK = 0.;
        self.HJPOELOJIIG = 0.;
        self.JANJNGLGAFB = 0.;
        self.DJJIBIEPNAK = 0.;
        self.FMOLMBHJOJP = 0.;
        self.HLMOECPIDHF = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FGHMCBJIABC {
        static instance: FGHMCBJIABC = FGHMCBJIABC {
            HCBLBJJAKHJ: 0.,
            TURN_FOOD_SWITCH_ATTACK: 0.,
            HJPOELOJIIG: 0.,
            JANJNGLGAFB: 0.,
            DJJIBIEPNAK: 0.,
            FMOLMBHJOJP: 0.,
            HLMOECPIDHF: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FGHMCBJIABC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FGHMCBJIABC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FGHMCBJIABC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FGHMCBJIABC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FGHMCBJIABC.proto\"\x90\x02\n\x0bFGHMCBJIABC\x12\x20\n\x0bHCBLBJJA\
    KHJ\x18\x01\x20\x01(\x01R\x0bHCBLBJJAKHJ\x125\n\x17TURN_FOOD_SWITCH_ATTA\
    CK\x18\x02\x20\x01(\x01R\x14TURNFOODSWITCHATTACK\x12\x20\n\x0bHJPOELOJII\
    G\x18\x03\x20\x01(\x01R\x0bHJPOELOJIIG\x12\x20\n\x0bJANJNGLGAFB\x18\x04\
    \x20\x01(\x01R\x0bJANJNGLGAFB\x12\x20\n\x0bDJJIBIEPNAK\x18\x05\x20\x01(\
    \x01R\x0bDJJIBIEPNAK\x12\x20\n\x0bFMOLMBHJOJP\x18\x06\x20\x01(\x01R\x0bF\
    MOLMBHJOJP\x12\x20\n\x0bHLMOECPIDHF\x18\x07\x20\x01(\x01R\x0bHLMOECPIDHF\
    b\x06proto3\
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
            messages.push(FGHMCBJIABC::generated_message_descriptor_data());
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
