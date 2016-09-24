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
pub struct ClientServerMessage {
    // message fields
    client_info: ::protobuf::SingularPtrField<super::common::ClientInfo>,
    session_token: ::protobuf::SingularField<::std::string::String>,
    // message oneof groups
    payload: ::std::option::Option<ClientServerMessage_oneof_payload>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientServerMessage {}

#[derive(Clone,PartialEq)]
pub enum ClientServerMessage_oneof_payload {
    connect(super::connect::ClientServerConnect),
    disconnect(super::disconnect::ClientServerDisconnect),
    ack_updates(super::ack_updates::ClientServerAckUpdates),
    request_updates(super::request_updates::ClientServerRequestUpdates),
    send_new_events(super::send_new_events::ClientServerSendNewEvents),
}

impl ClientServerMessage {
    pub fn new() -> ClientServerMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientServerMessage {
        static mut instance: ::protobuf::lazy::Lazy<ClientServerMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientServerMessage,
        };
        unsafe {
            instance.get(|| {
                ClientServerMessage {
                    client_info: ::protobuf::SingularPtrField::none(),
                    session_token: ::protobuf::SingularField::none(),
                    payload: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ClientInfo client_info = 1;

    pub fn clear_client_info(&mut self) {
        self.client_info.clear();
    }

    pub fn has_client_info(&self) -> bool {
        self.client_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_info(&mut self, v: super::common::ClientInfo) {
        self.client_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_client_info(&mut self) -> &mut super::common::ClientInfo {
        if self.client_info.is_none() {
            self.client_info.set_default();
        };
        self.client_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_client_info(&mut self) -> super::common::ClientInfo {
        self.client_info.take().unwrap_or_else(|| super::common::ClientInfo::new())
    }

    pub fn get_client_info(&self) -> &super::common::ClientInfo {
        self.client_info.as_ref().unwrap_or_else(|| super::common::ClientInfo::default_instance())
    }

    // optional string session_token = 2;

    pub fn clear_session_token(&mut self) {
        self.session_token.clear();
    }

    pub fn has_session_token(&self) -> bool {
        self.session_token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_session_token(&mut self, v: ::std::string::String) {
        self.session_token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_session_token(&mut self) -> &mut ::std::string::String {
        if self.session_token.is_none() {
            self.session_token.set_default();
        };
        self.session_token.as_mut().unwrap()
    }

    // Take field
    pub fn take_session_token(&mut self) -> ::std::string::String {
        self.session_token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_session_token(&self) -> &str {
        match self.session_token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .ClientServerConnect connect = 3;

    pub fn clear_connect(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_connect(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::connect(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_connect(&mut self, v: super::connect::ClientServerConnect) {
        self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::connect(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_connect(&mut self) -> &mut super::connect::ClientServerConnect {
        if let ::std::option::Option::Some(ClientServerMessage_oneof_payload::connect(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::connect(super::connect::ClientServerConnect::new()));
        }
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::connect(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_connect(&mut self) -> super::connect::ClientServerConnect {
        if self.has_connect() {
            match self.payload.take() {
                ::std::option::Option::Some(ClientServerMessage_oneof_payload::connect(v)) => v,
                _ => panic!(),
            }
        } else {
            super::connect::ClientServerConnect::new()
        }
    }

    pub fn get_connect(&self) -> &super::connect::ClientServerConnect {
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::connect(ref v)) => v,
            _ => super::connect::ClientServerConnect::default_instance(),
        }
    }

    // optional .ClientServerDisconnect disconnect = 4;

    pub fn clear_disconnect(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_disconnect(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::disconnect(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_disconnect(&mut self, v: super::disconnect::ClientServerDisconnect) {
        self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::disconnect(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_disconnect(&mut self) -> &mut super::disconnect::ClientServerDisconnect {
        if let ::std::option::Option::Some(ClientServerMessage_oneof_payload::disconnect(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::disconnect(super::disconnect::ClientServerDisconnect::new()));
        }
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::disconnect(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_disconnect(&mut self) -> super::disconnect::ClientServerDisconnect {
        if self.has_disconnect() {
            match self.payload.take() {
                ::std::option::Option::Some(ClientServerMessage_oneof_payload::disconnect(v)) => v,
                _ => panic!(),
            }
        } else {
            super::disconnect::ClientServerDisconnect::new()
        }
    }

    pub fn get_disconnect(&self) -> &super::disconnect::ClientServerDisconnect {
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::disconnect(ref v)) => v,
            _ => super::disconnect::ClientServerDisconnect::default_instance(),
        }
    }

    // optional .ClientServerAckUpdates ack_updates = 5;

    pub fn clear_ack_updates(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_ack_updates(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::ack_updates(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ack_updates(&mut self, v: super::ack_updates::ClientServerAckUpdates) {
        self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::ack_updates(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ack_updates(&mut self) -> &mut super::ack_updates::ClientServerAckUpdates {
        if let ::std::option::Option::Some(ClientServerMessage_oneof_payload::ack_updates(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::ack_updates(super::ack_updates::ClientServerAckUpdates::new()));
        }
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::ack_updates(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ack_updates(&mut self) -> super::ack_updates::ClientServerAckUpdates {
        if self.has_ack_updates() {
            match self.payload.take() {
                ::std::option::Option::Some(ClientServerMessage_oneof_payload::ack_updates(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ack_updates::ClientServerAckUpdates::new()
        }
    }

    pub fn get_ack_updates(&self) -> &super::ack_updates::ClientServerAckUpdates {
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::ack_updates(ref v)) => v,
            _ => super::ack_updates::ClientServerAckUpdates::default_instance(),
        }
    }

    // optional .ClientServerRequestUpdates request_updates = 6;

    pub fn clear_request_updates(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_request_updates(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::request_updates(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_request_updates(&mut self, v: super::request_updates::ClientServerRequestUpdates) {
        self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::request_updates(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_updates(&mut self) -> &mut super::request_updates::ClientServerRequestUpdates {
        if let ::std::option::Option::Some(ClientServerMessage_oneof_payload::request_updates(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::request_updates(super::request_updates::ClientServerRequestUpdates::new()));
        }
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::request_updates(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_request_updates(&mut self) -> super::request_updates::ClientServerRequestUpdates {
        if self.has_request_updates() {
            match self.payload.take() {
                ::std::option::Option::Some(ClientServerMessage_oneof_payload::request_updates(v)) => v,
                _ => panic!(),
            }
        } else {
            super::request_updates::ClientServerRequestUpdates::new()
        }
    }

    pub fn get_request_updates(&self) -> &super::request_updates::ClientServerRequestUpdates {
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::request_updates(ref v)) => v,
            _ => super::request_updates::ClientServerRequestUpdates::default_instance(),
        }
    }

    // optional .ClientServerSendNewEvents send_new_events = 7;

    pub fn clear_send_new_events(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_send_new_events(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::send_new_events(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_send_new_events(&mut self, v: super::send_new_events::ClientServerSendNewEvents) {
        self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::send_new_events(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_send_new_events(&mut self) -> &mut super::send_new_events::ClientServerSendNewEvents {
        if let ::std::option::Option::Some(ClientServerMessage_oneof_payload::send_new_events(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::send_new_events(super::send_new_events::ClientServerSendNewEvents::new()));
        }
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::send_new_events(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_send_new_events(&mut self) -> super::send_new_events::ClientServerSendNewEvents {
        if self.has_send_new_events() {
            match self.payload.take() {
                ::std::option::Option::Some(ClientServerMessage_oneof_payload::send_new_events(v)) => v,
                _ => panic!(),
            }
        } else {
            super::send_new_events::ClientServerSendNewEvents::new()
        }
    }

    pub fn get_send_new_events(&self) -> &super::send_new_events::ClientServerSendNewEvents {
        match self.payload {
            ::std::option::Option::Some(ClientServerMessage_oneof_payload::send_new_events(ref v)) => v,
            _ => super::send_new_events::ClientServerSendNewEvents::default_instance(),
        }
    }
}

impl ::protobuf::Message for ClientServerMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.client_info));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.session_token));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::connect(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::disconnect(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::ack_updates(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::request_updates(try!(is.read_message())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(ClientServerMessage_oneof_payload::send_new_events(try!(is.read_message())));
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
        for value in self.client_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.session_token.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &ClientServerMessage_oneof_payload::connect(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ClientServerMessage_oneof_payload::disconnect(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ClientServerMessage_oneof_payload::ack_updates(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ClientServerMessage_oneof_payload::request_updates(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ClientServerMessage_oneof_payload::send_new_events(ref v) => {
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
        if let Some(v) = self.client_info.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.session_token.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &ClientServerMessage_oneof_payload::connect(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ClientServerMessage_oneof_payload::disconnect(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ClientServerMessage_oneof_payload::ack_updates(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ClientServerMessage_oneof_payload::request_updates(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ClientServerMessage_oneof_payload::send_new_events(ref v) => {
                    try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<ClientServerMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClientServerMessage {
    fn new() -> ClientServerMessage {
        ClientServerMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientServerMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "client_info",
                    ClientServerMessage::has_client_info,
                    ClientServerMessage::get_client_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "session_token",
                    ClientServerMessage::has_session_token,
                    ClientServerMessage::get_session_token,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "connect",
                    ClientServerMessage::has_connect,
                    ClientServerMessage::get_connect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "disconnect",
                    ClientServerMessage::has_disconnect,
                    ClientServerMessage::get_disconnect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ack_updates",
                    ClientServerMessage::has_ack_updates,
                    ClientServerMessage::get_ack_updates,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "request_updates",
                    ClientServerMessage::has_request_updates,
                    ClientServerMessage::get_request_updates,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "send_new_events",
                    ClientServerMessage::has_send_new_events,
                    ClientServerMessage::get_send_new_events,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientServerMessage>(
                    "ClientServerMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientServerMessage {
    fn clear(&mut self) {
        self.clear_client_info();
        self.clear_session_token();
        self.clear_connect();
        self.clear_disconnect();
        self.clear_ack_updates();
        self.clear_request_updates();
        self.clear_send_new_events();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ClientServerMessage {
    fn eq(&self, other: &ClientServerMessage) -> bool {
        self.client_info == other.client_info &&
        self.session_token == other.session_token &&
        self.payload == other.payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClientServerMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ServerClientMessage {
    // message fields
    server_info: ::protobuf::SingularPtrField<super::common::ServerInfo>,
    // message oneof groups
    payload: ::std::option::Option<ServerClientMessage_oneof_payload>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerClientMessage {}

#[derive(Clone,PartialEq)]
pub enum ServerClientMessage_oneof_payload {
    world_update(super::world_update::ServerClientWorldUpdate),
}

impl ServerClientMessage {
    pub fn new() -> ServerClientMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerClientMessage {
        static mut instance: ::protobuf::lazy::Lazy<ServerClientMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerClientMessage,
        };
        unsafe {
            instance.get(|| {
                ServerClientMessage {
                    server_info: ::protobuf::SingularPtrField::none(),
                    payload: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ServerInfo server_info = 1;

    pub fn clear_server_info(&mut self) {
        self.server_info.clear();
    }

    pub fn has_server_info(&self) -> bool {
        self.server_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_info(&mut self, v: super::common::ServerInfo) {
        self.server_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_info(&mut self) -> &mut super::common::ServerInfo {
        if self.server_info.is_none() {
            self.server_info.set_default();
        };
        self.server_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_server_info(&mut self) -> super::common::ServerInfo {
        self.server_info.take().unwrap_or_else(|| super::common::ServerInfo::new())
    }

    pub fn get_server_info(&self) -> &super::common::ServerInfo {
        self.server_info.as_ref().unwrap_or_else(|| super::common::ServerInfo::default_instance())
    }

    // optional .ServerClientWorldUpdate world_update = 2;

    pub fn clear_world_update(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_world_update(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(ServerClientMessage_oneof_payload::world_update(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_world_update(&mut self, v: super::world_update::ServerClientWorldUpdate) {
        self.payload = ::std::option::Option::Some(ServerClientMessage_oneof_payload::world_update(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_world_update(&mut self) -> &mut super::world_update::ServerClientWorldUpdate {
        if let ::std::option::Option::Some(ServerClientMessage_oneof_payload::world_update(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(ServerClientMessage_oneof_payload::world_update(super::world_update::ServerClientWorldUpdate::new()));
        }
        match self.payload {
            ::std::option::Option::Some(ServerClientMessage_oneof_payload::world_update(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_world_update(&mut self) -> super::world_update::ServerClientWorldUpdate {
        if self.has_world_update() {
            match self.payload.take() {
                ::std::option::Option::Some(ServerClientMessage_oneof_payload::world_update(v)) => v,
                _ => panic!(),
            }
        } else {
            super::world_update::ServerClientWorldUpdate::new()
        }
    }

    pub fn get_world_update(&self) -> &super::world_update::ServerClientWorldUpdate {
        match self.payload {
            ::std::option::Option::Some(ServerClientMessage_oneof_payload::world_update(ref v)) => v,
            _ => super::world_update::ServerClientWorldUpdate::default_instance(),
        }
    }
}

impl ::protobuf::Message for ServerClientMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.server_info));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(ServerClientMessage_oneof_payload::world_update(try!(is.read_message())));
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
        for value in self.server_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &ServerClientMessage_oneof_payload::world_update(ref v) => {
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
        if let Some(v) = self.server_info.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &ServerClientMessage_oneof_payload::world_update(ref v) => {
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
        ::std::any::TypeId::of::<ServerClientMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ServerClientMessage {
    fn new() -> ServerClientMessage {
        ServerClientMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerClientMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "server_info",
                    ServerClientMessage::has_server_info,
                    ServerClientMessage::get_server_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "world_update",
                    ServerClientMessage::has_world_update,
                    ServerClientMessage::get_world_update,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServerClientMessage>(
                    "ServerClientMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerClientMessage {
    fn clear(&mut self) {
        self.clear_server_info();
        self.clear_world_update();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ServerClientMessage {
    fn eq(&self, other: &ServerClientMessage) -> bool {
        self.server_info == other.server_info &&
        self.payload == other.payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ServerClientMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x15, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65,
    0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0d, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x10, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x11, 0x61, 0x63, 0x6b, 0x5f, 0x75, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x15, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x5f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x15, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x6e, 0x65, 0x77, 0x5f, 0x65, 0x76, 0x65, 0x6e, 0x74,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x12, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x75,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xaa, 0x03, 0x0a, 0x13,
    0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x12, 0x2c, 0x0a, 0x0b, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x6e,
    0x66, 0x6f, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x0a, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x66,
    0x6f, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x6f, 0x6b,
    0x65, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f,
    0x6e, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x30, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63,
    0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x48, 0x00, 0x52,
    0x07, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x12, 0x39, 0x0a, 0x0a, 0x64, 0x69, 0x73, 0x63,
    0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x43,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x44, 0x69, 0x73, 0x63, 0x6f,
    0x6e, 0x6e, 0x65, 0x63, 0x74, 0x48, 0x00, 0x52, 0x0a, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x12, 0x3a, 0x0a, 0x0b, 0x61, 0x63, 0x6b, 0x5f, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x41, 0x63, 0x6b, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x73, 0x48, 0x00, 0x52, 0x0a, 0x61, 0x63, 0x6b, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x12,
    0x46, 0x0a, 0x0f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x55, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x73, 0x48, 0x00, 0x52, 0x0e, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x12, 0x44, 0x0a, 0x0f, 0x73, 0x65, 0x6e, 0x64, 0x5f,
    0x6e, 0x65, 0x77, 0x5f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1a, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x53,
    0x65, 0x6e, 0x64, 0x4e, 0x65, 0x77, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x48, 0x00, 0x52, 0x0d,
    0x73, 0x65, 0x6e, 0x64, 0x4e, 0x65, 0x77, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x42, 0x09, 0x0a,
    0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x22, 0x8d, 0x01, 0x0a, 0x13, 0x53, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x12, 0x2c, 0x0a, 0x0b, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x49, 0x6e,
    0x66, 0x6f, 0x52, 0x0a, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x3d,
    0x0a, 0x0c, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x48, 0x00,
    0x52, 0x0b, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x42, 0x09, 0x0a,
    0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x4a, 0x8b, 0x0a, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x27, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x02, 0x07, 0x15, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x03, 0x07, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x04, 0x07, 0x19, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x05, 0x07, 0x1a, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x06, 0x07, 0x1e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x05, 0x12, 0x03, 0x07, 0x07, 0x1e, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x06, 0x12, 0x03, 0x08, 0x07, 0x1b, 0x0a, 0x43, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x0b, 0x00, 0x1b, 0x01, 0x1a, 0x37, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e,
    0x73, 0x20, 0x61, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x62, 0x65, 0x74, 0x77,
    0x65, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x1b, 0x0a, 0x53, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x02, 0x1d, 0x1a, 0x46, 0x20, 0x41, 0x6c, 0x6c, 0x20, 0x69,
    0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x6b,
    0x6e, 0x6f, 0x77, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x2e, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0d, 0x02, 0x0b, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0d, 0x02, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x1b, 0x1c, 0x0a, 0x85, 0x01, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x11, 0x02, 0x1b, 0x1a, 0x78, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69,
    0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x74,
    0x6f, 0x6b, 0x65, 0x6e, 0x2e, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x6f,
    0x6e, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x6f,
    0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x27, 0x63, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x27, 0x2c, 0x0a, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x69, 0x73, 0x20, 0x77,
    0x68, 0x65, 0x6e, 0x20, 0x69, 0x74, 0x27, 0x73, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x11, 0x02, 0x0d, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x11, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x11, 0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x11, 0x19, 0x1a, 0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x00,
    0x08, 0x00, 0x12, 0x04, 0x14, 0x02, 0x1a, 0x03, 0x1a, 0x1d, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70,
    0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x01,
    0x12, 0x03, 0x14, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x15,
    0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x15, 0x04, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x15, 0x18, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x15, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x16, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x16, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x16, 0x1b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x16, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x17, 0x04, 0x2b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x17, 0x04, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x17, 0x1b, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x17, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x05, 0x12, 0x03, 0x18, 0x04, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06,
    0x12, 0x03, 0x18, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x18, 0x1f, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x18, 0x31,
    0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x19, 0x04, 0x32, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x19, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x19, 0x1e, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x03, 0x12, 0x03, 0x19, 0x30, 0x31, 0x0a, 0x90, 0x01, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x1f, 0x00, 0x27, 0x01, 0x1a, 0x83, 0x01, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x65,
    0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72,
    0x20, 0x61, 0x64, 0x20, 0x61, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x65,
    0x64, 0x2f, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x73, 0x2e, 0x0a, 0x20, 0x43, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x6e,
    0x74, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x75, 0x64,
    0x70, 0x20, 0x6f, 0x72, 0x20, 0x74, 0x63, 0x70, 0x2c, 0x20, 0x64, 0x65, 0x70, 0x65, 0x6e, 0x64,
    0x69, 0x6e, 0x67, 0x20, 0x6f, 0x6e, 0x20, 0x68, 0x6f, 0x77, 0x20, 0x63, 0x72, 0x69, 0x74, 0x69,
    0x63, 0x61, 0x6c, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x1b, 0x0a, 0x51, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x03, 0x21, 0x02, 0x1d, 0x1a, 0x44, 0x20, 0x41, 0x6c, 0x6c, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74,
    0x65, 0x64, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x21, 0x02, 0x1f, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x21, 0x02, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x21, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x21, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x08, 0x00, 0x12, 0x04, 0x23, 0x02, 0x26,
    0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x08, 0x00, 0x01, 0x12, 0x03, 0x23, 0x08, 0x0f, 0x0a,
    0x4b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x25, 0x04, 0x2d, 0x1a, 0x3e, 0x20, 0x53,
    0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x75, 0x64, 0x70, 0x20, 0x74, 0x6f, 0x20, 0x6e, 0x6f,
    0x74, 0x69, 0x66, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20,
    0x6f, 0x66, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x25, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x25, 0x1c, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x25, 0x2b, 0x2c, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
