#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start WDT"]
    pub tasks_start: crate::Reg<tasks_start::TASKS_START_SPEC>,
    #[doc = "0x04 - Stop WDT"]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    _reserved2: [u8; 0x78],
    #[doc = "0x80 - Subscribe configuration for task START"]
    pub subscribe_start: crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>,
    #[doc = "0x84 - Subscribe configuration for task STOP"]
    pub subscribe_stop: crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>,
    _reserved4: [u8; 0x78],
    #[doc = "0x100 - Watchdog timeout"]
    pub events_timeout: crate::Reg<events_timeout::EVENTS_TIMEOUT_SPEC>,
    #[doc = "0x104 - Watchdog stopped"]
    pub events_stopped: crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>,
    _reserved6: [u8; 0x78],
    #[doc = "0x180 - Publish configuration for event TIMEOUT"]
    pub publish_timeout: crate::Reg<publish_timeout::PUBLISH_TIMEOUT_SPEC>,
    #[doc = "0x184 - Publish configuration for event STOPPED"]
    pub publish_stopped: crate::Reg<publish_stopped::PUBLISH_STOPPED_SPEC>,
    _reserved8: [u8; 0x017c],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved10: [u8; 0x18],
    #[doc = "0x324 - Enable interrupt"]
    pub nmienset: crate::Reg<nmienset::NMIENSET_SPEC>,
    #[doc = "0x328 - Disable interrupt"]
    pub nmienclr: crate::Reg<nmienclr::NMIENCLR_SPEC>,
    _reserved12: [u8; 0xd4],
    #[doc = "0x400 - Run status"]
    pub runstatus: crate::Reg<runstatus::RUNSTATUS_SPEC>,
    #[doc = "0x404 - Request status"]
    pub reqstatus: crate::Reg<reqstatus::REQSTATUS_SPEC>,
    _reserved14: [u8; 0xfc],
    #[doc = "0x504 - Counter reload value"]
    pub crv: crate::Reg<crv::CRV_SPEC>,
    #[doc = "0x508 - Enable register for reload request registers"]
    pub rren: crate::Reg<rren::RREN_SPEC>,
    #[doc = "0x50c - Configuration register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved17: [u8; 0x10],
    #[doc = "0x520 - Task stop enable"]
    pub tsen: crate::Reg<tsen::TSEN_SPEC>,
    _reserved18: [u8; 0xdc],
    #[doc = "0x600..0x620 - Description collection: Reload request n"]
    pub rr: [crate::Reg<rr::RR_SPEC>; 8],
}
#[doc = "TASKS_START register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start WDT"]
pub mod tasks_start;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop WDT"]
pub mod tasks_stop;
#[doc = "SUBSCRIBE_START register accessor: an alias for `Reg<SUBSCRIBE_START_SPEC>`"]
pub type SUBSCRIBE_START = crate::Reg<subscribe_start::SUBSCRIBE_START_SPEC>;
#[doc = "Subscribe configuration for task START"]
pub mod subscribe_start;
#[doc = "SUBSCRIBE_STOP register accessor: an alias for `Reg<SUBSCRIBE_STOP_SPEC>`"]
pub type SUBSCRIBE_STOP = crate::Reg<subscribe_stop::SUBSCRIBE_STOP_SPEC>;
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "EVENTS_TIMEOUT register accessor: an alias for `Reg<EVENTS_TIMEOUT_SPEC>`"]
pub type EVENTS_TIMEOUT = crate::Reg<events_timeout::EVENTS_TIMEOUT_SPEC>;
#[doc = "Watchdog timeout"]
pub mod events_timeout;
#[doc = "EVENTS_STOPPED register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "Watchdog stopped"]
pub mod events_stopped;
#[doc = "PUBLISH_TIMEOUT register accessor: an alias for `Reg<PUBLISH_TIMEOUT_SPEC>`"]
pub type PUBLISH_TIMEOUT = crate::Reg<publish_timeout::PUBLISH_TIMEOUT_SPEC>;
#[doc = "Publish configuration for event TIMEOUT"]
pub mod publish_timeout;
#[doc = "PUBLISH_STOPPED register accessor: an alias for `Reg<PUBLISH_STOPPED_SPEC>`"]
pub type PUBLISH_STOPPED = crate::Reg<publish_stopped::PUBLISH_STOPPED_SPEC>;
#[doc = "Publish configuration for event STOPPED"]
pub mod publish_stopped;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "NMIENSET register accessor: an alias for `Reg<NMIENSET_SPEC>`"]
pub type NMIENSET = crate::Reg<nmienset::NMIENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod nmienset;
#[doc = "NMIENCLR register accessor: an alias for `Reg<NMIENCLR_SPEC>`"]
pub type NMIENCLR = crate::Reg<nmienclr::NMIENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod nmienclr;
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
#[doc = "TSEN register accessor: an alias for `Reg<TSEN_SPEC>`"]
pub type TSEN = crate::Reg<tsen::TSEN_SPEC>;
#[doc = "Task stop enable"]
pub mod tsen;
#[doc = "RR register accessor: an alias for `Reg<RR_SPEC>`"]
pub type RR = crate::Reg<rr::RR_SPEC>;
#[doc = "Description collection: Reload request n"]
pub mod rr;
