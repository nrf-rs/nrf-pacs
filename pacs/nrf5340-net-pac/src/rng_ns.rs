#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Task starting the random number generator"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Task stopping the random number generator"]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 0x78],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: SUBSCRIBE_START,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    _reserved4: [u8; 0x78],
    #[doc = "0x100 - Event being generated for every new random number written to the VALUE register"]
    pub events_valrdy: EVENTS_VALRDY,
    _reserved5: [u8; 0x7c],
    #[doc = "0x180 - Publish configuration for event VALRDY"]
    pub publish_valrdy: PUBLISH_VALRDY,
    _reserved6: [u8; 0x7c],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved7: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 0x01f8],
    #[doc = "0x504 - Configuration register"]
    pub config: CONFIG,
    #[doc = "0x508 - Output random number"]
    pub value: VALUE,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Task starting the random number generator"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Task stopping the random number generator"]
pub mod tasks_stop;
#[doc = "SUBSCRIBE_START (rw) register accessor: an alias for `Reg<SUBSCRIBE_START_SPEC>`"]
pub type SUBSCRIBE_START = crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP (rw) register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "EVENTS_VALRDY (rw) register accessor: an alias for `Reg<EVENTS_VALRDY_SPEC>`"]
pub type EVENTS_VALRDY = crate::Reg<events_valrdy::EVENTS_VALRDY_SPEC>;
#[doc = "Event being generated for every new random number written to the VALUE register"]
pub mod events_valrdy;
#[doc = "PUBLISH_VALRDY (rw) register accessor: an alias for `Reg<PUBLISH_VALRDY_SPEC>`"]
pub type PUBLISH_VALRDY = crate::Reg<publish_valrdy::PUBLISH_VALRDY_SPEC>;
#[doc = "Publish configuration for event VALRDY"]
pub mod publish_valrdy;
#[doc = "SHORTS (rw) register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "VALUE (r) register accessor: an alias for `Reg<VALUE_SPEC>`"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "Output random number"]
pub mod value;
