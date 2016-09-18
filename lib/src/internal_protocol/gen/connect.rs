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
pub struct ClientServerConnect {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientServerConnect {}

impl ClientServerConnect {
    pub fn new() -> ClientServerConnect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientServerConnect {
        static mut instance: ::protobuf::lazy::Lazy<ClientServerConnect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientServerConnect,
        };
        unsafe {
            instance.get(|| {
                ClientServerConnect {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for ClientServerConnect {
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
        ::std::any::TypeId::of::<ClientServerConnect>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClientServerConnect {
    fn new() -> ClientServerConnect {
        ClientServerConnect::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientServerConnect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ClientServerConnect>(
                    "ClientServerConnect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientServerConnect {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ClientServerConnect {
    fn eq(&self, other: &ClientServerConnect) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClientServerConnect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ServerClientConnectBootstrap {
    // message fields
    initial_world_state: ::protobuf::SingularPtrField<super::common::ServerWorldStateDiff>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerClientConnectBootstrap {}

impl ServerClientConnectBootstrap {
    pub fn new() -> ServerClientConnectBootstrap {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerClientConnectBootstrap {
        static mut instance: ::protobuf::lazy::Lazy<ServerClientConnectBootstrap> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerClientConnectBootstrap,
        };
        unsafe {
            instance.get(|| {
                ServerClientConnectBootstrap {
                    initial_world_state: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ServerWorldStateDiff initial_world_state = 2;

    pub fn clear_initial_world_state(&mut self) {
        self.initial_world_state.clear();
    }

    pub fn has_initial_world_state(&self) -> bool {
        self.initial_world_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_initial_world_state(&mut self, v: super::common::ServerWorldStateDiff) {
        self.initial_world_state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_initial_world_state(&mut self) -> &mut super::common::ServerWorldStateDiff {
        if self.initial_world_state.is_none() {
            self.initial_world_state.set_default();
        };
        self.initial_world_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_initial_world_state(&mut self) -> super::common::ServerWorldStateDiff {
        self.initial_world_state.take().unwrap_or_else(|| super::common::ServerWorldStateDiff::new())
    }

    pub fn get_initial_world_state(&self) -> &super::common::ServerWorldStateDiff {
        self.initial_world_state.as_ref().unwrap_or_else(|| super::common::ServerWorldStateDiff::default_instance())
    }
}

impl ::protobuf::Message for ServerClientConnectBootstrap {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.initial_world_state));
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
        for value in self.initial_world_state.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.initial_world_state.as_ref() {
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
        ::std::any::TypeId::of::<ServerClientConnectBootstrap>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ServerClientConnectBootstrap {
    fn new() -> ServerClientConnectBootstrap {
        ServerClientConnectBootstrap::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerClientConnectBootstrap>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "initial_world_state",
                    ServerClientConnectBootstrap::has_initial_world_state,
                    ServerClientConnectBootstrap::get_initial_world_state,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServerClientConnectBootstrap>(
                    "ServerClientConnectBootstrap",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerClientConnectBootstrap {
    fn clear(&mut self) {
        self.clear_initial_world_state();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ServerClientConnectBootstrap {
    fn eq(&self, other: &ServerClientConnectBootstrap) -> bool {
        self.initial_world_state == other.initial_world_state &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ServerClientConnectBootstrap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x0c, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x15, 0x0a,
    0x13, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x43, 0x6f, 0x6e,
    0x6e, 0x65, 0x63, 0x74, 0x22, 0x65, 0x0a, 0x1c, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x43, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x42, 0x6f, 0x6f, 0x74, 0x73,
    0x74, 0x72, 0x61, 0x70, 0x12, 0x45, 0x0a, 0x13, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x5f,
    0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x15, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x44, 0x69, 0x66, 0x66, 0x52, 0x11, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61,
    0x6c, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x4a, 0xd8, 0x01, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x0a, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02, 0x07, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x04, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x04, 0x08, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x07, 0x00, 0x0a, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x07, 0x08, 0x24, 0x0a, 0x50, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x2f, 0x1a, 0x43, 0x20, 0x52, 0x65, 0x70, 0x72, 0x65,
    0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x20, 0x73, 0x71, 0x75, 0x61, 0x73, 0x68, 0x65, 0x64,
    0x20, 0x64, 0x69, 0x66, 0x66, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x74, 0x61, 0x72, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x6f, 0x72, 0x6c,
    0x64, 0x20, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x20, 0x6e, 0x6f, 0x77, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x09, 0x02, 0x07, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x09, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x17, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x09, 0x2d, 0x2e, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
