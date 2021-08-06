#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start comparator"]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x04 - Stop comparator"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    #[doc = "0x08 - Sample comparator value"]
    pub tasks_sample: crate::Reg<tasks_sample::TASKS_SAMPLE_SPEC>,
    _reserved3: [u8; 0x74],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>,
    #[doc = "0x88 - Subscribe configuration for task SAMPLE"]
    pub subscribe_sample: crate::Reg<subscribe_sample::SUBSCRIBE_SAMPLE_SPEC>,
    _reserved6: [u8; 0x74],
    #[doc = "0x100 - LPCOMP is ready and output is valid"]
    pub events_ready: crate::Reg<events_ready::EVENTS_READY_SPEC>,
    #[doc = "0x104 - Downward crossing"]
    pub events_down: crate::Reg<events_down::EVENTS_DOWN_SPEC>,
    #[doc = "0x108 - Upward crossing"]
    pub events_up: crate::Reg<events_up::EVENTS_UP_SPEC>,
    #[doc = "0x10c - Downward or upward crossing"]
    pub events_cross: crate::Reg<events_cross::EVENTS_CROSS_SPEC>,
    _reserved10: [u8; 0x70],
    #[doc = "0x180 - Publish configuration for event READY"]
    pub publish_ready: crate::Reg<publish_ready::PUBLISH_READY_SPEC>,
    #[doc = "0x184 - Publish configuration for event DOWN"]
    pub publish_down: crate::Reg<publish_down::PUBLISH_DOWN_SPEC>,
    #[doc = "0x188 - Publish configuration for event UP"]
    pub publish_up: crate::Reg<publish_up::PUBLISH_UP_SPEC>,
    #[doc = "0x18c - Publish configuration for event CROSS"]
    pub publish_cross: crate::Reg<publish_cross::PUBLISH_CROSS_SPEC>,
    _reserved14: [u8; 0x70],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved15: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved17: [u8; 0xf4],
    #[doc = "0x400 - Compare result"]
    pub result: crate::Reg<result::RESULT_SPEC>,
    _reserved18: [u8; 0xfc],
    #[doc = "0x500 - Enable LPCOMP"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x504 - Input pin select"]
    pub psel: crate::Reg<psel::PSEL_SPEC>,
    #[doc = "0x508 - Reference select"]
    pub refsel: crate::Reg<refsel::REFSEL_SPEC>,
    #[doc = "0x50c - External reference select"]
    pub extrefsel: crate::Reg<extrefsel::EXTREFSEL_SPEC>,
    _reserved22: [u8; 0x10],
    #[doc = "0x520 - Analog detect configuration"]
    pub anadetect: crate::Reg<anadetect::ANADETECT_SPEC>,
    _reserved23: [u8; 0x14],
    #[doc = "0x538 - Comparator hysteresis enable"]
    pub hyst: crate::Reg<hyst::HYST_SPEC>,
}
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start comparator"]
pub mod tasks_start;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop comparator"]
pub mod tasks_stop;
#[doc = "TASKS_SAMPLE register accessor: an alias for `Reg<TASKS_SAMPLE_SPEC>`"]
pub type TASKS_SAMPLE = crate::Reg<tasks_sample::TASKS_SAMPLE_SPEC>;
#[doc = "Sample comparator value"]
pub mod tasks_sample;
#[doc = "SUBSCRIBE_START register accessor: an alias for `Reg<SUBSCRIBE_START_SPEC>`"]
pub type SUBSCRIBE_START = crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "SUBSCRIBE_SAMPLE register accessor: an alias for `Reg<SUBSCRIBE_SAMPLE_SPEC>`"]
pub type SUBSCRIBE_SAMPLE = crate::Reg<subscribe_sample::SUBSCRIBE_SAMPLE_SPEC>;
#[doc = "Subscribe configuration for task SAMPLE"]
pub mod subscribe_sample;
#[doc = "EVENTS_READY register accessor: an alias for `Reg<EVENTS_READY_SPEC>`"]
pub type EVENTS_READY = crate::Reg<events_ready::EVENTS_READY_SPEC>;
#[doc = "LPCOMP is ready and output is valid"]
pub mod events_ready;
#[doc = "EVENTS_DOWN register accessor: an alias for `Reg<EVENTS_DOWN_SPEC>`"]
pub type EVENTS_DOWN = crate::Reg<events_down::EVENTS_DOWN_SPEC>;
#[doc = "Downward crossing"]
pub mod events_down;
#[doc = "EVENTS_UP register accessor: an alias for `Reg<EVENTS_UP_SPEC>`"]
pub type EVENTS_UP = crate::Reg<events_up::EVENTS_UP_SPEC>;
#[doc = "Upward crossing"]
pub mod events_up;
#[doc = "EVENTS_CROSS register accessor: an alias for `Reg<EVENTS_CROSS_SPEC>`"]
pub type EVENTS_CROSS = crate::Reg<events_cross::EVENTS_CROSS_SPEC>;
#[doc = "Downward or upward crossing"]
pub mod events_cross;
#[doc = "PUBLISH_READY register accessor: an alias for `Reg<PUBLISH_READY_SPEC>`"]
pub type PUBLISH_READY = crate::Reg<publish_ready::PUBLISH_READY_SPEC>;
#[doc = "Publish configuration for event READY"]
pub mod publish_ready;
#[doc = "PUBLISH_DOWN register accessor: an alias for `Reg<PUBLISH_DOWN_SPEC>`"]
pub type PUBLISH_DOWN = crate::Reg<publish_down::PUBLISH_DOWN_SPEC>;
#[doc = "Publish configuration for event DOWN"]
pub mod publish_down;
#[doc = "PUBLISH_UP register accessor: an alias for `Reg<PUBLISH_UP_SPEC>`"]
pub type PUBLISH_UP = crate::Reg<publish_up::PUBLISH_UP_SPEC>;
#[doc = "Publish configuration for event UP"]
pub mod publish_up;
#[doc = "PUBLISH_CROSS register accessor: an alias for `Reg<PUBLISH_CROSS_SPEC>`"]
pub type PUBLISH_CROSS = crate::Reg<publish_cross::PUBLISH_CROSS_SPEC>;
#[doc = "Publish configuration for event CROSS"]
pub mod publish_cross;
#[doc = "SHORTS register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "RESULT register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Compare result"]
pub mod result;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable LPCOMP"]
pub mod enable;
#[doc = "PSEL register accessor: an alias for `Reg<PSEL_SPEC>`"]
pub type PSEL = crate::Reg<psel::PSEL_SPEC>;
#[doc = "Input pin select"]
pub mod psel;
#[doc = "REFSEL register accessor: an alias for `Reg<REFSEL_SPEC>`"]
pub type REFSEL = crate::Reg<refsel::REFSEL_SPEC>;
#[doc = "Reference select"]
pub mod refsel;
#[doc = "EXTREFSEL register accessor: an alias for `Reg<EXTREFSEL_SPEC>`"]
pub type EXTREFSEL = crate::Reg<extrefsel::EXTREFSEL_SPEC>;
#[doc = "External reference select"]
pub mod extrefsel;
#[doc = "ANADETECT register accessor: an alias for `Reg<ANADETECT_SPEC>`"]
pub type ANADETECT = crate::Reg<anadetect::ANADETECT_SPEC>;
#[doc = "Analog detect configuration"]
pub mod anadetect;
#[doc = "HYST register accessor: an alias for `Reg<HYST_SPEC>`"]
pub type HYST = crate::Reg<hyst::HYST_SPEC>;
#[doc = "Comparator hysteresis enable"]
pub mod hyst;
