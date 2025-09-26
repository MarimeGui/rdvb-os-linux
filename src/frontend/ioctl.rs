use std::ffi::c_uint;

use nix::{ioctl_read, ioctl_write_ptr};

use crate::{
    IOCTL_TYPE,
    frontend::{data::DvbFrontendInfo, property::DtvProperties},
};

pub const FE_GET_INFO: u8 = 61;
ioctl_read!(fe_get_info, IOCTL_TYPE, FE_GET_INFO, DvbFrontendInfo);

pub const FE_READ_STATUS: u8 = 69;
ioctl_read!(fe_read_status, IOCTL_TYPE, FE_READ_STATUS, c_uint); // Maps to FeStatus struct for bits

pub const FE_SET_PROPERTY: u8 = 82;
ioctl_write_ptr!(fe_set_property, IOCTL_TYPE, FE_SET_PROPERTY, DtvProperties);

pub const FE_GET_PROPERTY: u8 = 83;
ioctl_read!(fe_get_property, IOCTL_TYPE, FE_GET_PROPERTY, DtvProperties);
