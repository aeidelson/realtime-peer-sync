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
pub struct PublicClientInfo {
    // message fields
    client_id: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PublicClientInfo {}

impl PublicClientInfo {
    pub fn new() -> PublicClientInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublicClientInfo {
        static mut instance: ::protobuf::lazy::Lazy<PublicClientInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublicClientInfo,
        };
        unsafe {
            instance.get(|| {
                PublicClientInfo {
                    client_id: ::protobuf::SingularField::none(),
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string client_id = 1;

    pub fn clear_client_id(&mut self) {
        self.client_id.clear();
    }

    pub fn has_client_id(&self) -> bool {
        self.client_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_id(&mut self, v: ::std::string::String) {
        self.client_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_id(&mut self) -> &mut ::std::string::String {
        if self.client_id.is_none() {
            self.client_id.set_default();
        };
        self.client_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_id(&mut self) -> ::std::string::String {
        self.client_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_client_id(&self) -> &str {
        match self.client_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for PublicClientInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.client_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
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
        for value in self.client_id.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<PublicClientInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PublicClientInfo {
    fn new() -> PublicClientInfo {
        PublicClientInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<PublicClientInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "client_id",
                    PublicClientInfo::has_client_id,
                    PublicClientInfo::get_client_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    PublicClientInfo::has_name,
                    PublicClientInfo::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublicClientInfo>(
                    "PublicClientInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublicClientInfo {
    fn clear(&mut self) {
        self.clear_client_id();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PublicClientInfo {
    fn eq(&self, other: &PublicClientInfo) -> bool {
        self.client_id == other.client_id &&
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PublicClientInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PublicServerInfo {
    // message fields
    tcp_port: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PublicServerInfo {}

impl PublicServerInfo {
    pub fn new() -> PublicServerInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublicServerInfo {
        static mut instance: ::protobuf::lazy::Lazy<PublicServerInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublicServerInfo,
        };
        unsafe {
            instance.get(|| {
                PublicServerInfo {
                    tcp_port: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
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

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for PublicServerInfo {
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
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
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
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tcp_port {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<PublicServerInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PublicServerInfo {
    fn new() -> PublicServerInfo {
        PublicServerInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<PublicServerInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "tcp_port",
                    PublicServerInfo::has_tcp_port,
                    PublicServerInfo::get_tcp_port,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    PublicServerInfo::has_name,
                    PublicServerInfo::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublicServerInfo>(
                    "PublicServerInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublicServerInfo {
    fn clear(&mut self) {
        self.clear_tcp_port();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PublicServerInfo {
    fn eq(&self, other: &PublicServerInfo) -> bool {
        self.tcp_port == other.tcp_port &&
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PublicServerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ClientInfo {
    // message fields
    public_info: ::protobuf::SingularPtrField<PublicClientInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientInfo {}

impl ClientInfo {
    pub fn new() -> ClientInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientInfo {
        static mut instance: ::protobuf::lazy::Lazy<ClientInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientInfo,
        };
        unsafe {
            instance.get(|| {
                ClientInfo {
                    public_info: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .PublicClientInfo public_info = 1;

    pub fn clear_public_info(&mut self) {
        self.public_info.clear();
    }

    pub fn has_public_info(&self) -> bool {
        self.public_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_info(&mut self, v: PublicClientInfo) {
        self.public_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_info(&mut self) -> &mut PublicClientInfo {
        if self.public_info.is_none() {
            self.public_info.set_default();
        };
        self.public_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_public_info(&mut self) -> PublicClientInfo {
        self.public_info.take().unwrap_or_else(|| PublicClientInfo::new())
    }

    pub fn get_public_info(&self) -> &PublicClientInfo {
        self.public_info.as_ref().unwrap_or_else(|| PublicClientInfo::default_instance())
    }
}

impl ::protobuf::Message for ClientInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.public_info));
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
        for value in self.public_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.public_info.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<ClientInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClientInfo {
    fn new() -> ClientInfo {
        ClientInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "public_info",
                    ClientInfo::has_public_info,
                    ClientInfo::get_public_info,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientInfo>(
                    "ClientInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientInfo {
    fn clear(&mut self) {
        self.clear_public_info();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ClientInfo {
    fn eq(&self, other: &ClientInfo) -> bool {
        self.public_info == other.public_info &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClientInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ServerInfo {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerInfo {}

impl ServerInfo {
    pub fn new() -> ServerInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerInfo {
        static mut instance: ::protobuf::lazy::Lazy<ServerInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerInfo,
        };
        unsafe {
            instance.get(|| {
                ServerInfo {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for ServerInfo {
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
        ::std::any::TypeId::of::<ServerInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ServerInfo {
    fn new() -> ServerInfo {
        ServerInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ServerInfo>(
                    "ServerInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerInfo {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ServerInfo {
    fn eq(&self, other: &ServerInfo) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ServerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ObjectDiff {
    // message oneof groups
    modification: ::std::option::Option<ObjectDiff_oneof_modification>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObjectDiff {}

#[derive(Clone,PartialEq)]
pub enum ObjectDiff_oneof_modification {
    upserted_object(ObjectDiff_UpsertObjectModification),
    deleted_object(ObjectDiff_DeletedObjectModfication),
}

impl ObjectDiff {
    pub fn new() -> ObjectDiff {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObjectDiff {
        static mut instance: ::protobuf::lazy::Lazy<ObjectDiff> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjectDiff,
        };
        unsafe {
            instance.get(|| {
                ObjectDiff {
                    modification: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ObjectDiff.UpsertObjectModification upserted_object = 1;

    pub fn clear_upserted_object(&mut self) {
        self.modification = ::std::option::Option::None;
    }

    pub fn has_upserted_object(&self) -> bool {
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::upserted_object(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_upserted_object(&mut self, v: ObjectDiff_UpsertObjectModification) {
        self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::upserted_object(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_upserted_object(&mut self) -> &mut ObjectDiff_UpsertObjectModification {
        if let ::std::option::Option::Some(ObjectDiff_oneof_modification::upserted_object(_)) = self.modification {
        } else {
            self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::upserted_object(ObjectDiff_UpsertObjectModification::new()));
        }
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::upserted_object(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_upserted_object(&mut self) -> ObjectDiff_UpsertObjectModification {
        if self.has_upserted_object() {
            match self.modification.take() {
                ::std::option::Option::Some(ObjectDiff_oneof_modification::upserted_object(v)) => v,
                _ => panic!(),
            }
        } else {
            ObjectDiff_UpsertObjectModification::new()
        }
    }

    pub fn get_upserted_object(&self) -> &ObjectDiff_UpsertObjectModification {
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::upserted_object(ref v)) => v,
            _ => ObjectDiff_UpsertObjectModification::default_instance(),
        }
    }

    // optional .ObjectDiff.DeletedObjectModfication deleted_object = 2;

    pub fn clear_deleted_object(&mut self) {
        self.modification = ::std::option::Option::None;
    }

    pub fn has_deleted_object(&self) -> bool {
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::deleted_object(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_deleted_object(&mut self, v: ObjectDiff_DeletedObjectModfication) {
        self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::deleted_object(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deleted_object(&mut self) -> &mut ObjectDiff_DeletedObjectModfication {
        if let ::std::option::Option::Some(ObjectDiff_oneof_modification::deleted_object(_)) = self.modification {
        } else {
            self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::deleted_object(ObjectDiff_DeletedObjectModfication::new()));
        }
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::deleted_object(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_deleted_object(&mut self) -> ObjectDiff_DeletedObjectModfication {
        if self.has_deleted_object() {
            match self.modification.take() {
                ::std::option::Option::Some(ObjectDiff_oneof_modification::deleted_object(v)) => v,
                _ => panic!(),
            }
        } else {
            ObjectDiff_DeletedObjectModfication::new()
        }
    }

    pub fn get_deleted_object(&self) -> &ObjectDiff_DeletedObjectModfication {
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::deleted_object(ref v)) => v,
            _ => ObjectDiff_DeletedObjectModfication::default_instance(),
        }
    }
}

impl ::protobuf::Message for ObjectDiff {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::upserted_object(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::deleted_object(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.modification {
            match v {
                &ObjectDiff_oneof_modification::upserted_object(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ObjectDiff_oneof_modification::deleted_object(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.modification {
            match v {
                &ObjectDiff_oneof_modification::upserted_object(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ObjectDiff_oneof_modification::deleted_object(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
            };
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
        ::std::any::TypeId::of::<ObjectDiff>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ObjectDiff {
    fn new() -> ObjectDiff {
        ObjectDiff::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObjectDiff>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "upserted_object",
                    ObjectDiff::has_upserted_object,
                    ObjectDiff::get_upserted_object,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "deleted_object",
                    ObjectDiff::has_deleted_object,
                    ObjectDiff::get_deleted_object,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObjectDiff>(
                    "ObjectDiff",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObjectDiff {
    fn clear(&mut self) {
        self.clear_upserted_object();
        self.clear_deleted_object();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ObjectDiff {
    fn eq(&self, other: &ObjectDiff) -> bool {
        self.modification == other.modification &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ObjectDiff {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ObjectDiff_UpsertObjectModification {
    // message fields
    field_to_upsert: ::protobuf::RepeatedField<ObjectDiff_UpsertObjectModification_FieldToUpsertEntry>,
    field_to_delete: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObjectDiff_UpsertObjectModification {}

impl ObjectDiff_UpsertObjectModification {
    pub fn new() -> ObjectDiff_UpsertObjectModification {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObjectDiff_UpsertObjectModification {
        static mut instance: ::protobuf::lazy::Lazy<ObjectDiff_UpsertObjectModification> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjectDiff_UpsertObjectModification,
        };
        unsafe {
            instance.get(|| {
                ObjectDiff_UpsertObjectModification {
                    field_to_upsert: ::protobuf::RepeatedField::new(),
                    field_to_delete: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .ObjectDiff.UpsertObjectModification.FieldToUpsertEntry field_to_upsert = 1;

    pub fn clear_field_to_upsert(&mut self) {
        self.field_to_upsert.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_to_upsert(&mut self, v: ::protobuf::RepeatedField<ObjectDiff_UpsertObjectModification_FieldToUpsertEntry>) {
        self.field_to_upsert = v;
    }

    // Mutable pointer to the field.
    pub fn mut_field_to_upsert(&mut self) -> &mut ::protobuf::RepeatedField<ObjectDiff_UpsertObjectModification_FieldToUpsertEntry> {
        &mut self.field_to_upsert
    }

    // Take field
    pub fn take_field_to_upsert(&mut self) -> ::protobuf::RepeatedField<ObjectDiff_UpsertObjectModification_FieldToUpsertEntry> {
        ::std::mem::replace(&mut self.field_to_upsert, ::protobuf::RepeatedField::new())
    }

    pub fn get_field_to_upsert(&self) -> &[ObjectDiff_UpsertObjectModification_FieldToUpsertEntry] {
        &self.field_to_upsert
    }

    // repeated string field_to_delete = 2;

    pub fn clear_field_to_delete(&mut self) {
        self.field_to_delete.clear();
    }

    // Param is passed by value, moved
    pub fn set_field_to_delete(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.field_to_delete = v;
    }

    // Mutable pointer to the field.
    pub fn mut_field_to_delete(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.field_to_delete
    }

    // Take field
    pub fn take_field_to_delete(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.field_to_delete, ::protobuf::RepeatedField::new())
    }

    pub fn get_field_to_delete(&self) -> &[::std::string::String] {
        &self.field_to_delete
    }
}

impl ::protobuf::Message for ObjectDiff_UpsertObjectModification {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.field_to_upsert));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.field_to_delete));
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
        for value in self.field_to_upsert.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.field_to_delete.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.field_to_upsert.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.field_to_delete.iter() {
            try!(os.write_string(2, &v));
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
        ::std::any::TypeId::of::<ObjectDiff_UpsertObjectModification>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ObjectDiff_UpsertObjectModification {
    fn new() -> ObjectDiff_UpsertObjectModification {
        ObjectDiff_UpsertObjectModification::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObjectDiff_UpsertObjectModification>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "field_to_upsert",
                    ObjectDiff_UpsertObjectModification::get_field_to_upsert,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "field_to_delete",
                    ObjectDiff_UpsertObjectModification::get_field_to_delete,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObjectDiff_UpsertObjectModification>(
                    "ObjectDiff_UpsertObjectModification",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObjectDiff_UpsertObjectModification {
    fn clear(&mut self) {
        self.clear_field_to_upsert();
        self.clear_field_to_delete();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ObjectDiff_UpsertObjectModification {
    fn eq(&self, other: &ObjectDiff_UpsertObjectModification) -> bool {
        self.field_to_upsert == other.field_to_upsert &&
        self.field_to_delete == other.field_to_delete &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ObjectDiff_UpsertObjectModification {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ObjectDiff_UpsertObjectModification_FieldToUpsertEntry {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObjectDiff_UpsertObjectModification_FieldToUpsertEntry {}

impl ObjectDiff_UpsertObjectModification_FieldToUpsertEntry {
    pub fn new() -> ObjectDiff_UpsertObjectModification_FieldToUpsertEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObjectDiff_UpsertObjectModification_FieldToUpsertEntry {
        static mut instance: ::protobuf::lazy::Lazy<ObjectDiff_UpsertObjectModification_FieldToUpsertEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjectDiff_UpsertObjectModification_FieldToUpsertEntry,
        };
        unsafe {
            instance.get(|| {
                ObjectDiff_UpsertObjectModification_FieldToUpsertEntry {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for ObjectDiff_UpsertObjectModification_FieldToUpsertEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value));
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
        for value in self.key.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<ObjectDiff_UpsertObjectModification_FieldToUpsertEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ObjectDiff_UpsertObjectModification_FieldToUpsertEntry {
    fn new() -> ObjectDiff_UpsertObjectModification_FieldToUpsertEntry {
        ObjectDiff_UpsertObjectModification_FieldToUpsertEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObjectDiff_UpsertObjectModification_FieldToUpsertEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    ObjectDiff_UpsertObjectModification_FieldToUpsertEntry::has_key,
                    ObjectDiff_UpsertObjectModification_FieldToUpsertEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    ObjectDiff_UpsertObjectModification_FieldToUpsertEntry::has_value,
                    ObjectDiff_UpsertObjectModification_FieldToUpsertEntry::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObjectDiff_UpsertObjectModification_FieldToUpsertEntry>(
                    "ObjectDiff_UpsertObjectModification_FieldToUpsertEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObjectDiff_UpsertObjectModification_FieldToUpsertEntry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ObjectDiff_UpsertObjectModification_FieldToUpsertEntry {
    fn eq(&self, other: &ObjectDiff_UpsertObjectModification_FieldToUpsertEntry) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ObjectDiff_UpsertObjectModification_FieldToUpsertEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ObjectDiff_DeletedObjectModfication {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObjectDiff_DeletedObjectModfication {}

impl ObjectDiff_DeletedObjectModfication {
    pub fn new() -> ObjectDiff_DeletedObjectModfication {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObjectDiff_DeletedObjectModfication {
        static mut instance: ::protobuf::lazy::Lazy<ObjectDiff_DeletedObjectModfication> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjectDiff_DeletedObjectModfication,
        };
        unsafe {
            instance.get(|| {
                ObjectDiff_DeletedObjectModfication {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for ObjectDiff_DeletedObjectModfication {
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
        ::std::any::TypeId::of::<ObjectDiff_DeletedObjectModfication>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ObjectDiff_DeletedObjectModfication {
    fn new() -> ObjectDiff_DeletedObjectModfication {
        ObjectDiff_DeletedObjectModfication::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObjectDiff_DeletedObjectModfication>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ObjectDiff_DeletedObjectModfication>(
                    "ObjectDiff_DeletedObjectModfication",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObjectDiff_DeletedObjectModfication {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ObjectDiff_DeletedObjectModfication {
    fn eq(&self, other: &ObjectDiff_DeletedObjectModfication) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ObjectDiff_DeletedObjectModfication {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WorldStateDiff {
    // message fields
    object_change: ::protobuf::RepeatedField<WorldStateDiff_ObjectChangeEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WorldStateDiff {}

impl WorldStateDiff {
    pub fn new() -> WorldStateDiff {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WorldStateDiff {
        static mut instance: ::protobuf::lazy::Lazy<WorldStateDiff> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WorldStateDiff,
        };
        unsafe {
            instance.get(|| {
                WorldStateDiff {
                    object_change: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .WorldStateDiff.ObjectChangeEntry object_change = 1;

    pub fn clear_object_change(&mut self) {
        self.object_change.clear();
    }

    // Param is passed by value, moved
    pub fn set_object_change(&mut self, v: ::protobuf::RepeatedField<WorldStateDiff_ObjectChangeEntry>) {
        self.object_change = v;
    }

    // Mutable pointer to the field.
    pub fn mut_object_change(&mut self) -> &mut ::protobuf::RepeatedField<WorldStateDiff_ObjectChangeEntry> {
        &mut self.object_change
    }

    // Take field
    pub fn take_object_change(&mut self) -> ::protobuf::RepeatedField<WorldStateDiff_ObjectChangeEntry> {
        ::std::mem::replace(&mut self.object_change, ::protobuf::RepeatedField::new())
    }

    pub fn get_object_change(&self) -> &[WorldStateDiff_ObjectChangeEntry] {
        &self.object_change
    }
}

impl ::protobuf::Message for WorldStateDiff {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.object_change));
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
        for value in self.object_change.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.object_change.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<WorldStateDiff>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WorldStateDiff {
    fn new() -> WorldStateDiff {
        WorldStateDiff::new()
    }

    fn descriptor_static(_: ::std::option::Option<WorldStateDiff>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "object_change",
                    WorldStateDiff::get_object_change,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WorldStateDiff>(
                    "WorldStateDiff",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WorldStateDiff {
    fn clear(&mut self) {
        self.clear_object_change();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WorldStateDiff {
    fn eq(&self, other: &WorldStateDiff) -> bool {
        self.object_change == other.object_change &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WorldStateDiff {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WorldStateDiff_ObjectChangeEntry {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularPtrField<ObjectDiff>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WorldStateDiff_ObjectChangeEntry {}

impl WorldStateDiff_ObjectChangeEntry {
    pub fn new() -> WorldStateDiff_ObjectChangeEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WorldStateDiff_ObjectChangeEntry {
        static mut instance: ::protobuf::lazy::Lazy<WorldStateDiff_ObjectChangeEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WorldStateDiff_ObjectChangeEntry,
        };
        unsafe {
            instance.get(|| {
                WorldStateDiff_ObjectChangeEntry {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .ObjectDiff value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ObjectDiff) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ObjectDiff {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ObjectDiff {
        self.value.take().unwrap_or_else(|| ObjectDiff::new())
    }

    pub fn get_value(&self) -> &ObjectDiff {
        self.value.as_ref().unwrap_or_else(|| ObjectDiff::default_instance())
    }
}

impl ::protobuf::Message for WorldStateDiff_ObjectChangeEntry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value));
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
        for value in self.key.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.value.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
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
        ::std::any::TypeId::of::<WorldStateDiff_ObjectChangeEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for WorldStateDiff_ObjectChangeEntry {
    fn new() -> WorldStateDiff_ObjectChangeEntry {
        WorldStateDiff_ObjectChangeEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<WorldStateDiff_ObjectChangeEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    WorldStateDiff_ObjectChangeEntry::has_key,
                    WorldStateDiff_ObjectChangeEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "value",
                    WorldStateDiff_ObjectChangeEntry::has_value,
                    WorldStateDiff_ObjectChangeEntry::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WorldStateDiff_ObjectChangeEntry>(
                    "WorldStateDiff_ObjectChangeEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WorldStateDiff_ObjectChangeEntry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for WorldStateDiff_ObjectChangeEntry {
    fn eq(&self, other: &WorldStateDiff_ObjectChangeEntry) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for WorldStateDiff_ObjectChangeEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ServerWorldStateDiff {
    // message fields
    new_version: ::std::option::Option<u64>,
    changes: ::protobuf::SingularPtrField<WorldStateDiff>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerWorldStateDiff {}

impl ServerWorldStateDiff {
    pub fn new() -> ServerWorldStateDiff {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerWorldStateDiff {
        static mut instance: ::protobuf::lazy::Lazy<ServerWorldStateDiff> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerWorldStateDiff,
        };
        unsafe {
            instance.get(|| {
                ServerWorldStateDiff {
                    new_version: ::std::option::Option::None,
                    changes: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 new_version = 1;

    pub fn clear_new_version(&mut self) {
        self.new_version = ::std::option::Option::None;
    }

    pub fn has_new_version(&self) -> bool {
        self.new_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_version(&mut self, v: u64) {
        self.new_version = ::std::option::Option::Some(v);
    }

    pub fn get_new_version(&self) -> u64 {
        self.new_version.unwrap_or(0)
    }

    // optional .WorldStateDiff changes = 2;

    pub fn clear_changes(&mut self) {
        self.changes.clear();
    }

    pub fn has_changes(&self) -> bool {
        self.changes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_changes(&mut self, v: WorldStateDiff) {
        self.changes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_changes(&mut self) -> &mut WorldStateDiff {
        if self.changes.is_none() {
            self.changes.set_default();
        };
        self.changes.as_mut().unwrap()
    }

    // Take field
    pub fn take_changes(&mut self) -> WorldStateDiff {
        self.changes.take().unwrap_or_else(|| WorldStateDiff::new())
    }

    pub fn get_changes(&self) -> &WorldStateDiff {
        self.changes.as_ref().unwrap_or_else(|| WorldStateDiff::default_instance())
    }
}

impl ::protobuf::Message for ServerWorldStateDiff {
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
                    self.new_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.changes));
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
        for value in self.new_version.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.changes.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.new_version {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.changes.as_ref() {
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
        ::std::any::TypeId::of::<ServerWorldStateDiff>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ServerWorldStateDiff {
    fn new() -> ServerWorldStateDiff {
        ServerWorldStateDiff::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerWorldStateDiff>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "new_version",
                    ServerWorldStateDiff::has_new_version,
                    ServerWorldStateDiff::get_new_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "changes",
                    ServerWorldStateDiff::has_changes,
                    ServerWorldStateDiff::get_changes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServerWorldStateDiff>(
                    "ServerWorldStateDiff",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerWorldStateDiff {
    fn clear(&mut self) {
        self.clear_new_version();
        self.clear_changes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ServerWorldStateDiff {
    fn eq(&self, other: &ServerWorldStateDiff) -> bool {
        self.new_version == other.new_version &&
        self.changes == other.changes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ServerWorldStateDiff {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Event {
    // message fields
    event_id: ::protobuf::SingularField<::std::string::String>,
    acting_client_id: ::protobuf::SingularField<::std::string::String>,
    changes: ::protobuf::SingularPtrField<WorldStateDiff>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event {}

impl Event {
    pub fn new() -> Event {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event {
        static mut instance: ::protobuf::lazy::Lazy<Event> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event,
        };
        unsafe {
            instance.get(|| {
                Event {
                    event_id: ::protobuf::SingularField::none(),
                    acting_client_id: ::protobuf::SingularField::none(),
                    changes: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string event_id = 1;

    pub fn clear_event_id(&mut self) {
        self.event_id.clear();
    }

    pub fn has_event_id(&self) -> bool {
        self.event_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_id(&mut self, v: ::std::string::String) {
        self.event_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_id(&mut self) -> &mut ::std::string::String {
        if self.event_id.is_none() {
            self.event_id.set_default();
        };
        self.event_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_id(&mut self) -> ::std::string::String {
        self.event_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_event_id(&self) -> &str {
        match self.event_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string acting_client_id = 2;

    pub fn clear_acting_client_id(&mut self) {
        self.acting_client_id.clear();
    }

    pub fn has_acting_client_id(&self) -> bool {
        self.acting_client_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_acting_client_id(&mut self, v: ::std::string::String) {
        self.acting_client_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_acting_client_id(&mut self) -> &mut ::std::string::String {
        if self.acting_client_id.is_none() {
            self.acting_client_id.set_default();
        };
        self.acting_client_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_acting_client_id(&mut self) -> ::std::string::String {
        self.acting_client_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_acting_client_id(&self) -> &str {
        match self.acting_client_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .WorldStateDiff changes = 3;

    pub fn clear_changes(&mut self) {
        self.changes.clear();
    }

    pub fn has_changes(&self) -> bool {
        self.changes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_changes(&mut self, v: WorldStateDiff) {
        self.changes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_changes(&mut self) -> &mut WorldStateDiff {
        if self.changes.is_none() {
            self.changes.set_default();
        };
        self.changes.as_mut().unwrap()
    }

    // Take field
    pub fn take_changes(&mut self) -> WorldStateDiff {
        self.changes.take().unwrap_or_else(|| WorldStateDiff::new())
    }

    pub fn get_changes(&self) -> &WorldStateDiff {
        self.changes.as_ref().unwrap_or_else(|| WorldStateDiff::default_instance())
    }
}

impl ::protobuf::Message for Event {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.event_id));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.acting_client_id));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.changes));
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
        for value in self.event_id.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.acting_client_id.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.changes.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.event_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.acting_client_id.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.changes.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Event>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Event {
    fn new() -> Event {
        Event::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "event_id",
                    Event::has_event_id,
                    Event::get_event_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "acting_client_id",
                    Event::has_acting_client_id,
                    Event::get_acting_client_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "changes",
                    Event::has_changes,
                    Event::get_changes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event>(
                    "Event",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event {
    fn clear(&mut self) {
        self.clear_event_id();
        self.clear_acting_client_id();
        self.clear_changes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.event_id == other.event_id &&
        self.acting_client_id == other.acting_client_id &&
        self.changes == other.changes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0c, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x43,
    0x0a, 0x10, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x6e,
    0x66, 0x6f, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x22, 0x41, 0x0a, 0x10, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x53, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x19, 0x0a, 0x08, 0x74, 0x63, 0x70, 0x5f, 0x70,
    0x6f, 0x72, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x07, 0x74, 0x63, 0x70, 0x50, 0x6f,
    0x72, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x22, 0x40, 0x0a, 0x0a, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x49, 0x6e, 0x66, 0x6f, 0x12, 0x32, 0x0a, 0x0b, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x69,
    0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x50, 0x75, 0x62, 0x6c,
    0x69, 0x63, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x0a, 0x70, 0x75,
    0x62, 0x6c, 0x69, 0x63, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x0c, 0x0a, 0x0a, 0x53, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0xc0, 0x03, 0x0a, 0x0a, 0x4f, 0x62, 0x6a, 0x65, 0x63,
    0x74, 0x44, 0x69, 0x66, 0x66, 0x12, 0x4f, 0x0a, 0x0f, 0x75, 0x70, 0x73, 0x65, 0x72, 0x74, 0x65,
    0x64, 0x5f, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24,
    0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x44, 0x69, 0x66, 0x66, 0x2e, 0x55, 0x70, 0x73, 0x65,
    0x72, 0x74, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x0e, 0x75, 0x70, 0x73, 0x65, 0x72, 0x74, 0x65, 0x64,
    0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x4d, 0x0a, 0x0e, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65,
    0x64, 0x5f, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24,
    0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x44, 0x69, 0x66, 0x66, 0x2e, 0x44, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x64, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x4d, 0x6f, 0x64, 0x66, 0x69, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x0d, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x4f,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x1a, 0xe5, 0x01, 0x0a, 0x18, 0x55, 0x70, 0x73, 0x65, 0x72, 0x74,
    0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x12, 0x5f, 0x0a, 0x0f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x74, 0x6f, 0x5f, 0x75,
    0x70, 0x73, 0x65, 0x72, 0x74, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x37, 0x2e, 0x4f, 0x62,
    0x6a, 0x65, 0x63, 0x74, 0x44, 0x69, 0x66, 0x66, 0x2e, 0x55, 0x70, 0x73, 0x65, 0x72, 0x74, 0x4f,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x54, 0x6f, 0x55, 0x70, 0x73, 0x65, 0x72, 0x74, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x52, 0x0d, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x54, 0x6f, 0x55, 0x70, 0x73,
    0x65, 0x72, 0x74, 0x12, 0x26, 0x0a, 0x0f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x74, 0x6f, 0x5f,
    0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0d, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x54, 0x6f, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x1a, 0x40, 0x0a, 0x12, 0x46,
    0x69, 0x65, 0x6c, 0x64, 0x54, 0x6f, 0x55, 0x70, 0x73, 0x65, 0x72, 0x74, 0x45, 0x6e, 0x74, 0x72,
    0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03,
    0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x1a, 0x1a, 0x0a,
    0x18, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x4d, 0x6f,
    0x64, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x42, 0x0e, 0x0a, 0x0c, 0x6d, 0x6f, 0x64,
    0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x22, 0xa6, 0x01, 0x0a, 0x0e, 0x57, 0x6f,
    0x72, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x44, 0x69, 0x66, 0x66, 0x12, 0x46, 0x0a, 0x0d,
    0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x5f, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x74, 0x65,
    0x44, 0x69, 0x66, 0x66, 0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x43, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x0c, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x43, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x1a, 0x4c, 0x0a, 0x11, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x43, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x21, 0x0a, 0x05, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x4f, 0x62, 0x6a,
    0x65, 0x63, 0x74, 0x44, 0x69, 0x66, 0x66, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02,
    0x38, 0x01, 0x22, 0x62, 0x0a, 0x14, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x57, 0x6f, 0x72, 0x6c,
    0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x44, 0x69, 0x66, 0x66, 0x12, 0x1f, 0x0a, 0x0b, 0x6e, 0x65,
    0x77, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x0a, 0x6e, 0x65, 0x77, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x29, 0x0a, 0x07, 0x63,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x57,
    0x6f, 0x72, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x44, 0x69, 0x66, 0x66, 0x52, 0x07, 0x63,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x22, 0x77, 0x0a, 0x05, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12,
    0x19, 0x0a, 0x08, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x07, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x28, 0x0a, 0x10, 0x61, 0x63,
    0x74, 0x69, 0x6e, 0x67, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x61, 0x63, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x49, 0x64, 0x12, 0x29, 0x0a, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x53, 0x74, 0x61,
    0x74, 0x65, 0x44, 0x69, 0x66, 0x66, 0x52, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x4a,
    0xac, 0x17, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x51, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x08, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x18, 0x0a, 0x27, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x02, 0x17, 0x1a, 0x1a, 0x20, 0x52, 0x65, 0x71, 0x75,
    0x69, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x41, 0x20, 0x72, 0x61, 0x6e, 0x64, 0x6f, 0x6d, 0x20, 0x75,
    0x75, 0x69, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x04, 0x02, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x04,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x09, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x04, 0x15, 0x16, 0x0a, 0x18,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x02, 0x12, 0x1a, 0x0b, 0x20, 0x52, 0x65,
    0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x07, 0x02, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x07, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x07, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x07, 0x10,
    0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0a, 0x00, 0x10, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x18, 0x0a, 0x18, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x0c, 0x02, 0x16, 0x1a, 0x0b, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65,
    0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0c, 0x02,
    0x0a, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0c, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x09, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x14, 0x15, 0x0a, 0x18, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x02, 0x12, 0x1a, 0x0b, 0x20, 0x52, 0x65, 0x71, 0x75,
    0x69, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x0f, 0x02, 0x0c, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x0f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x09,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0f, 0x10, 0x11, 0x0a,
    0x9f, 0x01, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x14, 0x00, 0x17, 0x01, 0x1a, 0x92, 0x01, 0x20,
    0x41, 0x6c, 0x6c, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x20, 0x43, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20, 0x70, 0x75, 0x62, 0x6c, 0x69,
    0x63, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x6e,
    0x64, 0x0a, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x70, 0x72, 0x69, 0x76, 0x69, 0x6c, 0x65, 0x67, 0x65,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x14, 0x08, 0x12, 0x0a, 0x5c, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x16, 0x02, 0x23, 0x1a, 0x4f, 0x20, 0x52, 0x65, 0x71,
    0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x49, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20,
    0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6f, 0x74, 0x68, 0x65,
    0x72, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x16, 0x02, 0x14, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x16, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x16, 0x13, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x16, 0x21, 0x22, 0x0a, 0x53, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x1a, 0x00, 0x1b, 0x01,
    0x1a, 0x47, 0x20, 0x41, 0x6c, 0x6c, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x61, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x68, 0x61, 0x72, 0x61, 0x62,
    0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x1a, 0x08, 0x12, 0x0a, 0x80, 0x02, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x20, 0x00,
    0x33, 0x01, 0x1a, 0xf3, 0x01, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20,
    0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e,
    0x67, 0x6c, 0x65, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x2e, 0x0a, 0x20, 0x54, 0x4f, 0x44,
    0x4f, 0x28, 0x61, 0x65, 0x69, 0x64, 0x65, 0x6c, 0x73, 0x6f, 0x6e, 0x29, 0x3a, 0x20, 0x49, 0x74,
    0x27, 0x73, 0x20, 0x75, 0x6e, 0x63, 0x6c, 0x65, 0x61, 0x72, 0x20, 0x68, 0x6f, 0x77, 0x20, 0x73,
    0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x41, 0x50, 0x49,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x2e, 0x20, 0x53, 0x68, 0x6f, 0x75,
    0x6c, 0x64, 0x20, 0x72, 0x65, 0x2d, 0x65, 0x76, 0x61, 0x6c, 0x75, 0x61, 0x74, 0x65, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x66, 0x75, 0x74, 0x75,
    0x72, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x65, 0x65, 0x20, 0x69, 0x66, 0x20, 0x69, 0x74,
    0x20, 0x6d, 0x61, 0x6b, 0x65, 0x73, 0x20, 0x73, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x62, 0x65, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x72, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74,
    0x69, 0x76, 0x65, 0x20, 0x28, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x73, 0x65, 0x70, 0x65, 0x72, 0x61,
    0x74, 0x69, 0x6e, 0x67, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x61, 0x64, 0x64, 0x73, 0x29, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x03, 0x20, 0x08, 0x12, 0x0a, 0x64, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x00, 0x12, 0x04, 0x22, 0x02,
    0x28, 0x03, 0x1a, 0x56, 0x20, 0x55, 0x73, 0x65, 0x64, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x65, 0x76,
    0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x6f, 0x66,
    0x20, 0x61, 0x6e, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x61, 0x72, 0x65, 0x20, 0x75,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x2c, 0x20, 0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x69, 0x6e,
    0x67, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63,
    0x74, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x65, 0x77, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x03, 0x00, 0x01, 0x12, 0x03, 0x22, 0x0a, 0x22, 0x0a, 0x3a, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x24, 0x04, 0x2b, 0x1a, 0x2b, 0x20, 0x4d, 0x61, 0x70, 0x20, 0x6f, 0x66,
    0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x6b, 0x65, 0x79, 0x2c, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x69, 0x6e, 0x73, 0x65,
    0x72, 0x74, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x24, 0x04, 0x22, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x24, 0x04, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x24, 0x17, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x24, 0x29, 0x2a, 0x0a, 0x37, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x27, 0x04, 0x28, 0x1a, 0x28, 0x20, 0x52, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20,
    0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x6b, 0x65,
    0x79, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x27, 0x04, 0x0c, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x27, 0x0d, 0x13, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x27, 0x14, 0x23, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x27, 0x26, 0x27, 0x0a, 0x62,
    0x0a, 0x04, 0x04, 0x04, 0x03, 0x01, 0x12, 0x04, 0x2a, 0x02, 0x2c, 0x03, 0x22, 0x54, 0x20, 0x4e,
    0x6f, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x65, 0x64, 0x20,
    0x79, 0x65, 0x74, 0x2c, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x61,
    0x63, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20,
    0x70, 0x6f, 0x70, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x73,
    0x69, 0x67, 0x6e, 0x61, 0x6c, 0x20, 0x61, 0x20, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x69, 0x6f, 0x6e,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x0a, 0x22,
    0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x04, 0x08, 0x00, 0x12, 0x04, 0x2f, 0x02, 0x32, 0x03, 0x1a, 0x21,
    0x20, 0x4f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x73, 0x65, 0x20, 0x63, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x79, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x08, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x14, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x30, 0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x30, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x30, 0x1d, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x30, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x31, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x31, 0x04,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x31, 0x1d, 0x2b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x31, 0x2e, 0x2f, 0x0a, 0x6c, 0x0a,
    0x02, 0x04, 0x05, 0x12, 0x04, 0x37, 0x00, 0x3b, 0x01, 0x1a, 0x60, 0x20, 0x52, 0x65, 0x70, 0x72,
    0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20,
    0x6f, 0x66, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x2e, 0x20, 0x43,
    0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79,
    0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x6f,
    0x72, 0x0a, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x05, 0x01, 0x12, 0x03, 0x37, 0x08, 0x16, 0x0a, 0x87, 0x01, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00,
    0x12, 0x03, 0x3a, 0x02, 0x2c, 0x1a, 0x7a, 0x20, 0x41, 0x20, 0x6d, 0x61, 0x70, 0x20, 0x66, 0x72,
    0x6f, 0x6d, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x69, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x6f,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x20, 0x64, 0x6f, 0x65, 0x73, 0x6e, 0x27, 0x74, 0x20, 0x61, 0x6c,
    0x72, 0x65, 0x61, 0x64, 0x79, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x2c, 0x20, 0x69, 0x74, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x64, 0x65, 0x72,
    0x65, 0x64, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x04, 0x3a, 0x02, 0x37, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3a, 0x02, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3a, 0x1a, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3a, 0x2a, 0x2b, 0x0a, 0x6b, 0x0a, 0x02, 0x04, 0x06,
    0x12, 0x04, 0x3e, 0x00, 0x45, 0x01, 0x1a, 0x5f, 0x20, 0x52, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65,
    0x6e, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x61,
    0x74, 0x69, 0x76, 0x65, 0x20, 0x76, 0x69, 0x65, 0x77, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x63,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x6f, 0x72,
    0x6c, 0x64, 0x2c, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x77, 0x6f, 0x72,
    0x6c, 0x64, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e,
    0x6f, 0x74, 0x68, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03,
    0x3e, 0x08, 0x1c, 0x0a, 0x9e, 0x01, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x41, 0x02,
    0x19, 0x1a, 0x90, 0x01, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x61, 0x66, 0x74, 0x65,
    0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x69, 0x66, 0x66, 0x20, 0x69, 0x73, 0x20, 0x61, 0x70,
    0x70, 0x6c, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x69, 0x73, 0x20, 0x6c, 0x65, 0x73, 0x73, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x69, 0x66, 0x66,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x69, 0x67, 0x6e, 0x6f, 0x72,
    0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x41,
    0x02, 0x3e, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x41, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x41, 0x09, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x41, 0x17, 0x18, 0x0a, 0x41, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x44, 0x02, 0x1d, 0x1a, 0x34, 0x20, 0x52, 0x65, 0x71,
    0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f,
    0x20, 0x61, 0x70, 0x70, 0x6c, 0x79, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x6c, 0x79, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x04, 0x44, 0x02, 0x41, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x44, 0x02, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x44, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x44, 0x1b, 0x1c, 0x0a, 0x58, 0x0a, 0x02, 0x04, 0x07, 0x12,
    0x04, 0x48, 0x00, 0x51, 0x01, 0x1a, 0x4c, 0x20, 0x41, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65,
    0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d,
    0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x2c, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x64, 0x69,
    0x66, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x73, 0x74, 0x61, 0x74,
    0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x48, 0x08, 0x0d, 0x0a,
    0x3f, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x4a, 0x02, 0x16, 0x1a, 0x32, 0x20, 0x52,
    0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x41, 0x20, 0x72, 0x61, 0x6e, 0x64, 0x6f,
    0x6d, 0x20, 0x75, 0x75, 0x69, 0x64, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64,
    0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x4a, 0x02, 0x48, 0x0f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4a, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4a, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4a, 0x14, 0x15, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x01, 0x12, 0x03, 0x4d, 0x02, 0x1e, 0x1a, 0x26, 0x20, 0x54, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74,
    0x65, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0x4d, 0x02, 0x4a, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4d, 0x09, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x4d, 0x1c, 0x1d, 0x0a, 0x3c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12,
    0x03, 0x50, 0x02, 0x1d, 0x1a, 0x2f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f,
    0x66, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x20, 0x77, 0x61, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x70,
    0x70, 0x6c, 0x79, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x50, 0x02, 0x4d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x06, 0x12, 0x03, 0x50,
    0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x50, 0x11, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x50, 0x1b, 0x1c, 0x62, 0x06,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
