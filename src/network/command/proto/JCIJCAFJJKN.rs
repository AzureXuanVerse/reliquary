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

//! Generated file from `JCIJCAFJJKN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:JCIJCAFJJKN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JCIJCAFJJKN {
    // message fields
    // @@protoc_insertion_point(field:JCIJCAFJJKN.FOBAOEAFNAM)
    pub FOBAOEAFNAM: bool,
    // @@protoc_insertion_point(field:JCIJCAFJJKN.DMMAMJGNGNN)
    pub DMMAMJGNGNN: i32,
    // @@protoc_insertion_point(field:JCIJCAFJJKN.JIFKHCKPNFM)
    pub JIFKHCKPNFM: u32,
    // @@protoc_insertion_point(field:JCIJCAFJJKN.cost_data)
    pub cost_data: ::protobuf::MessageField<super::ItemCostData::ItemCostData>,
    // @@protoc_insertion_point(field:JCIJCAFJJKN.ANLINCFGFAN)
    pub ANLINCFGFAN: u32,
    // special fields
    // @@protoc_insertion_point(special_field:JCIJCAFJJKN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JCIJCAFJJKN {
    fn default() -> &'a JCIJCAFJJKN {
        <JCIJCAFJJKN as ::protobuf::Message>::default_instance()
    }
}

impl JCIJCAFJJKN {
    pub fn new() -> JCIJCAFJJKN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FOBAOEAFNAM",
            |m: &JCIJCAFJJKN| { &m.FOBAOEAFNAM },
            |m: &mut JCIJCAFJJKN| { &mut m.FOBAOEAFNAM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DMMAMJGNGNN",
            |m: &JCIJCAFJJKN| { &m.DMMAMJGNGNN },
            |m: &mut JCIJCAFJJKN| { &mut m.DMMAMJGNGNN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JIFKHCKPNFM",
            |m: &JCIJCAFJJKN| { &m.JIFKHCKPNFM },
            |m: &mut JCIJCAFJJKN| { &mut m.JIFKHCKPNFM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemCostData::ItemCostData>(
            "cost_data",
            |m: &JCIJCAFJJKN| { &m.cost_data },
            |m: &mut JCIJCAFJJKN| { &mut m.cost_data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ANLINCFGFAN",
            |m: &JCIJCAFJJKN| { &m.ANLINCFGFAN },
            |m: &mut JCIJCAFJJKN| { &mut m.ANLINCFGFAN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JCIJCAFJJKN>(
            "JCIJCAFJJKN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JCIJCAFJJKN {
    const NAME: &'static str = "JCIJCAFJJKN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.FOBAOEAFNAM = is.read_bool()?;
                },
                112 => {
                    self.DMMAMJGNGNN = is.read_int32()?;
                },
                72 => {
                    self.JIFKHCKPNFM = is.read_uint32()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.cost_data)?;
                },
                104 => {
                    self.ANLINCFGFAN = is.read_uint32()?;
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
        if self.FOBAOEAFNAM != false {
            my_size += 1 + 1;
        }
        if self.DMMAMJGNGNN != 0 {
            my_size += ::protobuf::rt::int32_size(14, self.DMMAMJGNGNN);
        }
        if self.JIFKHCKPNFM != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.JIFKHCKPNFM);
        }
        if let Some(v) = self.cost_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.ANLINCFGFAN != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.ANLINCFGFAN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.FOBAOEAFNAM != false {
            os.write_bool(10, self.FOBAOEAFNAM)?;
        }
        if self.DMMAMJGNGNN != 0 {
            os.write_int32(14, self.DMMAMJGNGNN)?;
        }
        if self.JIFKHCKPNFM != 0 {
            os.write_uint32(9, self.JIFKHCKPNFM)?;
        }
        if let Some(v) = self.cost_data.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if self.ANLINCFGFAN != 0 {
            os.write_uint32(13, self.ANLINCFGFAN)?;
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

    fn new() -> JCIJCAFJJKN {
        JCIJCAFJJKN::new()
    }

    fn clear(&mut self) {
        self.FOBAOEAFNAM = false;
        self.DMMAMJGNGNN = 0;
        self.JIFKHCKPNFM = 0;
        self.cost_data.clear();
        self.ANLINCFGFAN = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JCIJCAFJJKN {
        static instance: JCIJCAFJJKN = JCIJCAFJJKN {
            FOBAOEAFNAM: false,
            DMMAMJGNGNN: 0,
            JIFKHCKPNFM: 0,
            cost_data: ::protobuf::MessageField::none(),
            ANLINCFGFAN: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JCIJCAFJJKN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JCIJCAFJJKN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JCIJCAFJJKN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JCIJCAFJJKN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11JCIJCAFJJKN.proto\x1a\x12ItemCostData.proto\"\xc1\x01\n\x0bJCIJCAF\
    JJKN\x12\x20\n\x0bFOBAOEAFNAM\x18\n\x20\x01(\x08R\x0bFOBAOEAFNAM\x12\x20\
    \n\x0bDMMAMJGNGNN\x18\x0e\x20\x01(\x05R\x0bDMMAMJGNGNN\x12\x20\n\x0bJIFK\
    HCKPNFM\x18\t\x20\x01(\rR\x0bJIFKHCKPNFM\x12*\n\tcost_data\x18\x03\x20\
    \x01(\x0b2\r.ItemCostDataR\x08costData\x12\x20\n\x0bANLINCFGFAN\x18\r\
    \x20\x01(\rR\x0bANLINCFGFANb\x06proto3\
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
            deps.push(super::ItemCostData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JCIJCAFJJKN::generated_message_descriptor_data());
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
