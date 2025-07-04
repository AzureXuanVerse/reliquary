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

//! Generated file from `OFNGPLJKLOJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:OFNGPLJKLOJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct OFNGPLJKLOJ {
    // message fields
    // @@protoc_insertion_point(field:OFNGPLJKLOJ.FBBAJBINGLB)
    pub FBBAJBINGLB: ::std::vec::Vec<super::GHLEDKFIIJH::GHLEDKFIIJH>,
    // @@protoc_insertion_point(field:OFNGPLJKLOJ.NOBONCCPENG)
    pub NOBONCCPENG: ::std::vec::Vec<super::NKGHHAFANHN::NKGHHAFANHN>,
    // @@protoc_insertion_point(field:OFNGPLJKLOJ.MPMFAHLKEOB)
    pub MPMFAHLKEOB: ::std::vec::Vec<super::GHLEDKFIIJH::GHLEDKFIIJH>,
    // @@protoc_insertion_point(field:OFNGPLJKLOJ.avatar_id)
    pub avatar_id: u32,
    // @@protoc_insertion_point(field:OFNGPLJKLOJ.PDBHNHPCNNJ)
    pub PDBHNHPCNNJ: ::std::vec::Vec<super::NKGHHAFANHN::NKGHHAFANHN>,
    // @@protoc_insertion_point(field:OFNGPLJKLOJ.LGEJJAJPEDK)
    pub LGEJJAJPEDK: ::std::vec::Vec<super::GHLEDKFIIJH::GHLEDKFIIJH>,
    // @@protoc_insertion_point(field:OFNGPLJKLOJ.KKCMFGMHIMO)
    pub KKCMFGMHIMO: ::std::vec::Vec<super::GHLEDKFIIJH::GHLEDKFIIJH>,
    // special fields
    // @@protoc_insertion_point(special_field:OFNGPLJKLOJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a OFNGPLJKLOJ {
    fn default() -> &'a OFNGPLJKLOJ {
        <OFNGPLJKLOJ as ::protobuf::Message>::default_instance()
    }
}

impl OFNGPLJKLOJ {
    pub fn new() -> OFNGPLJKLOJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FBBAJBINGLB",
            |m: &OFNGPLJKLOJ| { &m.FBBAJBINGLB },
            |m: &mut OFNGPLJKLOJ| { &mut m.FBBAJBINGLB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NOBONCCPENG",
            |m: &OFNGPLJKLOJ| { &m.NOBONCCPENG },
            |m: &mut OFNGPLJKLOJ| { &mut m.NOBONCCPENG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MPMFAHLKEOB",
            |m: &OFNGPLJKLOJ| { &m.MPMFAHLKEOB },
            |m: &mut OFNGPLJKLOJ| { &mut m.MPMFAHLKEOB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &OFNGPLJKLOJ| { &m.avatar_id },
            |m: &mut OFNGPLJKLOJ| { &mut m.avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PDBHNHPCNNJ",
            |m: &OFNGPLJKLOJ| { &m.PDBHNHPCNNJ },
            |m: &mut OFNGPLJKLOJ| { &mut m.PDBHNHPCNNJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LGEJJAJPEDK",
            |m: &OFNGPLJKLOJ| { &m.LGEJJAJPEDK },
            |m: &mut OFNGPLJKLOJ| { &mut m.LGEJJAJPEDK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KKCMFGMHIMO",
            |m: &OFNGPLJKLOJ| { &m.KKCMFGMHIMO },
            |m: &mut OFNGPLJKLOJ| { &mut m.KKCMFGMHIMO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OFNGPLJKLOJ>(
            "OFNGPLJKLOJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for OFNGPLJKLOJ {
    const NAME: &'static str = "OFNGPLJKLOJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    self.FBBAJBINGLB.push(is.read_message()?);
                },
                42 => {
                    self.NOBONCCPENG.push(is.read_message()?);
                },
                10 => {
                    self.MPMFAHLKEOB.push(is.read_message()?);
                },
                112 => {
                    self.avatar_id = is.read_uint32()?;
                },
                66 => {
                    self.PDBHNHPCNNJ.push(is.read_message()?);
                },
                34 => {
                    self.LGEJJAJPEDK.push(is.read_message()?);
                },
                58 => {
                    self.KKCMFGMHIMO.push(is.read_message()?);
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
        for value in &self.FBBAJBINGLB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.NOBONCCPENG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.MPMFAHLKEOB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.avatar_id);
        }
        for value in &self.PDBHNHPCNNJ {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.LGEJJAJPEDK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.KKCMFGMHIMO {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.FBBAJBINGLB {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        for v in &self.NOBONCCPENG {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        for v in &self.MPMFAHLKEOB {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        if self.avatar_id != 0 {
            os.write_uint32(14, self.avatar_id)?;
        }
        for v in &self.PDBHNHPCNNJ {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        for v in &self.LGEJJAJPEDK {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        for v in &self.KKCMFGMHIMO {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> OFNGPLJKLOJ {
        OFNGPLJKLOJ::new()
    }

    fn clear(&mut self) {
        self.FBBAJBINGLB.clear();
        self.NOBONCCPENG.clear();
        self.MPMFAHLKEOB.clear();
        self.avatar_id = 0;
        self.PDBHNHPCNNJ.clear();
        self.LGEJJAJPEDK.clear();
        self.KKCMFGMHIMO.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static OFNGPLJKLOJ {
        static instance: OFNGPLJKLOJ = OFNGPLJKLOJ {
            FBBAJBINGLB: ::std::vec::Vec::new(),
            NOBONCCPENG: ::std::vec::Vec::new(),
            MPMFAHLKEOB: ::std::vec::Vec::new(),
            avatar_id: 0,
            PDBHNHPCNNJ: ::std::vec::Vec::new(),
            LGEJJAJPEDK: ::std::vec::Vec::new(),
            KKCMFGMHIMO: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for OFNGPLJKLOJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("OFNGPLJKLOJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for OFNGPLJKLOJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OFNGPLJKLOJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11OFNGPLJKLOJ.proto\x1a\x11GHLEDKFIIJH.proto\x1a\x11NKGHHAFANHN.prot\
    o\"\xca\x02\n\x0bOFNGPLJKLOJ\x12.\n\x0bFBBAJBINGLB\x18\x0b\x20\x03(\x0b2\
    \x0c.GHLEDKFIIJHR\x0bFBBAJBINGLB\x12.\n\x0bNOBONCCPENG\x18\x05\x20\x03(\
    \x0b2\x0c.NKGHHAFANHNR\x0bNOBONCCPENG\x12.\n\x0bMPMFAHLKEOB\x18\x01\x20\
    \x03(\x0b2\x0c.GHLEDKFIIJHR\x0bMPMFAHLKEOB\x12\x1b\n\tavatar_id\x18\x0e\
    \x20\x01(\rR\x08avatarId\x12.\n\x0bPDBHNHPCNNJ\x18\x08\x20\x03(\x0b2\x0c\
    .NKGHHAFANHNR\x0bPDBHNHPCNNJ\x12.\n\x0bLGEJJAJPEDK\x18\x04\x20\x03(\x0b2\
    \x0c.GHLEDKFIIJHR\x0bLGEJJAJPEDK\x12.\n\x0bKKCMFGMHIMO\x18\x07\x20\x03(\
    \x0b2\x0c.GHLEDKFIIJHR\x0bKKCMFGMHIMOb\x06proto3\
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
            deps.push(super::GHLEDKFIIJH::file_descriptor().clone());
            deps.push(super::NKGHHAFANHN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(OFNGPLJKLOJ::generated_message_descriptor_data());
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
