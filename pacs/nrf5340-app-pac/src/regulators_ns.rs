#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0428],
    #[doc = "0x428 - Main supply status"]
    pub mainregstatus: crate::Reg<mainregstatus::MAINREGSTATUS_SPEC>,
    _reserved1: [u8; 0xd4],
    #[doc = "0x500 - System OFF register"]
    pub systemoff: crate::Reg<systemoff::SYSTEMOFF_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x510 - Power-fail comparator configuration"]
    pub pofcon: crate::Reg<pofcon::POFCON_SPEC>,
    _reserved3: [u8; 0x01f0],
    #[doc = "0x704 - Unspecified"]
    pub vregmain: VREGMAIN,
    _reserved4: [u8; 0x01f8],
    #[doc = "0x900..0x908 - Unspecified"]
    pub vregradio: VREGRADIO,
    _reserved5: [u8; 0x01f8],
    #[doc = "0xb00 - Unspecified"]
    pub vregh: VREGH,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct VREGMAIN {
    #[doc = "0x00 - DC/DC enable register for VREGMAIN"]
    pub dcdcen: crate::Reg<self::vregmain::dcdcen::DCDCEN_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod vregmain;
#[doc = r"Register block"]
#[repr(C)]
pub struct VREGRADIO {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - DC/DC enable register for VREGRADIO"]
    pub dcdcen: crate::Reg<self::vregradio::dcdcen::DCDCEN_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod vregradio;
#[doc = r"Register block"]
#[repr(C)]
pub struct VREGH {
    #[doc = "0x00 - DC/DC enable register for VREGH"]
    pub dcdcen: crate::Reg<self::vregh::dcdcen::DCDCEN_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod vregh;
#[doc = "MAINREGSTATUS register accessor: an alias for `Reg<MAINREGSTATUS_SPEC>`"]
pub type MAINREGSTATUS = crate::Reg<mainregstatus::MAINREGSTATUS_SPEC>;
#[doc = "Main supply status"]
pub mod mainregstatus;
#[doc = "SYSTEMOFF register accessor: an alias for `Reg<SYSTEMOFF_SPEC>`"]
pub type SYSTEMOFF = crate::Reg<systemoff::SYSTEMOFF_SPEC>;
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "POFCON register accessor: an alias for `Reg<POFCON_SPEC>`"]
pub type POFCON = crate::Reg<pofcon::POFCON_SPEC>;
#[doc = "Power-fail comparator configuration"]
pub mod pofcon;
