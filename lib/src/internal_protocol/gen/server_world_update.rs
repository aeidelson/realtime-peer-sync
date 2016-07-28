// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct WorldUpdatePush {
    // message fields
    expected_client_version: ::std::option::Option<u64>,
    modified_world_state: ::protobuf::SingularPtrField<super::common::ServerWorldStateDiff>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WorldUpdatePush {}

impl WorldUpdatePush {
    pub fn new() -> WorldUpdatePush {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WorldUpdatePush {
        static mut instance: ::protobuf::lazy::Lazy<WorldUpdatePush> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WorldUpdatePush,
        };
        unsafe {
            instance.get(|| {
                WorldUpdatePush {
                    expected_client_version: ::std::option::Option::None,
                    modified_world_state: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 expected_client_version = 1;

    pub fn clear_expected_client_version(&mut self) {
        self.expected_client_version = ::std::option::Option::None;
    }

    pub fn has_expected_client_version(&self) -> bool {
        self.expected_client_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expected_client_version(&mut self, v: u64) {
        self.expected_client_version = ::std::option::Option::Some(v);
    }

    pub fn get_expected_client_version(&self) -> u64 {
        self.expected_client_version.unwrap_or(0)
    }

    // optional .ServerWorldStateDiff modified_world_state = 2;

    pub fn clear_modified_world_state(&mut self) {
        self.modified_world_state.clear();
    }

    pub fn has_modified_world_state(&self) -> bool {
        self.modified_world_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modified_world_state(&mut self, v: super::common::ServerWorldStateDiff) {
        self.modified_world_state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_modified_world_state(&mut self) -> &mut super::common::ServerWorldStateDiff {
        if self.modified_world_state.is_none() {
            self.modified_world_state.set_default();
        };
        self.modified_world_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_modified_world_state(&mut self) -> super::common::ServerWorldStateDiff {
        self.modified_world_state.take().unwrap_or_else(|| super::common::ServerWorldStateDiff::new())
    }

    pub fn get_modified_world_state(&self) -> &super::common::ServerWorldStateDiff {
        self.modified_world_state.as_ref().unwrap_or_else(|| super::common::ServerWorldStateDiff::default_instance())
    }
}

impl ::protobuf::Message for WorldUpdatePush {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.expected_client_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.modified_world_state));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.expected_client_version.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.modified_world_state.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.expected_client_version {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.modified_world_state.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<WorldUpdatePush>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WorldUpdatePush {
    fn new() -> WorldUpdatePush {
        WorldUpdatePush::new()
    }

    fn descriptor_static(_: ::std::option::Option<WorldUpdatePush>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "expected_client_version",
                    WorldUpdatePush::has_expected_client_version,
                    WorldUpdatePush::get_expected_client_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "modified_world_state",
                    WorldUpdatePush::has_modified_world_state,
                    WorldUpdatePush::get_modified_world_state,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WorldUpdatePush>(
                    "WorldUpdatePush",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WorldUpdatePush {
    fn clear(&mut self) {
        self.clear_expected_client_version();
        self.clear_modified_world_state();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WorldUpdatePush {
    fn eq(&self, other: &WorldUpdatePush) -> bool {
        self.expected_client_version == other.expected_client_version &&
        self.modified_world_state == other.modified_world_state &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WorldUpdatePush {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x19, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x75,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x63, 0x6f, 0x6d,
    0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x92, 0x01, 0x0a, 0x0f, 0x57, 0x6f,
    0x72, 0x6c, 0x64, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x50, 0x75, 0x73, 0x68, 0x12, 0x36, 0x0a,
    0x17, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x15,
    0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x47, 0x0a, 0x14, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65,
    0x64, 0x5f, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x57, 0x6f, 0x72, 0x6c,
    0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x44, 0x69, 0x66, 0x66, 0x52, 0x12, 0x6d, 0x6f, 0x64, 0x69,
    0x66, 0x69, 0x65, 0x64, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x4a, 0xd8,
    0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02, 0x07, 0x15, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x04, 0x08, 0x17, 0x0a, 0xd3, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x08, 0x02, 0x25, 0x1a, 0xc5, 0x01, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64,
    0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x62, 0x65, 0x6c, 0x69, 0x65,
    0x76, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x68,
    0x61, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x73,
    0x68, 0x6f, 0x75, 0x6c, 0x64, 0x0a, 0x20, 0x72, 0x65, 0x2d, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63,
    0x74, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x67, 0x72, 0x65,
    0x61, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x63,
    0x74, 0x75, 0x61, 0x6c, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x20, 0x28, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x74,
    0x68, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x65, 0x6e, 0x74, 0x20, 0x68, 0x6f, 0x72, 0x72, 0x69, 0x62,
    0x6c, 0x79, 0x0a, 0x20, 0x77, 0x72, 0x6f, 0x6e, 0x67, 0x29, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x08, 0x02, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x08, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x08, 0x09, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x08, 0x23, 0x24, 0x0a, 0x59, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0b,
    0x02, 0x30, 0x1a, 0x4c, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x43,
    0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x77, 0x68, 0x61, 0x74, 0x20, 0x68, 0x61, 0x73,
    0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x20, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x27, 0x73, 0x20, 0x6c, 0x61, 0x73, 0x74,
    0x20, 0x61, 0x63, 0x6b, 0x27, 0x64, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0b, 0x02, 0x08, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0b, 0x02, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x17, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x2e, 0x2f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
