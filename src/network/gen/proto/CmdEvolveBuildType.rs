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

//! Generated file from `CmdEvolveBuildType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdEvolveBuildType)
pub enum CmdEvolveBuildType {
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildNone)
    CmdEvolveBuildNone = 0,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildStartLevelCsReq)
    CmdEvolveBuildStartLevelCsReq = 7114,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildReRandomStageCsReq)
    CmdEvolveBuildReRandomStageCsReq = 7132,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityDownCsReq)
    CmdEvolveBuildShopAbilityDownCsReq = 7148,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildQueryInfoCsReq)
    CmdEvolveBuildQueryInfoCsReq = 7142,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityDownScRsp)
    CmdEvolveBuildShopAbilityDownScRsp = 7135,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildStartStageScRsp)
    CmdEvolveBuildStartStageScRsp = 7109,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildStartStageCsReq)
    CmdEvolveBuildStartStageCsReq = 7141,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildLeaveScRsp)
    CmdEvolveBuildLeaveScRsp = 7110,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildStartLevelScRsp)
    CmdEvolveBuildStartLevelScRsp = 7106,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildReRandomStageScRsp)
    CmdEvolveBuildReRandomStageScRsp = 7146,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityResetCsReq)
    CmdEvolveBuildShopAbilityResetCsReq = 7111,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildGiveupScRsp)
    CmdEvolveBuildGiveupScRsp = 7145,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityResetScRsp)
    CmdEvolveBuildShopAbilityResetScRsp = 7104,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildFinishScNotify)
    CmdEvolveBuildFinishScNotify = 7116,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildCoinNotify)
    CmdEvolveBuildCoinNotify = 7149,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildGiveupCsReq)
    CmdEvolveBuildGiveupCsReq = 7129,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildLeaveCsReq)
    CmdEvolveBuildLeaveCsReq = 7128,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildQueryInfoScRsp)
    CmdEvolveBuildQueryInfoScRsp = 7118,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildTakeExpRewardCsReq)
    CmdEvolveBuildTakeExpRewardCsReq = 7122,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildUnlockInfoNotify)
    CmdEvolveBuildUnlockInfoNotify = 7105,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildTakeExpRewardScRsp)
    CmdEvolveBuildTakeExpRewardScRsp = 7137,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityUpCsReq)
    CmdEvolveBuildShopAbilityUpCsReq = 7138,
    // @@protoc_insertion_point(enum_value:CmdEvolveBuildType.CmdEvolveBuildShopAbilityUpScRsp)
    CmdEvolveBuildShopAbilityUpScRsp = 7150,
}

impl ::protobuf::Enum for CmdEvolveBuildType {
    const NAME: &'static str = "CmdEvolveBuildType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdEvolveBuildType> {
        match value {
            0 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildNone),
            7114 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartLevelCsReq),
            7132 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildReRandomStageCsReq),
            7148 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownCsReq),
            7142 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildQueryInfoCsReq),
            7135 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownScRsp),
            7109 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartStageScRsp),
            7141 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartStageCsReq),
            7110 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildLeaveScRsp),
            7106 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartLevelScRsp),
            7146 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildReRandomStageScRsp),
            7111 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetCsReq),
            7145 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildGiveupScRsp),
            7104 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetScRsp),
            7116 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildFinishScNotify),
            7149 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildCoinNotify),
            7129 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildGiveupCsReq),
            7128 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildLeaveCsReq),
            7118 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildQueryInfoScRsp),
            7122 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardCsReq),
            7105 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildUnlockInfoNotify),
            7137 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardScRsp),
            7138 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpCsReq),
            7150 => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdEvolveBuildType> {
        match str {
            "CmdEvolveBuildNone" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildNone),
            "CmdEvolveBuildStartLevelCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartLevelCsReq),
            "CmdEvolveBuildReRandomStageCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildReRandomStageCsReq),
            "CmdEvolveBuildShopAbilityDownCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownCsReq),
            "CmdEvolveBuildQueryInfoCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildQueryInfoCsReq),
            "CmdEvolveBuildShopAbilityDownScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownScRsp),
            "CmdEvolveBuildStartStageScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartStageScRsp),
            "CmdEvolveBuildStartStageCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartStageCsReq),
            "CmdEvolveBuildLeaveScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildLeaveScRsp),
            "CmdEvolveBuildStartLevelScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildStartLevelScRsp),
            "CmdEvolveBuildReRandomStageScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildReRandomStageScRsp),
            "CmdEvolveBuildShopAbilityResetCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetCsReq),
            "CmdEvolveBuildGiveupScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildGiveupScRsp),
            "CmdEvolveBuildShopAbilityResetScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetScRsp),
            "CmdEvolveBuildFinishScNotify" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildFinishScNotify),
            "CmdEvolveBuildCoinNotify" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildCoinNotify),
            "CmdEvolveBuildGiveupCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildGiveupCsReq),
            "CmdEvolveBuildLeaveCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildLeaveCsReq),
            "CmdEvolveBuildQueryInfoScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildQueryInfoScRsp),
            "CmdEvolveBuildTakeExpRewardCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardCsReq),
            "CmdEvolveBuildUnlockInfoNotify" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildUnlockInfoNotify),
            "CmdEvolveBuildTakeExpRewardScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardScRsp),
            "CmdEvolveBuildShopAbilityUpCsReq" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpCsReq),
            "CmdEvolveBuildShopAbilityUpScRsp" => ::std::option::Option::Some(CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdEvolveBuildType] = &[
        CmdEvolveBuildType::CmdEvolveBuildNone,
        CmdEvolveBuildType::CmdEvolveBuildStartLevelCsReq,
        CmdEvolveBuildType::CmdEvolveBuildReRandomStageCsReq,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownCsReq,
        CmdEvolveBuildType::CmdEvolveBuildQueryInfoCsReq,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownScRsp,
        CmdEvolveBuildType::CmdEvolveBuildStartStageScRsp,
        CmdEvolveBuildType::CmdEvolveBuildStartStageCsReq,
        CmdEvolveBuildType::CmdEvolveBuildLeaveScRsp,
        CmdEvolveBuildType::CmdEvolveBuildStartLevelScRsp,
        CmdEvolveBuildType::CmdEvolveBuildReRandomStageScRsp,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetCsReq,
        CmdEvolveBuildType::CmdEvolveBuildGiveupScRsp,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetScRsp,
        CmdEvolveBuildType::CmdEvolveBuildFinishScNotify,
        CmdEvolveBuildType::CmdEvolveBuildCoinNotify,
        CmdEvolveBuildType::CmdEvolveBuildGiveupCsReq,
        CmdEvolveBuildType::CmdEvolveBuildLeaveCsReq,
        CmdEvolveBuildType::CmdEvolveBuildQueryInfoScRsp,
        CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardCsReq,
        CmdEvolveBuildType::CmdEvolveBuildUnlockInfoNotify,
        CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardScRsp,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpCsReq,
        CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdEvolveBuildType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdEvolveBuildType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdEvolveBuildType::CmdEvolveBuildNone => 0,
            CmdEvolveBuildType::CmdEvolveBuildStartLevelCsReq => 1,
            CmdEvolveBuildType::CmdEvolveBuildReRandomStageCsReq => 2,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownCsReq => 3,
            CmdEvolveBuildType::CmdEvolveBuildQueryInfoCsReq => 4,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityDownScRsp => 5,
            CmdEvolveBuildType::CmdEvolveBuildStartStageScRsp => 6,
            CmdEvolveBuildType::CmdEvolveBuildStartStageCsReq => 7,
            CmdEvolveBuildType::CmdEvolveBuildLeaveScRsp => 8,
            CmdEvolveBuildType::CmdEvolveBuildStartLevelScRsp => 9,
            CmdEvolveBuildType::CmdEvolveBuildReRandomStageScRsp => 10,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetCsReq => 11,
            CmdEvolveBuildType::CmdEvolveBuildGiveupScRsp => 12,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityResetScRsp => 13,
            CmdEvolveBuildType::CmdEvolveBuildFinishScNotify => 14,
            CmdEvolveBuildType::CmdEvolveBuildCoinNotify => 15,
            CmdEvolveBuildType::CmdEvolveBuildGiveupCsReq => 16,
            CmdEvolveBuildType::CmdEvolveBuildLeaveCsReq => 17,
            CmdEvolveBuildType::CmdEvolveBuildQueryInfoScRsp => 18,
            CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardCsReq => 19,
            CmdEvolveBuildType::CmdEvolveBuildUnlockInfoNotify => 20,
            CmdEvolveBuildType::CmdEvolveBuildTakeExpRewardScRsp => 21,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpCsReq => 22,
            CmdEvolveBuildType::CmdEvolveBuildShopAbilityUpScRsp => 23,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdEvolveBuildType {
    fn default() -> Self {
        CmdEvolveBuildType::CmdEvolveBuildNone
    }
}

impl CmdEvolveBuildType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdEvolveBuildType>("CmdEvolveBuildType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18CmdEvolveBuildType.proto*\xf7\x06\n\x12CmdEvolveBuildType\x12\x16\
    \n\x12CmdEvolveBuildNone\x10\0\x12\"\n\x1dCmdEvolveBuildStartLevelCsReq\
    \x10\xca7\x12%\n\x20CmdEvolveBuildReRandomStageCsReq\x10\xdc7\x12'\n\"Cm\
    dEvolveBuildShopAbilityDownCsReq\x10\xec7\x12!\n\x1cCmdEvolveBuildQueryI\
    nfoCsReq\x10\xe67\x12'\n\"CmdEvolveBuildShopAbilityDownScRsp\x10\xdf7\
    \x12\"\n\x1dCmdEvolveBuildStartStageScRsp\x10\xc57\x12\"\n\x1dCmdEvolveB\
    uildStartStageCsReq\x10\xe57\x12\x1d\n\x18CmdEvolveBuildLeaveScRsp\x10\
    \xc67\x12\"\n\x1dCmdEvolveBuildStartLevelScRsp\x10\xc27\x12%\n\x20CmdEvo\
    lveBuildReRandomStageScRsp\x10\xea7\x12(\n#CmdEvolveBuildShopAbilityRese\
    tCsReq\x10\xc77\x12\x1e\n\x19CmdEvolveBuildGiveupScRsp\x10\xe97\x12(\n#C\
    mdEvolveBuildShopAbilityResetScRsp\x10\xc07\x12!\n\x1cCmdEvolveBuildFini\
    shScNotify\x10\xcc7\x12\x1d\n\x18CmdEvolveBuildCoinNotify\x10\xed7\x12\
    \x1e\n\x19CmdEvolveBuildGiveupCsReq\x10\xd97\x12\x1d\n\x18CmdEvolveBuild\
    LeaveCsReq\x10\xd87\x12!\n\x1cCmdEvolveBuildQueryInfoScRsp\x10\xce7\x12%\
    \n\x20CmdEvolveBuildTakeExpRewardCsReq\x10\xd27\x12#\n\x1eCmdEvolveBuild\
    UnlockInfoNotify\x10\xc17\x12%\n\x20CmdEvolveBuildTakeExpRewardScRsp\x10\
    \xe17\x12%\n\x20CmdEvolveBuildShopAbilityUpCsReq\x10\xe27\x12%\n\x20CmdE\
    volveBuildShopAbilityUpScRsp\x10\xee7b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(CmdEvolveBuildType::generated_enum_descriptor_data());
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
