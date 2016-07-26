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
pub struct ServerLifelinePing {
    // message fields
    tcp_port: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerLifelinePing {}

impl ServerLifelinePing {
    pub fn new() -> ServerLifelinePing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerLifelinePing {
        static mut instance: ::protobuf::lazy::Lazy<ServerLifelinePing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerLifelinePing,
        };
        unsafe {
            instance.get(|| {
                ServerLifelinePing {
                    tcp_port: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 tcp_port = 1;

    pub fn clear_tcp_port(&mut self) {
        self.tcp_port = ::std::option::Option::None;
    }

    pub fn has_tcp_port(&self) -> bool {
        self.tcp_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tcp_port(&mut self, v: u32) {
        self.tcp_port = ::std::option::Option::Some(v);
    }

    pub fn get_tcp_port(&self) -> u32 {
        self.tcp_port.unwrap_or(0)
    }
}

impl ::protobuf::Message for ServerLifelinePing {
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
                    let tmp = try!(is.read_uint32());
                    self.tcp_port = ::std::option::Option::Some(tmp);
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
        for value in self.tcp_port.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tcp_port {
            try!(os.write_uint32(1, v));
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
        ::std::any::TypeId::of::<ServerLifelinePing>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ServerLifelinePing {
    fn new() -> ServerLifelinePing {
        ServerLifelinePing::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerLifelinePing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "tcp_port",
                    ServerLifelinePing::has_tcp_port,
                    ServerLifelinePing::get_tcp_port,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServerLifelinePing>(
                    "ServerLifelinePing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerLifelinePing {
    fn clear(&mut self) {
        self.clear_tcp_port();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ServerLifelinePing {
    fn eq(&self, other: &ServerLifelinePing) -> bool {
        self.tcp_port == other.tcp_port &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ServerLifelinePing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1a, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x6c, 0x69, 0x66, 0x65, 0x6c, 0x69, 0x6e,
    0x65, 0x5f, 0x70, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x2f, 0x0a, 0x12,
    0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x4c, 0x69, 0x66, 0x65, 0x6c, 0x69, 0x6e, 0x65, 0x50, 0x69,
    0x6e, 0x67, 0x12, 0x19, 0x0a, 0x08, 0x74, 0x63, 0x70, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x74, 0x63, 0x70, 0x50, 0x6f, 0x72, 0x74, 0x4a, 0xab, 0x02,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x08, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0xb1, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00, 0x08, 0x01, 0x1a,
    0x49, 0x20, 0x41, 0x20, 0x75, 0x64, 0x70, 0x20, 0x62, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73,
    0x74, 0x20, 0x74, 0x6f, 0x20, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6c, 0x69, 0x76, 0x65, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x22, 0x5a, 0x20, 0x4e, 0x6f, 0x74,
    0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64,
    0x20, 0x62, 0x65, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x62,
    0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x63, 0x6f, 0x6d, 0x69, 0x6e, 0x67, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03,
    0x08, 0x1a, 0x0a, 0x1e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x07, 0x02, 0x16, 0x1a,
    0x11, 0x20, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x74, 0x63, 0x70, 0x20, 0x70, 0x6f, 0x72,
    0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x07, 0x02, 0x03,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x07, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x09, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x14, 0x15, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
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
