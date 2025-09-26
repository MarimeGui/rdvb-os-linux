use nix::{ioctl_none, ioctl_read, ioctl_readwrite, ioctl_write_ptr};

use crate::{
    IOCTL_TYPE,
    demux::data::{DmxPesFilterParams, DmxSctFilterParams, DmxStc},
};

const DMX_START: u8 = 41;
ioctl_none!(dmx_start, IOCTL_TYPE, DMX_START);

const DMX_STOP: u8 = 42;
ioctl_none!(dmx_stop, IOCTL_TYPE, DMX_STOP);

const DMX_SET_FILTER: u8 = 43;
ioctl_write_ptr!(
    dmx_set_filter,
    IOCTL_TYPE,
    DMX_SET_FILTER,
    DmxSctFilterParams
);

const DMX_SET_PES_FILTER: u8 = 44;
ioctl_write_ptr!(
    dmx_set_pes_filter,
    IOCTL_TYPE,
    DMX_SET_PES_FILTER,
    DmxPesFilterParams
);

const DMX_SET_BUFFER_SIZE: u8 = 45;
// TODO: dmx.h and documentation are inconsistent, header says there is no parameter while docs want an unsigned long for size
ioctl_none!(dmx_set_buffer_size, IOCTL_TYPE, DMX_SET_BUFFER_SIZE);

const DMX_GET_PES_PIDS: u8 = 47;
ioctl_read!(dmx_get_pes_pids, IOCTL_TYPE, DMX_GET_PES_PIDS, [u16; 5]);

const DMX_GET_STC: u8 = 50;
ioctl_readwrite!(dmx_get_stc, IOCTL_TYPE, DMX_GET_STC, DmxStc);

const DMX_ADD_PID: u8 = 51;
ioctl_write_ptr!(dmx_add_pid, IOCTL_TYPE, DMX_ADD_PID, u16);

const DMX_REMOVE_PID: u8 = 52;
ioctl_write_ptr!(dmx_remove_pid, IOCTL_TYPE, DMX_REMOVE_PID, u16);

// TODO: Experimental IOCTLs
