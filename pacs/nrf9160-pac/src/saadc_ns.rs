#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the ADC and prepare the result buffer in RAM"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Take one ADC sample, if scan is enabled all channels are sampled"]
    pub tasks_sample: TASKS_SAMPLE,
    #[doc = "0x08 - Stop the ADC and terminate any on-going conversion"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x0c - Starts offset auto-calibration"]
    pub tasks_calibrateoffset: TASKS_CALIBRATEOFFSET,
    _reserved4: [u8; 0x70],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x84 - Subscribe configuration for task SAMPLE"]
    pub subscribe_sample: SUBSCRIBE_SAMPLE,
    #[doc = "0x88 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    #[doc = "0x8c - Subscribe configuration for task CALIBRATEOFFSET"]
    pub subscribe_calibrateoffset: SUBSCRIBE_CALIBRATEOFFSET,
    _reserved8: [u8; 0x70],
    #[doc = "0x100 - The ADC has started"]
    pub events_started: EVENTS_STARTED,
    #[doc = "0x104 - The ADC has filled up the Result buffer"]
    pub events_end: EVENTS_END,
    #[doc = "0x108 - A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
    pub events_done: EVENTS_DONE,
    #[doc = "0x10c - A result is ready to get transferred to RAM."]
    pub events_resultdone: EVENTS_RESULTDONE,
    #[doc = "0x110 - Calibration is complete"]
    pub events_calibratedone: EVENTS_CALIBRATEDONE,
    #[doc = "0x114 - The ADC has stopped"]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x118..0x158 - Peripheral events."]
    pub events_ch: [EVENTS_CH; 8],
    _reserved15: [u8; 0x28],
    #[doc = "0x180 - Publish configuration for event STARTED"]
    pub publish_started: PUBLISH_STARTED,
    #[doc = "0x184 - Publish configuration for event END"]
    pub publish_end: PUBLISH_END,
    #[doc = "0x188 - Publish configuration for event DONE"]
    pub publish_done: PUBLISH_DONE,
    #[doc = "0x18c - Publish configuration for event RESULTDONE"]
    pub publish_resultdone: PUBLISH_RESULTDONE,
    #[doc = "0x190 - Publish configuration for event CALIBRATEDONE"]
    pub publish_calibratedone: PUBLISH_CALIBRATEDONE,
    #[doc = "0x194 - Publish configuration for event STOPPED"]
    pub publish_stopped: PUBLISH_STOPPED,
    #[doc = "0x198..0x1d8 - Publish configuration for events"]
    pub publish_ch: [PUBLISH_CH; 8],
    _reserved22: [u8; 0x0128],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved25: [u8; 0xf4],
    #[doc = "0x400 - Status"]
    pub status: STATUS,
    _reserved26: [u8; 0xfc],
    #[doc = "0x500 - Enable or disable ADC"]
    pub enable: ENABLE,
    _reserved27: [u8; 0x0c],
    #[doc = "0x510..0x590 - Unspecified"]
    pub ch: [CH; 8],
    _reserved28: [u8; 0x60],
    #[doc = "0x5f0 - Resolution configuration"]
    pub resolution: RESOLUTION,
    #[doc = "0x5f4 - Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
    pub oversample: OVERSAMPLE,
    #[doc = "0x5f8 - Controls normal or continuous sample rate"]
    pub samplerate: SAMPLERATE,
    _reserved31: [u8; 0x30],
    #[doc = "0x62c..0x638 - RESULT EasyDMA channel"]
    pub result: RESULT,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start the ADC and prepare the result buffer in RAM"]
pub mod tasks_start;
#[doc = "TASKS_SAMPLE (w) register accessor: an alias for `Reg<TASKS_SAMPLE_SPEC>`"]
pub type TASKS_SAMPLE = crate::Reg<tasks_sample::TASKS_SAMPLE_SPEC>;
#[doc = "Take one ADC sample, if scan is enabled all channels are sampled"]
pub mod tasks_sample;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop the ADC and terminate any on-going conversion"]
pub mod tasks_stop;
#[doc = "TASKS_CALIBRATEOFFSET (w) register accessor: an alias for `Reg<TASKS_CALIBRATEOFFSET_SPEC>`"]
pub type TASKS_CALIBRATEOFFSET = crate::Reg<tasks_calibrateoffset::TASKS_CALIBRATEOFFSET_SPEC>;
#[doc = "Starts offset auto-calibration"]
pub mod tasks_calibrateoffset;
#[doc = "SUBSCRIBE_START (rw) register accessor: an alias for `Reg<SUBSCRIBE_START_SPEC>`"]
pub type SUBSCRIBE_START = crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_SAMPLE (rw) register accessor: an alias for `Reg<SUBSCRIBE_SAMPLE_SPEC>`"]
pub type SUBSCRIBE_SAMPLE = crate::Reg<subscribe_sample::SUBSCRIBE_SAMPLE_SPEC>;
#[doc = "Subscribe configuration for task SAMPLE"]
pub mod subscribe_sample;
#[doc = "SUBSCRIBE_STOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "SUBSCRIBE_CALIBRATEOFFSET (rw) register accessor: an alias for `Reg<SUBSCRIBE_CALIBRATEOFFSET_SPEC>`"]
pub type SUBSCRIBE_CALIBRATEOFFSET =
    crate::Reg<subscribe_calibrateoffset::SUBSCRIBE_CALIBRATEOFFSET_SPEC>;
#[doc = "Subscribe configuration for task CALIBRATEOFFSET"]
pub mod subscribe_calibrateoffset;
#[doc = "EVENTS_STARTED (rw) register accessor: an alias for `Reg<EVENTS_STARTED_SPEC>`"]
pub type EVENTS_STARTED = crate::Reg<events_started::EVENTS_STARTED_SPEC>;
#[doc = "The ADC has started"]
pub mod events_started;
#[doc = "EVENTS_END (rw) register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "The ADC has filled up the Result buffer"]
pub mod events_end;
#[doc = "EVENTS_DONE (rw) register accessor: an alias for `Reg<EVENTS_DONE_SPEC>`"]
pub type EVENTS_DONE = crate::Reg<events_done::EVENTS_DONE_SPEC>;
#[doc = "A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
pub mod events_done;
#[doc = "EVENTS_RESULTDONE (rw) register accessor: an alias for `Reg<EVENTS_RESULTDONE_SPEC>`"]
pub type EVENTS_RESULTDONE = crate::Reg<events_resultdone::EVENTS_RESULTDONE_SPEC>;
#[doc = "A result is ready to get transferred to RAM."]
pub mod events_resultdone;
#[doc = "EVENTS_CALIBRATEDONE (rw) register accessor: an alias for `Reg<EVENTS_CALIBRATEDONE_SPEC>`"]
pub type EVENTS_CALIBRATEDONE = crate::Reg<events_calibratedone::EVENTS_CALIBRATEDONE_SPEC>;
#[doc = "Calibration is complete"]
pub mod events_calibratedone;
#[doc = "EVENTS_STOPPED (rw) register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "The ADC has stopped"]
pub mod events_stopped;
#[doc = "Peripheral events."]
pub use events_ch::EVENTS_CH;
#[doc = r"Cluster"]
#[doc = "Peripheral events."]
pub mod events_ch;
#[doc = "PUBLISH_STARTED (rw) register accessor: an alias for `Reg<PUBLISH_STARTED_SPEC>`"]
pub type PUBLISH_STARTED = crate::Reg<publish_started::PUBLISH_STARTED_SPEC>;
#[doc = "Publish configuration for event STARTED"]
pub mod publish_started;
#[doc = "PUBLISH_END (rw) register accessor: an alias for `Reg<PUBLISH_END_SPEC>`"]
pub type PUBLISH_END = crate::Reg<publish_end::PUBLISH_END_SPEC>;
#[doc = "Publish configuration for event END"]
pub mod publish_end;
#[doc = "PUBLISH_DONE (rw) register accessor: an alias for `Reg<PUBLISH_DONE_SPEC>`"]
pub type PUBLISH_DONE = crate::Reg<publish_done::PUBLISH_DONE_SPEC>;
#[doc = "Publish configuration for event DONE"]
pub mod publish_done;
#[doc = "PUBLISH_RESULTDONE (rw) register accessor: an alias for `Reg<PUBLISH_RESULTDONE_SPEC>`"]
pub type PUBLISH_RESULTDONE = crate::Reg<publish_resultdone::PUBLISH_RESULTDONE_SPEC>;
#[doc = "Publish configuration for event RESULTDONE"]
pub mod publish_resultdone;
#[doc = "PUBLISH_CALIBRATEDONE (rw) register accessor: an alias for `Reg<PUBLISH_CALIBRATEDONE_SPEC>`"]
pub type PUBLISH_CALIBRATEDONE = crate::Reg<publish_calibratedone::PUBLISH_CALIBRATEDONE_SPEC>;
#[doc = "Publish configuration for event CALIBRATEDONE"]
pub mod publish_calibratedone;
#[doc = "PUBLISH_STOPPED (rw) register accessor: an alias for `Reg<PUBLISH_STOPPED_SPEC>`"]
pub type PUBLISH_STOPPED = crate::Reg<publish_stopped::PUBLISH_STOPPED_SPEC>;
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "Publish configuration for events"]
pub use publish_ch::PUBLISH_CH;
#[doc = r"Cluster"]
#[doc = "Publish configuration for events"]
pub mod publish_ch;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable or disable ADC"]
pub mod enable;
#[doc = "Unspecified"]
pub use ch::CH;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod ch;
#[doc = "RESOLUTION (rw) register accessor: an alias for `Reg<RESOLUTION_SPEC>`"]
pub type RESOLUTION = crate::Reg<resolution::RESOLUTION_SPEC>;
#[doc = "Resolution configuration"]
pub mod resolution;
#[doc = "OVERSAMPLE (rw) register accessor: an alias for `Reg<OVERSAMPLE_SPEC>`"]
pub type OVERSAMPLE = crate::Reg<oversample::OVERSAMPLE_SPEC>;
#[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
pub mod oversample;
#[doc = "SAMPLERATE (rw) register accessor: an alias for `Reg<SAMPLERATE_SPEC>`"]
pub type SAMPLERATE = crate::Reg<samplerate::SAMPLERATE_SPEC>;
#[doc = "Controls normal or continuous sample rate"]
pub mod samplerate;
#[doc = "RESULT EasyDMA channel"]
pub use result::RESULT;
#[doc = r"Cluster"]
#[doc = "RESULT EasyDMA channel"]
pub mod result;
