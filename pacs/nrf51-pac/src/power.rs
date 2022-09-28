#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x78],
    #[doc = "0x78 - Enable constant latency mode."]
    pub tasks_constlat: TASKS_CONSTLAT,
    #[doc = "0x7c - Enable low power mode (variable latency)."]
    pub tasks_lowpwr: TASKS_LOWPWR,
    _reserved2: [u8; 0x88],
    #[doc = "0x108 - Power failure warning."]
    pub events_pofwarn: EVENTS_POFWARN,
    _reserved3: [u8; 0x01f8],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 0xf4],
    #[doc = "0x400 - Reset reason."]
    pub resetreas: RESETREAS,
    _reserved6: [u8; 0x24],
    #[doc = "0x428 - Ram status register."]
    pub ramstatus: RAMSTATUS,
    _reserved7: [u8; 0xd4],
    #[doc = "0x500 - System off register."]
    pub systemoff: SYSTEMOFF,
    _reserved8: [u8; 0x0c],
    #[doc = "0x510 - Power failure configuration."]
    pub pofcon: POFCON,
    _reserved9: [u8; 0x08],
    #[doc = "0x51c - General purpose retention register. This register is a retained register."]
    pub gpregret: GPREGRET,
    _reserved10: [u8; 0x04],
    #[doc = "0x524 - Ram on/off."]
    pub ramon: RAMON,
    _reserved11: [u8; 0x1c],
    #[doc = "0x544 - Pin reset functionality configuration register. This register is a retained register."]
    pub reset: RESET,
    _reserved12: [u8; 0x0c],
    #[doc = "0x554 - Ram on/off."]
    pub ramonb: RAMONB,
    _reserved13: [u8; 0x20],
    #[doc = "0x578 - DCDC converter enable configuration register."]
    pub dcdcen: DCDCEN,
    _reserved14: [u8; 0x048c],
    #[doc = "0xa08 - DCDC power-up force register."]
    pub dcdcforce: DCDCFORCE,
}
#[doc = "TASKS_CONSTLAT (w) register accessor: an alias for `Reg<TASKS_CONSTLAT_SPEC>`"]
pub type TASKS_CONSTLAT = crate::Reg<tasks_constlat::TASKS_CONSTLAT_SPEC>;
#[doc = "Enable constant latency mode."]
pub mod tasks_constlat;
#[doc = "TASKS_LOWPWR (w) register accessor: an alias for `Reg<TASKS_LOWPWR_SPEC>`"]
pub type TASKS_LOWPWR = crate::Reg<tasks_lowpwr::TASKS_LOWPWR_SPEC>;
#[doc = "Enable low power mode (variable latency)."]
pub mod tasks_lowpwr;
#[doc = "EVENTS_POFWARN (rw) register accessor: an alias for `Reg<EVENTS_POFWARN_SPEC>`"]
pub type EVENTS_POFWARN = crate::Reg<events_pofwarn::EVENTS_POFWARN_SPEC>;
#[doc = "Power failure warning."]
pub mod events_pofwarn;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "RESETREAS (rw) register accessor: an alias for `Reg<RESETREAS_SPEC>`"]
pub type RESETREAS = crate::Reg<resetreas::RESETREAS_SPEC>;
#[doc = "Reset reason."]
pub mod resetreas;
#[doc = "RAMSTATUS (r) register accessor: an alias for `Reg<RAMSTATUS_SPEC>`"]
pub type RAMSTATUS = crate::Reg<ramstatus::RAMSTATUS_SPEC>;
#[doc = "Ram status register."]
pub mod ramstatus;
#[doc = "SYSTEMOFF (w) register accessor: an alias for `Reg<SYSTEMOFF_SPEC>`"]
pub type SYSTEMOFF = crate::Reg<systemoff::SYSTEMOFF_SPEC>;
#[doc = "System off register."]
pub mod systemoff;
#[doc = "POFCON (rw) register accessor: an alias for `Reg<POFCON_SPEC>`"]
pub type POFCON = crate::Reg<pofcon::POFCON_SPEC>;
#[doc = "Power failure configuration."]
pub mod pofcon;
#[doc = "GPREGRET (rw) register accessor: an alias for `Reg<GPREGRET_SPEC>`"]
pub type GPREGRET = crate::Reg<gpregret::GPREGRET_SPEC>;
#[doc = "General purpose retention register. This register is a retained register."]
pub mod gpregret;
#[doc = "RAMON (rw) register accessor: an alias for `Reg<RAMON_SPEC>`"]
pub type RAMON = crate::Reg<ramon::RAMON_SPEC>;
#[doc = "Ram on/off."]
pub mod ramon;
#[doc = "RESET (rw) register accessor: an alias for `Reg<RESET_SPEC>`"]
pub type RESET = crate::Reg<reset::RESET_SPEC>;
#[doc = "Pin reset functionality configuration register. This register is a retained register."]
pub mod reset;
#[doc = "RAMONB (rw) register accessor: an alias for `Reg<RAMONB_SPEC>`"]
pub type RAMONB = crate::Reg<ramonb::RAMONB_SPEC>;
#[doc = "Ram on/off."]
pub mod ramonb;
#[doc = "DCDCEN (rw) register accessor: an alias for `Reg<DCDCEN_SPEC>`"]
pub type DCDCEN = crate::Reg<dcdcen::DCDCEN_SPEC>;
#[doc = "DCDC converter enable configuration register."]
pub mod dcdcen;
#[doc = "DCDCFORCE (rw) register accessor: an alias for `Reg<DCDCFORCE_SPEC>`"]
pub type DCDCFORCE = crate::Reg<dcdcforce::DCDCFORCE_SPEC>;
#[doc = "DCDC power-up force register."]
pub mod dcdcforce;
