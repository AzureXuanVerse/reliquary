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

//! Generated file from `CmdRogueMagicType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdRogueMagicType)
pub enum CmdRogueMagicType {
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicNone)
    CmdRogueMagicNone = 0,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicScepterDressInUnitScRsp)
    CmdRogueMagicScepterDressInUnitScRsp = 7731,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicEnableTalentCsReq)
    CmdRogueMagicEnableTalentCsReq = 7703,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicEnterLayerCsReq)
    CmdRogueMagicEnterLayerCsReq = 7780,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicStoryInfoUpdateScNotify)
    CmdRogueMagicStoryInfoUpdateScNotify = 7719,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicEnterRoomCsReq)
    CmdRogueMagicEnterRoomCsReq = 7754,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicSettleCsReq)
    CmdRogueMagicSettleCsReq = 7770,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicGetTalentInfoScRsp)
    CmdRogueMagicGetTalentInfoScRsp = 7769,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicLeaveScRsp)
    CmdRogueMagicLeaveScRsp = 7721,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicSetAutoDressInMagicUnitCsReq)
    CmdRogueMagicSetAutoDressInMagicUnitCsReq = 7728,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicScepterDressInUnitCsReq)
    CmdRogueMagicScepterDressInUnitCsReq = 7706,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicAutoDressInUnitScRsp)
    CmdRogueMagicAutoDressInUnitScRsp = 7747,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicQueryCsReq)
    CmdRogueMagicQueryCsReq = 7736,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicUnitReforgeScRsp)
    CmdRogueMagicUnitReforgeScRsp = 7734,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicEnterLayerScRsp)
    CmdRogueMagicEnterLayerScRsp = 7702,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicStartScRsp)
    CmdRogueMagicStartScRsp = 7791,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicReviveCostUpdateScNotify)
    CmdRogueMagicReviveCostUpdateScNotify = 7752,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicBattleFailSettleInfoScNotify)
    CmdRogueMagicBattleFailSettleInfoScNotify = 7738,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicUnitReforgeCsReq)
    CmdRogueMagicUnitReforgeCsReq = 7778,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicAutoDressInMagicUnitChangeScNotify)
    CmdRogueMagicAutoDressInMagicUnitChangeScNotify = 7755,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicLevelInfoUpdateScNotify)
    CmdRogueMagicLevelInfoUpdateScNotify = 7705,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicScepterTakeOffUnitCsReq)
    CmdRogueMagicScepterTakeOffUnitCsReq = 7744,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicGetMiscRealTimeDataCsReq)
    CmdRogueMagicGetMiscRealTimeDataCsReq = 7722,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicAreaUpdateScNotify)
    CmdRogueMagicAreaUpdateScNotify = 7774,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicEnterRoomScRsp)
    CmdRogueMagicEnterRoomScRsp = 7777,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicLeaveCsReq)
    CmdRogueMagicLeaveCsReq = 7727,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicGetTalentInfoCsReq)
    CmdRogueMagicGetTalentInfoCsReq = 7745,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicAutoDressInUnitCsReq)
    CmdRogueMagicAutoDressInUnitCsReq = 7718,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicSettleScRsp)
    CmdRogueMagicSettleScRsp = 7759,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicQueryScRsp)
    CmdRogueMagicQueryScRsp = 7787,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicReviveAvatarScRsp)
    CmdRogueMagicReviveAvatarScRsp = 7794,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicUnitComposeCsReq)
    CmdRogueMagicUnitComposeCsReq = 7741,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicScepterTakeOffUnitScRsp)
    CmdRogueMagicScepterTakeOffUnitScRsp = 7749,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicStartCsReq)
    CmdRogueMagicStartCsReq = 7720,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicEnableTalentScRsp)
    CmdRogueMagicEnableTalentScRsp = 7724,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicGetMiscRealTimeDataScRsp)
    CmdRogueMagicGetMiscRealTimeDataScRsp = 7740,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicEnterCsReq)
    CmdRogueMagicEnterCsReq = 7767,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicUnitComposeScRsp)
    CmdRogueMagicUnitComposeScRsp = 7784,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicEnterScRsp)
    CmdRogueMagicEnterScRsp = 7739,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicReviveAvatarCsReq)
    CmdRogueMagicReviveAvatarCsReq = 7768,
    // @@protoc_insertion_point(enum_value:CmdRogueMagicType.CmdRogueMagicSetAutoDressInMagicUnitScRsp)
    CmdRogueMagicSetAutoDressInMagicUnitScRsp = 7723,
}

impl ::protobuf::Enum for CmdRogueMagicType {
    const NAME: &'static str = "CmdRogueMagicType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdRogueMagicType> {
        match value {
            0 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicNone),
            7731 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicScepterDressInUnitScRsp),
            7703 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnableTalentCsReq),
            7780 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnterLayerCsReq),
            7719 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicStoryInfoUpdateScNotify),
            7754 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnterRoomCsReq),
            7770 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicSettleCsReq),
            7769 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicGetTalentInfoScRsp),
            7721 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicLeaveScRsp),
            7728 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicSetAutoDressInMagicUnitCsReq),
            7706 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicScepterDressInUnitCsReq),
            7747 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicAutoDressInUnitScRsp),
            7736 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicQueryCsReq),
            7734 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicUnitReforgeScRsp),
            7702 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnterLayerScRsp),
            7791 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicStartScRsp),
            7752 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicReviveCostUpdateScNotify),
            7738 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicBattleFailSettleInfoScNotify),
            7778 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicUnitReforgeCsReq),
            7755 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicAutoDressInMagicUnitChangeScNotify),
            7705 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicLevelInfoUpdateScNotify),
            7744 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicScepterTakeOffUnitCsReq),
            7722 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicGetMiscRealTimeDataCsReq),
            7774 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicAreaUpdateScNotify),
            7777 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnterRoomScRsp),
            7727 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicLeaveCsReq),
            7745 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicGetTalentInfoCsReq),
            7718 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicAutoDressInUnitCsReq),
            7759 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicSettleScRsp),
            7787 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicQueryScRsp),
            7794 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicReviveAvatarScRsp),
            7741 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicUnitComposeCsReq),
            7749 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicScepterTakeOffUnitScRsp),
            7720 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicStartCsReq),
            7724 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnableTalentScRsp),
            7740 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicGetMiscRealTimeDataScRsp),
            7767 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnterCsReq),
            7784 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicUnitComposeScRsp),
            7739 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnterScRsp),
            7768 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicReviveAvatarCsReq),
            7723 => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicSetAutoDressInMagicUnitScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdRogueMagicType> {
        match str {
            "CmdRogueMagicNone" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicNone),
            "CmdRogueMagicScepterDressInUnitScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicScepterDressInUnitScRsp),
            "CmdRogueMagicEnableTalentCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnableTalentCsReq),
            "CmdRogueMagicEnterLayerCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnterLayerCsReq),
            "CmdRogueMagicStoryInfoUpdateScNotify" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicStoryInfoUpdateScNotify),
            "CmdRogueMagicEnterRoomCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnterRoomCsReq),
            "CmdRogueMagicSettleCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicSettleCsReq),
            "CmdRogueMagicGetTalentInfoScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicGetTalentInfoScRsp),
            "CmdRogueMagicLeaveScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicLeaveScRsp),
            "CmdRogueMagicSetAutoDressInMagicUnitCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicSetAutoDressInMagicUnitCsReq),
            "CmdRogueMagicScepterDressInUnitCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicScepterDressInUnitCsReq),
            "CmdRogueMagicAutoDressInUnitScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicAutoDressInUnitScRsp),
            "CmdRogueMagicQueryCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicQueryCsReq),
            "CmdRogueMagicUnitReforgeScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicUnitReforgeScRsp),
            "CmdRogueMagicEnterLayerScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnterLayerScRsp),
            "CmdRogueMagicStartScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicStartScRsp),
            "CmdRogueMagicReviveCostUpdateScNotify" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicReviveCostUpdateScNotify),
            "CmdRogueMagicBattleFailSettleInfoScNotify" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicBattleFailSettleInfoScNotify),
            "CmdRogueMagicUnitReforgeCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicUnitReforgeCsReq),
            "CmdRogueMagicAutoDressInMagicUnitChangeScNotify" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicAutoDressInMagicUnitChangeScNotify),
            "CmdRogueMagicLevelInfoUpdateScNotify" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicLevelInfoUpdateScNotify),
            "CmdRogueMagicScepterTakeOffUnitCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicScepterTakeOffUnitCsReq),
            "CmdRogueMagicGetMiscRealTimeDataCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicGetMiscRealTimeDataCsReq),
            "CmdRogueMagicAreaUpdateScNotify" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicAreaUpdateScNotify),
            "CmdRogueMagicEnterRoomScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnterRoomScRsp),
            "CmdRogueMagicLeaveCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicLeaveCsReq),
            "CmdRogueMagicGetTalentInfoCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicGetTalentInfoCsReq),
            "CmdRogueMagicAutoDressInUnitCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicAutoDressInUnitCsReq),
            "CmdRogueMagicSettleScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicSettleScRsp),
            "CmdRogueMagicQueryScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicQueryScRsp),
            "CmdRogueMagicReviveAvatarScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicReviveAvatarScRsp),
            "CmdRogueMagicUnitComposeCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicUnitComposeCsReq),
            "CmdRogueMagicScepterTakeOffUnitScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicScepterTakeOffUnitScRsp),
            "CmdRogueMagicStartCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicStartCsReq),
            "CmdRogueMagicEnableTalentScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnableTalentScRsp),
            "CmdRogueMagicGetMiscRealTimeDataScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicGetMiscRealTimeDataScRsp),
            "CmdRogueMagicEnterCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnterCsReq),
            "CmdRogueMagicUnitComposeScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicUnitComposeScRsp),
            "CmdRogueMagicEnterScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicEnterScRsp),
            "CmdRogueMagicReviveAvatarCsReq" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicReviveAvatarCsReq),
            "CmdRogueMagicSetAutoDressInMagicUnitScRsp" => ::std::option::Option::Some(CmdRogueMagicType::CmdRogueMagicSetAutoDressInMagicUnitScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdRogueMagicType] = &[
        CmdRogueMagicType::CmdRogueMagicNone,
        CmdRogueMagicType::CmdRogueMagicScepterDressInUnitScRsp,
        CmdRogueMagicType::CmdRogueMagicEnableTalentCsReq,
        CmdRogueMagicType::CmdRogueMagicEnterLayerCsReq,
        CmdRogueMagicType::CmdRogueMagicStoryInfoUpdateScNotify,
        CmdRogueMagicType::CmdRogueMagicEnterRoomCsReq,
        CmdRogueMagicType::CmdRogueMagicSettleCsReq,
        CmdRogueMagicType::CmdRogueMagicGetTalentInfoScRsp,
        CmdRogueMagicType::CmdRogueMagicLeaveScRsp,
        CmdRogueMagicType::CmdRogueMagicSetAutoDressInMagicUnitCsReq,
        CmdRogueMagicType::CmdRogueMagicScepterDressInUnitCsReq,
        CmdRogueMagicType::CmdRogueMagicAutoDressInUnitScRsp,
        CmdRogueMagicType::CmdRogueMagicQueryCsReq,
        CmdRogueMagicType::CmdRogueMagicUnitReforgeScRsp,
        CmdRogueMagicType::CmdRogueMagicEnterLayerScRsp,
        CmdRogueMagicType::CmdRogueMagicStartScRsp,
        CmdRogueMagicType::CmdRogueMagicReviveCostUpdateScNotify,
        CmdRogueMagicType::CmdRogueMagicBattleFailSettleInfoScNotify,
        CmdRogueMagicType::CmdRogueMagicUnitReforgeCsReq,
        CmdRogueMagicType::CmdRogueMagicAutoDressInMagicUnitChangeScNotify,
        CmdRogueMagicType::CmdRogueMagicLevelInfoUpdateScNotify,
        CmdRogueMagicType::CmdRogueMagicScepterTakeOffUnitCsReq,
        CmdRogueMagicType::CmdRogueMagicGetMiscRealTimeDataCsReq,
        CmdRogueMagicType::CmdRogueMagicAreaUpdateScNotify,
        CmdRogueMagicType::CmdRogueMagicEnterRoomScRsp,
        CmdRogueMagicType::CmdRogueMagicLeaveCsReq,
        CmdRogueMagicType::CmdRogueMagicGetTalentInfoCsReq,
        CmdRogueMagicType::CmdRogueMagicAutoDressInUnitCsReq,
        CmdRogueMagicType::CmdRogueMagicSettleScRsp,
        CmdRogueMagicType::CmdRogueMagicQueryScRsp,
        CmdRogueMagicType::CmdRogueMagicReviveAvatarScRsp,
        CmdRogueMagicType::CmdRogueMagicUnitComposeCsReq,
        CmdRogueMagicType::CmdRogueMagicScepterTakeOffUnitScRsp,
        CmdRogueMagicType::CmdRogueMagicStartCsReq,
        CmdRogueMagicType::CmdRogueMagicEnableTalentScRsp,
        CmdRogueMagicType::CmdRogueMagicGetMiscRealTimeDataScRsp,
        CmdRogueMagicType::CmdRogueMagicEnterCsReq,
        CmdRogueMagicType::CmdRogueMagicUnitComposeScRsp,
        CmdRogueMagicType::CmdRogueMagicEnterScRsp,
        CmdRogueMagicType::CmdRogueMagicReviveAvatarCsReq,
        CmdRogueMagicType::CmdRogueMagicSetAutoDressInMagicUnitScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdRogueMagicType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdRogueMagicType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdRogueMagicType::CmdRogueMagicNone => 0,
            CmdRogueMagicType::CmdRogueMagicScepterDressInUnitScRsp => 1,
            CmdRogueMagicType::CmdRogueMagicEnableTalentCsReq => 2,
            CmdRogueMagicType::CmdRogueMagicEnterLayerCsReq => 3,
            CmdRogueMagicType::CmdRogueMagicStoryInfoUpdateScNotify => 4,
            CmdRogueMagicType::CmdRogueMagicEnterRoomCsReq => 5,
            CmdRogueMagicType::CmdRogueMagicSettleCsReq => 6,
            CmdRogueMagicType::CmdRogueMagicGetTalentInfoScRsp => 7,
            CmdRogueMagicType::CmdRogueMagicLeaveScRsp => 8,
            CmdRogueMagicType::CmdRogueMagicSetAutoDressInMagicUnitCsReq => 9,
            CmdRogueMagicType::CmdRogueMagicScepterDressInUnitCsReq => 10,
            CmdRogueMagicType::CmdRogueMagicAutoDressInUnitScRsp => 11,
            CmdRogueMagicType::CmdRogueMagicQueryCsReq => 12,
            CmdRogueMagicType::CmdRogueMagicUnitReforgeScRsp => 13,
            CmdRogueMagicType::CmdRogueMagicEnterLayerScRsp => 14,
            CmdRogueMagicType::CmdRogueMagicStartScRsp => 15,
            CmdRogueMagicType::CmdRogueMagicReviveCostUpdateScNotify => 16,
            CmdRogueMagicType::CmdRogueMagicBattleFailSettleInfoScNotify => 17,
            CmdRogueMagicType::CmdRogueMagicUnitReforgeCsReq => 18,
            CmdRogueMagicType::CmdRogueMagicAutoDressInMagicUnitChangeScNotify => 19,
            CmdRogueMagicType::CmdRogueMagicLevelInfoUpdateScNotify => 20,
            CmdRogueMagicType::CmdRogueMagicScepterTakeOffUnitCsReq => 21,
            CmdRogueMagicType::CmdRogueMagicGetMiscRealTimeDataCsReq => 22,
            CmdRogueMagicType::CmdRogueMagicAreaUpdateScNotify => 23,
            CmdRogueMagicType::CmdRogueMagicEnterRoomScRsp => 24,
            CmdRogueMagicType::CmdRogueMagicLeaveCsReq => 25,
            CmdRogueMagicType::CmdRogueMagicGetTalentInfoCsReq => 26,
            CmdRogueMagicType::CmdRogueMagicAutoDressInUnitCsReq => 27,
            CmdRogueMagicType::CmdRogueMagicSettleScRsp => 28,
            CmdRogueMagicType::CmdRogueMagicQueryScRsp => 29,
            CmdRogueMagicType::CmdRogueMagicReviveAvatarScRsp => 30,
            CmdRogueMagicType::CmdRogueMagicUnitComposeCsReq => 31,
            CmdRogueMagicType::CmdRogueMagicScepterTakeOffUnitScRsp => 32,
            CmdRogueMagicType::CmdRogueMagicStartCsReq => 33,
            CmdRogueMagicType::CmdRogueMagicEnableTalentScRsp => 34,
            CmdRogueMagicType::CmdRogueMagicGetMiscRealTimeDataScRsp => 35,
            CmdRogueMagicType::CmdRogueMagicEnterCsReq => 36,
            CmdRogueMagicType::CmdRogueMagicUnitComposeScRsp => 37,
            CmdRogueMagicType::CmdRogueMagicEnterScRsp => 38,
            CmdRogueMagicType::CmdRogueMagicReviveAvatarCsReq => 39,
            CmdRogueMagicType::CmdRogueMagicSetAutoDressInMagicUnitScRsp => 40,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdRogueMagicType {
    fn default() -> Self {
        CmdRogueMagicType::CmdRogueMagicNone
    }
}

impl CmdRogueMagicType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdRogueMagicType>("CmdRogueMagicType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17CmdRogueMagicType.proto*\x94\x0c\n\x11CmdRogueMagicType\x12\x15\n\
    \x11CmdRogueMagicNone\x10\0\x12)\n$CmdRogueMagicScepterDressInUnitScRsp\
    \x10\xb3<\x12#\n\x1eCmdRogueMagicEnableTalentCsReq\x10\x97<\x12!\n\x1cCm\
    dRogueMagicEnterLayerCsReq\x10\xe4<\x12)\n$CmdRogueMagicStoryInfoUpdateS\
    cNotify\x10\xa7<\x12\x20\n\x1bCmdRogueMagicEnterRoomCsReq\x10\xca<\x12\
    \x1d\n\x18CmdRogueMagicSettleCsReq\x10\xda<\x12$\n\x1fCmdRogueMagicGetTa\
    lentInfoScRsp\x10\xd9<\x12\x1c\n\x17CmdRogueMagicLeaveScRsp\x10\xa9<\x12\
    .\n)CmdRogueMagicSetAutoDressInMagicUnitCsReq\x10\xb0<\x12)\n$CmdRogueMa\
    gicScepterDressInUnitCsReq\x10\x9a<\x12&\n!CmdRogueMagicAutoDressInUnitS\
    cRsp\x10\xc3<\x12\x1c\n\x17CmdRogueMagicQueryCsReq\x10\xb8<\x12\"\n\x1dC\
    mdRogueMagicUnitReforgeScRsp\x10\xb6<\x12!\n\x1cCmdRogueMagicEnterLayerS\
    cRsp\x10\x96<\x12\x1c\n\x17CmdRogueMagicStartScRsp\x10\xef<\x12*\n%CmdRo\
    gueMagicReviveCostUpdateScNotify\x10\xc8<\x12.\n)CmdRogueMagicBattleFail\
    SettleInfoScNotify\x10\xba<\x12\"\n\x1dCmdRogueMagicUnitReforgeCsReq\x10\
    \xe2<\x124\n/CmdRogueMagicAutoDressInMagicUnitChangeScNotify\x10\xcb<\
    \x12)\n$CmdRogueMagicLevelInfoUpdateScNotify\x10\x99<\x12)\n$CmdRogueMag\
    icScepterTakeOffUnitCsReq\x10\xc0<\x12*\n%CmdRogueMagicGetMiscRealTimeDa\
    taCsReq\x10\xaa<\x12$\n\x1fCmdRogueMagicAreaUpdateScNotify\x10\xde<\x12\
    \x20\n\x1bCmdRogueMagicEnterRoomScRsp\x10\xe1<\x12\x1c\n\x17CmdRogueMagi\
    cLeaveCsReq\x10\xaf<\x12$\n\x1fCmdRogueMagicGetTalentInfoCsReq\x10\xc1<\
    \x12&\n!CmdRogueMagicAutoDressInUnitCsReq\x10\xa6<\x12\x1d\n\x18CmdRogue\
    MagicSettleScRsp\x10\xcf<\x12\x1c\n\x17CmdRogueMagicQueryScRsp\x10\xeb<\
    \x12#\n\x1eCmdRogueMagicReviveAvatarScRsp\x10\xf2<\x12\"\n\x1dCmdRogueMa\
    gicUnitComposeCsReq\x10\xbd<\x12)\n$CmdRogueMagicScepterTakeOffUnitScRsp\
    \x10\xc5<\x12\x1c\n\x17CmdRogueMagicStartCsReq\x10\xa8<\x12#\n\x1eCmdRog\
    ueMagicEnableTalentScRsp\x10\xac<\x12*\n%CmdRogueMagicGetMiscRealTimeDat\
    aScRsp\x10\xbc<\x12\x1c\n\x17CmdRogueMagicEnterCsReq\x10\xd7<\x12\"\n\
    \x1dCmdRogueMagicUnitComposeScRsp\x10\xe8<\x12\x1c\n\x17CmdRogueMagicEnt\
    erScRsp\x10\xbb<\x12#\n\x1eCmdRogueMagicReviveAvatarCsReq\x10\xd8<\x12.\
    \n)CmdRogueMagicSetAutoDressInMagicUnitScRsp\x10\xab<b\x06proto3\
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
            enums.push(CmdRogueMagicType::generated_enum_descriptor_data());
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
