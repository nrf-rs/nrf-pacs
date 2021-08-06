#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Starts the SAADC and prepares the result buffer in RAM"]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x04 - Takes one SAADC sample"]
    pub tasks_sample: crate::Reg<tasks_sample::TASKS_SAMPLE_SPEC>,
    #[doc = "0x08 - Stops the SAADC and terminates all on-going conversions"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    #[doc = "0x0c - Starts offset auto-calibration"]
    pub tasks_calibrateoffset: crate::Reg<tasks_calibrateoffset::TASKS_CALIBRATEOFFSET_SPEC>,
    _reserved4: [u8; 0xf0],
    #[doc = "0x100 - The SAADC has started"]
    pub events_started: crate::Reg<events_started::EVENTS_STARTED_SPEC>,
    #[doc = "0x104 - The SAADC has filled up the result buffer"]
    pub events_end: crate::Reg<events_end::EVENTS_END_SPEC>,
    #[doc = "0x108 - A conversion task has been completed. Depending on the configuration, multiple conversions might be needed for a result to be transferred to RAM."]
    pub events_done: crate::Reg<events_done::EVENTS_DONE_SPEC>,
    #[doc = "0x10c - Result ready for transfer to RAM"]
    pub events_resultdone: crate::Reg<events_resultdone::EVENTS_RESULTDONE_SPEC>,
    #[doc = "0x110 - Calibration is complete"]
    pub events_calibratedone: crate::Reg<events_calibratedone::EVENTS_CALIBRATEDONE_SPEC>,
    #[doc = "0x114 - The SAADC has stopped"]
    pub events_stopped: crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>,
    #[doc = "0x118..0x158 - Peripheral events."]
    pub events_ch: [EVENTS_CH; 8],
    _reserved11: [u8; 0x01a8],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved14: [u8; 0xf4],
    #[doc = "0x400 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    _reserved15: [u8; 0xfc],
    #[doc = "0x500 - Enable or disable SAADC"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x510..0x590 - Unspecified"]
    pub ch: [CH; 8],
    _reserved17: [u8; 0x60],
    #[doc = "0x5f0 - Resolution configuration"]
    pub resolution: crate::Reg<resolution::RESOLUTION_SPEC>,
    #[doc = "0x5f4 - Oversampling configuration. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
    pub oversample: crate::Reg<oversample::OVERSAMPLE_SPEC>,
    #[doc = "0x5f8 - Controls normal or continuous sample rate"]
    pub samplerate: crate::Reg<samplerate::SAMPLERATE_SPEC>,
    _reserved20: [u8; 0x30],
    #[doc = "0x62c..0x638 - RESULT EasyDMA channel"]
    pub result: RESULT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct EVENTS_CH {
    #[doc = "0x00 - Description cluster: Last result is equal or above CH\\[n\\].LIMIT.HIGH"]
    pub limith: crate::Reg<self::events_ch::limith::LIMITH_SPEC>,
    #[doc = "0x04 - Description cluster: Last result is equal or below CH\\[n\\].LIMIT.LOW"]
    pub limitl: crate::Reg<self::events_ch::limitl::LIMITL_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Peripheral events."]
pub mod events_ch;
#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Description cluster: Input positive pin selection for CH\\[n\\]"]
    pub pselp: crate::Reg<self::ch::pselp::PSELP_SPEC>,
    #[doc = "0x04 - Description cluster: Input negative pin selection for CH\\[n\\]"]
    pub pseln: crate::Reg<self::ch::pseln::PSELN_SPEC>,
    #[doc = "0x08 - Description cluster: Input configuration for CH\\[n\\]"]
    pub config: crate::Reg<self::ch::config::CONFIG_SPEC>,
    #[doc = "0x0c - Description cluster: High/low limits for event monitoring of a channel"]
    pub limit: crate::Reg<self::ch::limit::LIMIT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod ch;
#[doc = r"Register block"]
#[repr(C)]
pub struct RESULT {
    #[doc = "0x00 - Data pointer"]
    pub ptr: crate::Reg<self::result::ptr::PTR_SPEC>,
    #[doc = "0x04 - Maximum number of 16-bit samples to be written to output RAM buffer"]
    pub maxcnt: crate::Reg<self::result::maxcnt::MAXCNT_SPEC>,
    #[doc = "0x08 - Number of 16-bit samples written to output RAM buffer since the previous START task"]
    pub amount: crate::Reg<self::result::amount::AMOUNT_SPEC>,
}
#[doc = r"Register block"]
#[doc = "RESULT EasyDMA channel"]
pub mod result;
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Starts the SAADC and prepares the result buffer in RAM"]
pub mod tasks_start;
#[doc = "TASKS_SAMPLE register accessor: an alias for `Reg<TASKS_SAMPLE_SPEC>`"]
pub type TASKS_SAMPLE = crate::Reg<tasks_sample::TASKS_SAMPLE_SPEC>;
#[doc = "Takes one SAADC sample"]
pub mod tasks_sample;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stops the SAADC and terminates all on-going conversions"]
pub mod tasks_stop;
#[doc = "TASKS_CALIBRATEOFFSET register accessor: an alias for `Reg<TASKS_CALIBRATEOFFSET_SPEC>`"]
pub type TASKS_CALIBRATEOFFSET = crate::Reg<tasks_calibrateoffset::TASKS_CALIBRATEOFFSET_SPEC>;
#[doc = "Starts offset auto-calibration"]
pub mod tasks_calibrateoffset;
#[doc = "EVENTS_STARTED register accessor: an alias for `Reg<EVENTS_STARTED_SPEC>`"]
pub type EVENTS_STARTED = crate::Reg<events_started::EVENTS_STARTED_SPEC>;
#[doc = "The SAADC has started"]
pub mod events_started;
#[doc = "EVENTS_END register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "The SAADC has filled up the result buffer"]
pub mod events_end;
#[doc = "EVENTS_DONE register accessor: an alias for `Reg<EVENTS_DONE_SPEC>`"]
pub type EVENTS_DONE = crate::Reg<events_done::EVENTS_DONE_SPEC>;
#[doc = "A conversion task has been completed. Depending on the configuration, multiple conversions might be needed for a result to be transferred to RAM."]
pub mod events_done;
#[doc = "EVENTS_RESULTDONE register accessor: an alias for `Reg<EVENTS_RESULTDONE_SPEC>`"]
pub type EVENTS_RESULTDONE = crate::Reg<events_resultdone::EVENTS_RESULTDONE_SPEC>;
#[doc = "Result ready for transfer to RAM"]
pub mod events_resultdone;
#[doc = "EVENTS_CALIBRATEDONE register accessor: an alias for `Reg<EVENTS_CALIBRATEDONE_SPEC>`"]
pub type EVENTS_CALIBRATEDONE = crate::Reg<events_calibratedone::EVENTS_CALIBRATEDONE_SPEC>;
#[doc = "Calibration is complete"]
pub mod events_calibratedone;
#[doc = "EVENTS_STOPPED register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "The SAADC has stopped"]
pub mod events_stopped;
#[doc = "INTEN register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable or disable SAADC"]
pub mod enable;
#[doc = "RESOLUTION register accessor: an alias for `Reg<RESOLUTION_SPEC>`"]
pub type RESOLUTION = crate::Reg<resolution::RESOLUTION_SPEC>;
#[doc = "Resolution configuration"]
pub mod resolution;
#[doc = "OVERSAMPLE register accessor: an alias for `Reg<OVERSAMPLE_SPEC>`"]
pub type OVERSAMPLE = crate::Reg<oversample::OVERSAMPLE_SPEC>;
#[doc = "Oversampling configuration. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
pub mod oversample;
#[doc = "SAMPLERATE register accessor: an alias for `Reg<SAMPLERATE_SPEC>`"]
pub type SAMPLERATE = crate::Reg<samplerate::SAMPLERATE_SPEC>;
#[doc = "Controls normal or continuous sample rate"]
pub mod samplerate;
