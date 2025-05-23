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

//! Generated file from `ClientObjDownloadData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:ClientObjDownloadData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ClientObjDownloadData {
    // message fields
    // @@protoc_insertion_point(field:ClientObjDownloadData.BIDJPEIMLLF)
    pub BIDJPEIMLLF: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:ClientObjDownloadData.client_obj_download_data)
    pub client_obj_download_data: ::protobuf::MessageField<super::ClientDownloadData::ClientDownloadData>,
    // @@protoc_insertion_point(field:ClientObjDownloadData.JEDHNEJHGNP)
    pub JEDHNEJHGNP: ::std::vec::Vec<super::ClientDownloadData::ClientDownloadData>,
    // special fields
    // @@protoc_insertion_point(special_field:ClientObjDownloadData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ClientObjDownloadData {
    fn default() -> &'a ClientObjDownloadData {
        <ClientObjDownloadData as ::protobuf::Message>::default_instance()
    }
}

impl ClientObjDownloadData {
    pub fn new() -> ClientObjDownloadData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BIDJPEIMLLF",
            |m: &ClientObjDownloadData| { &m.BIDJPEIMLLF },
            |m: &mut ClientObjDownloadData| { &mut m.BIDJPEIMLLF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ClientDownloadData::ClientDownloadData>(
            "client_obj_download_data",
            |m: &ClientObjDownloadData| { &m.client_obj_download_data },
            |m: &mut ClientObjDownloadData| { &mut m.client_obj_download_data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JEDHNEJHGNP",
            |m: &ClientObjDownloadData| { &m.JEDHNEJHGNP },
            |m: &mut ClientObjDownloadData| { &mut m.JEDHNEJHGNP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ClientObjDownloadData>(
            "ClientObjDownloadData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ClientObjDownloadData {
    const NAME: &'static str = "ClientObjDownloadData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.BIDJPEIMLLF = is.read_bytes()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.client_obj_download_data)?;
                },
                26 => {
                    self.JEDHNEJHGNP.push(is.read_message()?);
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
        if !self.BIDJPEIMLLF.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.BIDJPEIMLLF);
        }
        if let Some(v) = self.client_obj_download_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.JEDHNEJHGNP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.BIDJPEIMLLF.is_empty() {
            os.write_bytes(1, &self.BIDJPEIMLLF)?;
        }
        if let Some(v) = self.client_obj_download_data.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        for v in &self.JEDHNEJHGNP {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> ClientObjDownloadData {
        ClientObjDownloadData::new()
    }

    fn clear(&mut self) {
        self.BIDJPEIMLLF.clear();
        self.client_obj_download_data.clear();
        self.JEDHNEJHGNP.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ClientObjDownloadData {
        static instance: ClientObjDownloadData = ClientObjDownloadData {
            BIDJPEIMLLF: ::std::vec::Vec::new(),
            client_obj_download_data: ::protobuf::MessageField::none(),
            JEDHNEJHGNP: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ClientObjDownloadData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ClientObjDownloadData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ClientObjDownloadData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientObjDownloadData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bClientObjDownloadData.proto\x1a\x18ClientDownloadData.proto\"\xbe\
    \x01\n\x15ClientObjDownloadData\x12\x20\n\x0bBIDJPEIMLLF\x18\x01\x20\x01\
    (\x0cR\x0bBIDJPEIMLLF\x12L\n\x18client_obj_download_data\x18\x02\x20\x01\
    (\x0b2\x13.ClientDownloadDataR\x15clientObjDownloadData\x125\n\x0bJEDHNE\
    JHGNP\x18\x03\x20\x03(\x0b2\x13.ClientDownloadDataR\x0bJEDHNEJHGNPb\x06p\
    roto3\
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
            deps.push(super::ClientDownloadData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ClientObjDownloadData::generated_message_descriptor_data());
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
