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

//! Generated file from `CmdActivityType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdActivityType)
pub enum CmdActivityType {
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdActivityTypeNone)
    CmdActivityTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeTrialActivityRewardCsReq)
    CmdTakeTrialActivityRewardCsReq = 2666,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetMaterialSubmitActivityDataCsReq)
    CmdGetMaterialSubmitActivityDataCsReq = 2691,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdLeaveTrialActivityScRsp)
    CmdLeaveTrialActivityScRsp = 2664,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdStartTrialActivityCsReq)
    CmdStartTrialActivityCsReq = 2621,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdEnterTrialActivityStageScRsp)
    CmdEnterTrialActivityStageScRsp = 2669,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdSubmitMaterialSubmitActivityMaterialCsReq)
    CmdSubmitMaterialSubmitActivityMaterialCsReq = 2657,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetActivityScheduleConfigScRsp)
    CmdGetActivityScheduleConfigScRsp = 2606,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdAvatarDeliverRewardTakeRewardScRsp)
    CmdAvatarDeliverRewardTakeRewardScRsp = 2616,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdCurTrialActivityScNotify)
    CmdCurTrialActivityScNotify = 2601,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeTrialActivityRewardScRsp)
    CmdTakeTrialActivityRewardScRsp = 2699,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetTrialActivityDataCsReq)
    CmdGetTrialActivityDataCsReq = 2663,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeMaterialSubmitActivityRewardCsReq)
    CmdTakeMaterialSubmitActivityRewardCsReq = 2610,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetAvatarDeliverRewardActivityDataCsReq)
    CmdGetAvatarDeliverRewardActivityDataCsReq = 2640,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdLeaveTrialActivityCsReq)
    CmdLeaveTrialActivityCsReq = 2633,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTrialActivityDataChangeScNotify)
    CmdTrialActivityDataChangeScNotify = 2678,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetActivityScheduleConfigCsReq)
    CmdGetActivityScheduleConfigCsReq = 2635,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdSubmitMaterialSubmitActivityMaterialScRsp)
    CmdSubmitMaterialSubmitActivityMaterialScRsp = 2625,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetLoginActivityScRsp)
    CmdGetLoginActivityScRsp = 2613,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeLoginActivityRewardScRsp)
    CmdTakeLoginActivityRewardScRsp = 2609,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetAvatarDeliverRewardActivityDataScRsp)
    CmdGetAvatarDeliverRewardActivityDataScRsp = 2659,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdEnterTrialActivityStageCsReq)
    CmdEnterTrialActivityStageCsReq = 2696,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeLoginActivityRewardCsReq)
    CmdTakeLoginActivityRewardCsReq = 2647,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdAvatarDeliverRewardChooseAvatarScRsp)
    CmdAvatarDeliverRewardChooseAvatarScRsp = 2667,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetTrialActivityDataScRsp)
    CmdGetTrialActivityDataScRsp = 2604,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdAvatarDeliverRewardChooseAvatarCsReq)
    CmdAvatarDeliverRewardChooseAvatarCsReq = 2627,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdStartTrialActivityScRsp)
    CmdStartTrialActivityScRsp = 2608,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdTakeMaterialSubmitActivityRewardScRsp)
    CmdTakeMaterialSubmitActivityRewardScRsp = 2607,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetLoginActivityCsReq)
    CmdGetLoginActivityCsReq = 2611,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdAvatarDeliverRewardTakeRewardCsReq)
    CmdAvatarDeliverRewardTakeRewardCsReq = 2655,
    // @@protoc_insertion_point(enum_value:CmdActivityType.CmdGetMaterialSubmitActivityDataScRsp)
    CmdGetMaterialSubmitActivityDataScRsp = 2693,
}

impl ::protobuf::Enum for CmdActivityType {
    const NAME: &'static str = "CmdActivityType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdActivityType> {
        match value {
            0 => ::std::option::Option::Some(CmdActivityType::CmdActivityTypeNone),
            2666 => ::std::option::Option::Some(CmdActivityType::CmdTakeTrialActivityRewardCsReq),
            2691 => ::std::option::Option::Some(CmdActivityType::CmdGetMaterialSubmitActivityDataCsReq),
            2664 => ::std::option::Option::Some(CmdActivityType::CmdLeaveTrialActivityScRsp),
            2621 => ::std::option::Option::Some(CmdActivityType::CmdStartTrialActivityCsReq),
            2669 => ::std::option::Option::Some(CmdActivityType::CmdEnterTrialActivityStageScRsp),
            2657 => ::std::option::Option::Some(CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialCsReq),
            2606 => ::std::option::Option::Some(CmdActivityType::CmdGetActivityScheduleConfigScRsp),
            2616 => ::std::option::Option::Some(CmdActivityType::CmdAvatarDeliverRewardTakeRewardScRsp),
            2601 => ::std::option::Option::Some(CmdActivityType::CmdCurTrialActivityScNotify),
            2699 => ::std::option::Option::Some(CmdActivityType::CmdTakeTrialActivityRewardScRsp),
            2663 => ::std::option::Option::Some(CmdActivityType::CmdGetTrialActivityDataCsReq),
            2610 => ::std::option::Option::Some(CmdActivityType::CmdTakeMaterialSubmitActivityRewardCsReq),
            2640 => ::std::option::Option::Some(CmdActivityType::CmdGetAvatarDeliverRewardActivityDataCsReq),
            2633 => ::std::option::Option::Some(CmdActivityType::CmdLeaveTrialActivityCsReq),
            2678 => ::std::option::Option::Some(CmdActivityType::CmdTrialActivityDataChangeScNotify),
            2635 => ::std::option::Option::Some(CmdActivityType::CmdGetActivityScheduleConfigCsReq),
            2625 => ::std::option::Option::Some(CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialScRsp),
            2613 => ::std::option::Option::Some(CmdActivityType::CmdGetLoginActivityScRsp),
            2609 => ::std::option::Option::Some(CmdActivityType::CmdTakeLoginActivityRewardScRsp),
            2659 => ::std::option::Option::Some(CmdActivityType::CmdGetAvatarDeliverRewardActivityDataScRsp),
            2696 => ::std::option::Option::Some(CmdActivityType::CmdEnterTrialActivityStageCsReq),
            2647 => ::std::option::Option::Some(CmdActivityType::CmdTakeLoginActivityRewardCsReq),
            2667 => ::std::option::Option::Some(CmdActivityType::CmdAvatarDeliverRewardChooseAvatarScRsp),
            2604 => ::std::option::Option::Some(CmdActivityType::CmdGetTrialActivityDataScRsp),
            2627 => ::std::option::Option::Some(CmdActivityType::CmdAvatarDeliverRewardChooseAvatarCsReq),
            2608 => ::std::option::Option::Some(CmdActivityType::CmdStartTrialActivityScRsp),
            2607 => ::std::option::Option::Some(CmdActivityType::CmdTakeMaterialSubmitActivityRewardScRsp),
            2611 => ::std::option::Option::Some(CmdActivityType::CmdGetLoginActivityCsReq),
            2655 => ::std::option::Option::Some(CmdActivityType::CmdAvatarDeliverRewardTakeRewardCsReq),
            2693 => ::std::option::Option::Some(CmdActivityType::CmdGetMaterialSubmitActivityDataScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdActivityType> {
        match str {
            "CmdActivityTypeNone" => ::std::option::Option::Some(CmdActivityType::CmdActivityTypeNone),
            "CmdTakeTrialActivityRewardCsReq" => ::std::option::Option::Some(CmdActivityType::CmdTakeTrialActivityRewardCsReq),
            "CmdGetMaterialSubmitActivityDataCsReq" => ::std::option::Option::Some(CmdActivityType::CmdGetMaterialSubmitActivityDataCsReq),
            "CmdLeaveTrialActivityScRsp" => ::std::option::Option::Some(CmdActivityType::CmdLeaveTrialActivityScRsp),
            "CmdStartTrialActivityCsReq" => ::std::option::Option::Some(CmdActivityType::CmdStartTrialActivityCsReq),
            "CmdEnterTrialActivityStageScRsp" => ::std::option::Option::Some(CmdActivityType::CmdEnterTrialActivityStageScRsp),
            "CmdSubmitMaterialSubmitActivityMaterialCsReq" => ::std::option::Option::Some(CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialCsReq),
            "CmdGetActivityScheduleConfigScRsp" => ::std::option::Option::Some(CmdActivityType::CmdGetActivityScheduleConfigScRsp),
            "CmdAvatarDeliverRewardTakeRewardScRsp" => ::std::option::Option::Some(CmdActivityType::CmdAvatarDeliverRewardTakeRewardScRsp),
            "CmdCurTrialActivityScNotify" => ::std::option::Option::Some(CmdActivityType::CmdCurTrialActivityScNotify),
            "CmdTakeTrialActivityRewardScRsp" => ::std::option::Option::Some(CmdActivityType::CmdTakeTrialActivityRewardScRsp),
            "CmdGetTrialActivityDataCsReq" => ::std::option::Option::Some(CmdActivityType::CmdGetTrialActivityDataCsReq),
            "CmdTakeMaterialSubmitActivityRewardCsReq" => ::std::option::Option::Some(CmdActivityType::CmdTakeMaterialSubmitActivityRewardCsReq),
            "CmdGetAvatarDeliverRewardActivityDataCsReq" => ::std::option::Option::Some(CmdActivityType::CmdGetAvatarDeliverRewardActivityDataCsReq),
            "CmdLeaveTrialActivityCsReq" => ::std::option::Option::Some(CmdActivityType::CmdLeaveTrialActivityCsReq),
            "CmdTrialActivityDataChangeScNotify" => ::std::option::Option::Some(CmdActivityType::CmdTrialActivityDataChangeScNotify),
            "CmdGetActivityScheduleConfigCsReq" => ::std::option::Option::Some(CmdActivityType::CmdGetActivityScheduleConfigCsReq),
            "CmdSubmitMaterialSubmitActivityMaterialScRsp" => ::std::option::Option::Some(CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialScRsp),
            "CmdGetLoginActivityScRsp" => ::std::option::Option::Some(CmdActivityType::CmdGetLoginActivityScRsp),
            "CmdTakeLoginActivityRewardScRsp" => ::std::option::Option::Some(CmdActivityType::CmdTakeLoginActivityRewardScRsp),
            "CmdGetAvatarDeliverRewardActivityDataScRsp" => ::std::option::Option::Some(CmdActivityType::CmdGetAvatarDeliverRewardActivityDataScRsp),
            "CmdEnterTrialActivityStageCsReq" => ::std::option::Option::Some(CmdActivityType::CmdEnterTrialActivityStageCsReq),
            "CmdTakeLoginActivityRewardCsReq" => ::std::option::Option::Some(CmdActivityType::CmdTakeLoginActivityRewardCsReq),
            "CmdAvatarDeliverRewardChooseAvatarScRsp" => ::std::option::Option::Some(CmdActivityType::CmdAvatarDeliverRewardChooseAvatarScRsp),
            "CmdGetTrialActivityDataScRsp" => ::std::option::Option::Some(CmdActivityType::CmdGetTrialActivityDataScRsp),
            "CmdAvatarDeliverRewardChooseAvatarCsReq" => ::std::option::Option::Some(CmdActivityType::CmdAvatarDeliverRewardChooseAvatarCsReq),
            "CmdStartTrialActivityScRsp" => ::std::option::Option::Some(CmdActivityType::CmdStartTrialActivityScRsp),
            "CmdTakeMaterialSubmitActivityRewardScRsp" => ::std::option::Option::Some(CmdActivityType::CmdTakeMaterialSubmitActivityRewardScRsp),
            "CmdGetLoginActivityCsReq" => ::std::option::Option::Some(CmdActivityType::CmdGetLoginActivityCsReq),
            "CmdAvatarDeliverRewardTakeRewardCsReq" => ::std::option::Option::Some(CmdActivityType::CmdAvatarDeliverRewardTakeRewardCsReq),
            "CmdGetMaterialSubmitActivityDataScRsp" => ::std::option::Option::Some(CmdActivityType::CmdGetMaterialSubmitActivityDataScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdActivityType] = &[
        CmdActivityType::CmdActivityTypeNone,
        CmdActivityType::CmdTakeTrialActivityRewardCsReq,
        CmdActivityType::CmdGetMaterialSubmitActivityDataCsReq,
        CmdActivityType::CmdLeaveTrialActivityScRsp,
        CmdActivityType::CmdStartTrialActivityCsReq,
        CmdActivityType::CmdEnterTrialActivityStageScRsp,
        CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialCsReq,
        CmdActivityType::CmdGetActivityScheduleConfigScRsp,
        CmdActivityType::CmdAvatarDeliverRewardTakeRewardScRsp,
        CmdActivityType::CmdCurTrialActivityScNotify,
        CmdActivityType::CmdTakeTrialActivityRewardScRsp,
        CmdActivityType::CmdGetTrialActivityDataCsReq,
        CmdActivityType::CmdTakeMaterialSubmitActivityRewardCsReq,
        CmdActivityType::CmdGetAvatarDeliverRewardActivityDataCsReq,
        CmdActivityType::CmdLeaveTrialActivityCsReq,
        CmdActivityType::CmdTrialActivityDataChangeScNotify,
        CmdActivityType::CmdGetActivityScheduleConfigCsReq,
        CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialScRsp,
        CmdActivityType::CmdGetLoginActivityScRsp,
        CmdActivityType::CmdTakeLoginActivityRewardScRsp,
        CmdActivityType::CmdGetAvatarDeliverRewardActivityDataScRsp,
        CmdActivityType::CmdEnterTrialActivityStageCsReq,
        CmdActivityType::CmdTakeLoginActivityRewardCsReq,
        CmdActivityType::CmdAvatarDeliverRewardChooseAvatarScRsp,
        CmdActivityType::CmdGetTrialActivityDataScRsp,
        CmdActivityType::CmdAvatarDeliverRewardChooseAvatarCsReq,
        CmdActivityType::CmdStartTrialActivityScRsp,
        CmdActivityType::CmdTakeMaterialSubmitActivityRewardScRsp,
        CmdActivityType::CmdGetLoginActivityCsReq,
        CmdActivityType::CmdAvatarDeliverRewardTakeRewardCsReq,
        CmdActivityType::CmdGetMaterialSubmitActivityDataScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdActivityType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdActivityType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdActivityType::CmdActivityTypeNone => 0,
            CmdActivityType::CmdTakeTrialActivityRewardCsReq => 1,
            CmdActivityType::CmdGetMaterialSubmitActivityDataCsReq => 2,
            CmdActivityType::CmdLeaveTrialActivityScRsp => 3,
            CmdActivityType::CmdStartTrialActivityCsReq => 4,
            CmdActivityType::CmdEnterTrialActivityStageScRsp => 5,
            CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialCsReq => 6,
            CmdActivityType::CmdGetActivityScheduleConfigScRsp => 7,
            CmdActivityType::CmdAvatarDeliverRewardTakeRewardScRsp => 8,
            CmdActivityType::CmdCurTrialActivityScNotify => 9,
            CmdActivityType::CmdTakeTrialActivityRewardScRsp => 10,
            CmdActivityType::CmdGetTrialActivityDataCsReq => 11,
            CmdActivityType::CmdTakeMaterialSubmitActivityRewardCsReq => 12,
            CmdActivityType::CmdGetAvatarDeliverRewardActivityDataCsReq => 13,
            CmdActivityType::CmdLeaveTrialActivityCsReq => 14,
            CmdActivityType::CmdTrialActivityDataChangeScNotify => 15,
            CmdActivityType::CmdGetActivityScheduleConfigCsReq => 16,
            CmdActivityType::CmdSubmitMaterialSubmitActivityMaterialScRsp => 17,
            CmdActivityType::CmdGetLoginActivityScRsp => 18,
            CmdActivityType::CmdTakeLoginActivityRewardScRsp => 19,
            CmdActivityType::CmdGetAvatarDeliverRewardActivityDataScRsp => 20,
            CmdActivityType::CmdEnterTrialActivityStageCsReq => 21,
            CmdActivityType::CmdTakeLoginActivityRewardCsReq => 22,
            CmdActivityType::CmdAvatarDeliverRewardChooseAvatarScRsp => 23,
            CmdActivityType::CmdGetTrialActivityDataScRsp => 24,
            CmdActivityType::CmdAvatarDeliverRewardChooseAvatarCsReq => 25,
            CmdActivityType::CmdStartTrialActivityScRsp => 26,
            CmdActivityType::CmdTakeMaterialSubmitActivityRewardScRsp => 27,
            CmdActivityType::CmdGetLoginActivityCsReq => 28,
            CmdActivityType::CmdAvatarDeliverRewardTakeRewardCsReq => 29,
            CmdActivityType::CmdGetMaterialSubmitActivityDataScRsp => 30,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdActivityType {
    fn default() -> Self {
        CmdActivityType::CmdActivityTypeNone
    }
}

impl CmdActivityType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdActivityType>("CmdActivityType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15CmdActivityType.proto*\xe3\t\n\x0fCmdActivityType\x12\x17\n\x13Cmd\
    ActivityTypeNone\x10\0\x12$\n\x1fCmdTakeTrialActivityRewardCsReq\x10\xea\
    \x14\x12*\n%CmdGetMaterialSubmitActivityDataCsReq\x10\x83\x15\x12\x1f\n\
    \x1aCmdLeaveTrialActivityScRsp\x10\xe8\x14\x12\x1f\n\x1aCmdStartTrialAct\
    ivityCsReq\x10\xbd\x14\x12$\n\x1fCmdEnterTrialActivityStageScRsp\x10\xed\
    \x14\x121\n,CmdSubmitMaterialSubmitActivityMaterialCsReq\x10\xe1\x14\x12\
    &\n!CmdGetActivityScheduleConfigScRsp\x10\xae\x14\x12*\n%CmdAvatarDelive\
    rRewardTakeRewardScRsp\x10\xb8\x14\x12\x20\n\x1bCmdCurTrialActivityScNot\
    ify\x10\xa9\x14\x12$\n\x1fCmdTakeTrialActivityRewardScRsp\x10\x8b\x15\
    \x12!\n\x1cCmdGetTrialActivityDataCsReq\x10\xe7\x14\x12-\n(CmdTakeMateri\
    alSubmitActivityRewardCsReq\x10\xb2\x14\x12/\n*CmdGetAvatarDeliverReward\
    ActivityDataCsReq\x10\xd0\x14\x12\x1f\n\x1aCmdLeaveTrialActivityCsReq\
    \x10\xc9\x14\x12'\n\"CmdTrialActivityDataChangeScNotify\x10\xf6\x14\x12&\
    \n!CmdGetActivityScheduleConfigCsReq\x10\xcb\x14\x121\n,CmdSubmitMateria\
    lSubmitActivityMaterialScRsp\x10\xc1\x14\x12\x1d\n\x18CmdGetLoginActivit\
    yScRsp\x10\xb5\x14\x12$\n\x1fCmdTakeLoginActivityRewardScRsp\x10\xb1\x14\
    \x12/\n*CmdGetAvatarDeliverRewardActivityDataScRsp\x10\xe3\x14\x12$\n\
    \x1fCmdEnterTrialActivityStageCsReq\x10\x88\x15\x12$\n\x1fCmdTakeLoginAc\
    tivityRewardCsReq\x10\xd7\x14\x12,\n'CmdAvatarDeliverRewardChooseAvatarS\
    cRsp\x10\xeb\x14\x12!\n\x1cCmdGetTrialActivityDataScRsp\x10\xac\x14\x12,\
    \n'CmdAvatarDeliverRewardChooseAvatarCsReq\x10\xc3\x14\x12\x1f\n\x1aCmdS\
    tartTrialActivityScRsp\x10\xb0\x14\x12-\n(CmdTakeMaterialSubmitActivityR\
    ewardScRsp\x10\xaf\x14\x12\x1d\n\x18CmdGetLoginActivityCsReq\x10\xb3\x14\
    \x12*\n%CmdAvatarDeliverRewardTakeRewardCsReq\x10\xdf\x14\x12*\n%CmdGetM\
    aterialSubmitActivityDataScRsp\x10\x85\x15b\x06proto3\
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
            enums.push(CmdActivityType::generated_enum_descriptor_data());
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
