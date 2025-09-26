use std::ffi::c_uint;

pub const DMX_FILTER_SIZE: usize = 16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum DmxOutput {
    DMX_OUT_DECODER,
    DMX_OUT_TAP,
    DMX_OUT_TS_TAP,
    DMX_OUT_TSDEMUX_TAP,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum DmxInput {
    DMX_IN_FRONTEND,
    DMX_IN_DVR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum DmxTsPes {
    DMX_PES_AUDIO0,
    DMX_PES_VIDEO0,
    DMX_PES_TELETEXT0,
    DMX_PES_SUBTITLE0,
    DMX_PES_PCR0,

    DMX_PES_AUDIO1,
    DMX_PES_VIDEO1,
    DMX_PES_TELETEXT1,
    DMX_PES_SUBTITLE1,
    DMX_PES_PCR1,

    DMX_PES_AUDIO2,
    DMX_PES_VIDEO2,
    DMX_PES_TELETEXT2,
    DMX_PES_SUBTITLE2,
    DMX_PES_PCR2,

    DMX_PES_AUDIO3,
    DMX_PES_VIDEO3,
    DMX_PES_TELETEXT3,
    DMX_PES_SUBTITLE3,
    DMX_PES_PCR3,

    DMX_PES_OTHER,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DmxFilter {
    pub filter: [u8; DMX_FILTER_SIZE],
    pub mask: [u8; DMX_FILTER_SIZE],
    pub mode: [u8; DMX_FILTER_SIZE],
}

/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dmx_types.html#c.dmx_sct_filter_params))
///
/// Specifies a section filter.
///
/// Carries the configuration for a MPEG-TS section filter.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DmxSctFilterParams {
    /// PID to be filtered.
    pub pid: u16,
    /// section header filter, as defined by struct dmx_filter.
    pub filter: DmxFilter,
    /// maximum time to filter, in milliseconds.
    pub timeout: u32,
    // TODO: u32 struct for bits
    /// extra flags for the section filter.
    pub flags: u32,
}

/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dmx_types.html#c.dmx_pes_filter_params))
///
/// Specifies Packetized Elementary Stream (PES) filter parameters.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DmxPesFilterParams {
    /// PID to be filtered.
    pub pid: u16,
    /// Demux input, as specified by enum dmx_input.
    pub input: DmxInput,
    /// Demux output, as specified by enum dmx_output.
    pub output: DmxOutput,
    /// Type of the pes filter, as specified by enum dmx_pes_type.
    pub pes_type: DmxTsPes,
    // TODO: There is an enum for these flags
    /// Demux PES flags.
    pub flags: u32,
}

/// (taken from [official docs](https://www.linuxtv.org/downloads/v4l-dvb-apis-new/userspace-api/dvb/dmx_types.html#c.dmx_stc))
///
/// Stores System Time Counter (STC) information.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DmxStc {
    /// input data: number of the STC, from 0 to N.
    pub num: c_uint,
    /// output: divisor for STC to get 90 kHz clock.
    pub base: c_uint,
    /// output: stc in **base** * 90 kHz units.
    pub stc: u64,
}
