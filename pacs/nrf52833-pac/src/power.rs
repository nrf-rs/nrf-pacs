#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x78],
    #[doc = "0x78 - Enable Constant Latency mode"]
    pub tasks_constlat: crate::Reg<tasks_constlat::TASKS_CONSTLAT_SPEC>,
    #[doc = "0x7c - Enable Low-power mode (variable latency)"]
    pub tasks_lowpwr: crate::Reg<tasks_lowpwr::TASKS_LOWPWR_SPEC>,
    _reserved2: [u8; 0x88],
    #[doc = "0x108 - Power failure warning"]
    pub events_pofwarn: crate::Reg<events_pofwarn::EVENTS_POFWARN_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x114 - CPU entered WFI/WFE sleep"]
    pub events_sleepenter: crate::Reg<events_sleepenter::EVENTS_SLEEPENTER_SPEC>,
    #[doc = "0x118 - CPU exited WFI/WFE sleep"]
    pub events_sleepexit: crate::Reg<events_sleepexit::EVENTS_SLEEPEXIT_SPEC>,
    #[doc = "0x11c - Voltage supply detected on VBUS"]
    pub events_usbdetected: crate::Reg<events_usbdetected::EVENTS_USBDETECTED_SPEC>,
    #[doc = "0x120 - Voltage supply removed from VBUS"]
    pub events_usbremoved: crate::Reg<events_usbremoved::EVENTS_USBREMOVED_SPEC>,
    #[doc = "0x124 - USB 3.3 V supply ready"]
    pub events_usbpwrrdy: crate::Reg<events_usbpwrrdy::EVENTS_USBPWRRDY_SPEC>,
    _reserved8: [u8; 0x01dc],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved10: [u8; 0xf4],
    #[doc = "0x400 - Reset reason"]
    pub resetreas: crate::Reg<resetreas::RESETREAS_SPEC>,
    _reserved11: [u8; 0x24],
    #[doc = "0x428 - Deprecated register - RAM status register"]
    pub ramstatus: crate::Reg<ramstatus::RAMSTATUS_SPEC>,
    _reserved12: [u8; 0x0c],
    #[doc = "0x438 - USB supply status"]
    pub usbregstatus: crate::Reg<usbregstatus::USBREGSTATUS_SPEC>,
    _reserved13: [u8; 0xc4],
    #[doc = "0x500 - System OFF register"]
    pub systemoff: crate::Reg<systemoff::SYSTEMOFF_SPEC>,
    _reserved14: [u8; 0x0c],
    #[doc = "0x510 - Power-fail comparator configuration"]
    pub pofcon: crate::Reg<pofcon::POFCON_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x51c - General purpose retention register"]
    pub gpregret: crate::Reg<gpregret::GPREGRET_SPEC>,
    #[doc = "0x520 - General purpose retention register"]
    pub gpregret2: crate::Reg<gpregret2::GPREGRET2_SPEC>,
    _reserved17: [u8; 0x54],
    #[doc = "0x578 - Enable DC/DC converter for REG1 stage"]
    pub dcdcen: crate::Reg<dcdcen::DCDCEN_SPEC>,
    _reserved18: [u8; 0xc4],
    #[doc = "0x640 - Main supply status"]
    pub mainregstatus: crate::Reg<mainregstatus::MAINREGSTATUS_SPEC>,
    _reserved19: [u8; 0x02bc],
    #[doc = "0x900..0x90c - Unspecified"]
    pub ram0: RAM,
    _reserved20: [u8; 0x04],
    #[doc = "0x910..0x91c - Unspecified"]
    pub ram1: RAM,
    _reserved21: [u8; 0x04],
    #[doc = "0x920..0x92c - Unspecified"]
    pub ram2: RAM,
    _reserved22: [u8; 0x04],
    #[doc = "0x930..0x93c - Unspecified"]
    pub ram3: RAM,
    _reserved23: [u8; 0x04],
    #[doc = "0x940..0x94c - Unspecified"]
    pub ram4: RAM,
    _reserved24: [u8; 0x04],
    #[doc = "0x950..0x95c - Unspecified"]
    pub ram5: RAM,
    _reserved25: [u8; 0x04],
    #[doc = "0x960..0x96c - Unspecified"]
    pub ram6: RAM,
    _reserved26: [u8; 0x04],
    #[doc = "0x970..0x97c - Unspecified"]
    pub ram7: RAM,
    _reserved27: [u8; 0x04],
    #[doc = "0x980..0x98c - Unspecified"]
    pub ram8: RAM,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct RAM {
    #[doc = "0x00 - Description cluster: RAMn power control register"]
    pub power: crate::Reg<self::ram::power::POWER_SPEC>,
    #[doc = "0x04 - Description cluster: RAMn power control set register"]
    pub powerset: crate::Reg<self::ram::powerset::POWERSET_SPEC>,
    #[doc = "0x08 - Description cluster: RAMn power control clear register"]
    pub powerclr: crate::Reg<self::ram::powerclr::POWERCLR_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod ram;
#[doc = "TASKS_CONSTLAT register accessor: an alias for `Reg<TASKS_CONSTLAT_SPEC>`"]
pub type TASKS_CONSTLAT = crate::Reg<tasks_constlat::TASKS_CONSTLAT_SPEC>;
#[doc = "Enable Constant Latency mode"]
pub mod tasks_constlat;
#[doc = "TASKS_LOWPWR register accessor: an alias for `Reg<TASKS_LOWPWR_SPEC>`"]
pub type TASKS_LOWPWR = crate::Reg<tasks_lowpwr::TASKS_LOWPWR_SPEC>;
#[doc = "Enable Low-power mode (variable latency)"]
pub mod tasks_lowpwr;
#[doc = "EVENTS_POFWARN register accessor: an alias for `Reg<EVENTS_POFWARN_SPEC>`"]
pub type EVENTS_POFWARN = crate::Reg<events_pofwarn::EVENTS_POFWARN_SPEC>;
#[doc = "Power failure warning"]
pub mod events_pofwarn;
#[doc = "EVENTS_SLEEPENTER register accessor: an alias for `Reg<EVENTS_SLEEPENTER_SPEC>`"]
pub type EVENTS_SLEEPENTER = crate::Reg<events_sleepenter::EVENTS_SLEEPENTER_SPEC>;
#[doc = "CPU entered WFI/WFE sleep"]
pub mod events_sleepenter;
#[doc = "EVENTS_SLEEPEXIT register accessor: an alias for `Reg<EVENTS_SLEEPEXIT_SPEC>`"]
pub type EVENTS_SLEEPEXIT = crate::Reg<events_sleepexit::EVENTS_SLEEPEXIT_SPEC>;
#[doc = "CPU exited WFI/WFE sleep"]
pub mod events_sleepexit;
#[doc = "EVENTS_USBDETECTED register accessor: an alias for `Reg<EVENTS_USBDETECTED_SPEC>`"]
pub type EVENTS_USBDETECTED = crate::Reg<events_usbdetected::EVENTS_USBDETECTED_SPEC>;
#[doc = "Voltage supply detected on VBUS"]
pub mod events_usbdetected;
#[doc = "EVENTS_USBREMOVED register accessor: an alias for `Reg<EVENTS_USBREMOVED_SPEC>`"]
pub type EVENTS_USBREMOVED = crate::Reg<events_usbremoved::EVENTS_USBREMOVED_SPEC>;
#[doc = "Voltage supply removed from VBUS"]
pub mod events_usbremoved;
#[doc = "EVENTS_USBPWRRDY register accessor: an alias for `Reg<EVENTS_USBPWRRDY_SPEC>`"]
pub type EVENTS_USBPWRRDY = crate::Reg<events_usbpwrrdy::EVENTS_USBPWRRDY_SPEC>;
#[doc = "USB 3.3 V supply ready"]
pub mod events_usbpwrrdy;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "RESETREAS register accessor: an alias for `Reg<RESETREAS_SPEC>`"]
pub type RESETREAS = crate::Reg<resetreas::RESETREAS_SPEC>;
#[doc = "Reset reason"]
pub mod resetreas;
#[doc = "RAMSTATUS register accessor: an alias for `Reg<RAMSTATUS_SPEC>`"]
pub type RAMSTATUS = crate::Reg<ramstatus::RAMSTATUS_SPEC>;
#[doc = "Deprecated register - RAM status register"]
pub mod ramstatus;
#[doc = "USBREGSTATUS register accessor: an alias for `Reg<USBREGSTATUS_SPEC>`"]
pub type USBREGSTATUS = crate::Reg<usbregstatus::USBREGSTATUS_SPEC>;
#[doc = "USB supply status"]
pub mod usbregstatus;
#[doc = "SYSTEMOFF register accessor: an alias for `Reg<SYSTEMOFF_SPEC>`"]
pub type SYSTEMOFF = crate::Reg<systemoff::SYSTEMOFF_SPEC>;
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "POFCON register accessor: an alias for `Reg<POFCON_SPEC>`"]
pub type POFCON = crate::Reg<pofcon::POFCON_SPEC>;
#[doc = "Power-fail comparator configuration"]
pub mod pofcon;
#[doc = "GPREGRET register accessor: an alias for `Reg<GPREGRET_SPEC>`"]
pub type GPREGRET = crate::Reg<gpregret::GPREGRET_SPEC>;
#[doc = "General purpose retention register"]
pub mod gpregret;
#[doc = "GPREGRET2 register accessor: an alias for `Reg<GPREGRET2_SPEC>`"]
pub type GPREGRET2 = crate::Reg<gpregret2::GPREGRET2_SPEC>;
#[doc = "General purpose retention register"]
pub mod gpregret2;
#[doc = "DCDCEN register accessor: an alias for `Reg<DCDCEN_SPEC>`"]
pub type DCDCEN = crate::Reg<dcdcen::DCDCEN_SPEC>;
#[doc = "Enable DC/DC converter for REG1 stage"]
pub mod dcdcen;
#[doc = "MAINREGSTATUS register accessor: an alias for `Reg<MAINREGSTATUS_SPEC>`"]
pub type MAINREGSTATUS = crate::Reg<mainregstatus::MAINREGSTATUS_SPEC>;
#[doc = "Main supply status"]
pub mod mainregstatus;
