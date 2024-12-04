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

//! Generated file from `PLDBMBNBHJE.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PLDBMBNBHJE)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PLDBMBNBHJE {
    // message fields
    // @@protoc_insertion_point(field:PLDBMBNBHJE.COIFBDJBCDE)
    pub COIFBDJBCDE: u32,
    // @@protoc_insertion_point(field:PLDBMBNBHJE.LJEJAHAEADN)
    pub LJEJAHAEADN: ::protobuf::MessageField<super::GCPAMIEDFIP::GCPAMIEDFIP>,
    // @@protoc_insertion_point(field:PLDBMBNBHJE.MOOJJGAEBGC)
    pub MOOJJGAEBGC: bool,
    // @@protoc_insertion_point(field:PLDBMBNBHJE.INMBFKKDMCD)
    pub INMBFKKDMCD: ::std::vec::Vec<super::CPKCMLKPMLI::CPKCMLKPMLI>,
    // @@protoc_insertion_point(field:PLDBMBNBHJE.LPDKBAPGNBP)
    pub LPDKBAPGNBP: u32,
    // @@protoc_insertion_point(field:PLDBMBNBHJE.DGNPBMHAOGF)
    pub DGNPBMHAOGF: u32,
    // @@protoc_insertion_point(field:PLDBMBNBHJE.KBDFLKCBFKA)
    pub KBDFLKCBFKA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PLDBMBNBHJE.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PLDBMBNBHJE {
    fn default() -> &'a PLDBMBNBHJE {
        <PLDBMBNBHJE as ::protobuf::Message>::default_instance()
    }
}

impl PLDBMBNBHJE {
    pub fn new() -> PLDBMBNBHJE {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "COIFBDJBCDE",
            |m: &PLDBMBNBHJE| { &m.COIFBDJBCDE },
            |m: &mut PLDBMBNBHJE| { &mut m.COIFBDJBCDE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::GCPAMIEDFIP::GCPAMIEDFIP>(
            "LJEJAHAEADN",
            |m: &PLDBMBNBHJE| { &m.LJEJAHAEADN },
            |m: &mut PLDBMBNBHJE| { &mut m.LJEJAHAEADN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MOOJJGAEBGC",
            |m: &PLDBMBNBHJE| { &m.MOOJJGAEBGC },
            |m: &mut PLDBMBNBHJE| { &mut m.MOOJJGAEBGC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "INMBFKKDMCD",
            |m: &PLDBMBNBHJE| { &m.INMBFKKDMCD },
            |m: &mut PLDBMBNBHJE| { &mut m.INMBFKKDMCD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LPDKBAPGNBP",
            |m: &PLDBMBNBHJE| { &m.LPDKBAPGNBP },
            |m: &mut PLDBMBNBHJE| { &mut m.LPDKBAPGNBP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DGNPBMHAOGF",
            |m: &PLDBMBNBHJE| { &m.DGNPBMHAOGF },
            |m: &mut PLDBMBNBHJE| { &mut m.DGNPBMHAOGF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KBDFLKCBFKA",
            |m: &PLDBMBNBHJE| { &m.KBDFLKCBFKA },
            |m: &mut PLDBMBNBHJE| { &mut m.KBDFLKCBFKA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PLDBMBNBHJE>(
            "PLDBMBNBHJE",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PLDBMBNBHJE {
    const NAME: &'static str = "PLDBMBNBHJE";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.COIFBDJBCDE = is.read_uint32()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.LJEJAHAEADN)?;
                },
                48 => {
                    self.MOOJJGAEBGC = is.read_bool()?;
                },
                10 => {
                    self.INMBFKKDMCD.push(is.read_message()?);
                },
                64 => {
                    self.LPDKBAPGNBP = is.read_uint32()?;
                },
                112 => {
                    self.DGNPBMHAOGF = is.read_uint32()?;
                },
                96 => {
                    self.KBDFLKCBFKA = is.read_uint32()?;
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
        if self.COIFBDJBCDE != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.COIFBDJBCDE);
        }
        if let Some(v) = self.LJEJAHAEADN.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.MOOJJGAEBGC != false {
            my_size += 1 + 1;
        }
        for value in &self.INMBFKKDMCD {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.LPDKBAPGNBP != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.LPDKBAPGNBP);
        }
        if self.DGNPBMHAOGF != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.DGNPBMHAOGF);
        }
        if self.KBDFLKCBFKA != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.KBDFLKCBFKA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.COIFBDJBCDE != 0 {
            os.write_uint32(7, self.COIFBDJBCDE)?;
        }
        if let Some(v) = self.LJEJAHAEADN.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if self.MOOJJGAEBGC != false {
            os.write_bool(6, self.MOOJJGAEBGC)?;
        }
        for v in &self.INMBFKKDMCD {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.LPDKBAPGNBP != 0 {
            os.write_uint32(8, self.LPDKBAPGNBP)?;
        }
        if self.DGNPBMHAOGF != 0 {
            os.write_uint32(14, self.DGNPBMHAOGF)?;
        }
        if self.KBDFLKCBFKA != 0 {
            os.write_uint32(12, self.KBDFLKCBFKA)?;
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

    fn new() -> PLDBMBNBHJE {
        PLDBMBNBHJE::new()
    }

    fn clear(&mut self) {
        self.COIFBDJBCDE = 0;
        self.LJEJAHAEADN.clear();
        self.MOOJJGAEBGC = false;
        self.INMBFKKDMCD.clear();
        self.LPDKBAPGNBP = 0;
        self.DGNPBMHAOGF = 0;
        self.KBDFLKCBFKA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PLDBMBNBHJE {
        static instance: PLDBMBNBHJE = PLDBMBNBHJE {
            COIFBDJBCDE: 0,
            LJEJAHAEADN: ::protobuf::MessageField::none(),
            MOOJJGAEBGC: false,
            INMBFKKDMCD: ::std::vec::Vec::new(),
            LPDKBAPGNBP: 0,
            DGNPBMHAOGF: 0,
            KBDFLKCBFKA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PLDBMBNBHJE {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PLDBMBNBHJE").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PLDBMBNBHJE {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PLDBMBNBHJE {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PLDBMBNBHJE.proto\x1a\x11CPKCMLKPMLI.proto\x1a\x11GCPAMIEDFIP.prot\
    o\"\x97\x02\n\x0bPLDBMBNBHJE\x12\x20\n\x0bCOIFBDJBCDE\x18\x07\x20\x01(\r\
    R\x0bCOIFBDJBCDE\x12.\n\x0bLJEJAHAEADN\x18\x0f\x20\x01(\x0b2\x0c.GCPAMIE\
    DFIPR\x0bLJEJAHAEADN\x12\x20\n\x0bMOOJJGAEBGC\x18\x06\x20\x01(\x08R\x0bM\
    OOJJGAEBGC\x12.\n\x0bINMBFKKDMCD\x18\x01\x20\x03(\x0b2\x0c.CPKCMLKPMLIR\
    \x0bINMBFKKDMCD\x12\x20\n\x0bLPDKBAPGNBP\x18\x08\x20\x01(\rR\x0bLPDKBAPG\
    NBP\x12\x20\n\x0bDGNPBMHAOGF\x18\x0e\x20\x01(\rR\x0bDGNPBMHAOGF\x12\x20\
    \n\x0bKBDFLKCBFKA\x18\x0c\x20\x01(\rR\x0bKBDFLKCBFKAb\x06proto3\
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
            deps.push(super::CPKCMLKPMLI::file_descriptor().clone());
            deps.push(super::GCPAMIEDFIP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PLDBMBNBHJE::generated_message_descriptor_data());
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
