#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the random number generator."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop the random number generator."]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - New random number generated and written to VALUE register."]
    pub events_valrdy: EVENTS_VALRDY,
    _reserved3: [u8; 0xfc],
    #[doc = "0x200 - Shortcuts for the RNG."]
    pub shorts: SHORTS,
    _reserved4: [u8; 0x0100],
    #[doc = "0x304 - Interrupt enable set register"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register"]
    pub intenclr: INTENCLR,
    _reserved6: [u8; 0x01f8],
    #[doc = "0x504 - Configuration register."]
    pub config: CONFIG,
    #[doc = "0x508 - RNG random number."]
    pub value: VALUE,
    _reserved8: [u8; 0x0af0],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start the random number generator."]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop the random number generator."]
pub mod tasks_stop;
#[doc = "EVENTS_VALRDY (rw) register accessor: an alias for `Reg<EVENTS_VALRDY_SPEC>`"]
pub type EVENTS_VALRDY = crate::Reg<events_valrdy::EVENTS_VALRDY_SPEC>;
#[doc = "New random number generated and written to VALUE register."]
pub mod events_valrdy;
#[doc = "SHORTS (rw) register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts for the RNG."]
pub mod shorts;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register"]
pub mod intenclr;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register."]
pub mod config;
#[doc = "VALUE (r) register accessor: an alias for `Reg<VALUE_SPEC>`"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "RNG random number."]
pub mod value;
#[doc = "POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
