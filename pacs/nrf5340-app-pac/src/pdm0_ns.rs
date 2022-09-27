#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Starts continuous PDM transfer"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stops PDM transfer"]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 0x78],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    _reserved4: [u8; 0x78],
    #[doc = "0x100 - PDM transfer has started"]
    pub events_started: EVENTS_STARTED,
    #[doc = "0x104 - PDM transfer has finished"]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x108 - The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
    pub events_end: EVENTS_END,
    _reserved7: [u8; 0x74],
    #[doc = "0x180 - Publish configuration for event STARTED"]
    pub publish_started: PUBLISH_STARTED,
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    pub publish_stopped: PUBLISH_STOPPED,
    #[doc = "0x188 - Publish configuration for event END"]
    pub publish_end: PUBLISH_END,
    _reserved10: [u8; 0x0174],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved13: [u8; 0x01f4],
    #[doc = "0x500 - PDM module enable register"]
    pub enable: ENABLE,
    #[doc = "0x504 - PDM clock generator control"]
    pub pdmclkctrl: PDMCLKCTRL,
    #[doc = "0x508 - Defines the routing of the connected PDM microphones' signals"]
    pub mode: MODE,
    _reserved16: [u8; 0x0c],
    #[doc = "0x518 - Left output gain adjustment"]
    pub gainl: GAINL,
    #[doc = "0x51c - Right output gain adjustment"]
    pub gainr: GAINR,
    #[doc = "0x520 - Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly."]
    pub ratio: RATIO,
    _reserved19: [u8; 0x1c],
    #[doc = "0x540..0x548 - Unspecified"]
    pub psel: PSEL,
    _reserved20: [u8; 0x04],
    #[doc = "0x54c - Master clock generator configuration"]
    pub mclkconfig: MCLKCONFIG,
    _reserved21: [u8; 0x10],
    #[doc = "0x560..0x568 - Unspecified"]
    pub sample: SAMPLE,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Starts continuous PDM transfer"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stops PDM transfer"]
pub mod tasks_stop;
#[doc = "SUBSCRIBE_START (rw) register accessor: an alias for `Reg<SUBSCRIBE_START_SPEC>`"]
pub type SUBSCRIBE_START = crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "EVENTS_STARTED (rw) register accessor: an alias for `Reg<EVENTS_STARTED_SPEC>`"]
pub type EVENTS_STARTED = crate::Reg<events_started::EVENTS_STARTED_SPEC>;
#[doc = "PDM transfer has started"]
pub mod events_started;
#[doc = "EVENTS_STOPPED (rw) register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "PDM transfer has finished"]
pub mod events_stopped;
#[doc = "EVENTS_END (rw) register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
pub mod events_end;
#[doc = "PUBLISH_STARTED (rw) register accessor: an alias for `Reg<PUBLISH_STARTED_SPEC>`"]
pub type PUBLISH_STARTED = crate::Reg<publish_started::PUBLISH_STARTED_SPEC>;
#[doc = "Publish configuration for event STARTED"]
pub mod publish_started;
#[doc = "PUBLISH_STOPPED (rw) register accessor: an alias for `Reg<PUBLISH_STOPPED_SPEC>`"]
pub type PUBLISH_STOPPED = crate::Reg<publish_stopped::PUBLISH_STOPPED_SPEC>;
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "PUBLISH_END (rw) register accessor: an alias for `Reg<PUBLISH_END_SPEC>`"]
pub type PUBLISH_END = crate::Reg<publish_end::PUBLISH_END_SPEC>;
#[doc = "Publish configuration for event END"]
pub mod publish_end;
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
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "PDM module enable register"]
pub mod enable;
#[doc = "PDMCLKCTRL (rw) register accessor: an alias for `Reg<PDMCLKCTRL_SPEC>`"]
pub type PDMCLKCTRL = crate::Reg<pdmclkctrl::PDMCLKCTRL_SPEC>;
#[doc = "PDM clock generator control"]
pub mod pdmclkctrl;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Defines the routing of the connected PDM microphones' signals"]
pub mod mode;
#[doc = "GAINL (rw) register accessor: an alias for `Reg<GAINL_SPEC>`"]
pub type GAINL = crate::Reg<gainl::GAINL_SPEC>;
#[doc = "Left output gain adjustment"]
pub mod gainl;
#[doc = "GAINR (rw) register accessor: an alias for `Reg<GAINR_SPEC>`"]
pub type GAINR = crate::Reg<gainr::GAINR_SPEC>;
#[doc = "Right output gain adjustment"]
pub mod gainr;
#[doc = "RATIO (rw) register accessor: an alias for `Reg<RATIO_SPEC>`"]
pub type RATIO = crate::Reg<ratio::RATIO_SPEC>;
#[doc = "Selects the ratio between PDM_CLK and output sample rate. Change PDMCLKCTRL accordingly."]
pub mod ratio;
#[doc = "Unspecified"]
pub use psel::PSEL;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "MCLKCONFIG (rw) register accessor: an alias for `Reg<MCLKCONFIG_SPEC>`"]
pub type MCLKCONFIG = crate::Reg<mclkconfig::MCLKCONFIG_SPEC>;
#[doc = "Master clock generator configuration"]
pub mod mclkconfig;
#[doc = "Unspecified"]
pub use sample::SAMPLE;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod sample;
