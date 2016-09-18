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
pub struct ClientServerAckUpdates {
    // message fields
    world_version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientServerAckUpdates {}

impl ClientServerAckUpdates {
    pub fn new() -> ClientServerAckUpdates {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientServerAckUpdates {
        static mut instance: ::protobuf::lazy::Lazy<ClientServerAckUpdates> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientServerAckUpdates,
        };
        unsafe {
            instance.get(|| {
                ClientServerAckUpdates {
                    world_version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 world_version = 1;

    pub fn clear_world_version(&mut self) {
        self.world_version = ::std::option::Option::None;
    }

    pub fn has_world_version(&self) -> bool {
        self.world_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_world_version(&mut self, v: u64) {
        self.world_version = ::std::option::Option::Some(v);
    }

    pub fn get_world_version(&self) -> u64 {
        self.world_version.unwrap_or(0)
    }
}

impl ::protobuf::Message for ClientServerAckUpdates {
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
                    self.world_version = ::std::option::Option::Some(tmp);
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
        for value in self.world_version.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.world_version {
            try!(os.write_uint64(1, v));
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
        ::std::any::TypeId::of::<ClientServerAckUpdates>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClientServerAckUpdates {
    fn new() -> ClientServerAckUpdates {
        ClientServerAckUpdates::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientServerAckUpdates>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "world_version",
                    ClientServerAckUpdates::has_world_version,
                    ClientServerAckUpdates::get_world_version,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientServerAckUpdates>(
                    "ClientServerAckUpdates",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientServerAckUpdates {
    fn clear(&mut self) {
        self.clear_world_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ClientServerAckUpdates {
    fn eq(&self, other: &ClientServerAckUpdates) -> bool {
        self.world_version == other.world_version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClientServerAckUpdates {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x61, 0x63, 0x6b, 0x5f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0x3d, 0x0a, 0x16, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x41, 0x63, 0x6b, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x12, 0x23, 0x0a,
    0x0d, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x56, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x4a, 0xff, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x06, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x98, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x04, 0x00, 0x06, 0x01, 0x1a, 0x8b, 0x01, 0x20, 0x49, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20,
    0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f,
    0x20, 0x61, 0x63, 0x6b, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x61, 0x74,
    0x69, 0x76, 0x65, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x20, 0x53, 0x68, 0x6f, 0x75,
    0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x65, 0x64, 0x20, 0x69, 0x6d, 0x6d,
    0x65, 0x64, 0x69, 0x61, 0x74, 0x65, 0x6c, 0x79, 0x0a, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20,
    0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x53, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x44, 0x69, 0x66, 0x66,
    0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x05, 0x02, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x05, 0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x05, 0x19, 0x1a, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
