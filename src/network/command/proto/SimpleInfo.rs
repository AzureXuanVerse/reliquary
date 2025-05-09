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

//! Generated file from `SimpleInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:SimpleInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SimpleInfo {
    // message fields
    // @@protoc_insertion_point(field:SimpleInfo.platform_type)
    pub platform_type: ::protobuf::EnumOrUnknown<super::PlatformType::PlatformType>,
    // @@protoc_insertion_point(field:SimpleInfo.ANPLLAOBFJI)
    pub ANPLLAOBFJI: u32,
    // @@protoc_insertion_point(field:SimpleInfo.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:SimpleInfo.head_icon)
    pub head_icon: u32,
    // @@protoc_insertion_point(field:SimpleInfo.nickname)
    pub nickname: ::std::string::String,
    // @@protoc_insertion_point(field:SimpleInfo.signature)
    pub signature: ::std::string::String,
    // @@protoc_insertion_point(field:SimpleInfo.JFGAEKJJPIE)
    pub JFGAEKJJPIE: bool,
    // @@protoc_insertion_point(field:SimpleInfo.GMALCPNOHBF)
    pub GMALCPNOHBF: ::std::string::String,
    // @@protoc_insertion_point(field:SimpleInfo.assist_simple_info)
    pub assist_simple_info: ::std::vec::Vec<super::AssistSimpleInfo::AssistSimpleInfo>,
    // @@protoc_insertion_point(field:SimpleInfo.PADMHPNGPNO)
    pub PADMHPNGPNO: u32,
    // @@protoc_insertion_point(field:SimpleInfo.last_active_time)
    pub last_active_time: i64,
    // @@protoc_insertion_point(field:SimpleInfo.level)
    pub level: u32,
    // @@protoc_insertion_point(field:SimpleInfo.online_status)
    pub online_status: ::protobuf::EnumOrUnknown<super::FriendOnlineStatus::FriendOnlineStatus>,
    // @@protoc_insertion_point(field:SimpleInfo.AKCEJFCFBAN)
    pub AKCEJFCFBAN: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:SimpleInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SimpleInfo {
    fn default() -> &'a SimpleInfo {
        <SimpleInfo as ::protobuf::Message>::default_instance()
    }
}

impl SimpleInfo {
    pub fn new() -> SimpleInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(14);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "platform_type",
            |m: &SimpleInfo| { &m.platform_type },
            |m: &mut SimpleInfo| { &mut m.platform_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ANPLLAOBFJI",
            |m: &SimpleInfo| { &m.ANPLLAOBFJI },
            |m: &mut SimpleInfo| { &mut m.ANPLLAOBFJI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &SimpleInfo| { &m.uid },
            |m: &mut SimpleInfo| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "head_icon",
            |m: &SimpleInfo| { &m.head_icon },
            |m: &mut SimpleInfo| { &mut m.head_icon },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "nickname",
            |m: &SimpleInfo| { &m.nickname },
            |m: &mut SimpleInfo| { &mut m.nickname },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "signature",
            |m: &SimpleInfo| { &m.signature },
            |m: &mut SimpleInfo| { &mut m.signature },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JFGAEKJJPIE",
            |m: &SimpleInfo| { &m.JFGAEKJJPIE },
            |m: &mut SimpleInfo| { &mut m.JFGAEKJJPIE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GMALCPNOHBF",
            |m: &SimpleInfo| { &m.GMALCPNOHBF },
            |m: &mut SimpleInfo| { &mut m.GMALCPNOHBF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "assist_simple_info",
            |m: &SimpleInfo| { &m.assist_simple_info },
            |m: &mut SimpleInfo| { &mut m.assist_simple_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PADMHPNGPNO",
            |m: &SimpleInfo| { &m.PADMHPNGPNO },
            |m: &mut SimpleInfo| { &mut m.PADMHPNGPNO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "last_active_time",
            |m: &SimpleInfo| { &m.last_active_time },
            |m: &mut SimpleInfo| { &mut m.last_active_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &SimpleInfo| { &m.level },
            |m: &mut SimpleInfo| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "online_status",
            |m: &SimpleInfo| { &m.online_status },
            |m: &mut SimpleInfo| { &mut m.online_status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AKCEJFCFBAN",
            |m: &SimpleInfo| { &m.AKCEJFCFBAN },
            |m: &mut SimpleInfo| { &mut m.AKCEJFCFBAN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SimpleInfo>(
            "SimpleInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SimpleInfo {
    const NAME: &'static str = "SimpleInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.platform_type = is.read_enum_or_unknown()?;
                },
                112 => {
                    self.ANPLLAOBFJI = is.read_uint32()?;
                },
                80 => {
                    self.uid = is.read_uint32()?;
                },
                16 => {
                    self.head_icon = is.read_uint32()?;
                },
                122 => {
                    self.nickname = is.read_string()?;
                },
                50 => {
                    self.signature = is.read_string()?;
                },
                56 => {
                    self.JFGAEKJJPIE = is.read_bool()?;
                },
                106 => {
                    self.GMALCPNOHBF = is.read_string()?;
                },
                42 => {
                    self.assist_simple_info.push(is.read_message()?);
                },
                64 => {
                    self.PADMHPNGPNO = is.read_uint32()?;
                },
                88 => {
                    self.last_active_time = is.read_int64()?;
                },
                24 => {
                    self.level = is.read_uint32()?;
                },
                96 => {
                    self.online_status = is.read_enum_or_unknown()?;
                },
                34 => {
                    self.AKCEJFCFBAN = is.read_string()?;
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
        if self.platform_type != ::protobuf::EnumOrUnknown::new(super::PlatformType::PlatformType::EDITOR) {
            my_size += ::protobuf::rt::int32_size(9, self.platform_type.value());
        }
        if self.ANPLLAOBFJI != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.ANPLLAOBFJI);
        }
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.uid);
        }
        if self.head_icon != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.head_icon);
        }
        if !self.nickname.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.nickname);
        }
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.signature);
        }
        if self.JFGAEKJJPIE != false {
            my_size += 1 + 1;
        }
        if !self.GMALCPNOHBF.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.GMALCPNOHBF);
        }
        for value in &self.assist_simple_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.PADMHPNGPNO != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.PADMHPNGPNO);
        }
        if self.last_active_time != 0 {
            my_size += ::protobuf::rt::int64_size(11, self.last_active_time);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.level);
        }
        if self.online_status != ::protobuf::EnumOrUnknown::new(super::FriendOnlineStatus::FriendOnlineStatus::FRIEND_ONLINE_STATUS_OFFLINE) {
            my_size += ::protobuf::rt::int32_size(12, self.online_status.value());
        }
        if !self.AKCEJFCFBAN.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.AKCEJFCFBAN);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.platform_type != ::protobuf::EnumOrUnknown::new(super::PlatformType::PlatformType::EDITOR) {
            os.write_enum(9, ::protobuf::EnumOrUnknown::value(&self.platform_type))?;
        }
        if self.ANPLLAOBFJI != 0 {
            os.write_uint32(14, self.ANPLLAOBFJI)?;
        }
        if self.uid != 0 {
            os.write_uint32(10, self.uid)?;
        }
        if self.head_icon != 0 {
            os.write_uint32(2, self.head_icon)?;
        }
        if !self.nickname.is_empty() {
            os.write_string(15, &self.nickname)?;
        }
        if !self.signature.is_empty() {
            os.write_string(6, &self.signature)?;
        }
        if self.JFGAEKJJPIE != false {
            os.write_bool(7, self.JFGAEKJJPIE)?;
        }
        if !self.GMALCPNOHBF.is_empty() {
            os.write_string(13, &self.GMALCPNOHBF)?;
        }
        for v in &self.assist_simple_info {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if self.PADMHPNGPNO != 0 {
            os.write_uint32(8, self.PADMHPNGPNO)?;
        }
        if self.last_active_time != 0 {
            os.write_int64(11, self.last_active_time)?;
        }
        if self.level != 0 {
            os.write_uint32(3, self.level)?;
        }
        if self.online_status != ::protobuf::EnumOrUnknown::new(super::FriendOnlineStatus::FriendOnlineStatus::FRIEND_ONLINE_STATUS_OFFLINE) {
            os.write_enum(12, ::protobuf::EnumOrUnknown::value(&self.online_status))?;
        }
        if !self.AKCEJFCFBAN.is_empty() {
            os.write_string(4, &self.AKCEJFCFBAN)?;
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

    fn new() -> SimpleInfo {
        SimpleInfo::new()
    }

    fn clear(&mut self) {
        self.platform_type = ::protobuf::EnumOrUnknown::new(super::PlatformType::PlatformType::EDITOR);
        self.ANPLLAOBFJI = 0;
        self.uid = 0;
        self.head_icon = 0;
        self.nickname.clear();
        self.signature.clear();
        self.JFGAEKJJPIE = false;
        self.GMALCPNOHBF.clear();
        self.assist_simple_info.clear();
        self.PADMHPNGPNO = 0;
        self.last_active_time = 0;
        self.level = 0;
        self.online_status = ::protobuf::EnumOrUnknown::new(super::FriendOnlineStatus::FriendOnlineStatus::FRIEND_ONLINE_STATUS_OFFLINE);
        self.AKCEJFCFBAN.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SimpleInfo {
        static instance: SimpleInfo = SimpleInfo {
            platform_type: ::protobuf::EnumOrUnknown::from_i32(0),
            ANPLLAOBFJI: 0,
            uid: 0,
            head_icon: 0,
            nickname: ::std::string::String::new(),
            signature: ::std::string::String::new(),
            JFGAEKJJPIE: false,
            GMALCPNOHBF: ::std::string::String::new(),
            assist_simple_info: ::std::vec::Vec::new(),
            PADMHPNGPNO: 0,
            last_active_time: 0,
            level: 0,
            online_status: ::protobuf::EnumOrUnknown::from_i32(0),
            AKCEJFCFBAN: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SimpleInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SimpleInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SimpleInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SimpleInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10SimpleInfo.proto\x1a\x16AssistSimpleInfo.proto\x1a\x18FriendOnline\
    Status.proto\x1a\x12PlatformType.proto\"\x8e\x04\n\nSimpleInfo\x122\n\rp\
    latform_type\x18\t\x20\x01(\x0e2\r.PlatformTypeR\x0cplatformType\x12\x20\
    \n\x0bANPLLAOBFJI\x18\x0e\x20\x01(\rR\x0bANPLLAOBFJI\x12\x10\n\x03uid\
    \x18\n\x20\x01(\rR\x03uid\x12\x1b\n\thead_icon\x18\x02\x20\x01(\rR\x08he\
    adIcon\x12\x1a\n\x08nickname\x18\x0f\x20\x01(\tR\x08nickname\x12\x1c\n\t\
    signature\x18\x06\x20\x01(\tR\tsignature\x12\x20\n\x0bJFGAEKJJPIE\x18\
    \x07\x20\x01(\x08R\x0bJFGAEKJJPIE\x12\x20\n\x0bGMALCPNOHBF\x18\r\x20\x01\
    (\tR\x0bGMALCPNOHBF\x12?\n\x12assist_simple_info\x18\x05\x20\x03(\x0b2\
    \x11.AssistSimpleInfoR\x10assistSimpleInfo\x12\x20\n\x0bPADMHPNGPNO\x18\
    \x08\x20\x01(\rR\x0bPADMHPNGPNO\x12(\n\x10last_active_time\x18\x0b\x20\
    \x01(\x03R\x0elastActiveTime\x12\x14\n\x05level\x18\x03\x20\x01(\rR\x05l\
    evel\x128\n\ronline_status\x18\x0c\x20\x01(\x0e2\x13.FriendOnlineStatusR\
    \x0conlineStatus\x12\x20\n\x0bAKCEJFCFBAN\x18\x04\x20\x01(\tR\x0bAKCEJFC\
    FBANb\x06proto3\
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
            deps.push(super::AssistSimpleInfo::file_descriptor().clone());
            deps.push(super::FriendOnlineStatus::file_descriptor().clone());
            deps.push(super::PlatformType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SimpleInfo::generated_message_descriptor_data());
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
