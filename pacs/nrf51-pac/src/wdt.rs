#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the watchdog."]
    pub tasks_start: TASKS_START,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100 - Watchdog timeout."]
    pub events_timeout: EVENTS_TIMEOUT,
    _reserved2: [u8; 0x0200],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved4: [u8; 0xf4],
    #[doc = "0x400 - Watchdog running status."]
    pub runstatus: RUNSTATUS,
    #[doc = "0x404 - Request status."]
    pub reqstatus: REQSTATUS,
    _reserved6: [u8; 0xfc],
    #[doc = "0x504 - Counter reload value in number of 32kiHz clock cycles."]
    pub crv: CRV,
    #[doc = "0x508 - Reload request enable."]
    pub rren: RREN,
    #[doc = "0x50c - Configuration register."]
    pub config: CONFIG,
    _reserved9: [u8; 0xf0],
    #[doc = "0x600..0x620 - Reload requests registers."]
    pub rr: [RR; 8],
    _reserved10: [u8; 0x09dc],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start the watchdog."]
pub mod tasks_start;
#[doc = "EVENTS_TIMEOUT (rw) register accessor: an alias for `Reg<EVENTS_TIMEOUT_SPEC>`"]
pub type EVENTS_TIMEOUT = crate::Reg<events_timeout::EVENTS_TIMEOUT_SPEC>;
#[doc = "Watchdog timeout."]
pub mod events_timeout;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "RUNSTATUS (r) register accessor: an alias for `Reg<RUNSTATUS_SPEC>`"]
pub type RUNSTATUS = crate::Reg<runstatus::RUNSTATUS_SPEC>;
#[doc = "Watchdog running status."]
pub mod runstatus;
#[doc = "REQSTATUS (r) register accessor: an alias for `Reg<REQSTATUS_SPEC>`"]
pub type REQSTATUS = crate::Reg<reqstatus::REQSTATUS_SPEC>;
#[doc = "Request status."]
pub mod reqstatus;
#[doc = "CRV (rw) register accessor: an alias for `Reg<CRV_SPEC>`"]
pub type CRV = crate::Reg<crv::CRV_SPEC>;
#[doc = "Counter reload value in number of 32kiHz clock cycles."]
pub mod crv;
#[doc = "RREN (rw) register accessor: an alias for `Reg<RREN_SPEC>`"]
pub type RREN = crate::Reg<rren::RREN_SPEC>;
#[doc = "Reload request enable."]
pub mod rren;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register."]
pub mod config;
#[doc = "RR (w) register accessor: an alias for `Reg<RR_SPEC>`"]
pub type RR = crate::Reg<rr::RR_SPEC>;
#[doc = "Reload requests registers."]
pub mod rr;
#[doc = "POWER (rw) register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
