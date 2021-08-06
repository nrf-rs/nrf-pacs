#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start Timer."]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x04 - Stop Timer."]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    #[doc = "0x08 - Increment Timer (In counter mode)."]
    pub tasks_count: crate::Reg<tasks_count::TASKS_COUNT_SPEC>,
    #[doc = "0x0c - Clear timer."]
    pub tasks_clear: crate::Reg<tasks_clear::TASKS_CLEAR_SPEC>,
    #[doc = "0x10 - Shutdown timer."]
    pub tasks_shutdown: crate::Reg<tasks_shutdown::TASKS_SHUTDOWN_SPEC>,
    _reserved5: [u8; 0x2c],
    #[doc = "0x40..0x50 - Capture Timer value to CC\\[n\\]
registers."]
    pub tasks_capture: [crate::Reg<tasks_capture::TASKS_CAPTURE_SPEC>; 4],
    _reserved6: [u8; 0xf0],
    #[doc = "0x140..0x150 - Compare event on CC\\[n\\]
match."]
    pub events_compare: [crate::Reg<events_compare::EVENTS_COMPARE_SPEC>; 4],
    _reserved7: [u8; 0xb0],
    #[doc = "0x200 - Shortcuts for Timer."]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved8: [u8; 0x0100],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved10: [u8; 0x01f8],
    #[doc = "0x504 - Timer Mode selection."]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x508 - Sets timer behaviour."]
    pub bitmode: crate::Reg<bitmode::BITMODE_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x510 - 4-bit prescaler to source clock frequency (max value 9). Source clock frequency is divided by 2^SCALE."]
    pub prescaler: crate::Reg<prescaler::PRESCALER_SPEC>,
    _reserved13: [u8; 0x2c],
    #[doc = "0x540..0x550 - Capture/compare registers."]
    pub cc: [crate::Reg<cc::CC_SPEC>; 4],
    _reserved14: [u8; 0x0aac],
    #[doc = "0xffc - Peripheral power control."]
    pub power: crate::Reg<power::POWER_SPEC>,
}
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start Timer."]
pub mod tasks_start;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop Timer."]
pub mod tasks_stop;
#[doc = "TASKS_COUNT register accessor: an alias for `Reg<TASKS_COUNT_SPEC>`"]
pub type TASKS_COUNT = crate::Reg<tasks_count::TASKS_COUNT_SPEC>;
#[doc = "Increment Timer (In counter mode)."]
pub mod tasks_count;
#[doc = "TASKS_CLEAR register accessor: an alias for `Reg<TASKS_CLEAR_SPEC>`"]
pub type TASKS_CLEAR = crate::Reg<tasks_clear::TASKS_CLEAR_SPEC>;
#[doc = "Clear timer."]
pub mod tasks_clear;
#[doc = "TASKS_SHUTDOWN register accessor: an alias for `Reg<TASKS_SHUTDOWN_SPEC>`"]
pub type TASKS_SHUTDOWN = crate::Reg<tasks_shutdown::TASKS_SHUTDOWN_SPEC>;
#[doc = "Shutdown timer."]
pub mod tasks_shutdown;
#[doc = "TASKS_CAPTURE register accessor: an alias for `Reg<TASKS_CAPTURE_SPEC>`"]
pub type TASKS_CAPTURE = crate::Reg<tasks_capture::TASKS_CAPTURE_SPEC>;
#[doc = "Capture Timer value to CC\\[n\\]
registers."]
pub mod tasks_capture;
#[doc = "EVENTS_COMPARE register accessor: an alias for `Reg<EVENTS_COMPARE_SPEC>`"]
pub type EVENTS_COMPARE = crate::Reg<events_compare::EVENTS_COMPARE_SPEC>;
#[doc = "Compare event on CC\\[n\\]
match."]
pub mod events_compare;
#[doc = "SHORTS register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts for Timer."]
pub mod shorts;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Timer Mode selection."]
pub mod mode;
#[doc = "BITMODE register accessor: an alias for `Reg<BITMODE_SPEC>`"]
pub type BITMODE = crate::Reg<bitmode::BITMODE_SPEC>;
#[doc = "Sets timer behaviour."]
pub mod bitmode;
#[doc = "PRESCALER register accessor: an alias for `Reg<PRESCALER_SPEC>`"]
pub type PRESCALER = crate::Reg<prescaler::PRESCALER_SPEC>;
#[doc = "4-bit prescaler to source clock frequency (max value 9). Source clock frequency is divided by 2^SCALE."]
pub mod prescaler;
#[doc = "CC register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Capture/compare registers."]
pub mod cc;
#[doc = "POWER register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
