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
pub struct SendNewEventsRequest {
    // message fields
    client_world_version: ::std::option::Option<u64>,
    event: ::protobuf::RepeatedField<super::common::Event>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SendNewEventsRequest {}

impl SendNewEventsRequest {
    pub fn new() -> SendNewEventsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SendNewEventsRequest {
        static mut instance: ::protobuf::lazy::Lazy<SendNewEventsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SendNewEventsRequest,
        };
        unsafe {
            instance.get(|| {
                SendNewEventsRequest {
                    client_world_version: ::std::option::Option::None,
                    event: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 client_world_version = 1;

    pub fn clear_client_world_version(&mut self) {
        self.client_world_version = ::std::option::Option::None;
    }

    pub fn has_client_world_version(&self) -> bool {
        self.client_world_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_world_version(&mut self, v: u64) {
        self.client_world_version = ::std::option::Option::Some(v);
    }

    pub fn get_client_world_version(&self) -> u64 {
        self.client_world_version.unwrap_or(0)
    }

    // repeated .Event event = 2;

    pub fn clear_event(&mut self) {
        self.event.clear();
    }

    // Param is passed by value, moved
    pub fn set_event(&mut self, v: ::protobuf::RepeatedField<super::common::Event>) {
        self.event = v;
    }

    // Mutable pointer to the field.
    pub fn mut_event(&mut self) -> &mut ::protobuf::RepeatedField<super::common::Event> {
        &mut self.event
    }

    // Take field
    pub fn take_event(&mut self) -> ::protobuf::RepeatedField<super::common::Event> {
        ::std::mem::replace(&mut self.event, ::protobuf::RepeatedField::new())
    }

    pub fn get_event(&self) -> &[super::common::Event] {
        &self.event
    }
}

impl ::protobuf::Message for SendNewEventsRequest {
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
                    self.client_world_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.event));
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
        for value in self.client_world_version.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.event.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_world_version {
            try!(os.write_uint64(1, v));
        };
        for v in self.event.iter() {
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
        ::std::any::TypeId::of::<SendNewEventsRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SendNewEventsRequest {
    fn new() -> SendNewEventsRequest {
        SendNewEventsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SendNewEventsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "client_world_version",
                    SendNewEventsRequest::has_client_world_version,
                    SendNewEventsRequest::get_client_world_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "event",
                    SendNewEventsRequest::get_event,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SendNewEventsRequest>(
                    "SendNewEventsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SendNewEventsRequest {
    fn clear(&mut self) {
        self.clear_client_world_version();
        self.clear_event();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SendNewEventsRequest {
    fn eq(&self, other: &SendNewEventsRequest) -> bool {
        self.client_world_version == other.client_world_version &&
        self.event == other.event &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SendNewEventsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SendNewEventsResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SendNewEventsResponse {}

impl SendNewEventsResponse {
    pub fn new() -> SendNewEventsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SendNewEventsResponse {
        static mut instance: ::protobuf::lazy::Lazy<SendNewEventsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SendNewEventsResponse,
        };
        unsafe {
            instance.get(|| {
                SendNewEventsResponse {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for SendNewEventsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
        ::std::any::TypeId::of::<SendNewEventsResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SendNewEventsResponse {
    fn new() -> SendNewEventsResponse {
        SendNewEventsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SendNewEventsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<SendNewEventsResponse>(
                    "SendNewEventsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SendNewEventsResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SendNewEventsResponse {
    fn eq(&self, other: &SendNewEventsResponse) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SendNewEventsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x15, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x6e, 0x65, 0x77, 0x5f, 0x65, 0x76, 0x65, 0x6e, 0x74,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x66, 0x0a, 0x14, 0x53, 0x65, 0x6e, 0x64, 0x4e, 0x65, 0x77,
    0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x30, 0x0a,
    0x14, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x12, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12,
    0x1c, 0x0a, 0x05, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x06,
    0x2e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x05, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x22, 0x17, 0x0a,
    0x15, 0x53, 0x65, 0x6e, 0x64, 0x4e, 0x65, 0x77, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x4a, 0xfe, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x10,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x02, 0x07, 0x15, 0x0a, 0x4c, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x05, 0x00,
    0x0d, 0x01, 0x1a, 0x40, 0x20, 0x4e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x6e, 0x65, 0x77, 0x20,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2d, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x74, 0x65,
    0x64, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x28, 0x65, 0x76, 0x65, 0x6e, 0x74,
    0x73, 0x29, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x05, 0x08, 0x1c,
    0x0a, 0x86, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x02, 0x22, 0x1a, 0x79,
    0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6c,
    0x61, 0x74, 0x65, 0x73, 0x74, 0x20, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x61, 0x74,
    0x69, 0x76, 0x65, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x76, 0x65, 0x72, 0x73, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x68, 0x61,
    0x73, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x43, 0x61, 0x6e,
    0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x75, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x62, 0x65, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x20, 0x63, 0x6f,
    0x6e, 0x66, 0x6c, 0x69, 0x63, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x08, 0x02, 0x05, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x08, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x08, 0x09, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08,
    0x20, 0x21, 0x0a, 0x73, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x1b, 0x1a,
    0x66, 0x20, 0x41, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74,
    0x73, 0x20, 0x74, 0x6f, 0x20, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x2e, 0x0a, 0x20, 0x4e,
    0x6f, 0x74, 0x65, 0x3a, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x61, 0x6e, 0x20, 0x69, 0x64, 0x20, 0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79, 0x20, 0x69,
    0x6e, 0x20, 0x75, 0x73, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x69, 0x67,
    0x6e, 0x6f, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x0c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x11,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x19, 0x1a, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0f, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x1d, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
