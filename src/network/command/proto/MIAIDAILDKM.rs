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

//! Generated file from `MIAIDAILDKM.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:MIAIDAILDKM)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MIAIDAILDKM {
    // message fields
    // @@protoc_insertion_point(field:MIAIDAILDKM.EEFLGHCOBML)
    pub EEFLGHCOBML: ::std::vec::Vec<super::HBLDPBBKHPB::HBLDPBBKHPB>,
    // @@protoc_insertion_point(field:MIAIDAILDKM.KGBHEHHFMPI)
    pub KGBHEHHFMPI: ::std::string::String,
    // @@protoc_insertion_point(field:MIAIDAILDKM.CPCLJGPDNAN)
    pub CPCLJGPDNAN: ::std::string::String,
    // @@protoc_insertion_point(field:MIAIDAILDKM.HLAGIMENBJG)
    pub HLAGIMENBJG: ::std::vec::Vec<super::FEFDAHEOOKP::FEFDAHEOOKP>,
    // @@protoc_insertion_point(field:MIAIDAILDKM.plane_id)
    pub plane_id: u32,
    // @@protoc_insertion_point(field:MIAIDAILDKM.floor_id)
    pub floor_id: u32,
    // @@protoc_insertion_point(field:MIAIDAILDKM.EBHLFAEGLCD)
    pub EBHLFAEGLCD: u32,
    // @@protoc_insertion_point(field:MIAIDAILDKM.BNJMMLKOFCP)
    pub BNJMMLKOFCP: u32,
    // @@protoc_insertion_point(field:MIAIDAILDKM.EDHBGDEICNC)
    pub EDHBGDEICNC: ::protobuf::MessageField<super::MBMCFOLIOLO::MBMCFOLIOLO>,
    // special fields
    // @@protoc_insertion_point(special_field:MIAIDAILDKM.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MIAIDAILDKM {
    fn default() -> &'a MIAIDAILDKM {
        <MIAIDAILDKM as ::protobuf::Message>::default_instance()
    }
}

impl MIAIDAILDKM {
    pub fn new() -> MIAIDAILDKM {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EEFLGHCOBML",
            |m: &MIAIDAILDKM| { &m.EEFLGHCOBML },
            |m: &mut MIAIDAILDKM| { &mut m.EEFLGHCOBML },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KGBHEHHFMPI",
            |m: &MIAIDAILDKM| { &m.KGBHEHHFMPI },
            |m: &mut MIAIDAILDKM| { &mut m.KGBHEHHFMPI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CPCLJGPDNAN",
            |m: &MIAIDAILDKM| { &m.CPCLJGPDNAN },
            |m: &mut MIAIDAILDKM| { &mut m.CPCLJGPDNAN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HLAGIMENBJG",
            |m: &MIAIDAILDKM| { &m.HLAGIMENBJG },
            |m: &mut MIAIDAILDKM| { &mut m.HLAGIMENBJG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "plane_id",
            |m: &MIAIDAILDKM| { &m.plane_id },
            |m: &mut MIAIDAILDKM| { &mut m.plane_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "floor_id",
            |m: &MIAIDAILDKM| { &m.floor_id },
            |m: &mut MIAIDAILDKM| { &mut m.floor_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EBHLFAEGLCD",
            |m: &MIAIDAILDKM| { &m.EBHLFAEGLCD },
            |m: &mut MIAIDAILDKM| { &mut m.EBHLFAEGLCD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BNJMMLKOFCP",
            |m: &MIAIDAILDKM| { &m.BNJMMLKOFCP },
            |m: &mut MIAIDAILDKM| { &mut m.BNJMMLKOFCP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MBMCFOLIOLO::MBMCFOLIOLO>(
            "EDHBGDEICNC",
            |m: &MIAIDAILDKM| { &m.EDHBGDEICNC },
            |m: &mut MIAIDAILDKM| { &mut m.EDHBGDEICNC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MIAIDAILDKM>(
            "MIAIDAILDKM",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MIAIDAILDKM {
    const NAME: &'static str = "MIAIDAILDKM";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.EEFLGHCOBML.push(is.read_message()?);
                },
                18 => {
                    self.KGBHEHHFMPI = is.read_string()?;
                },
                26 => {
                    self.CPCLJGPDNAN = is.read_string()?;
                },
                34 => {
                    self.HLAGIMENBJG.push(is.read_message()?);
                },
                40 => {
                    self.plane_id = is.read_uint32()?;
                },
                48 => {
                    self.floor_id = is.read_uint32()?;
                },
                56 => {
                    self.EBHLFAEGLCD = is.read_uint32()?;
                },
                64 => {
                    self.BNJMMLKOFCP = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EDHBGDEICNC)?;
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
        for value in &self.EEFLGHCOBML {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if !self.KGBHEHHFMPI.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.KGBHEHHFMPI);
        }
        if !self.CPCLJGPDNAN.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.CPCLJGPDNAN);
        }
        for value in &self.HLAGIMENBJG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.plane_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.plane_id);
        }
        if self.floor_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.floor_id);
        }
        if self.EBHLFAEGLCD != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.EBHLFAEGLCD);
        }
        if self.BNJMMLKOFCP != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.BNJMMLKOFCP);
        }
        if let Some(v) = self.EDHBGDEICNC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.EEFLGHCOBML {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if !self.KGBHEHHFMPI.is_empty() {
            os.write_string(2, &self.KGBHEHHFMPI)?;
        }
        if !self.CPCLJGPDNAN.is_empty() {
            os.write_string(3, &self.CPCLJGPDNAN)?;
        }
        for v in &self.HLAGIMENBJG {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.plane_id != 0 {
            os.write_uint32(5, self.plane_id)?;
        }
        if self.floor_id != 0 {
            os.write_uint32(6, self.floor_id)?;
        }
        if self.EBHLFAEGLCD != 0 {
            os.write_uint32(7, self.EBHLFAEGLCD)?;
        }
        if self.BNJMMLKOFCP != 0 {
            os.write_uint32(8, self.BNJMMLKOFCP)?;
        }
        if let Some(v) = self.EDHBGDEICNC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
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

    fn new() -> MIAIDAILDKM {
        MIAIDAILDKM::new()
    }

    fn clear(&mut self) {
        self.EEFLGHCOBML.clear();
        self.KGBHEHHFMPI.clear();
        self.CPCLJGPDNAN.clear();
        self.HLAGIMENBJG.clear();
        self.plane_id = 0;
        self.floor_id = 0;
        self.EBHLFAEGLCD = 0;
        self.BNJMMLKOFCP = 0;
        self.EDHBGDEICNC.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MIAIDAILDKM {
        static instance: MIAIDAILDKM = MIAIDAILDKM {
            EEFLGHCOBML: ::std::vec::Vec::new(),
            KGBHEHHFMPI: ::std::string::String::new(),
            CPCLJGPDNAN: ::std::string::String::new(),
            HLAGIMENBJG: ::std::vec::Vec::new(),
            plane_id: 0,
            floor_id: 0,
            EBHLFAEGLCD: 0,
            BNJMMLKOFCP: 0,
            EDHBGDEICNC: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MIAIDAILDKM {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MIAIDAILDKM").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MIAIDAILDKM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MIAIDAILDKM {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MIAIDAILDKM.proto\x1a\x11FEFDAHEOOKP.proto\x1a\x11HBLDPBBKHPB.prot\
    o\x1a\x11MBMCFOLIOLO.proto\"\xdb\x02\n\x0bMIAIDAILDKM\x12.\n\x0bEEFLGHCO\
    BML\x18\x01\x20\x03(\x0b2\x0c.HBLDPBBKHPBR\x0bEEFLGHCOBML\x12\x20\n\x0bK\
    GBHEHHFMPI\x18\x02\x20\x01(\tR\x0bKGBHEHHFMPI\x12\x20\n\x0bCPCLJGPDNAN\
    \x18\x03\x20\x01(\tR\x0bCPCLJGPDNAN\x12.\n\x0bHLAGIMENBJG\x18\x04\x20\
    \x03(\x0b2\x0c.FEFDAHEOOKPR\x0bHLAGIMENBJG\x12\x19\n\x08plane_id\x18\x05\
    \x20\x01(\rR\x07planeId\x12\x19\n\x08floor_id\x18\x06\x20\x01(\rR\x07flo\
    orId\x12\x20\n\x0bEBHLFAEGLCD\x18\x07\x20\x01(\rR\x0bEBHLFAEGLCD\x12\x20\
    \n\x0bBNJMMLKOFCP\x18\x08\x20\x01(\rR\x0bBNJMMLKOFCP\x12.\n\x0bEDHBGDEIC\
    NC\x18\t\x20\x01(\x0b2\x0c.MBMCFOLIOLOR\x0bEDHBGDEICNCb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::FEFDAHEOOKP::file_descriptor().clone());
            deps.push(super::HBLDPBBKHPB::file_descriptor().clone());
            deps.push(super::MBMCFOLIOLO::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MIAIDAILDKM::generated_message_descriptor_data());
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
