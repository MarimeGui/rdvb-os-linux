use std::os::fd::{AsRawFd as _, BorrowedFd};

use nix::errno::Errno;

use crate::{
    demux::{
        data::{DmxPesFilterParams, DmxSctFilterParams},
        ioctl::{
            dmx_add_pid, dmx_remove_pid, dmx_set_filter, dmx_set_pes_filter, dmx_start, dmx_stop,
        },
    },
    error::{DmxSetPesFilterError, DmxStartError},
};

/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dmx-start.html#description))
///
/// This ioctl call is used to start the actual filtering operation defined via the ioctl calls DMX_SET_FILTER or DMX_SET_PES_FILTER.
pub fn start(fd: BorrowedFd) -> Result<(), DmxStartError> {
    // SAFETY: The argument is always a valid file descriptor. There should be no conditions or unhandled side-effects.
    unsafe { dmx_start(fd.as_raw_fd()) }.map_err(DmxStartError::from)?;
    Ok(())
}

pub fn stop(fd: BorrowedFd) -> Result<(), Errno> {
    // SAFETY: The argument is always a valid file descriptor. There should be no conditions or unhandled side-effects.
    unsafe { dmx_stop(fd.as_raw_fd()) }?;
    Ok(())
}

pub fn set_filter(fd: BorrowedFd, params: &DmxSctFilterParams) -> Result<(), Errno> {
    // SAFETY: The argument is always a valid file descriptor and C-compatible DmxSctFilterParams. There should be no conditions or unhandled side-effects.
    unsafe { dmx_set_filter(fd.as_raw_fd(), params) }?;
    Ok(())
}

pub fn set_pes_filter(
    fd: BorrowedFd,
    params: &DmxPesFilterParams,
) -> Result<(), DmxSetPesFilterError> {
    // SAFETY: FD is always valid, DmxPesFilterParams is C-compatible and always valid. There should be no conditions or unhandled side-effects.
    unsafe { dmx_set_pes_filter(fd.as_raw_fd(), params) }.map_err(DmxSetPesFilterError::from)?;
    Ok(())
}

/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dmx-add-pid.html#description))
///
/// This ioctl call allows to add multiple PIDs to a transport stream filter previously
/// set up with DMX_SET_PES_FILTER and output equal to DMX_OUT_TSDEMUX_TAP.
pub fn add_pid(fd: BorrowedFd, pid: u16) -> Result<(), Errno> {
    // SAFETY: FD is always valid, PID can be any u16. There should be no conditions or unhandled side-effects.
    unsafe { dmx_add_pid(fd.as_raw_fd(), &pid) }?;
    Ok(())
}

/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dmx-remove-pid.html#description))
///
/// This ioctl call allows to remove a PID when multiple PIDs are set on a transport stream filter,
/// e. g. a filter previously set up with output equal to DMX_OUT_TSDEMUX_TAP,
/// created via either DMX_SET_PES_FILTER or DMX_ADD_PID.
pub fn remove_pid(fd: BorrowedFd, pid: u16) -> Result<(), Errno> {
    // SAFETY: FD is always valid, PID can be any u16. There should be no conditions or unhandled side-effects.
    unsafe { dmx_remove_pid(fd.as_raw_fd(), &pid) }?;
    Ok(())
}
