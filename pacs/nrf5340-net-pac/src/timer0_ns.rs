#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start Timer"]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x04 - Stop Timer"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    #[doc = "0x08 - Increment Timer (Counter mode only)"]
    pub tasks_count: crate::Reg<tasks_count::TASKS_COUNT_SPEC>,
    #[doc = "0x0c - Clear time"]
    pub tasks_clear: crate::Reg<tasks_clear::TASKS_CLEAR_SPEC>,
    #[doc = "0x10 - Deprecated register - Shut down timer"]
    pub tasks_shutdown: crate::Reg<tasks_shutdown::TASKS_SHUTDOWN_SPEC>,
    _reserved5: [u8; 0x2c],
    #[doc = "0x40..0x60 - Description collection: Capture Timer value to CC\\[n\\]
register"]
    pub tasks_capture: [crate::Reg<tasks_capture::TASKS_CAPTURE_SPEC>; 8],
    _reserved6: [u8; 0x20],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>,
    #[doc = "0x88 - Subscribe configuration for task COUNT"]
    pub subscribe_count: crate::Reg<subscribe_count::SUBSCRIBE_COUNT_SPEC>,
    #[doc = "0x8c - Subscribe configuration for task CLEAR"]
    pub subscribe_clear: crate::Reg<subscribe_clear::SUBSCRIBE_CLEAR_SPEC>,
    #[doc = "0x90 - Deprecated register - Subscribe configuration for task SHUTDOWN"]
    pub subscribe_shutdown: crate::Reg<subscribe_shutdown::SUBSCRIBE_SHUTDOWN_SPEC>,
    _reserved11: [u8; 0x2c],
    #[doc = "0xc0..0xe0 - Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
    pub subscribe_capture: [crate::Reg<subscribe_capture::SUBSCRIBE_CAPTURE_SPEC>; 8],
    _reserved12: [u8; 0x60],
    #[doc = "0x140..0x160 - Description collection: Compare event on CC\\[n\\]
match"]
    pub events_compare: [crate::Reg<events_compare::EVENTS_COMPARE_SPEC>; 8],
    _reserved13: [u8; 0x60],
    #[doc = "0x1c0..0x1e0 - Description collection: Publish configuration for event COMPARE\\[n\\]"]
    pub publish_compare: [crate::Reg<publish_compare::PUBLISH_COMPARE_SPEC>; 8],
    _reserved14: [u8; 0x20],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved15: [u8; 0xfc],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: crate::Reg<inten::INTEN_SPEC>,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved18: [u8; 0x01f8],
    #[doc = "0x504 - Timer mode selection"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x508 - Configure the number of bits used by the TIMER"]
    pub bitmode: crate::Reg<bitmode::BITMODE_SPEC>,
    _reserved20: [u8; 0x04],
    #[doc = "0x510 - Timer prescaler register"]
    pub prescaler: crate::Reg<prescaler::PRESCALER_SPEC>,
    _reserved21: [u8; 0x2c],
    #[doc = "0x540..0x560 - Description collection: Capture/Compare register n"]
    pub cc: [crate::Reg<cc::CC_SPEC>; 8],
    _reserved22: [u8; 0x20],
    #[doc = "0x580..0x5a0 - Description collection: Enable one-shot operation for Capture/Compare channel n"]
    pub oneshoten: [crate::Reg<oneshoten::ONESHOTEN_SPEC>; 8],
}
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start Timer"]
pub mod tasks_start;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop Timer"]
pub mod tasks_stop;
#[doc = "TASKS_COUNT register accessor: an alias for `Reg<TASKS_COUNT_SPEC>`"]
pub type TASKS_COUNT = crate::Reg<tasks_count::TASKS_COUNT_SPEC>;
#[doc = "Increment Timer (Counter mode only)"]
pub mod tasks_count;
#[doc = "TASKS_CLEAR register accessor: an alias for `Reg<TASKS_CLEAR_SPEC>`"]
pub type TASKS_CLEAR = crate::Reg<tasks_clear::TASKS_CLEAR_SPEC>;
#[doc = "Clear time"]
pub mod tasks_clear;
#[doc = "TASKS_SHUTDOWN register accessor: an alias for `Reg<TASKS_SHUTDOWN_SPEC>`"]
pub type TASKS_SHUTDOWN = crate::Reg<tasks_shutdown::TASKS_SHUTDOWN_SPEC>;
#[doc = "Deprecated register - Shut down timer"]
pub mod tasks_shutdown;
#[doc = "TASKS_CAPTURE register accessor: an alias for `Reg<TASKS_CAPTURE_SPEC>`"]
pub type TASKS_CAPTURE = crate::Reg<tasks_capture::TASKS_CAPTURE_SPEC>;
#[doc = "Description collection: Capture Timer value to CC\\[n\\]
register"]
pub mod tasks_capture;
#[doc = "SUBSCRIBE_START register accessor: an alias for `Reg<SUBSCRIBE_START_SPEC>`"]
pub type SUBSCRIBE_START = crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "SUBSCRIBE_COUNT register accessor: an alias for `Reg<SUBSCRIBE_COUNT_SPEC>`"]
pub type SUBSCRIBE_COUNT = crate::Reg<subscribe_count::SUBSCRIBE_COUNT_SPEC>;
#[doc = "Subscribe configuration for task COUNT"]
pub mod subscribe_count;
#[doc = "SUBSCRIBE_CLEAR register accessor: an alias for `Reg<SUBSCRIBE_CLEAR_SPEC>`"]
pub type SUBSCRIBE_CLEAR = crate::Reg<subscribe_clear::SUBSCRIBE_CLEAR_SPEC>;
#[doc = "Subscribe configuration for task CLEAR"]
pub mod subscribe_clear;
#[doc = "SUBSCRIBE_SHUTDOWN register accessor: an alias for `Reg<SUBSCRIBE_SHUTDOWN_SPEC>`"]
pub type SUBSCRIBE_SHUTDOWN = crate::Reg<subscribe_shutdown::SUBSCRIBE_SHUTDOWN_SPEC>;
#[doc = "Deprecated register - Subscribe configuration for task SHUTDOWN"]
pub mod subscribe_shutdown;
#[doc = "SUBSCRIBE_CAPTURE register accessor: an alias for `Reg<SUBSCRIBE_CAPTURE_SPEC>`"]
pub type SUBSCRIBE_CAPTURE = crate::Reg<subscribe_capture::SUBSCRIBE_CAPTURE_SPEC>;
#[doc = "Description collection: Subscribe configuration for task CAPTURE\\[n\\]"]
pub mod subscribe_capture;
#[doc = "EVENTS_COMPARE register accessor: an alias for `Reg<EVENTS_COMPARE_SPEC>`"]
pub type EVENTS_COMPARE = crate::Reg<events_compare::EVENTS_COMPARE_SPEC>;
#[doc = "Description collection: Compare event on CC\\[n\\]
match"]
pub mod events_compare;
#[doc = "PUBLISH_COMPARE register accessor: an alias for `Reg<PUBLISH_COMPARE_SPEC>`"]
pub type PUBLISH_COMPARE = crate::Reg<publish_compare::PUBLISH_COMPARE_SPEC>;
#[doc = "Description collection: Publish configuration for event COMPARE\\[n\\]"]
pub mod publish_compare;
#[doc = "SHORTS register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
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
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Timer mode selection"]
pub mod mode;
#[doc = "BITMODE register accessor: an alias for `Reg<BITMODE_SPEC>`"]
pub type BITMODE = crate::Reg<bitmode::BITMODE_SPEC>;
#[doc = "Configure the number of bits used by the TIMER"]
pub mod bitmode;
#[doc = "PRESCALER register accessor: an alias for `Reg<PRESCALER_SPEC>`"]
pub type PRESCALER = crate::Reg<prescaler::PRESCALER_SPEC>;
#[doc = "Timer prescaler register"]
pub mod prescaler;
#[doc = "CC register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Description collection: Capture/Compare register n"]
pub mod cc;
#[doc = "ONESHOTEN register accessor: an alias for `Reg<ONESHOTEN_SPEC>`"]
pub type ONESHOTEN = crate::Reg<oneshoten::ONESHOTEN_SPEC>;
#[doc = "Description collection: Enable one-shot operation for Capture/Compare channel n"]
pub mod oneshoten;
