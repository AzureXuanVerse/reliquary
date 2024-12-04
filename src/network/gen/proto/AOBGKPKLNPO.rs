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

//! Generated file from `AOBGKPKLNPO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AOBGKPKLNPO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AOBGKPKLNPO {
    // message oneof groups
    pub NHEPOCFCGMC: ::std::option::Option<aobgkpklnpo::NHEPOCFCGMC>,
    // special fields
    // @@protoc_insertion_point(special_field:AOBGKPKLNPO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AOBGKPKLNPO {
    fn default() -> &'a AOBGKPKLNPO {
        <AOBGKPKLNPO as ::protobuf::Message>::default_instance()
    }
}

impl AOBGKPKLNPO {
    pub fn new() -> AOBGKPKLNPO {
        ::std::default::Default::default()
    }

    // .GBGMBGHODGP NDKMFAJOOPG = 14;

    pub fn NDKMFAJOOPG(&self) -> &super::GBGMBGHODGP::GBGMBGHODGP {
        match self.NHEPOCFCGMC {
            ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::NDKMFAJOOPG(ref v)) => v,
            _ => <super::GBGMBGHODGP::GBGMBGHODGP as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_NDKMFAJOOPG(&mut self) {
        self.NHEPOCFCGMC = ::std::option::Option::None;
    }

    pub fn has_NDKMFAJOOPG(&self) -> bool {
        match self.NHEPOCFCGMC {
            ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::NDKMFAJOOPG(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_NDKMFAJOOPG(&mut self, v: super::GBGMBGHODGP::GBGMBGHODGP) {
        self.NHEPOCFCGMC = ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::NDKMFAJOOPG(v))
    }

    // Mutable pointer to the field.
    pub fn mut_NDKMFAJOOPG(&mut self) -> &mut super::GBGMBGHODGP::GBGMBGHODGP {
        if let ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::NDKMFAJOOPG(_)) = self.NHEPOCFCGMC {
        } else {
            self.NHEPOCFCGMC = ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::NDKMFAJOOPG(super::GBGMBGHODGP::GBGMBGHODGP::new()));
        }
        match self.NHEPOCFCGMC {
            ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::NDKMFAJOOPG(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_NDKMFAJOOPG(&mut self) -> super::GBGMBGHODGP::GBGMBGHODGP {
        if self.has_NDKMFAJOOPG() {
            match self.NHEPOCFCGMC.take() {
                ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::NDKMFAJOOPG(v)) => v,
                _ => panic!(),
            }
        } else {
            super::GBGMBGHODGP::GBGMBGHODGP::new()
        }
    }

    // .BOIFNIIGMJG JEBIMAKIPGF = 13;

    pub fn JEBIMAKIPGF(&self) -> &super::BOIFNIIGMJG::BOIFNIIGMJG {
        match self.NHEPOCFCGMC {
            ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::JEBIMAKIPGF(ref v)) => v,
            _ => <super::BOIFNIIGMJG::BOIFNIIGMJG as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_JEBIMAKIPGF(&mut self) {
        self.NHEPOCFCGMC = ::std::option::Option::None;
    }

    pub fn has_JEBIMAKIPGF(&self) -> bool {
        match self.NHEPOCFCGMC {
            ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::JEBIMAKIPGF(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JEBIMAKIPGF(&mut self, v: super::BOIFNIIGMJG::BOIFNIIGMJG) {
        self.NHEPOCFCGMC = ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::JEBIMAKIPGF(v))
    }

    // Mutable pointer to the field.
    pub fn mut_JEBIMAKIPGF(&mut self) -> &mut super::BOIFNIIGMJG::BOIFNIIGMJG {
        if let ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::JEBIMAKIPGF(_)) = self.NHEPOCFCGMC {
        } else {
            self.NHEPOCFCGMC = ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::JEBIMAKIPGF(super::BOIFNIIGMJG::BOIFNIIGMJG::new()));
        }
        match self.NHEPOCFCGMC {
            ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::JEBIMAKIPGF(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_JEBIMAKIPGF(&mut self) -> super::BOIFNIIGMJG::BOIFNIIGMJG {
        if self.has_JEBIMAKIPGF() {
            match self.NHEPOCFCGMC.take() {
                ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::JEBIMAKIPGF(v)) => v,
                _ => panic!(),
            }
        } else {
            super::BOIFNIIGMJG::BOIFNIIGMJG::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::GBGMBGHODGP::GBGMBGHODGP>(
            "NDKMFAJOOPG",
            AOBGKPKLNPO::has_NDKMFAJOOPG,
            AOBGKPKLNPO::NDKMFAJOOPG,
            AOBGKPKLNPO::mut_NDKMFAJOOPG,
            AOBGKPKLNPO::set_NDKMFAJOOPG,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::BOIFNIIGMJG::BOIFNIIGMJG>(
            "JEBIMAKIPGF",
            AOBGKPKLNPO::has_JEBIMAKIPGF,
            AOBGKPKLNPO::JEBIMAKIPGF,
            AOBGKPKLNPO::mut_JEBIMAKIPGF,
            AOBGKPKLNPO::set_JEBIMAKIPGF,
        ));
        oneofs.push(aobgkpklnpo::NHEPOCFCGMC::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AOBGKPKLNPO>(
            "AOBGKPKLNPO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AOBGKPKLNPO {
    const NAME: &'static str = "AOBGKPKLNPO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    self.NHEPOCFCGMC = ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::NDKMFAJOOPG(is.read_message()?));
                },
                106 => {
                    self.NHEPOCFCGMC = ::std::option::Option::Some(aobgkpklnpo::NHEPOCFCGMC::JEBIMAKIPGF(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.NHEPOCFCGMC {
            match v {
                &aobgkpklnpo::NHEPOCFCGMC::NDKMFAJOOPG(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &aobgkpklnpo::NHEPOCFCGMC::JEBIMAKIPGF(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.NHEPOCFCGMC {
            match v {
                &aobgkpklnpo::NHEPOCFCGMC::NDKMFAJOOPG(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
                },
                &aobgkpklnpo::NHEPOCFCGMC::JEBIMAKIPGF(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
                },
            };
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

    fn new() -> AOBGKPKLNPO {
        AOBGKPKLNPO::new()
    }

    fn clear(&mut self) {
        self.NHEPOCFCGMC = ::std::option::Option::None;
        self.NHEPOCFCGMC = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AOBGKPKLNPO {
        static instance: AOBGKPKLNPO = AOBGKPKLNPO {
            NHEPOCFCGMC: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AOBGKPKLNPO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AOBGKPKLNPO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AOBGKPKLNPO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AOBGKPKLNPO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `AOBGKPKLNPO`
pub mod aobgkpklnpo {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:AOBGKPKLNPO.NHEPOCFCGMC)
    pub enum NHEPOCFCGMC {
        // @@protoc_insertion_point(oneof_field:AOBGKPKLNPO.NDKMFAJOOPG)
        NDKMFAJOOPG(super::super::GBGMBGHODGP::GBGMBGHODGP),
        // @@protoc_insertion_point(oneof_field:AOBGKPKLNPO.JEBIMAKIPGF)
        JEBIMAKIPGF(super::super::BOIFNIIGMJG::BOIFNIIGMJG),
    }

    impl ::protobuf::Oneof for NHEPOCFCGMC {
    }

    impl ::protobuf::OneofFull for NHEPOCFCGMC {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::AOBGKPKLNPO as ::protobuf::MessageFull>::descriptor().oneof_by_name("NHEPOCFCGMC").unwrap()).clone()
        }
    }

    impl NHEPOCFCGMC {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<NHEPOCFCGMC>("NHEPOCFCGMC")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AOBGKPKLNPO.proto\x1a\x11BOIFNIIGMJG.proto\x1a\x11GBGMBGHODGP.prot\
    o\"\x80\x01\n\x0bAOBGKPKLNPO\x120\n\x0bNDKMFAJOOPG\x18\x0e\x20\x01(\x0b2\
    \x0c.GBGMBGHODGPH\0R\x0bNDKMFAJOOPG\x120\n\x0bJEBIMAKIPGF\x18\r\x20\x01(\
    \x0b2\x0c.BOIFNIIGMJGH\0R\x0bJEBIMAKIPGFB\r\n\x0bNHEPOCFCGMCb\x06proto3\
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
            deps.push(super::BOIFNIIGMJG::file_descriptor().clone());
            deps.push(super::GBGMBGHODGP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AOBGKPKLNPO::generated_message_descriptor_data());
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
