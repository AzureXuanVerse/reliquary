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

//! Generated file from `KAOAHKAOHFI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:KAOAHKAOHFI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct KAOAHKAOHFI {
    // message fields
    // @@protoc_insertion_point(field:KAOAHKAOHFI.DHLPKMIHDNM)
    pub DHLPKMIHDNM: ::protobuf::MessageField<super::PunkLordBattleRecordList::PunkLordBattleRecordList>,
    // @@protoc_insertion_point(field:KAOAHKAOHFI.EKKJLAOKIJI)
    pub EKKJLAOKIJI: ::protobuf::EnumOrUnknown<super::PunkLordAttackerStatus::PunkLordAttackerStatus>,
    // @@protoc_insertion_point(field:KAOAHKAOHFI.basic_info)
    pub basic_info: ::protobuf::MessageField<super::PunkLordMonsterBasicInfo::PunkLordMonsterBasicInfo>,
    // @@protoc_insertion_point(field:KAOAHKAOHFI.COJKEIFJNEK)
    pub COJKEIFJNEK: u32,
    // special fields
    // @@protoc_insertion_point(special_field:KAOAHKAOHFI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a KAOAHKAOHFI {
    fn default() -> &'a KAOAHKAOHFI {
        <KAOAHKAOHFI as ::protobuf::Message>::default_instance()
    }
}

impl KAOAHKAOHFI {
    pub fn new() -> KAOAHKAOHFI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PunkLordBattleRecordList::PunkLordBattleRecordList>(
            "DHLPKMIHDNM",
            |m: &KAOAHKAOHFI| { &m.DHLPKMIHDNM },
            |m: &mut KAOAHKAOHFI| { &mut m.DHLPKMIHDNM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EKKJLAOKIJI",
            |m: &KAOAHKAOHFI| { &m.EKKJLAOKIJI },
            |m: &mut KAOAHKAOHFI| { &mut m.EKKJLAOKIJI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PunkLordMonsterBasicInfo::PunkLordMonsterBasicInfo>(
            "basic_info",
            |m: &KAOAHKAOHFI| { &m.basic_info },
            |m: &mut KAOAHKAOHFI| { &mut m.basic_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "COJKEIFJNEK",
            |m: &KAOAHKAOHFI| { &m.COJKEIFJNEK },
            |m: &mut KAOAHKAOHFI| { &mut m.COJKEIFJNEK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<KAOAHKAOHFI>(
            "KAOAHKAOHFI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for KAOAHKAOHFI {
    const NAME: &'static str = "KAOAHKAOHFI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DHLPKMIHDNM)?;
                },
                96 => {
                    self.EKKJLAOKIJI = is.read_enum_or_unknown()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.basic_info)?;
                },
                16 => {
                    self.COJKEIFJNEK = is.read_uint32()?;
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
        if let Some(v) = self.DHLPKMIHDNM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.EKKJLAOKIJI != ::protobuf::EnumOrUnknown::new(super::PunkLordAttackerStatus::PunkLordAttackerStatus::PUNK_LORD_ATTACKER_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(12, self.EKKJLAOKIJI.value());
        }
        if let Some(v) = self.basic_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.COJKEIFJNEK != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.COJKEIFJNEK);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.DHLPKMIHDNM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if self.EKKJLAOKIJI != ::protobuf::EnumOrUnknown::new(super::PunkLordAttackerStatus::PunkLordAttackerStatus::PUNK_LORD_ATTACKER_STATUS_NONE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.EKKJLAOKIJI))?;
        }
        if let Some(v) = self.basic_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if self.COJKEIFJNEK != 0 {
            os.write_uint32(2, self.COJKEIFJNEK)?;
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

    fn new() -> KAOAHKAOHFI {
        KAOAHKAOHFI::new()
    }

    fn clear(&mut self) {
        self.DHLPKMIHDNM.clear();
        self.EKKJLAOKIJI = ::protobuf::EnumOrUnknown::new(super::PunkLordAttackerStatus::PunkLordAttackerStatus::PUNK_LORD_ATTACKER_STATUS_NONE);
        self.basic_info.clear();
        self.COJKEIFJNEK = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static KAOAHKAOHFI {
        static instance: KAOAHKAOHFI = KAOAHKAOHFI {
            DHLPKMIHDNM: ::protobuf::MessageField::none(),
            EKKJLAOKIJI: ::protobuf::EnumOrUnknown::from_i32(0),
            basic_info: ::protobuf::MessageField::none(),
            COJKEIFJNEK: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for KAOAHKAOHFI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("KAOAHKAOHFI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for KAOAHKAOHFI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KAOAHKAOHFI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11KAOAHKAOHFI.proto\x1a\x1cPunkLordAttackerStatus.proto\x1a\x1ePunkL\
    ordBattleRecordList.proto\x1a\x1ePunkLordMonsterBasicInfo.proto\"\xe1\
    \x01\n\x0bKAOAHKAOHFI\x12;\n\x0bDHLPKMIHDNM\x18\x0f\x20\x01(\x0b2\x19.Pu\
    nkLordBattleRecordListR\x0bDHLPKMIHDNM\x129\n\x0bEKKJLAOKIJI\x18\x0c\x20\
    \x01(\x0e2\x17.PunkLordAttackerStatusR\x0bEKKJLAOKIJI\x128\n\nbasic_info\
    \x18\t\x20\x01(\x0b2\x19.PunkLordMonsterBasicInfoR\tbasicInfo\x12\x20\n\
    \x0bCOJKEIFJNEK\x18\x02\x20\x01(\rR\x0bCOJKEIFJNEKb\x06proto3\
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
            deps.push(super::PunkLordAttackerStatus::file_descriptor().clone());
            deps.push(super::PunkLordBattleRecordList::file_descriptor().clone());
            deps.push(super::PunkLordMonsterBasicInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(KAOAHKAOHFI::generated_message_descriptor_data());
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
