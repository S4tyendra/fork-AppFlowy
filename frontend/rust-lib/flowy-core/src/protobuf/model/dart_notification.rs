// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `dart_notification.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum WorkspaceNotification {
    Unknown = 0,
    UserCreateWorkspace = 10,
    UserDeleteWorkspace = 11,
    WorkspaceUpdated = 12,
    WorkspaceListUpdated = 13,
    WorkspaceAppsChanged = 14,
    AppUpdated = 21,
    AppViewsChanged = 24,
    ViewUpdated = 31,
    ViewDeleted = 32,
    ViewRestored = 33,
    UserUnauthorized = 100,
    TrashUpdated = 1000,
}

impl ::protobuf::ProtobufEnum for WorkspaceNotification {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<WorkspaceNotification> {
        match value {
            0 => ::std::option::Option::Some(WorkspaceNotification::Unknown),
            10 => ::std::option::Option::Some(WorkspaceNotification::UserCreateWorkspace),
            11 => ::std::option::Option::Some(WorkspaceNotification::UserDeleteWorkspace),
            12 => ::std::option::Option::Some(WorkspaceNotification::WorkspaceUpdated),
            13 => ::std::option::Option::Some(WorkspaceNotification::WorkspaceListUpdated),
            14 => ::std::option::Option::Some(WorkspaceNotification::WorkspaceAppsChanged),
            21 => ::std::option::Option::Some(WorkspaceNotification::AppUpdated),
            24 => ::std::option::Option::Some(WorkspaceNotification::AppViewsChanged),
            31 => ::std::option::Option::Some(WorkspaceNotification::ViewUpdated),
            32 => ::std::option::Option::Some(WorkspaceNotification::ViewDeleted),
            33 => ::std::option::Option::Some(WorkspaceNotification::ViewRestored),
            100 => ::std::option::Option::Some(WorkspaceNotification::UserUnauthorized),
            1000 => ::std::option::Option::Some(WorkspaceNotification::TrashUpdated),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [WorkspaceNotification] = &[
            WorkspaceNotification::Unknown,
            WorkspaceNotification::UserCreateWorkspace,
            WorkspaceNotification::UserDeleteWorkspace,
            WorkspaceNotification::WorkspaceUpdated,
            WorkspaceNotification::WorkspaceListUpdated,
            WorkspaceNotification::WorkspaceAppsChanged,
            WorkspaceNotification::AppUpdated,
            WorkspaceNotification::AppViewsChanged,
            WorkspaceNotification::ViewUpdated,
            WorkspaceNotification::ViewDeleted,
            WorkspaceNotification::ViewRestored,
            WorkspaceNotification::UserUnauthorized,
            WorkspaceNotification::TrashUpdated,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<WorkspaceNotification>("WorkspaceNotification", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for WorkspaceNotification {
}

impl ::std::default::Default for WorkspaceNotification {
    fn default() -> Self {
        WorkspaceNotification::Unknown
    }
}

impl ::protobuf::reflect::ProtobufValue for WorkspaceNotification {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17dart_notification.proto*\xa2\x02\n\x15WorkspaceNotification\x12\
    \x0b\n\x07Unknown\x10\0\x12\x17\n\x13UserCreateWorkspace\x10\n\x12\x17\n\
    \x13UserDeleteWorkspace\x10\x0b\x12\x14\n\x10WorkspaceUpdated\x10\x0c\
    \x12\x18\n\x14WorkspaceListUpdated\x10\r\x12\x18\n\x14WorkspaceAppsChang\
    ed\x10\x0e\x12\x0e\n\nAppUpdated\x10\x15\x12\x13\n\x0fAppViewsChanged\
    \x10\x18\x12\x0f\n\x0bViewUpdated\x10\x1f\x12\x0f\n\x0bViewDeleted\x10\
    \x20\x12\x10\n\x0cViewRestored\x10!\x12\x14\n\x10UserUnauthorized\x10d\
    \x12\x11\n\x0cTrashUpdated\x10\xe8\x07J\xbf\x04\n\x06\x12\x04\0\0\x10\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x05\0\x12\x04\x02\0\x10\
    \x01\n\n\n\x03\x05\0\x01\x12\x03\x02\x05\x1a\n\x0b\n\x04\x05\0\x02\0\x12\
    \x03\x03\x04\x10\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x03\x04\x0b\n\x0c\n\
    \x05\x05\0\x02\0\x02\x12\x03\x03\x0e\x0f\n\x0b\n\x04\x05\0\x02\x01\x12\
    \x03\x04\x04\x1d\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x04\x04\x17\n\x0c\
    \n\x05\x05\0\x02\x01\x02\x12\x03\x04\x1a\x1c\n\x0b\n\x04\x05\0\x02\x02\
    \x12\x03\x05\x04\x1d\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\x05\x04\x17\n\
    \x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x05\x1a\x1c\n\x0b\n\x04\x05\0\x02\
    \x03\x12\x03\x06\x04\x1a\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\x06\x04\
    \x14\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x06\x17\x19\n\x0b\n\x04\x05\0\
    \x02\x04\x12\x03\x07\x04\x1e\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\x07\
    \x04\x18\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x07\x1b\x1d\n\x0b\n\x04\
    \x05\0\x02\x05\x12\x03\x08\x04\x1e\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03\
    \x08\x04\x18\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03\x08\x1b\x1d\n\x0b\n\
    \x04\x05\0\x02\x06\x12\x03\t\x04\x14\n\x0c\n\x05\x05\0\x02\x06\x01\x12\
    \x03\t\x04\x0e\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03\t\x11\x13\n\x0b\n\
    \x04\x05\0\x02\x07\x12\x03\n\x04\x19\n\x0c\n\x05\x05\0\x02\x07\x01\x12\
    \x03\n\x04\x13\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x03\n\x16\x18\n\x0b\n\
    \x04\x05\0\x02\x08\x12\x03\x0b\x04\x15\n\x0c\n\x05\x05\0\x02\x08\x01\x12\
    \x03\x0b\x04\x0f\n\x0c\n\x05\x05\0\x02\x08\x02\x12\x03\x0b\x12\x14\n\x0b\
    \n\x04\x05\0\x02\t\x12\x03\x0c\x04\x15\n\x0c\n\x05\x05\0\x02\t\x01\x12\
    \x03\x0c\x04\x0f\n\x0c\n\x05\x05\0\x02\t\x02\x12\x03\x0c\x12\x14\n\x0b\n\
    \x04\x05\0\x02\n\x12\x03\r\x04\x16\n\x0c\n\x05\x05\0\x02\n\x01\x12\x03\r\
    \x04\x10\n\x0c\n\x05\x05\0\x02\n\x02\x12\x03\r\x13\x15\n\x0b\n\x04\x05\0\
    \x02\x0b\x12\x03\x0e\x04\x1b\n\x0c\n\x05\x05\0\x02\x0b\x01\x12\x03\x0e\
    \x04\x14\n\x0c\n\x05\x05\0\x02\x0b\x02\x12\x03\x0e\x17\x1a\n\x0b\n\x04\
    \x05\0\x02\x0c\x12\x03\x0f\x04\x18\n\x0c\n\x05\x05\0\x02\x0c\x01\x12\x03\
    \x0f\x04\x10\n\x0c\n\x05\x05\0\x02\x0c\x02\x12\x03\x0f\x13\x17b\x06proto\
    3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
