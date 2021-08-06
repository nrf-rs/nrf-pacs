#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x10 - Tasks asssociated with GPIOTE channels."]
    pub tasks_out: [crate::Reg<tasks_out::TASKS_OUT_SPEC>; 4],
    _reserved1: [u8; 0xf0],
    #[doc = "0x100..0x110 - Tasks asssociated with GPIOTE channels."]
    pub events_in: [crate::Reg<events_in::EVENTS_IN_SPEC>; 4],
    _reserved2: [u8; 0x6c],
    #[doc = "0x17c - Event generated from multiple pins."]
    pub events_port: crate::Reg<events_port::EVENTS_PORT_SPEC>,
    _reserved3: [u8; 0x0184],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved5: [u8; 0x0204],
    #[doc = "0x510..0x520 - Channel configuration registers."]
    pub config: [crate::Reg<config::CONFIG_SPEC>; 4],
    _reserved6: [u8; 0x0adc],
    #[doc = "0xffc - Peripheral power control."]
    pub power: crate::Reg<power::POWER_SPEC>,
}
#[doc = "TASKS_OUT register accessor: an alias for `Reg<TASKS_OUT_SPEC>`"]
pub type TASKS_OUT = crate::Reg<tasks_out::TASKS_OUT_SPEC>;
#[doc = "Tasks asssociated with GPIOTE channels."]
pub mod tasks_out;
#[doc = "EVENTS_IN register accessor: an alias for `Reg<EVENTS_IN_SPEC>`"]
pub type EVENTS_IN = crate::Reg<events_in::EVENTS_IN_SPEC>;
#[doc = "Tasks asssociated with GPIOTE channels."]
pub mod events_in;
#[doc = "EVENTS_PORT register accessor: an alias for `Reg<EVENTS_PORT_SPEC>`"]
pub type EVENTS_PORT = crate::Reg<events_port::EVENTS_PORT_SPEC>;
#[doc = "Event generated from multiple pins."]
pub mod events_port;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Channel configuration registers."]
pub mod config;
#[doc = "POWER register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
