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

//! Generated file from `BKAAEBJKJKH.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:BKAAEBJKJKH)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BKAAEBJKJKH {
    // message fields
    // @@protoc_insertion_point(field:BKAAEBJKJKH.MKEGBHJLJNH)
    pub MKEGBHJLJNH: ::protobuf::EnumOrUnknown<super::RogueStatus::RogueStatus>,
    // @@protoc_insertion_point(field:BKAAEBJKJKH.area_id)
    pub area_id: u32,
    // @@protoc_insertion_point(field:BKAAEBJKJKH.FJOILOHPHLC)
    pub FJOILOHPHLC: bool,
    // @@protoc_insertion_point(field:BKAAEBJKJKH.GMPIIAEGGEK)
    pub GMPIIAEGGEK: u32,
    // @@protoc_insertion_point(field:BKAAEBJKJKH.map_id)
    pub map_id: u32,
    // @@protoc_insertion_point(field:BKAAEBJKJKH.NOOKGAJMFJI)
    pub NOOKGAJMFJI: ::protobuf::EnumOrUnknown<super::RogueAreaStatus::RogueAreaStatus>,
    // special fields
    // @@protoc_insertion_point(special_field:BKAAEBJKJKH.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BKAAEBJKJKH {
    fn default() -> &'a BKAAEBJKJKH {
        <BKAAEBJKJKH as ::protobuf::Message>::default_instance()
    }
}

impl BKAAEBJKJKH {
    pub fn new() -> BKAAEBJKJKH {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MKEGBHJLJNH",
            |m: &BKAAEBJKJKH| { &m.MKEGBHJLJNH },
            |m: &mut BKAAEBJKJKH| { &mut m.MKEGBHJLJNH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "area_id",
            |m: &BKAAEBJKJKH| { &m.area_id },
            |m: &mut BKAAEBJKJKH| { &mut m.area_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJOILOHPHLC",
            |m: &BKAAEBJKJKH| { &m.FJOILOHPHLC },
            |m: &mut BKAAEBJKJKH| { &mut m.FJOILOHPHLC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GMPIIAEGGEK",
            |m: &BKAAEBJKJKH| { &m.GMPIIAEGGEK },
            |m: &mut BKAAEBJKJKH| { &mut m.GMPIIAEGGEK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "map_id",
            |m: &BKAAEBJKJKH| { &m.map_id },
            |m: &mut BKAAEBJKJKH| { &mut m.map_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NOOKGAJMFJI",
            |m: &BKAAEBJKJKH| { &m.NOOKGAJMFJI },
            |m: &mut BKAAEBJKJKH| { &mut m.NOOKGAJMFJI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BKAAEBJKJKH>(
            "BKAAEBJKJKH",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BKAAEBJKJKH {
    const NAME: &'static str = "BKAAEBJKJKH";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.MKEGBHJLJNH = is.read_enum_or_unknown()?;
                },
                48 => {
                    self.area_id = is.read_uint32()?;
                },
                16 => {
                    self.FJOILOHPHLC = is.read_bool()?;
                },
                64 => {
                    self.GMPIIAEGGEK = is.read_uint32()?;
                },
                24 => {
                    self.map_id = is.read_uint32()?;
                },
                96 => {
                    self.NOOKGAJMFJI = is.read_enum_or_unknown()?;
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
        if self.MKEGBHJLJNH != ::protobuf::EnumOrUnknown::new(super::RogueStatus::RogueStatus::ROGUE_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(10, self.MKEGBHJLJNH.value());
        }
        if self.area_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.area_id);
        }
        if self.FJOILOHPHLC != false {
            my_size += 1 + 1;
        }
        if self.GMPIIAEGGEK != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.GMPIIAEGGEK);
        }
        if self.map_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.map_id);
        }
        if self.NOOKGAJMFJI != ::protobuf::EnumOrUnknown::new(super::RogueAreaStatus::RogueAreaStatus::ROGUE_AREA_STATUS_LOCK) {
            my_size += ::protobuf::rt::int32_size(12, self.NOOKGAJMFJI.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.MKEGBHJLJNH != ::protobuf::EnumOrUnknown::new(super::RogueStatus::RogueStatus::ROGUE_STATUS_NONE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.MKEGBHJLJNH))?;
        }
        if self.area_id != 0 {
            os.write_uint32(6, self.area_id)?;
        }
        if self.FJOILOHPHLC != false {
            os.write_bool(2, self.FJOILOHPHLC)?;
        }
        if self.GMPIIAEGGEK != 0 {
            os.write_uint32(8, self.GMPIIAEGGEK)?;
        }
        if self.map_id != 0 {
            os.write_uint32(3, self.map_id)?;
        }
        if self.NOOKGAJMFJI != ::protobuf::EnumOrUnknown::new(super::RogueAreaStatus::RogueAreaStatus::ROGUE_AREA_STATUS_LOCK) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.NOOKGAJMFJI))?;
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

    fn new() -> BKAAEBJKJKH {
        BKAAEBJKJKH::new()
    }

    fn clear(&mut self) {
        self.MKEGBHJLJNH = ::protobuf::EnumOrUnknown::new(super::RogueStatus::RogueStatus::ROGUE_STATUS_NONE);
        self.area_id = 0;
        self.FJOILOHPHLC = false;
        self.GMPIIAEGGEK = 0;
        self.map_id = 0;
        self.NOOKGAJMFJI = ::protobuf::EnumOrUnknown::new(super::RogueAreaStatus::RogueAreaStatus::ROGUE_AREA_STATUS_LOCK);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BKAAEBJKJKH {
        static instance: BKAAEBJKJKH = BKAAEBJKJKH {
            MKEGBHJLJNH: ::protobuf::EnumOrUnknown::from_i32(0),
            area_id: 0,
            FJOILOHPHLC: false,
            GMPIIAEGGEK: 0,
            map_id: 0,
            NOOKGAJMFJI: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BKAAEBJKJKH {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BKAAEBJKJKH").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BKAAEBJKJKH {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BKAAEBJKJKH {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BKAAEBJKJKH.proto\x1a\x15RogueAreaStatus.proto\x1a\x11RogueStatus.\
    proto\"\xe5\x01\n\x0bBKAAEBJKJKH\x12.\n\x0bMKEGBHJLJNH\x18\n\x20\x01(\
    \x0e2\x0c.RogueStatusR\x0bMKEGBHJLJNH\x12\x17\n\x07area_id\x18\x06\x20\
    \x01(\rR\x06areaId\x12\x20\n\x0bFJOILOHPHLC\x18\x02\x20\x01(\x08R\x0bFJO\
    ILOHPHLC\x12\x20\n\x0bGMPIIAEGGEK\x18\x08\x20\x01(\rR\x0bGMPIIAEGGEK\x12\
    \x15\n\x06map_id\x18\x03\x20\x01(\rR\x05mapId\x122\n\x0bNOOKGAJMFJI\x18\
    \x0c\x20\x01(\x0e2\x10.RogueAreaStatusR\x0bNOOKGAJMFJIb\x06proto3\
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
            deps.push(super::RogueAreaStatus::file_descriptor().clone());
            deps.push(super::RogueStatus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BKAAEBJKJKH::generated_message_descriptor_data());
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
