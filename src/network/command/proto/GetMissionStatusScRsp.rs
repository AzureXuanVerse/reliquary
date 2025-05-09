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

//! Generated file from `GetMissionStatusScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetMissionStatusScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetMissionStatusScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetMissionStatusScRsp.finished_main_mission_id_list)
    pub finished_main_mission_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetMissionStatusScRsp.unfinished_main_mission_id_list)
    pub unfinished_main_mission_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetMissionStatusScRsp.curversion_finished_main_mission_id_list)
    pub curversion_finished_main_mission_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetMissionStatusScRsp.disabled_main_mission_id_list)
    pub disabled_main_mission_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetMissionStatusScRsp.sub_mission_status_list)
    pub sub_mission_status_list: ::std::vec::Vec<super::Mission::Mission>,
    // @@protoc_insertion_point(field:GetMissionStatusScRsp.main_mission_mcv_list)
    pub main_mission_mcv_list: ::std::vec::Vec<super::FEAHPJPKGOA::FEAHPJPKGOA>,
    // @@protoc_insertion_point(field:GetMissionStatusScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GetMissionStatusScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetMissionStatusScRsp {
    fn default() -> &'a GetMissionStatusScRsp {
        <GetMissionStatusScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetMissionStatusScRsp {
    pub fn new() -> GetMissionStatusScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "finished_main_mission_id_list",
            |m: &GetMissionStatusScRsp| { &m.finished_main_mission_id_list },
            |m: &mut GetMissionStatusScRsp| { &mut m.finished_main_mission_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unfinished_main_mission_id_list",
            |m: &GetMissionStatusScRsp| { &m.unfinished_main_mission_id_list },
            |m: &mut GetMissionStatusScRsp| { &mut m.unfinished_main_mission_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "curversion_finished_main_mission_id_list",
            |m: &GetMissionStatusScRsp| { &m.curversion_finished_main_mission_id_list },
            |m: &mut GetMissionStatusScRsp| { &mut m.curversion_finished_main_mission_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "disabled_main_mission_id_list",
            |m: &GetMissionStatusScRsp| { &m.disabled_main_mission_id_list },
            |m: &mut GetMissionStatusScRsp| { &mut m.disabled_main_mission_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "sub_mission_status_list",
            |m: &GetMissionStatusScRsp| { &m.sub_mission_status_list },
            |m: &mut GetMissionStatusScRsp| { &mut m.sub_mission_status_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "main_mission_mcv_list",
            |m: &GetMissionStatusScRsp| { &m.main_mission_mcv_list },
            |m: &mut GetMissionStatusScRsp| { &mut m.main_mission_mcv_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetMissionStatusScRsp| { &m.retcode },
            |m: &mut GetMissionStatusScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetMissionStatusScRsp>(
            "GetMissionStatusScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetMissionStatusScRsp {
    const NAME: &'static str = "GetMissionStatusScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.finished_main_mission_id_list)?;
                },
                24 => {
                    self.finished_main_mission_id_list.push(is.read_uint32()?);
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.unfinished_main_mission_id_list)?;
                },
                96 => {
                    self.unfinished_main_mission_id_list.push(is.read_uint32()?);
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.curversion_finished_main_mission_id_list)?;
                },
                8 => {
                    self.curversion_finished_main_mission_id_list.push(is.read_uint32()?);
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.disabled_main_mission_id_list)?;
                },
                104 => {
                    self.disabled_main_mission_id_list.push(is.read_uint32()?);
                },
                58 => {
                    self.sub_mission_status_list.push(is.read_message()?);
                },
                50 => {
                    self.main_mission_mcv_list.push(is.read_message()?);
                },
                40 => {
                    self.retcode = is.read_uint32()?;
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
        my_size += ::protobuf::rt::vec_packed_uint32_size(3, &self.finished_main_mission_id_list);
        my_size += ::protobuf::rt::vec_packed_uint32_size(12, &self.unfinished_main_mission_id_list);
        my_size += ::protobuf::rt::vec_packed_uint32_size(1, &self.curversion_finished_main_mission_id_list);
        my_size += ::protobuf::rt::vec_packed_uint32_size(13, &self.disabled_main_mission_id_list);
        for value in &self.sub_mission_status_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.main_mission_mcv_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_uint32(3, &self.finished_main_mission_id_list)?;
        os.write_repeated_packed_uint32(12, &self.unfinished_main_mission_id_list)?;
        os.write_repeated_packed_uint32(1, &self.curversion_finished_main_mission_id_list)?;
        os.write_repeated_packed_uint32(13, &self.disabled_main_mission_id_list)?;
        for v in &self.sub_mission_status_list {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        for v in &self.main_mission_mcv_list {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        if self.retcode != 0 {
            os.write_uint32(5, self.retcode)?;
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

    fn new() -> GetMissionStatusScRsp {
        GetMissionStatusScRsp::new()
    }

    fn clear(&mut self) {
        self.finished_main_mission_id_list.clear();
        self.unfinished_main_mission_id_list.clear();
        self.curversion_finished_main_mission_id_list.clear();
        self.disabled_main_mission_id_list.clear();
        self.sub_mission_status_list.clear();
        self.main_mission_mcv_list.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetMissionStatusScRsp {
        static instance: GetMissionStatusScRsp = GetMissionStatusScRsp {
            finished_main_mission_id_list: ::std::vec::Vec::new(),
            unfinished_main_mission_id_list: ::std::vec::Vec::new(),
            curversion_finished_main_mission_id_list: ::std::vec::Vec::new(),
            disabled_main_mission_id_list: ::std::vec::Vec::new(),
            sub_mission_status_list: ::std::vec::Vec::new(),
            main_mission_mcv_list: ::std::vec::Vec::new(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetMissionStatusScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetMissionStatusScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetMissionStatusScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetMissionStatusScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bGetMissionStatusScRsp.proto\x1a\x11FEAHPJPKGOA.proto\x1a\rMission.\
    proto\"\xd4\x03\n\x15GetMissionStatusScRsp\x12@\n\x1dfinished_main_missi\
    on_id_list\x18\x03\x20\x03(\rR\x19finishedMainMissionIdList\x12D\n\x1fun\
    finished_main_mission_id_list\x18\x0c\x20\x03(\rR\x1bunfinishedMainMissi\
    onIdList\x12U\n(curversion_finished_main_mission_id_list\x18\x01\x20\x03\
    (\rR#curversionFinishedMainMissionIdList\x12@\n\x1ddisabled_main_mission\
    _id_list\x18\r\x20\x03(\rR\x19disabledMainMissionIdList\x12?\n\x17sub_mi\
    ssion_status_list\x18\x07\x20\x03(\x0b2\x08.MissionR\x14subMissionStatus\
    List\x12?\n\x15main_mission_mcv_list\x18\x06\x20\x03(\x0b2\x0c.FEAHPJPKG\
    OAR\x12mainMissionMcvList\x12\x18\n\x07retcode\x18\x05\x20\x01(\rR\x07re\
    tcodeb\x06proto3\
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
            deps.push(super::FEAHPJPKGOA::file_descriptor().clone());
            deps.push(super::Mission::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetMissionStatusScRsp::generated_message_descriptor_data());
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
