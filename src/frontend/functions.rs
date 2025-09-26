use std::{
    ffi::c_uint,
    mem::MaybeUninit,
    os::fd::{AsRawFd as _, BorrowedFd},
};

use nix::errno::Errno;

use crate::{
    error::PropertyError,
    frontend::{
        data::{DTV_IOCTL_MAX_MSGS, DvbFrontendInfo},
        ioctl::{fe_get_info, fe_get_property, fe_read_status, fe_set_property},
        property::{DtvProperties, DtvProperty},
    },
};

pub fn get_info(fd: BorrowedFd) -> Result<DvbFrontendInfo, Errno> {
    let mut info = MaybeUninit::uninit();
    unsafe { fe_get_info(fd.as_raw_fd(), info.as_mut_ptr()) }?;
    // SAFETY: If fe_get_info did not throw an error, memory should now be initialized.
    let info = unsafe { info.assume_init() };
    Ok(info)
}

pub fn read_status(fd: BorrowedFd) -> Result<c_uint, Errno> {
    let mut status = MaybeUninit::uninit();
    unsafe { fe_read_status(fd.as_raw_fd(), status.as_mut_ptr()) }?;
    // SAFETY: If fe_read_status did not throw an error, memory should now be initialized.
    let status = unsafe { status.assume_init() };
    Ok(status)
}

pub fn get_set_properties_raw(
    fd: BorrowedFd,
    set: bool,
    count: usize,
    ptr: *mut DtvProperty,
) -> Result<(), PropertyError> {
    if count == 0 {
        return Ok(());
    }

    if count > DTV_IOCTL_MAX_MSGS {
        return Err(PropertyError::TooManyParameters);
    }

    let mut properties = DtvProperties {
        num: count as u32,
        props: ptr,
    };

    if set {
        unsafe { fe_set_property(fd.as_raw_fd(), &mut properties as *mut DtvProperties) }
            .map_err(PropertyError::SetProperty)?;
    } else {
        unsafe { fe_get_property(fd.as_raw_fd(), &mut properties as *mut DtvProperties) }
            .map_err(PropertyError::GetProperty)?;
    }

    Ok(())
}
