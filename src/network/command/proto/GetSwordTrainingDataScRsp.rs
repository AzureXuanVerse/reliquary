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

//! Generated file from `GetSwordTrainingDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetSwordTrainingDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetSwordTrainingDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetSwordTrainingDataScRsp.KLBPECANFIG)
    pub KLBPECANFIG: bool,
    // @@protoc_insertion_point(field:GetSwordTrainingDataScRsp.CNBCKEFNFGE)
    pub CNBCKEFNFGE: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetSwordTrainingDataScRsp.KJKBKEGIGHK)
    pub KJKBKEGIGHK: ::protobuf::MessageField<super::NOKODMNOHMN::NOKODMNOHMN>,
    // @@protoc_insertion_point(field:GetSwordTrainingDataScRsp.FABKPHMJGHL)
    pub FABKPHMJGHL: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetSwordTrainingDataScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetSwordTrainingDataScRsp.GAJBFPCPIGM)
    pub GAJBFPCPIGM: ::protobuf::MessageField<super::ALEFDNLLKLB::ALEFDNLLKLB>,
    // @@protoc_insertion_point(field:GetSwordTrainingDataScRsp.DCHGIODEDDK)
    pub DCHGIODEDDK: u32,
    // @@protoc_insertion_point(field:GetSwordTrainingDataScRsp.JOEFNHGGAGO)
    pub JOEFNHGGAGO: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:GetSwordTrainingDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetSwordTrainingDataScRsp {
    fn default() -> &'a GetSwordTrainingDataScRsp {
        <GetSwordTrainingDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetSwordTrainingDataScRsp {
    pub fn new() -> GetSwordTrainingDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KLBPECANFIG",
            |m: &GetSwordTrainingDataScRsp| { &m.KLBPECANFIG },
            |m: &mut GetSwordTrainingDataScRsp| { &mut m.KLBPECANFIG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CNBCKEFNFGE",
            |m: &GetSwordTrainingDataScRsp| { &m.CNBCKEFNFGE },
            |m: &mut GetSwordTrainingDataScRsp| { &mut m.CNBCKEFNFGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NOKODMNOHMN::NOKODMNOHMN>(
            "KJKBKEGIGHK",
            |m: &GetSwordTrainingDataScRsp| { &m.KJKBKEGIGHK },
            |m: &mut GetSwordTrainingDataScRsp| { &mut m.KJKBKEGIGHK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FABKPHMJGHL",
            |m: &GetSwordTrainingDataScRsp| { &m.FABKPHMJGHL },
            |m: &mut GetSwordTrainingDataScRsp| { &mut m.FABKPHMJGHL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetSwordTrainingDataScRsp| { &m.retcode },
            |m: &mut GetSwordTrainingDataScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ALEFDNLLKLB::ALEFDNLLKLB>(
            "GAJBFPCPIGM",
            |m: &GetSwordTrainingDataScRsp| { &m.GAJBFPCPIGM },
            |m: &mut GetSwordTrainingDataScRsp| { &mut m.GAJBFPCPIGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DCHGIODEDDK",
            |m: &GetSwordTrainingDataScRsp| { &m.DCHGIODEDDK },
            |m: &mut GetSwordTrainingDataScRsp| { &mut m.DCHGIODEDDK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JOEFNHGGAGO",
            |m: &GetSwordTrainingDataScRsp| { &m.JOEFNHGGAGO },
            |m: &mut GetSwordTrainingDataScRsp| { &mut m.JOEFNHGGAGO },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetSwordTrainingDataScRsp>(
            "GetSwordTrainingDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetSwordTrainingDataScRsp {
    const NAME: &'static str = "GetSwordTrainingDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.KLBPECANFIG = is.read_bool()?;
                },
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.CNBCKEFNFGE)?;
                },
                88 => {
                    self.CNBCKEFNFGE.push(is.read_uint32()?);
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KJKBKEGIGHK)?;
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.FABKPHMJGHL)?;
                },
                120 => {
                    self.FABKPHMJGHL.push(is.read_uint32()?);
                },
                112 => {
                    self.retcode = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GAJBFPCPIGM)?;
                },
                64 => {
                    self.DCHGIODEDDK = is.read_uint32()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.JOEFNHGGAGO)?;
                },
                16 => {
                    self.JOEFNHGGAGO.push(is.read_uint32()?);
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
        if self.KLBPECANFIG != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(11, &self.CNBCKEFNFGE);
        if let Some(v) = self.KJKBKEGIGHK.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(15, &self.FABKPHMJGHL);
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.retcode);
        }
        if let Some(v) = self.GAJBFPCPIGM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.DCHGIODEDDK != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.DCHGIODEDDK);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(2, &self.JOEFNHGGAGO);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KLBPECANFIG != false {
            os.write_bool(3, self.KLBPECANFIG)?;
        }
        os.write_repeated_packed_uint32(11, &self.CNBCKEFNFGE)?;
        if let Some(v) = self.KJKBKEGIGHK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        os.write_repeated_packed_uint32(15, &self.FABKPHMJGHL)?;
        if self.retcode != 0 {
            os.write_uint32(14, self.retcode)?;
        }
        if let Some(v) = self.GAJBFPCPIGM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.DCHGIODEDDK != 0 {
            os.write_uint32(8, self.DCHGIODEDDK)?;
        }
        os.write_repeated_packed_uint32(2, &self.JOEFNHGGAGO)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GetSwordTrainingDataScRsp {
        GetSwordTrainingDataScRsp::new()
    }

    fn clear(&mut self) {
        self.KLBPECANFIG = false;
        self.CNBCKEFNFGE.clear();
        self.KJKBKEGIGHK.clear();
        self.FABKPHMJGHL.clear();
        self.retcode = 0;
        self.GAJBFPCPIGM.clear();
        self.DCHGIODEDDK = 0;
        self.JOEFNHGGAGO.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetSwordTrainingDataScRsp {
        static instance: GetSwordTrainingDataScRsp = GetSwordTrainingDataScRsp {
            KLBPECANFIG: false,
            CNBCKEFNFGE: ::std::vec::Vec::new(),
            KJKBKEGIGHK: ::protobuf::MessageField::none(),
            FABKPHMJGHL: ::std::vec::Vec::new(),
            retcode: 0,
            GAJBFPCPIGM: ::protobuf::MessageField::none(),
            DCHGIODEDDK: 0,
            JOEFNHGGAGO: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetSwordTrainingDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetSwordTrainingDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetSwordTrainingDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetSwordTrainingDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fGetSwordTrainingDataScRsp.proto\x1a\x11ALEFDNLLKLB.proto\x1a\x11NO\
    KODMNOHMN.proto\"\xbf\x02\n\x19GetSwordTrainingDataScRsp\x12\x20\n\x0bKL\
    BPECANFIG\x18\x03\x20\x01(\x08R\x0bKLBPECANFIG\x12\x20\n\x0bCNBCKEFNFGE\
    \x18\x0b\x20\x03(\rR\x0bCNBCKEFNFGE\x12.\n\x0bKJKBKEGIGHK\x18\x01\x20\
    \x01(\x0b2\x0c.NOKODMNOHMNR\x0bKJKBKEGIGHK\x12\x20\n\x0bFABKPHMJGHL\x18\
    \x0f\x20\x03(\rR\x0bFABKPHMJGHL\x12\x18\n\x07retcode\x18\x0e\x20\x01(\rR\
    \x07retcode\x12.\n\x0bGAJBFPCPIGM\x18\x07\x20\x01(\x0b2\x0c.ALEFDNLLKLBR\
    \x0bGAJBFPCPIGM\x12\x20\n\x0bDCHGIODEDDK\x18\x08\x20\x01(\rR\x0bDCHGIODE\
    DDK\x12\x20\n\x0bJOEFNHGGAGO\x18\x02\x20\x03(\rR\x0bJOEFNHGGAGOb\x06prot\
    o3\
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
            deps.push(super::ALEFDNLLKLB::file_descriptor().clone());
            deps.push(super::NOKODMNOHMN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetSwordTrainingDataScRsp::generated_message_descriptor_data());
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
