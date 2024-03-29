#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start an ADC conversion."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop ADC."]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - ADC conversion complete."]
    pub events_end: EVENTS_END,
    _reserved3: [u8; 0x0200],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 0xf4],
    #[doc = "0x400 - ADC busy register."]
    pub busy: BUSY,
    _reserved6: [u8; 0xfc],
    #[doc = "0x500 - ADC enable."]
    pub enable: ENABLE,
    #[doc = "0x504 - ADC configuration register."]
    pub config: CONFIG,
    #[doc = "0x508 - Result of ADC conversion."]
    pub result: RESULT,
    _reserved9: [u8; 0x0af0],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start an ADC conversion."]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop ADC."]
pub mod tasks_stop;
#[doc = "EVENTS_END (rw) register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "ADC conversion complete."]
pub mod events_end;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "BUSY (r) register accessor: an alias for `Reg<BUSY_SPEC>`"]
pub type BUSY = crate::Reg<busy::BUSY_SPEC>;
#[doc = "ADC busy register."]
pub mod busy;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "ADC enable."]
pub mod enable;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "ADC configuration register."]
pub mod config;
#[doc = "RESULT (r) register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Result of ADC conversion."]
pub mod result;
#[doc = "POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
