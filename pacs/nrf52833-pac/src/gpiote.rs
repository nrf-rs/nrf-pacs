#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
    pub tasks_out: [crate::Reg<tasks_out::TASKS_OUT_SPEC>; 8],
    _reserved1: [u8; 0x10],
    #[doc = "0x30..0x50 - Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high."]
    pub tasks_set: [crate::Reg<tasks_set::TASKS_SET_SPEC>; 8],
    _reserved2: [u8; 0x10],
    #[doc = "0x60..0x80 - Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low."]
    pub tasks_clr: [crate::Reg<tasks_clr::TASKS_CLR_SPEC>; 8],
    _reserved3: [u8; 0x80],
    #[doc = "0x100..0x120 - Description collection: Event generated from pin specified in CONFIG\\[n\\].PSEL"]
    pub events_in: [crate::Reg<events_in::EVENTS_IN_SPEC>; 8],
    _reserved4: [u8; 0x5c],
    #[doc = "0x17c - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
    pub events_port: crate::Reg<events_port::EVENTS_PORT_SPEC>,
    _reserved5: [u8; 0x0184],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved7: [u8; 0x0204],
    #[doc = "0x510..0x530 - Description collection: Configuration for OUT\\[n\\], SET\\[n\\], and CLR\\[n\\]
tasks and IN\\[n\\]
event"]
    pub config: [crate::Reg<config::CONFIG_SPEC>; 8],
}
#[doc = "TASKS_OUT register accessor: an alias for `Reg<TASKS_OUT_SPEC>`"]
pub type TASKS_OUT = crate::Reg<tasks_out::TASKS_OUT_SPEC>;
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
pub mod tasks_out;
#[doc = "TASKS_SET register accessor: an alias for `Reg<TASKS_SET_SPEC>`"]
pub type TASKS_SET = crate::Reg<tasks_set::TASKS_SET_SPEC>;
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it high."]
pub mod tasks_set;
#[doc = "TASKS_CLR register accessor: an alias for `Reg<TASKS_CLR_SPEC>`"]
pub type TASKS_CLR = crate::Reg<tasks_clr::TASKS_CLR_SPEC>;
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is to set it low."]
pub mod tasks_clr;
#[doc = "EVENTS_IN register accessor: an alias for `Reg<EVENTS_IN_SPEC>`"]
pub type EVENTS_IN = crate::Reg<events_in::EVENTS_IN_SPEC>;
#[doc = "Description collection: Event generated from pin specified in CONFIG\\[n\\].PSEL"]
pub mod events_in;
#[doc = "EVENTS_PORT register accessor: an alias for `Reg<EVENTS_PORT_SPEC>`"]
pub type EVENTS_PORT = crate::Reg<events_port::EVENTS_PORT_SPEC>;
#[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
pub mod events_port;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\], and CLR\\[n\\]
tasks and IN\\[n\\]
event"]
pub mod config;
