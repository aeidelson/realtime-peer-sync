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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_id.as_ref() {
            try!(os.write_string(1, &v));
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
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PublicClientInfo {
    fn eq(&self, other: &PublicClientInfo) -> bool {
        self.client_id == other.client_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PublicClientInfo {
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
pub struct Object {
    // message fields
    object_id: ::protobuf::SingularField<::std::string::String>,
    payload: ::protobuf::RepeatedField<Object_PayloadEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Object {}

impl Object {
    pub fn new() -> Object {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Object {
        static mut instance: ::protobuf::lazy::Lazy<Object> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Object,
        };
        unsafe {
            instance.get(|| {
                Object {
                    object_id: ::protobuf::SingularField::none(),
                    payload: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string object_id = 1;

    pub fn clear_object_id(&mut self) {
        self.object_id.clear();
    }

    pub fn has_object_id(&self) -> bool {
        self.object_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_object_id(&mut self, v: ::std::string::String) {
        self.object_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_object_id(&mut self) -> &mut ::std::string::String {
        if self.object_id.is_none() {
            self.object_id.set_default();
        };
        self.object_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_object_id(&mut self) -> ::std::string::String {
        self.object_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_object_id(&self) -> &str {
        match self.object_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .Object.PayloadEntry payload = 2;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::protobuf::RepeatedField<Object_PayloadEntry>) {
        self.payload = v;
    }

    // Mutable pointer to the field.
    pub fn mut_payload(&mut self) -> &mut ::protobuf::RepeatedField<Object_PayloadEntry> {
        &mut self.payload
    }

    // Take field
    pub fn take_payload(&mut self) -> ::protobuf::RepeatedField<Object_PayloadEntry> {
        ::std::mem::replace(&mut self.payload, ::protobuf::RepeatedField::new())
    }

    pub fn get_payload(&self) -> &[Object_PayloadEntry] {
        &self.payload
    }
}

impl ::protobuf::Message for Object {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.object_id));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.payload));
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
        for value in self.object_id.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.payload.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.object_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in self.payload.iter() {
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
        ::std::any::TypeId::of::<Object>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Object {
    fn new() -> Object {
        Object::new()
    }

    fn descriptor_static(_: ::std::option::Option<Object>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "object_id",
                    Object::has_object_id,
                    Object::get_object_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "payload",
                    Object::get_payload,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Object>(
                    "Object",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Object {
    fn clear(&mut self) {
        self.clear_object_id();
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Object {
    fn eq(&self, other: &Object) -> bool {
        self.object_id == other.object_id &&
        self.payload == other.payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Object {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Object_PayloadEntry {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Object_PayloadEntry {}

impl Object_PayloadEntry {
    pub fn new() -> Object_PayloadEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Object_PayloadEntry {
        static mut instance: ::protobuf::lazy::Lazy<Object_PayloadEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Object_PayloadEntry,
        };
        unsafe {
            instance.get(|| {
                Object_PayloadEntry {
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

impl ::protobuf::Message for Object_PayloadEntry {
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
        ::std::any::TypeId::of::<Object_PayloadEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Object_PayloadEntry {
    fn new() -> Object_PayloadEntry {
        Object_PayloadEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<Object_PayloadEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    Object_PayloadEntry::has_key,
                    Object_PayloadEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    Object_PayloadEntry::has_value,
                    Object_PayloadEntry::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Object_PayloadEntry>(
                    "Object_PayloadEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Object_PayloadEntry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Object_PayloadEntry {
    fn eq(&self, other: &Object_PayloadEntry) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Object_PayloadEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ObjectDiff {
    // message fields
    object_id: ::protobuf::SingularField<::std::string::String>,
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
    new_object(ObjectDiff_NewObjectModification),
    deleted_object(ObjectDiff_DeletedObjectModfication),
    added_fields(ObjectDiff_AddedFieldsModification),
    updated_fields(ObjectDiff_UpdatedFieldsModification),
    removed_fields(ObjectDiff_RemovedFieldsModification),
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
                    object_id: ::protobuf::SingularField::none(),
                    modification: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string object_id = 1;

    pub fn clear_object_id(&mut self) {
        self.object_id.clear();
    }

    pub fn has_object_id(&self) -> bool {
        self.object_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_object_id(&mut self, v: ::std::string::String) {
        self.object_id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_object_id(&mut self) -> &mut ::std::string::String {
        if self.object_id.is_none() {
            self.object_id.set_default();
        };
        self.object_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_object_id(&mut self) -> ::std::string::String {
        self.object_id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_object_id(&self) -> &str {
        match self.object_id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .ObjectDiff.NewObjectModification new_object = 2;

    pub fn clear_new_object(&mut self) {
        self.modification = ::std::option::Option::None;
    }

    pub fn has_new_object(&self) -> bool {
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::new_object(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_new_object(&mut self, v: ObjectDiff_NewObjectModification) {
        self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::new_object(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_new_object(&mut self) -> &mut ObjectDiff_NewObjectModification {
        if let ::std::option::Option::Some(ObjectDiff_oneof_modification::new_object(_)) = self.modification {
        } else {
            self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::new_object(ObjectDiff_NewObjectModification::new()));
        }
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::new_object(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_new_object(&mut self) -> ObjectDiff_NewObjectModification {
        if self.has_new_object() {
            match self.modification.take() {
                ::std::option::Option::Some(ObjectDiff_oneof_modification::new_object(v)) => v,
                _ => panic!(),
            }
        } else {
            ObjectDiff_NewObjectModification::new()
        }
    }

    pub fn get_new_object(&self) -> &ObjectDiff_NewObjectModification {
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::new_object(ref v)) => v,
            _ => ObjectDiff_NewObjectModification::default_instance(),
        }
    }

    // optional .ObjectDiff.DeletedObjectModfication deleted_object = 3;

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

    // optional .ObjectDiff.AddedFieldsModification added_fields = 4;

    pub fn clear_added_fields(&mut self) {
        self.modification = ::std::option::Option::None;
    }

    pub fn has_added_fields(&self) -> bool {
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::added_fields(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_added_fields(&mut self, v: ObjectDiff_AddedFieldsModification) {
        self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::added_fields(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_added_fields(&mut self) -> &mut ObjectDiff_AddedFieldsModification {
        if let ::std::option::Option::Some(ObjectDiff_oneof_modification::added_fields(_)) = self.modification {
        } else {
            self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::added_fields(ObjectDiff_AddedFieldsModification::new()));
        }
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::added_fields(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_added_fields(&mut self) -> ObjectDiff_AddedFieldsModification {
        if self.has_added_fields() {
            match self.modification.take() {
                ::std::option::Option::Some(ObjectDiff_oneof_modification::added_fields(v)) => v,
                _ => panic!(),
            }
        } else {
            ObjectDiff_AddedFieldsModification::new()
        }
    }

    pub fn get_added_fields(&self) -> &ObjectDiff_AddedFieldsModification {
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::added_fields(ref v)) => v,
            _ => ObjectDiff_AddedFieldsModification::default_instance(),
        }
    }

    // optional .ObjectDiff.UpdatedFieldsModification updated_fields = 5;

    pub fn clear_updated_fields(&mut self) {
        self.modification = ::std::option::Option::None;
    }

    pub fn has_updated_fields(&self) -> bool {
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::updated_fields(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_updated_fields(&mut self, v: ObjectDiff_UpdatedFieldsModification) {
        self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::updated_fields(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_updated_fields(&mut self) -> &mut ObjectDiff_UpdatedFieldsModification {
        if let ::std::option::Option::Some(ObjectDiff_oneof_modification::updated_fields(_)) = self.modification {
        } else {
            self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::updated_fields(ObjectDiff_UpdatedFieldsModification::new()));
        }
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::updated_fields(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_updated_fields(&mut self) -> ObjectDiff_UpdatedFieldsModification {
        if self.has_updated_fields() {
            match self.modification.take() {
                ::std::option::Option::Some(ObjectDiff_oneof_modification::updated_fields(v)) => v,
                _ => panic!(),
            }
        } else {
            ObjectDiff_UpdatedFieldsModification::new()
        }
    }

    pub fn get_updated_fields(&self) -> &ObjectDiff_UpdatedFieldsModification {
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::updated_fields(ref v)) => v,
            _ => ObjectDiff_UpdatedFieldsModification::default_instance(),
        }
    }

    // optional .ObjectDiff.RemovedFieldsModification removed_fields = 6;

    pub fn clear_removed_fields(&mut self) {
        self.modification = ::std::option::Option::None;
    }

    pub fn has_removed_fields(&self) -> bool {
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::removed_fields(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_removed_fields(&mut self, v: ObjectDiff_RemovedFieldsModification) {
        self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::removed_fields(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_removed_fields(&mut self) -> &mut ObjectDiff_RemovedFieldsModification {
        if let ::std::option::Option::Some(ObjectDiff_oneof_modification::removed_fields(_)) = self.modification {
        } else {
            self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::removed_fields(ObjectDiff_RemovedFieldsModification::new()));
        }
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::removed_fields(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_removed_fields(&mut self) -> ObjectDiff_RemovedFieldsModification {
        if self.has_removed_fields() {
            match self.modification.take() {
                ::std::option::Option::Some(ObjectDiff_oneof_modification::removed_fields(v)) => v,
                _ => panic!(),
            }
        } else {
            ObjectDiff_RemovedFieldsModification::new()
        }
    }

    pub fn get_removed_fields(&self) -> &ObjectDiff_RemovedFieldsModification {
        match self.modification {
            ::std::option::Option::Some(ObjectDiff_oneof_modification::removed_fields(ref v)) => v,
            _ => ObjectDiff_RemovedFieldsModification::default_instance(),
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.object_id));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::new_object(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::deleted_object(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::added_fields(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::updated_fields(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.modification = ::std::option::Option::Some(ObjectDiff_oneof_modification::removed_fields(try!(is.read_message())));
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
        for value in self.object_id.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if let ::std::option::Option::Some(ref v) = self.modification {
            match v {
                &ObjectDiff_oneof_modification::new_object(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ObjectDiff_oneof_modification::deleted_object(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ObjectDiff_oneof_modification::added_fields(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ObjectDiff_oneof_modification::updated_fields(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ObjectDiff_oneof_modification::removed_fields(ref v) => {
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
        if let Some(v) = self.object_id.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let ::std::option::Option::Some(ref v) = self.modification {
            match v {
                &ObjectDiff_oneof_modification::new_object(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ObjectDiff_oneof_modification::deleted_object(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ObjectDiff_oneof_modification::added_fields(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ObjectDiff_oneof_modification::updated_fields(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ObjectDiff_oneof_modification::removed_fields(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "object_id",
                    ObjectDiff::has_object_id,
                    ObjectDiff::get_object_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "new_object",
                    ObjectDiff::has_new_object,
                    ObjectDiff::get_new_object,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "deleted_object",
                    ObjectDiff::has_deleted_object,
                    ObjectDiff::get_deleted_object,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "added_fields",
                    ObjectDiff::has_added_fields,
                    ObjectDiff::get_added_fields,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "updated_fields",
                    ObjectDiff::has_updated_fields,
                    ObjectDiff::get_updated_fields,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "removed_fields",
                    ObjectDiff::has_removed_fields,
                    ObjectDiff::get_removed_fields,
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
        self.clear_object_id();
        self.clear_new_object();
        self.clear_deleted_object();
        self.clear_added_fields();
        self.clear_updated_fields();
        self.clear_removed_fields();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ObjectDiff {
    fn eq(&self, other: &ObjectDiff) -> bool {
        self.object_id == other.object_id &&
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
pub struct ObjectDiff_NewObjectModification {
    // message fields
    full_payload: ::protobuf::RepeatedField<ObjectDiff_NewObjectModification_FullPayloadEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObjectDiff_NewObjectModification {}

impl ObjectDiff_NewObjectModification {
    pub fn new() -> ObjectDiff_NewObjectModification {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObjectDiff_NewObjectModification {
        static mut instance: ::protobuf::lazy::Lazy<ObjectDiff_NewObjectModification> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjectDiff_NewObjectModification,
        };
        unsafe {
            instance.get(|| {
                ObjectDiff_NewObjectModification {
                    full_payload: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .ObjectDiff.NewObjectModification.FullPayloadEntry full_payload = 1;

    pub fn clear_full_payload(&mut self) {
        self.full_payload.clear();
    }

    // Param is passed by value, moved
    pub fn set_full_payload(&mut self, v: ::protobuf::RepeatedField<ObjectDiff_NewObjectModification_FullPayloadEntry>) {
        self.full_payload = v;
    }

    // Mutable pointer to the field.
    pub fn mut_full_payload(&mut self) -> &mut ::protobuf::RepeatedField<ObjectDiff_NewObjectModification_FullPayloadEntry> {
        &mut self.full_payload
    }

    // Take field
    pub fn take_full_payload(&mut self) -> ::protobuf::RepeatedField<ObjectDiff_NewObjectModification_FullPayloadEntry> {
        ::std::mem::replace(&mut self.full_payload, ::protobuf::RepeatedField::new())
    }

    pub fn get_full_payload(&self) -> &[ObjectDiff_NewObjectModification_FullPayloadEntry] {
        &self.full_payload
    }
}

impl ::protobuf::Message for ObjectDiff_NewObjectModification {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.full_payload));
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
        for value in self.full_payload.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.full_payload.iter() {
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
        ::std::any::TypeId::of::<ObjectDiff_NewObjectModification>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ObjectDiff_NewObjectModification {
    fn new() -> ObjectDiff_NewObjectModification {
        ObjectDiff_NewObjectModification::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObjectDiff_NewObjectModification>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "full_payload",
                    ObjectDiff_NewObjectModification::get_full_payload,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObjectDiff_NewObjectModification>(
                    "ObjectDiff_NewObjectModification",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObjectDiff_NewObjectModification {
    fn clear(&mut self) {
        self.clear_full_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ObjectDiff_NewObjectModification {
    fn eq(&self, other: &ObjectDiff_NewObjectModification) -> bool {
        self.full_payload == other.full_payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ObjectDiff_NewObjectModification {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ObjectDiff_NewObjectModification_FullPayloadEntry {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObjectDiff_NewObjectModification_FullPayloadEntry {}

impl ObjectDiff_NewObjectModification_FullPayloadEntry {
    pub fn new() -> ObjectDiff_NewObjectModification_FullPayloadEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObjectDiff_NewObjectModification_FullPayloadEntry {
        static mut instance: ::protobuf::lazy::Lazy<ObjectDiff_NewObjectModification_FullPayloadEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjectDiff_NewObjectModification_FullPayloadEntry,
        };
        unsafe {
            instance.get(|| {
                ObjectDiff_NewObjectModification_FullPayloadEntry {
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

impl ::protobuf::Message for ObjectDiff_NewObjectModification_FullPayloadEntry {
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
        ::std::any::TypeId::of::<ObjectDiff_NewObjectModification_FullPayloadEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ObjectDiff_NewObjectModification_FullPayloadEntry {
    fn new() -> ObjectDiff_NewObjectModification_FullPayloadEntry {
        ObjectDiff_NewObjectModification_FullPayloadEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObjectDiff_NewObjectModification_FullPayloadEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    ObjectDiff_NewObjectModification_FullPayloadEntry::has_key,
                    ObjectDiff_NewObjectModification_FullPayloadEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    ObjectDiff_NewObjectModification_FullPayloadEntry::has_value,
                    ObjectDiff_NewObjectModification_FullPayloadEntry::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObjectDiff_NewObjectModification_FullPayloadEntry>(
                    "ObjectDiff_NewObjectModification_FullPayloadEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObjectDiff_NewObjectModification_FullPayloadEntry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ObjectDiff_NewObjectModification_FullPayloadEntry {
    fn eq(&self, other: &ObjectDiff_NewObjectModification_FullPayloadEntry) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ObjectDiff_NewObjectModification_FullPayloadEntry {
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
pub struct ObjectDiff_AddedFieldsModification {
    // message fields
    added: ::protobuf::RepeatedField<ObjectDiff_AddedFieldsModification_AddedEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObjectDiff_AddedFieldsModification {}

impl ObjectDiff_AddedFieldsModification {
    pub fn new() -> ObjectDiff_AddedFieldsModification {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObjectDiff_AddedFieldsModification {
        static mut instance: ::protobuf::lazy::Lazy<ObjectDiff_AddedFieldsModification> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjectDiff_AddedFieldsModification,
        };
        unsafe {
            instance.get(|| {
                ObjectDiff_AddedFieldsModification {
                    added: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .ObjectDiff.AddedFieldsModification.AddedEntry added = 1;

    pub fn clear_added(&mut self) {
        self.added.clear();
    }

    // Param is passed by value, moved
    pub fn set_added(&mut self, v: ::protobuf::RepeatedField<ObjectDiff_AddedFieldsModification_AddedEntry>) {
        self.added = v;
    }

    // Mutable pointer to the field.
    pub fn mut_added(&mut self) -> &mut ::protobuf::RepeatedField<ObjectDiff_AddedFieldsModification_AddedEntry> {
        &mut self.added
    }

    // Take field
    pub fn take_added(&mut self) -> ::protobuf::RepeatedField<ObjectDiff_AddedFieldsModification_AddedEntry> {
        ::std::mem::replace(&mut self.added, ::protobuf::RepeatedField::new())
    }

    pub fn get_added(&self) -> &[ObjectDiff_AddedFieldsModification_AddedEntry] {
        &self.added
    }
}

impl ::protobuf::Message for ObjectDiff_AddedFieldsModification {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.added));
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
        for value in self.added.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.added.iter() {
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
        ::std::any::TypeId::of::<ObjectDiff_AddedFieldsModification>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ObjectDiff_AddedFieldsModification {
    fn new() -> ObjectDiff_AddedFieldsModification {
        ObjectDiff_AddedFieldsModification::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObjectDiff_AddedFieldsModification>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "added",
                    ObjectDiff_AddedFieldsModification::get_added,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObjectDiff_AddedFieldsModification>(
                    "ObjectDiff_AddedFieldsModification",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObjectDiff_AddedFieldsModification {
    fn clear(&mut self) {
        self.clear_added();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ObjectDiff_AddedFieldsModification {
    fn eq(&self, other: &ObjectDiff_AddedFieldsModification) -> bool {
        self.added == other.added &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ObjectDiff_AddedFieldsModification {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ObjectDiff_AddedFieldsModification_AddedEntry {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObjectDiff_AddedFieldsModification_AddedEntry {}

impl ObjectDiff_AddedFieldsModification_AddedEntry {
    pub fn new() -> ObjectDiff_AddedFieldsModification_AddedEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObjectDiff_AddedFieldsModification_AddedEntry {
        static mut instance: ::protobuf::lazy::Lazy<ObjectDiff_AddedFieldsModification_AddedEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjectDiff_AddedFieldsModification_AddedEntry,
        };
        unsafe {
            instance.get(|| {
                ObjectDiff_AddedFieldsModification_AddedEntry {
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

impl ::protobuf::Message for ObjectDiff_AddedFieldsModification_AddedEntry {
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
        ::std::any::TypeId::of::<ObjectDiff_AddedFieldsModification_AddedEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ObjectDiff_AddedFieldsModification_AddedEntry {
    fn new() -> ObjectDiff_AddedFieldsModification_AddedEntry {
        ObjectDiff_AddedFieldsModification_AddedEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObjectDiff_AddedFieldsModification_AddedEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    ObjectDiff_AddedFieldsModification_AddedEntry::has_key,
                    ObjectDiff_AddedFieldsModification_AddedEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    ObjectDiff_AddedFieldsModification_AddedEntry::has_value,
                    ObjectDiff_AddedFieldsModification_AddedEntry::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObjectDiff_AddedFieldsModification_AddedEntry>(
                    "ObjectDiff_AddedFieldsModification_AddedEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObjectDiff_AddedFieldsModification_AddedEntry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ObjectDiff_AddedFieldsModification_AddedEntry {
    fn eq(&self, other: &ObjectDiff_AddedFieldsModification_AddedEntry) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ObjectDiff_AddedFieldsModification_AddedEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ObjectDiff_UpdatedFieldsModification {
    // message fields
    updated: ::protobuf::RepeatedField<ObjectDiff_UpdatedFieldsModification_UpdatedEntry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObjectDiff_UpdatedFieldsModification {}

impl ObjectDiff_UpdatedFieldsModification {
    pub fn new() -> ObjectDiff_UpdatedFieldsModification {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObjectDiff_UpdatedFieldsModification {
        static mut instance: ::protobuf::lazy::Lazy<ObjectDiff_UpdatedFieldsModification> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjectDiff_UpdatedFieldsModification,
        };
        unsafe {
            instance.get(|| {
                ObjectDiff_UpdatedFieldsModification {
                    updated: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .ObjectDiff.UpdatedFieldsModification.UpdatedEntry updated = 1;

    pub fn clear_updated(&mut self) {
        self.updated.clear();
    }

    // Param is passed by value, moved
    pub fn set_updated(&mut self, v: ::protobuf::RepeatedField<ObjectDiff_UpdatedFieldsModification_UpdatedEntry>) {
        self.updated = v;
    }

    // Mutable pointer to the field.
    pub fn mut_updated(&mut self) -> &mut ::protobuf::RepeatedField<ObjectDiff_UpdatedFieldsModification_UpdatedEntry> {
        &mut self.updated
    }

    // Take field
    pub fn take_updated(&mut self) -> ::protobuf::RepeatedField<ObjectDiff_UpdatedFieldsModification_UpdatedEntry> {
        ::std::mem::replace(&mut self.updated, ::protobuf::RepeatedField::new())
    }

    pub fn get_updated(&self) -> &[ObjectDiff_UpdatedFieldsModification_UpdatedEntry] {
        &self.updated
    }
}

impl ::protobuf::Message for ObjectDiff_UpdatedFieldsModification {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.updated));
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
        for value in self.updated.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.updated.iter() {
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
        ::std::any::TypeId::of::<ObjectDiff_UpdatedFieldsModification>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ObjectDiff_UpdatedFieldsModification {
    fn new() -> ObjectDiff_UpdatedFieldsModification {
        ObjectDiff_UpdatedFieldsModification::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObjectDiff_UpdatedFieldsModification>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "updated",
                    ObjectDiff_UpdatedFieldsModification::get_updated,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObjectDiff_UpdatedFieldsModification>(
                    "ObjectDiff_UpdatedFieldsModification",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObjectDiff_UpdatedFieldsModification {
    fn clear(&mut self) {
        self.clear_updated();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ObjectDiff_UpdatedFieldsModification {
    fn eq(&self, other: &ObjectDiff_UpdatedFieldsModification) -> bool {
        self.updated == other.updated &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ObjectDiff_UpdatedFieldsModification {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ObjectDiff_UpdatedFieldsModification_UpdatedEntry {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObjectDiff_UpdatedFieldsModification_UpdatedEntry {}

impl ObjectDiff_UpdatedFieldsModification_UpdatedEntry {
    pub fn new() -> ObjectDiff_UpdatedFieldsModification_UpdatedEntry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObjectDiff_UpdatedFieldsModification_UpdatedEntry {
        static mut instance: ::protobuf::lazy::Lazy<ObjectDiff_UpdatedFieldsModification_UpdatedEntry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjectDiff_UpdatedFieldsModification_UpdatedEntry,
        };
        unsafe {
            instance.get(|| {
                ObjectDiff_UpdatedFieldsModification_UpdatedEntry {
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

impl ::protobuf::Message for ObjectDiff_UpdatedFieldsModification_UpdatedEntry {
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
        ::std::any::TypeId::of::<ObjectDiff_UpdatedFieldsModification_UpdatedEntry>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ObjectDiff_UpdatedFieldsModification_UpdatedEntry {
    fn new() -> ObjectDiff_UpdatedFieldsModification_UpdatedEntry {
        ObjectDiff_UpdatedFieldsModification_UpdatedEntry::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObjectDiff_UpdatedFieldsModification_UpdatedEntry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    ObjectDiff_UpdatedFieldsModification_UpdatedEntry::has_key,
                    ObjectDiff_UpdatedFieldsModification_UpdatedEntry::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    ObjectDiff_UpdatedFieldsModification_UpdatedEntry::has_value,
                    ObjectDiff_UpdatedFieldsModification_UpdatedEntry::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObjectDiff_UpdatedFieldsModification_UpdatedEntry>(
                    "ObjectDiff_UpdatedFieldsModification_UpdatedEntry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObjectDiff_UpdatedFieldsModification_UpdatedEntry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ObjectDiff_UpdatedFieldsModification_UpdatedEntry {
    fn eq(&self, other: &ObjectDiff_UpdatedFieldsModification_UpdatedEntry) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ObjectDiff_UpdatedFieldsModification_UpdatedEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ObjectDiff_RemovedFieldsModification {
    // message fields
    removed_field: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObjectDiff_RemovedFieldsModification {}

impl ObjectDiff_RemovedFieldsModification {
    pub fn new() -> ObjectDiff_RemovedFieldsModification {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObjectDiff_RemovedFieldsModification {
        static mut instance: ::protobuf::lazy::Lazy<ObjectDiff_RemovedFieldsModification> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjectDiff_RemovedFieldsModification,
        };
        unsafe {
            instance.get(|| {
                ObjectDiff_RemovedFieldsModification {
                    removed_field: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string removed_field = 1;

    pub fn clear_removed_field(&mut self) {
        self.removed_field.clear();
    }

    // Param is passed by value, moved
    pub fn set_removed_field(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.removed_field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_removed_field(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.removed_field
    }

    // Take field
    pub fn take_removed_field(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.removed_field, ::protobuf::RepeatedField::new())
    }

    pub fn get_removed_field(&self) -> &[::std::string::String] {
        &self.removed_field
    }
}

impl ::protobuf::Message for ObjectDiff_RemovedFieldsModification {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.removed_field));
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
        for value in self.removed_field.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.removed_field.iter() {
            try!(os.write_string(1, &v));
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
        ::std::any::TypeId::of::<ObjectDiff_RemovedFieldsModification>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ObjectDiff_RemovedFieldsModification {
    fn new() -> ObjectDiff_RemovedFieldsModification {
        ObjectDiff_RemovedFieldsModification::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObjectDiff_RemovedFieldsModification>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "removed_field",
                    ObjectDiff_RemovedFieldsModification::get_removed_field,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObjectDiff_RemovedFieldsModification>(
                    "ObjectDiff_RemovedFieldsModification",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObjectDiff_RemovedFieldsModification {
    fn clear(&mut self) {
        self.clear_removed_field();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ObjectDiff_RemovedFieldsModification {
    fn eq(&self, other: &ObjectDiff_RemovedFieldsModification) -> bool {
        self.removed_field == other.removed_field &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ObjectDiff_RemovedFieldsModification {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct WorldStateDiff {
    // message fields
    object_change: ::protobuf::RepeatedField<ObjectDiff>,
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

    // repeated .ObjectDiff object_change = 1;

    pub fn clear_object_change(&mut self) {
        self.object_change.clear();
    }

    // Param is passed by value, moved
    pub fn set_object_change(&mut self, v: ::protobuf::RepeatedField<ObjectDiff>) {
        self.object_change = v;
    }

    // Mutable pointer to the field.
    pub fn mut_object_change(&mut self) -> &mut ::protobuf::RepeatedField<ObjectDiff> {
        &mut self.object_change
    }

    // Take field
    pub fn take_object_change(&mut self) -> ::protobuf::RepeatedField<ObjectDiff> {
        ::std::mem::replace(&mut self.object_change, ::protobuf::RepeatedField::new())
    }

    pub fn get_object_change(&self) -> &[ObjectDiff] {
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
pub struct ServerWorldStateDiff {
    // message fields
    expected_client_version: ::std::option::Option<u64>,
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
                    expected_client_version: ::std::option::Option::None,
                    new_version: ::std::option::Option::None,
                    changes: ::protobuf::SingularPtrField::none(),
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

    // optional uint64 new_version = 2;

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
                    self.expected_client_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.new_version = ::std::option::Option::Some(tmp);
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
        for value in self.expected_client_version.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.new_version.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.expected_client_version {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.new_version {
            try!(os.write_uint64(2, v));
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
                    "expected_client_version",
                    ServerWorldStateDiff::has_expected_client_version,
                    ServerWorldStateDiff::get_expected_client_version,
                ));
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
        self.clear_expected_client_version();
        self.clear_new_version();
        self.clear_changes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ServerWorldStateDiff {
    fn eq(&self, other: &ServerWorldStateDiff) -> bool {
        self.expected_client_version == other.expected_client_version &&
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
    0x0a, 0x0c, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x2f,
    0x0a, 0x10, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x6e,
    0x66, 0x6f, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x22,
    0x40, 0x0a, 0x0a, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x32, 0x0a,
    0x0b, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x11, 0x2e, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x0a, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x49, 0x6e, 0x66,
    0x6f, 0x22, 0x0c, 0x0a, 0x0a, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x22,
    0x91, 0x01, 0x0a, 0x06, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x12, 0x1b, 0x0a, 0x09, 0x6f, 0x62,
    0x6a, 0x65, 0x63, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x6f,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x49, 0x64, 0x12, 0x2e, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f,
    0x61, 0x64, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63,
    0x74, 0x2e, 0x50, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x07,
    0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x1a, 0x3a, 0x0a, 0x0c, 0x50, 0x61, 0x79, 0x6c, 0x6f,
    0x61, 0x64, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a,
    0x02, 0x38, 0x01, 0x22, 0x89, 0x08, 0x0a, 0x0a, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x44, 0x69,
    0x66, 0x66, 0x12, 0x1b, 0x0a, 0x09, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x49, 0x64, 0x12,
    0x42, 0x0a, 0x0a, 0x6e, 0x65, 0x77, 0x5f, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x44, 0x69, 0x66, 0x66,
    0x2e, 0x4e, 0x65, 0x77, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x09, 0x6e, 0x65, 0x77, 0x4f, 0x62, 0x6a,
    0x65, 0x63, 0x74, 0x12, 0x4d, 0x0a, 0x0e, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x5f, 0x6f,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x4f, 0x62,
    0x6a, 0x65, 0x63, 0x74, 0x44, 0x69, 0x66, 0x66, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64,
    0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x4d, 0x6f, 0x64, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x48, 0x00, 0x52, 0x0d, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x4f, 0x62, 0x6a, 0x65,
    0x63, 0x74, 0x12, 0x48, 0x0a, 0x0c, 0x61, 0x64, 0x64, 0x65, 0x64, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63,
    0x74, 0x44, 0x69, 0x66, 0x66, 0x2e, 0x41, 0x64, 0x64, 0x65, 0x64, 0x46, 0x69, 0x65, 0x6c, 0x64,
    0x73, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52,
    0x0b, 0x61, 0x64, 0x64, 0x65, 0x64, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x12, 0x4e, 0x0a, 0x0e,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x44, 0x69, 0x66,
    0x66, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x4d,
    0x6f, 0x64, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x0d, 0x75,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x12, 0x4e, 0x0a, 0x0e,
    0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x44, 0x69, 0x66,
    0x66, 0x2e, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x4d,
    0x6f, 0x64, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x00, 0x52, 0x0d, 0x72,
    0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x1a, 0xae, 0x01, 0x0a,
    0x15, 0x4e, 0x65, 0x77, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x55, 0x0a, 0x0c, 0x66, 0x75, 0x6c, 0x6c, 0x5f, 0x70,
    0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x4f,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x44, 0x69, 0x66, 0x66, 0x2e, 0x4e, 0x65, 0x77, 0x4f, 0x62, 0x6a,
    0x65, 0x63, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e,
    0x46, 0x75, 0x6c, 0x6c, 0x50, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x45, 0x6e, 0x74, 0x72, 0x79,
    0x52, 0x0b, 0x66, 0x75, 0x6c, 0x6c, 0x50, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x1a, 0x3e, 0x0a,
    0x10, 0x46, 0x75, 0x6c, 0x6c, 0x50, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x45, 0x6e, 0x74, 0x72,
    0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03,
    0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x1a, 0x1a, 0x0a,
    0x18, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x4d, 0x6f,
    0x64, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x1a, 0x99, 0x01, 0x0a, 0x17, 0x41, 0x64,
    0x64, 0x65, 0x64, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x44, 0x0a, 0x05, 0x61, 0x64, 0x64, 0x65, 0x64, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x44, 0x69, 0x66,
    0x66, 0x2e, 0x41, 0x64, 0x64, 0x65, 0x64, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x4d, 0x6f, 0x64,
    0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x41, 0x64, 0x64, 0x65, 0x64, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x52, 0x05, 0x61, 0x64, 0x64, 0x65, 0x64, 0x1a, 0x38, 0x0a, 0x0a, 0x41,
    0x64, 0x64, 0x65, 0x64, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x3a, 0x02, 0x38, 0x01, 0x1a, 0xa5, 0x01, 0x0a, 0x19, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x64, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x12, 0x4c, 0x0a, 0x07, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x44, 0x69, 0x66,
    0x66, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x4d,
    0x6f, 0x64, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x64, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x07, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x64, 0x1a, 0x3a, 0x0a, 0x0c, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x64, 0x45, 0x6e, 0x74, 0x72,
    0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03,
    0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x1a, 0x40, 0x0a,
    0x19, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x4d, 0x6f,
    0x64, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x23, 0x0a, 0x0d, 0x72, 0x65,
    0x6d, 0x6f, 0x76, 0x65, 0x64, 0x5f, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x09, 0x52, 0x0c, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x42,
    0x0e, 0x0a, 0x0c, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x22,
    0x42, 0x0a, 0x0e, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x44, 0x69, 0x66,
    0x66, 0x12, 0x30, 0x0a, 0x0d, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x5f, 0x63, 0x68, 0x61, 0x6e,
    0x67, 0x65, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x4f, 0x62, 0x6a, 0x65, 0x63,
    0x74, 0x44, 0x69, 0x66, 0x66, 0x52, 0x0c, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x43, 0x68, 0x61,
    0x6e, 0x67, 0x65, 0x22, 0x9a, 0x01, 0x0a, 0x14, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x57, 0x6f,
    0x72, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x44, 0x69, 0x66, 0x66, 0x12, 0x36, 0x0a, 0x17,
    0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x15, 0x65,
    0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x12, 0x1f, 0x0a, 0x0b, 0x6e, 0x65, 0x77, 0x5f, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x6e, 0x65, 0x77, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x29, 0x0a, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x44, 0x69, 0x66, 0x66, 0x52, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73,
    0x22, 0x77, 0x0a, 0x05, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x19, 0x0a, 0x08, 0x65, 0x76, 0x65,
    0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x65, 0x76, 0x65,
    0x6e, 0x74, 0x49, 0x64, 0x12, 0x28, 0x0a, 0x10, 0x61, 0x63, 0x74, 0x69, 0x6e, 0x67, 0x5f, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e,
    0x61, 0x63, 0x74, 0x69, 0x6e, 0x67, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x29,
    0x0a, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0f, 0x2e, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x44, 0x69, 0x66, 0x66,
    0x52, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x4a, 0xb1, 0x1c, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x67, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x02, 0x08, 0x18, 0x0a, 0x18, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x04, 0x02, 0x17, 0x1a, 0x0b, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x04, 0x02, 0x02, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x04, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x04, 0x15, 0x16, 0x0a, 0x9f, 0x01, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x09, 0x00, 0x0c, 0x01, 0x1a, 0x92, 0x01, 0x20, 0x41, 0x6c, 0x6c, 0x20, 0x69, 0x6e,
    0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20,
    0x62, 0x6f, 0x74, 0x68, 0x20, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x20, 0x69, 0x6e, 0x66, 0x6f,
    0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x69, 0x6e, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73,
    0x20, 0x70, 0x72, 0x69, 0x76, 0x69, 0x6c, 0x65, 0x67, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x6a,
    0x75, 0x73, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x09, 0x08, 0x12, 0x0a, 0x5c, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x0b, 0x02, 0x23, 0x1a, 0x4f, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e,
    0x20, 0x49, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x62, 0x6f,
    0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x0b, 0x02, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0b,
    0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x13, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x21, 0x22, 0x0a, 0x53,
    0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x0f, 0x00, 0x10, 0x01, 0x1a, 0x47, 0x20, 0x41, 0x6c, 0x6c,
    0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x62, 0x6f,
    0x75, 0x74, 0x20, 0x61, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x69, 0x73, 0x20, 0x73, 0x68, 0x61, 0x72, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x12, 0x0a,
    0x91, 0x01, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x15, 0x00, 0x1c, 0x01, 0x1a, 0x84, 0x01, 0x20,
    0x41, 0x20, 0x73, 0x68, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74,
    0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x69, 0x6e, 0x20, 0x64, 0x61, 0x74,
    0x61, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6d, 0x61, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x75, 0x70,
    0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x20, 0x54,
    0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x76, 0x65, 0x6e, 0x69,
    0x65, 0x6e, 0x63, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x72, 0x65,
    0x61, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x73, 0x74, 0x61, 0x74,
    0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x15, 0x08, 0x0e, 0x0a,
    0x18, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x17, 0x02, 0x17, 0x1a, 0x0b, 0x20, 0x52,
    0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x17, 0x02, 0x15, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x17, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x17, 0x09, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x17,
    0x15, 0x16, 0x0a, 0x97, 0x01, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x1b, 0x02, 0x21,
    0x1a, 0x89, 0x01, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x63, 0x74, 0x75, 0x61,
    0x6c, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x62,
    0x6a, 0x65, 0x63, 0x74, 0x2e, 0x20, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x63, 0x61, 0x6e,
    0x20, 0x62, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2c,
    0x20, 0x73, 0x6f, 0x20, 0x69, 0x74, 0x27, 0x73, 0x0a, 0x20, 0x75, 0x70, 0x20, 0x74, 0x6f, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x64, 0x65, 0x76, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x6e, 0x66,
    0x6f, 0x72, 0x63, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x04, 0x1b, 0x02, 0x17, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1b, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x1b, 0x15, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x1b, 0x1f, 0x20, 0x0a, 0x73, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x21, 0x00, 0x45,
    0x01, 0x1a, 0x67, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64,
    0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x65,
    0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e,
    0x61, 0x6c, 0x6c, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e,
    0x74, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67, 0x6c,
    0x65, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04,
    0x01, 0x12, 0x03, 0x21, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x00, 0x12, 0x04,
    0x22, 0x02, 0x25, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x00, 0x01, 0x12, 0x03, 0x22,
    0x0a, 0x1f, 0x0a, 0x1a, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x24, 0x04,
    0x28, 0x1a, 0x0b, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x24, 0x04, 0x22, 0x21, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x24, 0x04, 0x16, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x17, 0x23, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x26, 0x27, 0x0a,
    0x62, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x01, 0x12, 0x04, 0x27, 0x02, 0x29, 0x03, 0x22, 0x54, 0x20,
    0x4e, 0x6f, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x65, 0x64,
    0x20, 0x79, 0x65, 0x74, 0x2c, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66,
    0x61, 0x63, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73,
    0x20, 0x70, 0x6f, 0x70, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x20, 0x61, 0x20, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x01, 0x01, 0x12, 0x03, 0x27, 0x0a,
    0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x02, 0x12, 0x04, 0x2b, 0x02, 0x2e, 0x03, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x0a, 0x21, 0x0a, 0x1a, 0x0a,
    0x06, 0x04, 0x04, 0x03, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x04, 0x21, 0x1a, 0x0b, 0x20, 0x52,
    0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x03,
    0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x2d, 0x04, 0x2b, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04,
    0x03, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2d, 0x04, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04,
    0x03, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x17, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04,
    0x03, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04,
    0x03, 0x03, 0x12, 0x04, 0x30, 0x02, 0x33, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x03,
    0x01, 0x12, 0x03, 0x30, 0x0a, 0x23, 0x0a, 0x1a, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x03, 0x02, 0x00,
    0x12, 0x03, 0x32, 0x04, 0x23, 0x1a, 0x0b, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64,
    0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0x32,
    0x04, 0x30, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x32, 0x04, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x32, 0x17, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x32, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x04, 0x12, 0x04, 0x35, 0x02, 0x38,
    0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x04, 0x01, 0x12, 0x03, 0x35, 0x0a, 0x23, 0x0a,
    0x2a, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x04, 0x02, 0x00, 0x12, 0x03, 0x37, 0x04, 0x26, 0x1a, 0x1b,
    0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20,
    0x6e, 0x6f, 0x6e, 0x2d, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x03, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x37, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x03, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x37, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x03, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x14, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x04, 0x03, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x24, 0x25, 0x0a, 0x18, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x00, 0x12, 0x03, 0x3b, 0x02, 0x17, 0x1a, 0x0b, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69,
    0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x3b, 0x02, 0x38, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3b,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b, 0x09, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x15, 0x16, 0x0a, 0x2f,
    0x0a, 0x04, 0x04, 0x04, 0x08, 0x00, 0x12, 0x04, 0x3e, 0x02, 0x44, 0x03, 0x1a, 0x21, 0x20, 0x4f,
    0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x73, 0x65, 0x20, 0x63, 0x68, 0x61, 0x6e,
    0x67, 0x65, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x79, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x08, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x08, 0x14, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x06, 0x12, 0x03, 0x3f, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x3f, 0x1a, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x3f, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x40, 0x04,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x40, 0x04, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x40, 0x1d, 0x2b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x40, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x03, 0x12, 0x03, 0x41, 0x04, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03,
    0x06, 0x12, 0x03, 0x41, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x41, 0x1c, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x41,
    0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x42, 0x04, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x06, 0x12, 0x03, 0x42, 0x04, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12, 0x03, 0x42, 0x1e, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x04, 0x03, 0x12, 0x03, 0x42, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x05, 0x12, 0x03, 0x43, 0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x06, 0x12,
    0x03, 0x43, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x43,
    0x1e, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x03, 0x12, 0x03, 0x43, 0x2f, 0x30,
    0x0a, 0x6c, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x49, 0x00, 0x4b, 0x01, 0x1a, 0x60, 0x20, 0x52,
    0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x20, 0x73,
    0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x2e, 0x20, 0x43, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64,
    0x20, 0x62, 0x79, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x20, 0x6f, 0x72, 0x0a, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x49, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x00, 0x12, 0x03, 0x4a, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x4a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x4a, 0x11, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4a, 0x1c,
    0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4a, 0x2c, 0x2d, 0x0a,
    0x6b, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x4e, 0x00, 0x5a, 0x01, 0x1a, 0x5f, 0x20, 0x52, 0x65,
    0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x75, 0x74, 0x68,
    0x6f, 0x72, 0x69, 0x74, 0x61, 0x74, 0x69, 0x76, 0x65, 0x20, 0x76, 0x69, 0x65, 0x77, 0x20, 0x6f,
    0x66, 0x20, 0x61, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x2c, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x6f, 0x6e,
    0x65, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20,
    0x74, 0x6f, 0x20, 0x61, 0x6e, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x06, 0x01, 0x12, 0x03, 0x4e, 0x08, 0x1c, 0x0a, 0xd3, 0x01, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x00, 0x12, 0x03, 0x52, 0x02, 0x25, 0x1a, 0xc5, 0x01, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72,
    0x65, 0x64, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x62, 0x65, 0x6c,
    0x69, 0x65, 0x76, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x20, 0x68, 0x61, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x0a, 0x20, 0x72, 0x65, 0x2d, 0x63, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x67,
    0x72, 0x65, 0x61, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x28, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20, 0x73, 0x6f, 0x6d,
    0x65, 0x74, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x65, 0x6e, 0x74, 0x20, 0x68, 0x6f, 0x72, 0x72,
    0x69, 0x62, 0x6c, 0x79, 0x0a, 0x20, 0x77, 0x72, 0x6f, 0x6e, 0x67, 0x29, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x04, 0x52, 0x02, 0x4e, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x52, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x09, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x52, 0x23, 0x24, 0x0a, 0x9e, 0x01, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01,
    0x12, 0x03, 0x56, 0x02, 0x19, 0x1a, 0x90, 0x01, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65,
    0x64, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20,
    0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x69, 0x66, 0x66, 0x20, 0x69,
    0x73, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x6c, 0x65, 0x73, 0x73, 0x20, 0x74, 0x68, 0x61, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x64, 0x69, 0x66, 0x66, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x69,
    0x67, 0x6e, 0x6f, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x56, 0x02, 0x52, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x56, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x56, 0x09, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x56, 0x17,
    0x18, 0x0a, 0x40, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x59, 0x02, 0x1d, 0x1a, 0x33,
    0x20, 0x52, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x20, 0x74, 0x6f, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x79, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x6c,
    0x79, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x04, 0x59, 0x02,
    0x56, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03, 0x59, 0x02, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x59, 0x11, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x59, 0x1b, 0x1c, 0x0a, 0x58, 0x0a, 0x02,
    0x04, 0x07, 0x12, 0x04, 0x5d, 0x00, 0x67, 0x01, 0x1a, 0x4c, 0x20, 0x41, 0x20, 0x73, 0x69, 0x6e,
    0x67, 0x6c, 0x65, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x66,
    0x72, 0x6f, 0x6d, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x2c, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6d,
    0x6f, 0x64, 0x69, 0x66, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x20, 0x73,
    0x74, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x5d,
    0x08, 0x0d, 0x0a, 0x3f, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x5f, 0x02, 0x16, 0x1a,
    0x32, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x41, 0x20, 0x72, 0x61,
    0x6e, 0x64, 0x6f, 0x6d, 0x20, 0x75, 0x75, 0x69, 0x64, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61,
    0x74, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x5f, 0x02,
    0x5d, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5f, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5f, 0x09, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5f, 0x14, 0x15, 0x0a, 0x8e, 0x01, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x63, 0x02, 0x1e, 0x1a, 0x80, 0x01, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x67, 0x65,
    0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x76, 0x65, 0x6e,
    0x74, 0x2e, 0x0a, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x61, 0x65, 0x69, 0x64, 0x65, 0x6c, 0x73,
    0x6f, 0x6e, 0x29, 0x3a, 0x20, 0x46, 0x69, 0x67, 0x75, 0x72, 0x65, 0x20, 0x6f, 0x75, 0x74, 0x20,
    0x68, 0x6f, 0x77, 0x20, 0x77, 0x65, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74,
    0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x28,
    0x69, 0x2e, 0x65, 0x2e, 0x20, 0x61, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x70, 0x68,
    0x79, 0x73, 0x69, 0x63, 0x73, 0x20, 0x6c, 0x6f, 0x6f, 0x70, 0x29, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0x63, 0x02, 0x5f, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x63, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x63, 0x09, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x63, 0x1c, 0x1d, 0x0a, 0x3c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03,
    0x66, 0x02, 0x1d, 0x1a, 0x2f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x20, 0x77, 0x61, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x70, 0x70,
    0x6c, 0x79, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x04, 0x66,
    0x02, 0x63, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x06, 0x12, 0x03, 0x66, 0x02,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x66, 0x11, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x66, 0x1b, 0x1c, 0x62, 0x06, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33,
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
