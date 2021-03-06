// This file is generated by rust-protobuf 2.7.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `proto/groupcache.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_7_0;

#[derive(PartialEq,Clone,Default)]
pub struct GetRequest {
    // message fields
    group: ::protobuf::SingularField<::std::string::String>,
    key: ::protobuf::SingularField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GetRequest {
    fn default() -> &'a GetRequest {
        <GetRequest as ::protobuf::Message>::default_instance()
    }
}

impl GetRequest {
    pub fn new() -> GetRequest {
        ::std::default::Default::default()
    }

    // required string group = 1;


    pub fn get_group(&self) -> &str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group(&mut self) -> &mut ::std::string::String {
        if self.group.is_none() {
            self.group.set_default();
        }
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> ::std::string::String {
        self.group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // required string key = 2;


    pub fn get_key(&self) -> &str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
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
        }
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::protobuf::Message for GetRequest {
    fn is_initialized(&self) -> bool {
        if self.group.is_none() {
            return false;
        }
        if self.key.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.group.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.key.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.group.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.key.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GetRequest {
        GetRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    |m: &GetRequest| { &m.group },
                    |m: &mut GetRequest| { &mut m.group },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    |m: &GetRequest| { &m.key },
                    |m: &mut GetRequest| { &mut m.key },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetRequest>(
                    "GetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static GetRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetRequest,
        };
        unsafe {
            instance.get(GetRequest::new)
        }
    }
}

impl ::protobuf::Clear for GetRequest {
    fn clear(&mut self) {
        self.group.clear();
        self.key.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetResponse {
    // message fields
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    minute_qps: ::std::option::Option<f64>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GetResponse {
    fn default() -> &'a GetResponse {
        <GetResponse as ::protobuf::Message>::default_instance()
    }
}

impl GetResponse {
    pub fn new() -> GetResponse {
        ::std::default::Default::default()
    }

    // optional bytes value = 1;


    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
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
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    // optional double minute_qps = 2;


    pub fn get_minute_qps(&self) -> f64 {
        self.minute_qps.unwrap_or(0.)
    }
    pub fn clear_minute_qps(&mut self) {
        self.minute_qps = ::std::option::Option::None;
    }

    pub fn has_minute_qps(&self) -> bool {
        self.minute_qps.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minute_qps(&mut self, v: f64) {
        self.minute_qps = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for GetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.minute_qps = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.minute_qps {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.value.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(v) = self.minute_qps {
            os.write_double(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GetResponse {
        GetResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    |m: &GetResponse| { &m.value },
                    |m: &mut GetResponse| { &mut m.value },
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "minute_qps",
                    |m: &GetResponse| { &m.minute_qps },
                    |m: &mut GetResponse| { &mut m.minute_qps },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetResponse>(
                    "GetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static GetResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetResponse,
        };
        unsafe {
            instance.get(GetResponse::new)
        }
    }
}

impl ::protobuf::Clear for GetResponse {
    fn clear(&mut self) {
        self.value.clear();
        self.minute_qps = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16proto/groupcache.proto\x12\x0cgroupcachepb\"4\n\nGetRequest\x12\
    \x14\n\x05group\x18\x01\x20\x02(\tR\x05group\x12\x10\n\x03key\x18\x02\
    \x20\x02(\tR\x03key\"B\n\x0bGetResponse\x12\x14\n\x05value\x18\x01\x20\
    \x01(\x0cR\x05value\x12\x1d\n\nminute_qps\x18\x02\x20\x01(\x01R\tminuteQ\
    ps2J\n\nGroupCache\x12<\n\x03Get\x12\x18.groupcachepb.GetRequest\x1a\x19\
    .groupcachepb.GetResponse\"\0J\x86\x08\n\x06\x12\x04\r\0\x1e\x01\n\xad\
    \x04\n\x01\x0c\x12\x03\r\0\x122\xa2\x04\nCopyright\x202012\x20Google\x20\
    Inc.\nLicensed\x20under\x20the\x20Apache\x20License,\x20Version\x202.0\
    \x20(the\x20\"License\");\nyou\x20may\x20not\x20use\x20this\x20file\x20e\
    xcept\x20in\x20compliance\x20with\x20the\x20License.\nYou\x20may\x20obta\
    in\x20a\x20copy\x20of\x20the\x20License\x20at\nhttp://www.apache.org/lic\
    enses/LICENSE-2.0\nUnless\x20required\x20by\x20applicable\x20law\x20or\
    \x20agreed\x20to\x20in\x20writing,\x20software\ndistributed\x20under\x20\
    the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS\
    ,\nWITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20ei\
    ther\x20express\x20or\x20implied.\nSee\x20the\x20License\x20for\x20the\
    \x20specific\x20language\x20governing\x20permissions\x20and\nlimitations\
    \x20under\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x0f\0\x15\n\n\n\
    \x02\x04\0\x12\x04\x11\0\x14\x01\n\n\n\x03\x04\0\x01\x12\x03\x11\x08\x12\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\x12\x02\x1c\n\x0c\n\x05\x04\0\x02\0\x04\
    \x12\x03\x12\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x12\x0b\x11\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x12\x12\x17\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x12\x1a\x1b\n;\n\x04\x04\0\x02\x01\x12\x03\x13\x02\x1a\".\x20no\
    t\x20actually\x20required/guaranteed\x20to\x20be\x20UTF-8\n\n\x0c\n\x05\
    \x04\0\x02\x01\x04\x12\x03\x13\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\
    \x03\x13\x0b\x11\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x13\x12\x15\n\x0c\
    \n\x05\x04\0\x02\x01\x03\x12\x03\x13\x18\x19\n\n\n\x02\x04\x01\x12\x04\
    \x16\0\x19\x01\n\n\n\x03\x04\x01\x01\x12\x03\x16\x08\x13\n\x0b\n\x04\x04\
    \x01\x02\0\x12\x03\x17\x02\x1b\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x17\
    \x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x17\x0b\x10\n\x0c\n\x05\x04\
    \x01\x02\0\x01\x12\x03\x17\x11\x16\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\
    \x17\x19\x1a\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x18\x02!\n\x0c\n\x05\
    \x04\x01\x02\x01\x04\x12\x03\x18\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x05\
    \x12\x03\x18\x0b\x11\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x18\x12\x1c\
    \n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x18\x1f\x20\n\n\n\x02\x06\0\x12\
    \x04\x1b\0\x1e\x01\n\n\n\x03\x06\0\x01\x12\x03\x1b\x08\x12\n\x0c\n\x04\
    \x06\0\x02\0\x12\x04\x1c\x02\x1d\x03\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\
    \x1c\x06\t\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x1c\n\x14\n\x0c\n\x05\x06\
    \0\x02\0\x03\x12\x03\x1c\x1f*\
";

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
