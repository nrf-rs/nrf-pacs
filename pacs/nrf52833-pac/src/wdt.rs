#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the watchdog"]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100 - Watchdog timeout"]
    pub events_timeout: crate::Reg<events_timeout::EVENTS_TIMEOUT_SPEC>,
    _reserved2: [u8; 0x0200],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved4: [u8; 0xf4],
    #[doc = "0x400 - Run status"]
    pub runstatus: crate::Reg<runstatus::RUNSTATUS_SPEC>,
    #[doc = "0x404 - Request status"]
    pub reqstatus: crate::Reg<reqstatus::REQSTATUS_SPEC>,
    _reserved6: [u8; 0xfc],
    #[doc = "0x504 - Counter reload value"]
    pub crv: crate::Reg<crv::CRV_SPEC>,
    #[doc = "0x508 - Enable register for reload request registers"]
    pub rren: crate::Reg<rren::RREN_SPEC>,
    #[doc = "0x50c - Configuration register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved9: [u8; 0xf0],
    #[doc = "0x600..0x620 - Description collection: Reload request n"]
    pub rr: [crate::Reg<rr::RR_SPEC>; 8],
}
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start the watchdog"]
pub mod tasks_start;
#[doc = "EVENTS_TIMEOUT register accessor: an alias for `Reg<EVENTS_TIMEOUT_SPEC>`"]
pub type EVENTS_TIMEOUT = crate::Reg<events_timeout::EVENTS_TIMEOUT_SPEC>;
#[doc = "Watchdog timeout"]
pub mod events_timeout;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "RUNSTATUS register accessor: an alias for `Reg<RUNSTATUS_SPEC>`"]
pub type RUNSTATUS = crate::Reg<runstatus::RUNSTATUS_SPEC>;
#[doc = "Run status"]
pub mod runstatus;
#[doc = "REQSTATUS register accessor: an alias for `Reg<REQSTATUS_SPEC>`"]
pub type REQSTATUS = crate::Reg<reqstatus::REQSTATUS_SPEC>;
#[doc = "Request status"]
pub mod reqstatus;
#[doc = "CRV register accessor: an alias for `Reg<CRV_SPEC>`"]
pub type CRV = crate::Reg<crv::CRV_SPEC>;
#[doc = "Counter reload value"]
pub mod crv;
#[doc = "RREN register accessor: an alias for `Reg<RREN_SPEC>`"]
pub type RREN = crate::Reg<rren::RREN_SPEC>;
#[doc = "Enable register for reload request registers"]
pub mod rren;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "RR register accessor: an alias for `Reg<RR_SPEC>`"]
pub type RR = crate::Reg<rr::RR_SPEC>;
#[doc = "Description collection: Reload request n"]
pub mod rr;
