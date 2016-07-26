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
pub struct RequestWrapper {
    // message fields
    session_token: ::protobuf::SingularField<::std::string::String>,
    client_id: ::protobuf::SingularField<::std::string::String>,
    // message oneof groups
    payload: ::std::option::Option<RequestWrapper_oneof_payload>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestWrapper {}

#[derive(Clone,PartialEq)]
pub enum RequestWrapper_oneof_payload {
    connect(super::connect::ConnectRequest),
    disconnect(super::disconnect::DisconnectRequest),
    ack_updates(super::ack_updates::AckUpdatesRequest),
    request_updates(super::request_updates::RequestUpdatesRequest),
    send_new_events(super::send_new_events::SendNewEventsRequest),
}

impl RequestWrapper {
    pub fn new() -> RequestWrapper {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestWrapper {
        static mut instance: ::protobuf::lazy::Lazy<RequestWrapper> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestWrapper,
        };
        unsafe {
            instance.get(|| {
                RequestWrapper {
                    session_token: ::protobuf::SingularField::none(),
                    client_id: ::protobuf::SingularField::none(),
                    payload: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string session_token = 1;

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

    // optional string client_id = 2;

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

    // optional .ConnectRequest connect = 3;

    pub fn clear_connect(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_connect(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::connect(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_connect(&mut self, v: super::connect::ConnectRequest) {
        self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::connect(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_connect(&mut self) -> &mut super::connect::ConnectRequest {
        if let ::std::option::Option::Some(RequestWrapper_oneof_payload::connect(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::connect(super::connect::ConnectRequest::new()));
        }
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::connect(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_connect(&mut self) -> super::connect::ConnectRequest {
        if self.has_connect() {
            match self.payload.take() {
                ::std::option::Option::Some(RequestWrapper_oneof_payload::connect(v)) => v,
                _ => panic!(),
            }
        } else {
            super::connect::ConnectRequest::new()
        }
    }

    pub fn get_connect(&self) -> &super::connect::ConnectRequest {
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::connect(ref v)) => v,
            _ => super::connect::ConnectRequest::default_instance(),
        }
    }

    // optional .DisconnectRequest disconnect = 4;

    pub fn clear_disconnect(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_disconnect(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::disconnect(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_disconnect(&mut self, v: super::disconnect::DisconnectRequest) {
        self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::disconnect(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_disconnect(&mut self) -> &mut super::disconnect::DisconnectRequest {
        if let ::std::option::Option::Some(RequestWrapper_oneof_payload::disconnect(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::disconnect(super::disconnect::DisconnectRequest::new()));
        }
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::disconnect(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_disconnect(&mut self) -> super::disconnect::DisconnectRequest {
        if self.has_disconnect() {
            match self.payload.take() {
                ::std::option::Option::Some(RequestWrapper_oneof_payload::disconnect(v)) => v,
                _ => panic!(),
            }
        } else {
            super::disconnect::DisconnectRequest::new()
        }
    }

    pub fn get_disconnect(&self) -> &super::disconnect::DisconnectRequest {
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::disconnect(ref v)) => v,
            _ => super::disconnect::DisconnectRequest::default_instance(),
        }
    }

    // optional .AckUpdatesRequest ack_updates = 5;

    pub fn clear_ack_updates(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_ack_updates(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::ack_updates(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ack_updates(&mut self, v: super::ack_updates::AckUpdatesRequest) {
        self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::ack_updates(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ack_updates(&mut self) -> &mut super::ack_updates::AckUpdatesRequest {
        if let ::std::option::Option::Some(RequestWrapper_oneof_payload::ack_updates(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::ack_updates(super::ack_updates::AckUpdatesRequest::new()));
        }
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::ack_updates(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ack_updates(&mut self) -> super::ack_updates::AckUpdatesRequest {
        if self.has_ack_updates() {
            match self.payload.take() {
                ::std::option::Option::Some(RequestWrapper_oneof_payload::ack_updates(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ack_updates::AckUpdatesRequest::new()
        }
    }

    pub fn get_ack_updates(&self) -> &super::ack_updates::AckUpdatesRequest {
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::ack_updates(ref v)) => v,
            _ => super::ack_updates::AckUpdatesRequest::default_instance(),
        }
    }

    // optional .RequestUpdatesRequest request_updates = 6;

    pub fn clear_request_updates(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_request_updates(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::request_updates(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_request_updates(&mut self, v: super::request_updates::RequestUpdatesRequest) {
        self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::request_updates(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_updates(&mut self) -> &mut super::request_updates::RequestUpdatesRequest {
        if let ::std::option::Option::Some(RequestWrapper_oneof_payload::request_updates(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::request_updates(super::request_updates::RequestUpdatesRequest::new()));
        }
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::request_updates(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_request_updates(&mut self) -> super::request_updates::RequestUpdatesRequest {
        if self.has_request_updates() {
            match self.payload.take() {
                ::std::option::Option::Some(RequestWrapper_oneof_payload::request_updates(v)) => v,
                _ => panic!(),
            }
        } else {
            super::request_updates::RequestUpdatesRequest::new()
        }
    }

    pub fn get_request_updates(&self) -> &super::request_updates::RequestUpdatesRequest {
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::request_updates(ref v)) => v,
            _ => super::request_updates::RequestUpdatesRequest::default_instance(),
        }
    }

    // optional .SendNewEventsRequest send_new_events = 7;

    pub fn clear_send_new_events(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_send_new_events(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::send_new_events(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_send_new_events(&mut self, v: super::send_new_events::SendNewEventsRequest) {
        self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::send_new_events(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_send_new_events(&mut self) -> &mut super::send_new_events::SendNewEventsRequest {
        if let ::std::option::Option::Some(RequestWrapper_oneof_payload::send_new_events(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::send_new_events(super::send_new_events::SendNewEventsRequest::new()));
        }
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::send_new_events(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_send_new_events(&mut self) -> super::send_new_events::SendNewEventsRequest {
        if self.has_send_new_events() {
            match self.payload.take() {
                ::std::option::Option::Some(RequestWrapper_oneof_payload::send_new_events(v)) => v,
                _ => panic!(),
            }
        } else {
            super::send_new_events::SendNewEventsRequest::new()
        }
    }

    pub fn get_send_new_events(&self) -> &super::send_new_events::SendNewEventsRequest {
        match self.payload {
            ::std::option::Option::Some(RequestWrapper_oneof_payload::send_new_events(ref v)) => v,
            _ => super::send_new_events::SendNewEventsRequest::default_instance(),
        }
    }
}

impl ::protobuf::Message for RequestWrapper {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.session_token));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.client_id));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::connect(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::disconnect(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::ack_updates(try!(is.read_message())));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::request_updates(try!(is.read_message())));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(RequestWrapper_oneof_payload::send_new_events(try!(is.read_message())));
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
        for value in self.session_token.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.client_id.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &RequestWrapper_oneof_payload::connect(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestWrapper_oneof_payload::disconnect(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestWrapper_oneof_payload::ack_updates(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestWrapper_oneof_payload::request_updates(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestWrapper_oneof_payload::send_new_events(ref v) => {
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
        if let Some(v) = self.session_token.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.client_id.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &RequestWrapper_oneof_payload::connect(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &RequestWrapper_oneof_payload::disconnect(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &RequestWrapper_oneof_payload::ack_updates(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &RequestWrapper_oneof_payload::request_updates(ref v) => {
                    try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &RequestWrapper_oneof_payload::send_new_events(ref v) => {
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
        ::std::any::TypeId::of::<RequestWrapper>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestWrapper {
    fn new() -> RequestWrapper {
        RequestWrapper::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestWrapper>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "session_token",
                    RequestWrapper::has_session_token,
                    RequestWrapper::get_session_token,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "client_id",
                    RequestWrapper::has_client_id,
                    RequestWrapper::get_client_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "connect",
                    RequestWrapper::has_connect,
                    RequestWrapper::get_connect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "disconnect",
                    RequestWrapper::has_disconnect,
                    RequestWrapper::get_disconnect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ack_updates",
                    RequestWrapper::has_ack_updates,
                    RequestWrapper::get_ack_updates,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "request_updates",
                    RequestWrapper::has_request_updates,
                    RequestWrapper::get_request_updates,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "send_new_events",
                    RequestWrapper::has_send_new_events,
                    RequestWrapper::get_send_new_events,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestWrapper>(
                    "RequestWrapper",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestWrapper {
    fn clear(&mut self) {
        self.clear_session_token();
        self.clear_client_id();
        self.clear_connect();
        self.clear_disconnect();
        self.clear_ack_updates();
        self.clear_request_updates();
        self.clear_send_new_events();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RequestWrapper {
    fn eq(&self, other: &RequestWrapper) -> bool {
        self.session_token == other.session_token &&
        self.client_id == other.client_id &&
        self.payload == other.payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RequestWrapper {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ResponseWrapper {
    // message oneof groups
    payload: ::std::option::Option<ResponseWrapper_oneof_payload>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseWrapper {}

#[derive(Clone,PartialEq)]
pub enum ResponseWrapper_oneof_payload {
    connect(super::connect::ConnectResponse),
    disconnect(super::disconnect::DisconnectResponse),
    ack_updates(super::ack_updates::AckUpdatesResponse),
    request_updates(super::request_updates::RequestUpdatesResponse),
    send_new_events(super::send_new_events::SendNewEventsResponse),
}

impl ResponseWrapper {
    pub fn new() -> ResponseWrapper {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseWrapper {
        static mut instance: ::protobuf::lazy::Lazy<ResponseWrapper> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseWrapper,
        };
        unsafe {
            instance.get(|| {
                ResponseWrapper {
                    payload: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .ConnectResponse connect = 1;

    pub fn clear_connect(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_connect(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::connect(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_connect(&mut self, v: super::connect::ConnectResponse) {
        self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::connect(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_connect(&mut self) -> &mut super::connect::ConnectResponse {
        if let ::std::option::Option::Some(ResponseWrapper_oneof_payload::connect(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::connect(super::connect::ConnectResponse::new()));
        }
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::connect(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_connect(&mut self) -> super::connect::ConnectResponse {
        if self.has_connect() {
            match self.payload.take() {
                ::std::option::Option::Some(ResponseWrapper_oneof_payload::connect(v)) => v,
                _ => panic!(),
            }
        } else {
            super::connect::ConnectResponse::new()
        }
    }

    pub fn get_connect(&self) -> &super::connect::ConnectResponse {
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::connect(ref v)) => v,
            _ => super::connect::ConnectResponse::default_instance(),
        }
    }

    // optional .DisconnectResponse disconnect = 2;

    pub fn clear_disconnect(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_disconnect(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::disconnect(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_disconnect(&mut self, v: super::disconnect::DisconnectResponse) {
        self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::disconnect(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_disconnect(&mut self) -> &mut super::disconnect::DisconnectResponse {
        if let ::std::option::Option::Some(ResponseWrapper_oneof_payload::disconnect(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::disconnect(super::disconnect::DisconnectResponse::new()));
        }
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::disconnect(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_disconnect(&mut self) -> super::disconnect::DisconnectResponse {
        if self.has_disconnect() {
            match self.payload.take() {
                ::std::option::Option::Some(ResponseWrapper_oneof_payload::disconnect(v)) => v,
                _ => panic!(),
            }
        } else {
            super::disconnect::DisconnectResponse::new()
        }
    }

    pub fn get_disconnect(&self) -> &super::disconnect::DisconnectResponse {
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::disconnect(ref v)) => v,
            _ => super::disconnect::DisconnectResponse::default_instance(),
        }
    }

    // optional .AckUpdatesResponse ack_updates = 3;

    pub fn clear_ack_updates(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_ack_updates(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::ack_updates(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ack_updates(&mut self, v: super::ack_updates::AckUpdatesResponse) {
        self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::ack_updates(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ack_updates(&mut self) -> &mut super::ack_updates::AckUpdatesResponse {
        if let ::std::option::Option::Some(ResponseWrapper_oneof_payload::ack_updates(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::ack_updates(super::ack_updates::AckUpdatesResponse::new()));
        }
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::ack_updates(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ack_updates(&mut self) -> super::ack_updates::AckUpdatesResponse {
        if self.has_ack_updates() {
            match self.payload.take() {
                ::std::option::Option::Some(ResponseWrapper_oneof_payload::ack_updates(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ack_updates::AckUpdatesResponse::new()
        }
    }

    pub fn get_ack_updates(&self) -> &super::ack_updates::AckUpdatesResponse {
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::ack_updates(ref v)) => v,
            _ => super::ack_updates::AckUpdatesResponse::default_instance(),
        }
    }

    // optional .RequestUpdatesResponse request_updates = 4;

    pub fn clear_request_updates(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_request_updates(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::request_updates(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_request_updates(&mut self, v: super::request_updates::RequestUpdatesResponse) {
        self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::request_updates(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request_updates(&mut self) -> &mut super::request_updates::RequestUpdatesResponse {
        if let ::std::option::Option::Some(ResponseWrapper_oneof_payload::request_updates(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::request_updates(super::request_updates::RequestUpdatesResponse::new()));
        }
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::request_updates(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_request_updates(&mut self) -> super::request_updates::RequestUpdatesResponse {
        if self.has_request_updates() {
            match self.payload.take() {
                ::std::option::Option::Some(ResponseWrapper_oneof_payload::request_updates(v)) => v,
                _ => panic!(),
            }
        } else {
            super::request_updates::RequestUpdatesResponse::new()
        }
    }

    pub fn get_request_updates(&self) -> &super::request_updates::RequestUpdatesResponse {
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::request_updates(ref v)) => v,
            _ => super::request_updates::RequestUpdatesResponse::default_instance(),
        }
    }

    // optional .SendNewEventsResponse send_new_events = 5;

    pub fn clear_send_new_events(&mut self) {
        self.payload = ::std::option::Option::None;
    }

    pub fn has_send_new_events(&self) -> bool {
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::send_new_events(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_send_new_events(&mut self, v: super::send_new_events::SendNewEventsResponse) {
        self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::send_new_events(v))
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_send_new_events(&mut self) -> &mut super::send_new_events::SendNewEventsResponse {
        if let ::std::option::Option::Some(ResponseWrapper_oneof_payload::send_new_events(_)) = self.payload {
        } else {
            self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::send_new_events(super::send_new_events::SendNewEventsResponse::new()));
        }
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::send_new_events(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_send_new_events(&mut self) -> super::send_new_events::SendNewEventsResponse {
        if self.has_send_new_events() {
            match self.payload.take() {
                ::std::option::Option::Some(ResponseWrapper_oneof_payload::send_new_events(v)) => v,
                _ => panic!(),
            }
        } else {
            super::send_new_events::SendNewEventsResponse::new()
        }
    }

    pub fn get_send_new_events(&self) -> &super::send_new_events::SendNewEventsResponse {
        match self.payload {
            ::std::option::Option::Some(ResponseWrapper_oneof_payload::send_new_events(ref v)) => v,
            _ => super::send_new_events::SendNewEventsResponse::default_instance(),
        }
    }
}

impl ::protobuf::Message for ResponseWrapper {
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
                    self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::connect(try!(is.read_message())));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::disconnect(try!(is.read_message())));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::ack_updates(try!(is.read_message())));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::request_updates(try!(is.read_message())));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    self.payload = ::std::option::Option::Some(ResponseWrapper_oneof_payload::send_new_events(try!(is.read_message())));
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
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &ResponseWrapper_oneof_payload::connect(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseWrapper_oneof_payload::disconnect(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseWrapper_oneof_payload::ack_updates(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseWrapper_oneof_payload::request_updates(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ResponseWrapper_oneof_payload::send_new_events(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.payload {
            match v {
                &ResponseWrapper_oneof_payload::connect(ref v) => {
                    try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ResponseWrapper_oneof_payload::disconnect(ref v) => {
                    try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ResponseWrapper_oneof_payload::ack_updates(ref v) => {
                    try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ResponseWrapper_oneof_payload::request_updates(ref v) => {
                    try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                    try!(os.write_raw_varint32(v.get_cached_size()));
                    try!(v.write_to_with_cached_sizes(os));
                },
                &ResponseWrapper_oneof_payload::send_new_events(ref v) => {
                    try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<ResponseWrapper>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseWrapper {
    fn new() -> ResponseWrapper {
        ResponseWrapper::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseWrapper>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "connect",
                    ResponseWrapper::has_connect,
                    ResponseWrapper::get_connect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "disconnect",
                    ResponseWrapper::has_disconnect,
                    ResponseWrapper::get_disconnect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ack_updates",
                    ResponseWrapper::has_ack_updates,
                    ResponseWrapper::get_ack_updates,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "request_updates",
                    ResponseWrapper::has_request_updates,
                    ResponseWrapper::get_request_updates,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "send_new_events",
                    ResponseWrapper::has_send_new_events,
                    ResponseWrapper::get_send_new_events,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseWrapper>(
                    "ResponseWrapper",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseWrapper {
    fn clear(&mut self) {
        self.clear_connect();
        self.clear_disconnect();
        self.clear_ack_updates();
        self.clear_request_updates();
        self.clear_send_new_events();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ResponseWrapper {
    fn eq(&self, other: &ResponseWrapper) -> bool {
        self.payload == other.payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ResponseWrapper {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1e, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x5f, 0x77, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x0d, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x10, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x11, 0x61, 0x63, 0x6b, 0x5f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x15, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x75, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x15, 0x73, 0x65, 0x6e,
    0x64, 0x5f, 0x6e, 0x65, 0x77, 0x5f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0xfb, 0x02, 0x0a, 0x0e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x57, 0x72,
    0x61, 0x70, 0x70, 0x65, 0x72, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x65, 0x73, 0x73, 0x69, 0x6f, 0x6e,
    0x5f, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x73, 0x65,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x12, 0x1b, 0x0a, 0x09, 0x63, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x12, 0x2b, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x43, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x52, 0x07, 0x63, 0x6f, 0x6e,
    0x6e, 0x65, 0x63, 0x74, 0x12, 0x34, 0x0a, 0x0a, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x44, 0x69, 0x73, 0x63, 0x6f,
    0x6e, 0x6e, 0x65, 0x63, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x52, 0x0a,
    0x64, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x12, 0x35, 0x0a, 0x0b, 0x61, 0x63,
    0x6b, 0x5f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x12, 0x2e, 0x41, 0x63, 0x6b, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x48, 0x00, 0x52, 0x0a, 0x61, 0x63, 0x6b, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x73, 0x12, 0x41, 0x0a, 0x0f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x75, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x48, 0x00, 0x52, 0x0e, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x55, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x73, 0x12, 0x3f, 0x0a, 0x0f, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x6e, 0x65, 0x77,
    0x5f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e,
    0x53, 0x65, 0x6e, 0x64, 0x4e, 0x65, 0x77, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x48, 0x00, 0x52, 0x0d, 0x73, 0x65, 0x6e, 0x64, 0x4e, 0x65, 0x77, 0x45,
    0x76, 0x65, 0x6e, 0x74, 0x73, 0x42, 0x09, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64,
    0x22, 0xbf, 0x02, 0x0a, 0x0f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x57, 0x72, 0x61,
    0x70, 0x70, 0x65, 0x72, 0x12, 0x2c, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x52, 0x07, 0x63, 0x6f, 0x6e, 0x6e, 0x65,
    0x63, 0x74, 0x12, 0x35, 0x0a, 0x0a, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x52, 0x0a, 0x64,
    0x69, 0x73, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x12, 0x36, 0x0a, 0x0b, 0x61, 0x63, 0x6b,
    0x5f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13,
    0x2e, 0x41, 0x63, 0x6b, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x48, 0x00, 0x52, 0x0a, 0x61, 0x63, 0x6b, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x73, 0x12, 0x42, 0x0a, 0x0f, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x75, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x48, 0x00, 0x52, 0x0e, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x55, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x73, 0x12, 0x40, 0x0a, 0x0f, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x6e, 0x65,
    0x77, 0x5f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16,
    0x2e, 0x53, 0x65, 0x6e, 0x64, 0x4e, 0x65, 0x77, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x00, 0x52, 0x0d, 0x73, 0x65, 0x6e, 0x64, 0x4e, 0x65,
    0x77, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x42, 0x09, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f,
    0x61, 0x64, 0x4a, 0x88, 0x0a, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x24, 0x01, 0x0a, 0x08, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02,
    0x07, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x03, 0x07, 0x19, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x04, 0x07, 0x1a, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x05, 0x07, 0x1e, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x06, 0x07, 0x1e, 0x0a, 0x43,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x19, 0x01, 0x1a, 0x37, 0x20, 0x43, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x61, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20,
    0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x16, 0x0a,
    0x85, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02, 0x1b, 0x1a, 0x78, 0x20,
    0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x73,
    0x73, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2e, 0x20, 0x52, 0x65, 0x71, 0x75,
    0x69, 0x72, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x73, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20,
    0x27, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x27, 0x2c, 0x0a, 0x20, 0x77, 0x68, 0x69, 0x63,
    0x68, 0x20, 0x69, 0x73, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x69, 0x74, 0x27, 0x73, 0x20, 0x72,
    0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63,
    0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x0c, 0x02, 0x09, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c,
    0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x19, 0x1a,
    0x0a, 0x61, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x02, 0x17, 0x1a, 0x54, 0x20,
    0x41, 0x20, 0x72, 0x61, 0x6e, 0x64, 0x6f, 0x6d, 0x20, 0x75, 0x75, 0x69, 0x64, 0x20, 0x67, 0x65,
    0x6e, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x20, 0x74, 0x68, 0x72, 0x6f, 0x75, 0x67, 0x68, 0x74, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6c, 0x69, 0x66, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x2e, 0x20, 0x52, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65,
    0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0f, 0x02,
    0x0c, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0f, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f, 0x09, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0f, 0x15, 0x16, 0x0a, 0x59, 0x0a, 0x04,
    0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x12, 0x02, 0x18, 0x03, 0x1a, 0x4b, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x20, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x20,
    0x43, 0x6f, 0x72, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20,
    0x61, 0x62, 0x6f, 0x76, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x01,
    0x12, 0x03, 0x12, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x13,
    0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x13, 0x04, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x13, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x13, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x14, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x14, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x14, 0x16, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x14, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x15, 0x04, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x15, 0x04, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x15, 0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x15, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x05, 0x12, 0x03, 0x16, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06,
    0x12, 0x03, 0x16, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x16, 0x1a, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x16, 0x2c,
    0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x17, 0x04, 0x2d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06, 0x12, 0x03, 0x17, 0x04, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x17, 0x19, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x03, 0x12, 0x03, 0x17, 0x2b, 0x2c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x1b, 0x00, 0x24, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x17,
    0x0a, 0x5a, 0x0a, 0x04, 0x04, 0x01, 0x08, 0x00, 0x12, 0x04, 0x1d, 0x02, 0x23, 0x03, 0x1a, 0x4c,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x61, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x20, 0x70, 0x61, 0x79, 0x6c,
    0x6f, 0x61, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x2e, 0x20, 0x43, 0x6f, 0x72, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x73,
    0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20,
    0x74, 0x79, 0x70, 0x65, 0x20, 0x61, 0x62, 0x6f, 0x76, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x08, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x1e, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x1e, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x1e, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x04, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1f, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x17, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x1f, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02,
    0x12, 0x03, 0x20, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x20, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x20, 0x17,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x20, 0x25, 0x26, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x21, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x06, 0x12, 0x03, 0x21, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x21, 0x1b, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x21, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03,
    0x22, 0x04, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x06, 0x12, 0x03, 0x22, 0x04,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x22, 0x1a, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x22, 0x2c, 0x2d, 0x62, 0x06, 0x70,
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
