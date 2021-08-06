#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start comparator"]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x04 - Stop comparator"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    #[doc = "0x08 - Sample comparator value"]
    pub tasks_sample: crate::Reg<tasks_sample::TASKS_SAMPLE_SPEC>,
    _reserved3: [u8; 0xf4],
    #[doc = "0x100 - COMP is ready and output is valid"]
    pub events_ready: crate::Reg<events_ready::EVENTS_READY_SPEC>,
    #[doc = "0x104 - Downward crossing"]
    pub events_down: crate::Reg<events_down::EVENTS_DOWN_SPEC>,
    #[doc = "0x108 - Upward crossing"]
    pub events_up: crate::Reg<events_up::EVENTS_UP_SPEC>,
    #[doc = "0x10c - Downward or upward crossing"]
    pub events_cross: crate::Reg<events_cross::EVENTS_CROSS_SPEC>,
    _reserved7: [u8; 0xf0],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved8: [u8; 0xfc],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved11: [u8; 0xf4],
    #[doc = "0x400 - Compare result"]
    pub result: crate::Reg<result::RESULT_SPEC>,
    _reserved12: [u8; 0xfc],
    #[doc = "0x500 - COMP enable"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x504 - Pin select"]
    pub psel: crate::Reg<psel::PSEL_SPEC>,
    #[doc = "0x508 - Reference source select for single-ended mode"]
    pub refsel: crate::Reg<refsel::REFSEL_SPEC>,
    #[doc = "0x50c - External reference select"]
    pub extrefsel: crate::Reg<extrefsel::EXTREFSEL_SPEC>,
    _reserved16: [u8; 0x20],
    #[doc = "0x530 - Threshold configuration for hysteresis unit"]
    pub th: crate::Reg<th::TH_SPEC>,
    #[doc = "0x534 - Mode configuration"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
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
#[doc = "EVENTS_READY register accessor: an alias for `Reg<EVENTS_READY_SPEC>`"]
pub type EVENTS_READY = crate::Reg<events_ready::EVENTS_READY_SPEC>;
#[doc = "COMP is ready and output is valid"]
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
#[doc = "SHORTS register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcut register"]
pub mod shorts;
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
#[doc = "RESULT register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Compare result"]
pub mod result;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "COMP enable"]
pub mod enable;
#[doc = "PSEL register accessor: an alias for `Reg<PSEL_SPEC>`"]
pub type PSEL = crate::Reg<psel::PSEL_SPEC>;
#[doc = "Pin select"]
pub mod psel;
#[doc = "REFSEL register accessor: an alias for `Reg<REFSEL_SPEC>`"]
pub type REFSEL = crate::Reg<refsel::REFSEL_SPEC>;
#[doc = "Reference source select for single-ended mode"]
pub mod refsel;
#[doc = "EXTREFSEL register accessor: an alias for `Reg<EXTREFSEL_SPEC>`"]
pub type EXTREFSEL = crate::Reg<extrefsel::EXTREFSEL_SPEC>;
#[doc = "External reference select"]
pub mod extrefsel;
#[doc = "TH register accessor: an alias for `Reg<TH_SPEC>`"]
pub type TH = crate::Reg<th::TH_SPEC>;
#[doc = "Threshold configuration for hysteresis unit"]
pub mod th;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Mode configuration"]
pub mod mode;
#[doc = "HYST register accessor: an alias for `Reg<HYST_SPEC>`"]
pub type HYST = crate::Reg<hyst::HYST_SPEC>;
#[doc = "Comparator hysteresis enable"]
pub mod hyst;
