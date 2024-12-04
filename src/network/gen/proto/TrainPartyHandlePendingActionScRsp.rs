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

//! Generated file from `TrainPartyHandlePendingActionScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TrainPartyHandlePendingActionScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TrainPartyHandlePendingActionScRsp {
    // message fields
    // @@protoc_insertion_point(field:TrainPartyHandlePendingActionScRsp.LFJLHJHPLGI)
    pub LFJLHJHPLGI: bool,
    // @@protoc_insertion_point(field:TrainPartyHandlePendingActionScRsp.KILHOKNHHBD)
    pub KILHOKNHHBD: ::protobuf::MessageField<super::JNJKMBIGMPE::JNJKMBIGMPE>,
    // @@protoc_insertion_point(field:TrainPartyHandlePendingActionScRsp.LMFHNELKFOC)
    pub LMFHNELKFOC: u32,
    // @@protoc_insertion_point(field:TrainPartyHandlePendingActionScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // message oneof groups
    pub NOACJGOFODI: ::std::option::Option<train_party_handle_pending_action_sc_rsp::NOACJGOFODI>,
    // special fields
    // @@protoc_insertion_point(special_field:TrainPartyHandlePendingActionScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TrainPartyHandlePendingActionScRsp {
    fn default() -> &'a TrainPartyHandlePendingActionScRsp {
        <TrainPartyHandlePendingActionScRsp as ::protobuf::Message>::default_instance()
    }
}

impl TrainPartyHandlePendingActionScRsp {
    pub fn new() -> TrainPartyHandlePendingActionScRsp {
        ::std::default::Default::default()
    }

    // .FFFICNKONJJ IBLPFMCNABD = 93;

    pub fn IBLPFMCNABD(&self) -> &super::FFFICNKONJJ::FFFICNKONJJ {
        match self.NOACJGOFODI {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::IBLPFMCNABD(ref v)) => v,
            _ => <super::FFFICNKONJJ::FFFICNKONJJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_IBLPFMCNABD(&mut self) {
        self.NOACJGOFODI = ::std::option::Option::None;
    }

    pub fn has_IBLPFMCNABD(&self) -> bool {
        match self.NOACJGOFODI {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::IBLPFMCNABD(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_IBLPFMCNABD(&mut self, v: super::FFFICNKONJJ::FFFICNKONJJ) {
        self.NOACJGOFODI = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::IBLPFMCNABD(v))
    }

    // Mutable pointer to the field.
    pub fn mut_IBLPFMCNABD(&mut self) -> &mut super::FFFICNKONJJ::FFFICNKONJJ {
        if let ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::IBLPFMCNABD(_)) = self.NOACJGOFODI {
        } else {
            self.NOACJGOFODI = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::IBLPFMCNABD(super::FFFICNKONJJ::FFFICNKONJJ::new()));
        }
        match self.NOACJGOFODI {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::IBLPFMCNABD(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_IBLPFMCNABD(&mut self) -> super::FFFICNKONJJ::FFFICNKONJJ {
        if self.has_IBLPFMCNABD() {
            match self.NOACJGOFODI.take() {
                ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::IBLPFMCNABD(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FFFICNKONJJ::FFFICNKONJJ::new()
        }
    }

    // .JIADKOFHHJM KPPFCHGDODL = 1506;

    pub fn KPPFCHGDODL(&self) -> &super::JIADKOFHHJM::JIADKOFHHJM {
        match self.NOACJGOFODI {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KPPFCHGDODL(ref v)) => v,
            _ => <super::JIADKOFHHJM::JIADKOFHHJM as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_KPPFCHGDODL(&mut self) {
        self.NOACJGOFODI = ::std::option::Option::None;
    }

    pub fn has_KPPFCHGDODL(&self) -> bool {
        match self.NOACJGOFODI {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KPPFCHGDODL(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KPPFCHGDODL(&mut self, v: super::JIADKOFHHJM::JIADKOFHHJM) {
        self.NOACJGOFODI = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KPPFCHGDODL(v))
    }

    // Mutable pointer to the field.
    pub fn mut_KPPFCHGDODL(&mut self) -> &mut super::JIADKOFHHJM::JIADKOFHHJM {
        if let ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KPPFCHGDODL(_)) = self.NOACJGOFODI {
        } else {
            self.NOACJGOFODI = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KPPFCHGDODL(super::JIADKOFHHJM::JIADKOFHHJM::new()));
        }
        match self.NOACJGOFODI {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KPPFCHGDODL(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_KPPFCHGDODL(&mut self) -> super::JIADKOFHHJM::JIADKOFHHJM {
        if self.has_KPPFCHGDODL() {
            match self.NOACJGOFODI.take() {
                ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KPPFCHGDODL(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JIADKOFHHJM::JIADKOFHHJM::new()
        }
    }

    // .JGNHFJAGHKA LFCEFFHMLIG = 69;

    pub fn LFCEFFHMLIG(&self) -> &super::JGNHFJAGHKA::JGNHFJAGHKA {
        match self.NOACJGOFODI {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::LFCEFFHMLIG(ref v)) => v,
            _ => <super::JGNHFJAGHKA::JGNHFJAGHKA as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_LFCEFFHMLIG(&mut self) {
        self.NOACJGOFODI = ::std::option::Option::None;
    }

    pub fn has_LFCEFFHMLIG(&self) -> bool {
        match self.NOACJGOFODI {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::LFCEFFHMLIG(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_LFCEFFHMLIG(&mut self, v: super::JGNHFJAGHKA::JGNHFJAGHKA) {
        self.NOACJGOFODI = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::LFCEFFHMLIG(v))
    }

    // Mutable pointer to the field.
    pub fn mut_LFCEFFHMLIG(&mut self) -> &mut super::JGNHFJAGHKA::JGNHFJAGHKA {
        if let ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::LFCEFFHMLIG(_)) = self.NOACJGOFODI {
        } else {
            self.NOACJGOFODI = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::LFCEFFHMLIG(super::JGNHFJAGHKA::JGNHFJAGHKA::new()));
        }
        match self.NOACJGOFODI {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::LFCEFFHMLIG(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_LFCEFFHMLIG(&mut self) -> super::JGNHFJAGHKA::JGNHFJAGHKA {
        if self.has_LFCEFFHMLIG() {
            match self.NOACJGOFODI.take() {
                ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::LFCEFFHMLIG(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JGNHFJAGHKA::JGNHFJAGHKA::new()
        }
    }

    // .CKCNEGHAMGM KIBAIMIKFED = 366;

    pub fn KIBAIMIKFED(&self) -> &super::CKCNEGHAMGM::CKCNEGHAMGM {
        match self.NOACJGOFODI {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KIBAIMIKFED(ref v)) => v,
            _ => <super::CKCNEGHAMGM::CKCNEGHAMGM as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_KIBAIMIKFED(&mut self) {
        self.NOACJGOFODI = ::std::option::Option::None;
    }

    pub fn has_KIBAIMIKFED(&self) -> bool {
        match self.NOACJGOFODI {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KIBAIMIKFED(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KIBAIMIKFED(&mut self, v: super::CKCNEGHAMGM::CKCNEGHAMGM) {
        self.NOACJGOFODI = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KIBAIMIKFED(v))
    }

    // Mutable pointer to the field.
    pub fn mut_KIBAIMIKFED(&mut self) -> &mut super::CKCNEGHAMGM::CKCNEGHAMGM {
        if let ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KIBAIMIKFED(_)) = self.NOACJGOFODI {
        } else {
            self.NOACJGOFODI = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KIBAIMIKFED(super::CKCNEGHAMGM::CKCNEGHAMGM::new()));
        }
        match self.NOACJGOFODI {
            ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KIBAIMIKFED(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_KIBAIMIKFED(&mut self) -> super::CKCNEGHAMGM::CKCNEGHAMGM {
        if self.has_KIBAIMIKFED() {
            match self.NOACJGOFODI.take() {
                ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KIBAIMIKFED(v)) => v,
                _ => panic!(),
            }
        } else {
            super::CKCNEGHAMGM::CKCNEGHAMGM::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LFJLHJHPLGI",
            |m: &TrainPartyHandlePendingActionScRsp| { &m.LFJLHJHPLGI },
            |m: &mut TrainPartyHandlePendingActionScRsp| { &mut m.LFJLHJHPLGI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::JNJKMBIGMPE::JNJKMBIGMPE>(
            "KILHOKNHHBD",
            |m: &TrainPartyHandlePendingActionScRsp| { &m.KILHOKNHHBD },
            |m: &mut TrainPartyHandlePendingActionScRsp| { &mut m.KILHOKNHHBD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LMFHNELKFOC",
            |m: &TrainPartyHandlePendingActionScRsp| { &m.LMFHNELKFOC },
            |m: &mut TrainPartyHandlePendingActionScRsp| { &mut m.LMFHNELKFOC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &TrainPartyHandlePendingActionScRsp| { &m.ADADHIHDHJC },
            |m: &mut TrainPartyHandlePendingActionScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FFFICNKONJJ::FFFICNKONJJ>(
            "IBLPFMCNABD",
            TrainPartyHandlePendingActionScRsp::has_IBLPFMCNABD,
            TrainPartyHandlePendingActionScRsp::IBLPFMCNABD,
            TrainPartyHandlePendingActionScRsp::mut_IBLPFMCNABD,
            TrainPartyHandlePendingActionScRsp::set_IBLPFMCNABD,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JIADKOFHHJM::JIADKOFHHJM>(
            "KPPFCHGDODL",
            TrainPartyHandlePendingActionScRsp::has_KPPFCHGDODL,
            TrainPartyHandlePendingActionScRsp::KPPFCHGDODL,
            TrainPartyHandlePendingActionScRsp::mut_KPPFCHGDODL,
            TrainPartyHandlePendingActionScRsp::set_KPPFCHGDODL,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JGNHFJAGHKA::JGNHFJAGHKA>(
            "LFCEFFHMLIG",
            TrainPartyHandlePendingActionScRsp::has_LFCEFFHMLIG,
            TrainPartyHandlePendingActionScRsp::LFCEFFHMLIG,
            TrainPartyHandlePendingActionScRsp::mut_LFCEFFHMLIG,
            TrainPartyHandlePendingActionScRsp::set_LFCEFFHMLIG,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::CKCNEGHAMGM::CKCNEGHAMGM>(
            "KIBAIMIKFED",
            TrainPartyHandlePendingActionScRsp::has_KIBAIMIKFED,
            TrainPartyHandlePendingActionScRsp::KIBAIMIKFED,
            TrainPartyHandlePendingActionScRsp::mut_KIBAIMIKFED,
            TrainPartyHandlePendingActionScRsp::set_KIBAIMIKFED,
        ));
        oneofs.push(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TrainPartyHandlePendingActionScRsp>(
            "TrainPartyHandlePendingActionScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TrainPartyHandlePendingActionScRsp {
    const NAME: &'static str = "TrainPartyHandlePendingActionScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.LFJLHJHPLGI = is.read_bool()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KILHOKNHHBD)?;
                },
                32 => {
                    self.LMFHNELKFOC = is.read_uint32()?;
                },
                48 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                746 => {
                    self.NOACJGOFODI = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::IBLPFMCNABD(is.read_message()?));
                },
                12050 => {
                    self.NOACJGOFODI = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KPPFCHGDODL(is.read_message()?));
                },
                554 => {
                    self.NOACJGOFODI = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::LFCEFFHMLIG(is.read_message()?));
                },
                2930 => {
                    self.NOACJGOFODI = ::std::option::Option::Some(train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KIBAIMIKFED(is.read_message()?));
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
        if self.LFJLHJHPLGI != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.KILHOKNHHBD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.LMFHNELKFOC != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.LMFHNELKFOC);
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.ADADHIHDHJC);
        }
        if let ::std::option::Option::Some(ref v) = self.NOACJGOFODI {
            match v {
                &train_party_handle_pending_action_sc_rsp::NOACJGOFODI::IBLPFMCNABD(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KPPFCHGDODL(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &train_party_handle_pending_action_sc_rsp::NOACJGOFODI::LFCEFFHMLIG(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KIBAIMIKFED(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LFJLHJHPLGI != false {
            os.write_bool(3, self.LFJLHJHPLGI)?;
        }
        if let Some(v) = self.KILHOKNHHBD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.LMFHNELKFOC != 0 {
            os.write_uint32(4, self.LMFHNELKFOC)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(6, self.ADADHIHDHJC)?;
        }
        if let ::std::option::Option::Some(ref v) = self.NOACJGOFODI {
            match v {
                &train_party_handle_pending_action_sc_rsp::NOACJGOFODI::IBLPFMCNABD(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(93, v, os)?;
                },
                &train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KPPFCHGDODL(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1506, v, os)?;
                },
                &train_party_handle_pending_action_sc_rsp::NOACJGOFODI::LFCEFFHMLIG(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(69, v, os)?;
                },
                &train_party_handle_pending_action_sc_rsp::NOACJGOFODI::KIBAIMIKFED(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(366, v, os)?;
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

    fn new() -> TrainPartyHandlePendingActionScRsp {
        TrainPartyHandlePendingActionScRsp::new()
    }

    fn clear(&mut self) {
        self.LFJLHJHPLGI = false;
        self.KILHOKNHHBD.clear();
        self.LMFHNELKFOC = 0;
        self.ADADHIHDHJC = 0;
        self.NOACJGOFODI = ::std::option::Option::None;
        self.NOACJGOFODI = ::std::option::Option::None;
        self.NOACJGOFODI = ::std::option::Option::None;
        self.NOACJGOFODI = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TrainPartyHandlePendingActionScRsp {
        static instance: TrainPartyHandlePendingActionScRsp = TrainPartyHandlePendingActionScRsp {
            LFJLHJHPLGI: false,
            KILHOKNHHBD: ::protobuf::MessageField::none(),
            LMFHNELKFOC: 0,
            ADADHIHDHJC: 0,
            NOACJGOFODI: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TrainPartyHandlePendingActionScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TrainPartyHandlePendingActionScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TrainPartyHandlePendingActionScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrainPartyHandlePendingActionScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `TrainPartyHandlePendingActionScRsp`
pub mod train_party_handle_pending_action_sc_rsp {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:TrainPartyHandlePendingActionScRsp.NOACJGOFODI)
    pub enum NOACJGOFODI {
        // @@protoc_insertion_point(oneof_field:TrainPartyHandlePendingActionScRsp.IBLPFMCNABD)
        IBLPFMCNABD(super::super::FFFICNKONJJ::FFFICNKONJJ),
        // @@protoc_insertion_point(oneof_field:TrainPartyHandlePendingActionScRsp.KPPFCHGDODL)
        KPPFCHGDODL(super::super::JIADKOFHHJM::JIADKOFHHJM),
        // @@protoc_insertion_point(oneof_field:TrainPartyHandlePendingActionScRsp.LFCEFFHMLIG)
        LFCEFFHMLIG(super::super::JGNHFJAGHKA::JGNHFJAGHKA),
        // @@protoc_insertion_point(oneof_field:TrainPartyHandlePendingActionScRsp.KIBAIMIKFED)
        KIBAIMIKFED(super::super::CKCNEGHAMGM::CKCNEGHAMGM),
    }

    impl ::protobuf::Oneof for NOACJGOFODI {
    }

    impl ::protobuf::OneofFull for NOACJGOFODI {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::TrainPartyHandlePendingActionScRsp as ::protobuf::MessageFull>::descriptor().oneof_by_name("NOACJGOFODI").unwrap()).clone()
        }
    }

    impl NOACJGOFODI {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<NOACJGOFODI>("NOACJGOFODI")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(TrainPartyHandlePendingActionScRsp.proto\x1a\x11CKCNEGHAMGM.proto\x1a\
    \x11FFFICNKONJJ.proto\x1a\x11JGNHFJAGHKA.proto\x1a\x11JIADKOFHHJM.proto\
    \x1a\x11JNJKMBIGMPE.proto\"\x93\x03\n\"TrainPartyHandlePendingActionScRs\
    p\x12\x20\n\x0bLFJLHJHPLGI\x18\x03\x20\x01(\x08R\x0bLFJLHJHPLGI\x12.\n\
    \x0bKILHOKNHHBD\x18\r\x20\x01(\x0b2\x0c.JNJKMBIGMPER\x0bKILHOKNHHBD\x12\
    \x20\n\x0bLMFHNELKFOC\x18\x04\x20\x01(\rR\x0bLMFHNELKFOC\x12\x20\n\x0bAD\
    ADHIHDHJC\x18\x06\x20\x01(\rR\x0bADADHIHDHJC\x120\n\x0bIBLPFMCNABD\x18]\
    \x20\x01(\x0b2\x0c.FFFICNKONJJH\0R\x0bIBLPFMCNABD\x121\n\x0bKPPFCHGDODL\
    \x18\xe2\x0b\x20\x01(\x0b2\x0c.JIADKOFHHJMH\0R\x0bKPPFCHGDODL\x120\n\x0b\
    LFCEFFHMLIG\x18E\x20\x01(\x0b2\x0c.JGNHFJAGHKAH\0R\x0bLFCEFFHMLIG\x121\n\
    \x0bKIBAIMIKFED\x18\xee\x02\x20\x01(\x0b2\x0c.CKCNEGHAMGMH\0R\x0bKIBAIMI\
    KFEDB\r\n\x0bNOACJGOFODIb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::CKCNEGHAMGM::file_descriptor().clone());
            deps.push(super::FFFICNKONJJ::file_descriptor().clone());
            deps.push(super::JGNHFJAGHKA::file_descriptor().clone());
            deps.push(super::JIADKOFHHJM::file_descriptor().clone());
            deps.push(super::JNJKMBIGMPE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TrainPartyHandlePendingActionScRsp::generated_message_descriptor_data());
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
