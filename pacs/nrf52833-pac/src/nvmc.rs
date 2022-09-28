#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400 - Ready flag"]
    pub ready: READY,
    _reserved1: [u8; 0x04],
    #[doc = "0x408 - Ready flag"]
    pub readynext: READYNEXT,
    _reserved2: [u8; 0xf8],
    #[doc = "0x504 - Configuration register"]
    pub config: CONFIG,
    _reserved_3_erasepage: [u8; 0x04],
    #[doc = "0x50c - Register for erasing all non-volatile user memory"]
    pub eraseall: ERASEALL,
    #[doc = "0x510 - Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
    pub erasepcr0: ERASEPCR0,
    #[doc = "0x514 - Register for erasing user information configuration registers"]
    pub eraseuicr: ERASEUICR,
    #[doc = "0x518 - Register for partial erase of a page in code area"]
    pub erasepagepartial: ERASEPAGEPARTIAL,
    #[doc = "0x51c - Register for partial erase configuration"]
    pub erasepagepartialcfg: ERASEPAGEPARTIALCFG,
    _reserved9: [u8; 0x20],
    #[doc = "0x540 - I-code cache configuration register"]
    pub icachecnf: ICACHECNF,
    _reserved10: [u8; 0x04],
    #[doc = "0x548 - I-code cache hit counter"]
    pub ihit: IHIT,
    #[doc = "0x54c - I-code cache miss counter"]
    pub imiss: IMISS,
}
impl RegisterBlock {
    #[doc = "0x508 - Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
    #[inline(always)]
    pub fn erasepcr1(&self) -> &ERASEPCR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(1288usize) as *const ERASEPCR1) }
    }
    #[doc = "0x508 - Register for erasing a page in code area"]
    #[inline(always)]
    pub fn erasepage(&self) -> &ERASEPAGE {
        unsafe { &*(((self as *const Self) as *const u8).add(1288usize) as *const ERASEPAGE) }
    }
}
#[doc = "READY (r) register accessor: an alias for `Reg<READY_SPEC>`"]
pub type READY = crate::Reg<ready::READY_SPEC>;
#[doc = "Ready flag"]
pub mod ready;
#[doc = "READYNEXT (r) register accessor: an alias for `Reg<READYNEXT_SPEC>`"]
pub type READYNEXT = crate::Reg<readynext::READYNEXT_SPEC>;
#[doc = "Ready flag"]
pub mod readynext;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "ERASEPAGE (w) register accessor: an alias for `Reg<ERASEPAGE_SPEC>`"]
pub type ERASEPAGE = crate::Reg<erasepage::ERASEPAGE_SPEC>;
#[doc = "Register for erasing a page in code area"]
pub mod erasepage;
#[doc = "ERASEPCR1 (w) register accessor: an alias for `Reg<ERASEPCR1_SPEC>`"]
pub type ERASEPCR1 = crate::Reg<erasepcr1::ERASEPCR1_SPEC>;
#[doc = "Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
pub mod erasepcr1;
#[doc = "ERASEALL (w) register accessor: an alias for `Reg<ERASEALL_SPEC>`"]
pub type ERASEALL = crate::Reg<eraseall::ERASEALL_SPEC>;
#[doc = "Register for erasing all non-volatile user memory"]
pub mod eraseall;
#[doc = "ERASEPCR0 (w) register accessor: an alias for `Reg<ERASEPCR0_SPEC>`"]
pub type ERASEPCR0 = crate::Reg<erasepcr0::ERASEPCR0_SPEC>;
#[doc = "Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
pub mod erasepcr0;
#[doc = "ERASEUICR (w) register accessor: an alias for `Reg<ERASEUICR_SPEC>`"]
pub type ERASEUICR = crate::Reg<eraseuicr::ERASEUICR_SPEC>;
#[doc = "Register for erasing user information configuration registers"]
pub mod eraseuicr;
#[doc = "ERASEPAGEPARTIAL (w) register accessor: an alias for `Reg<ERASEPAGEPARTIAL_SPEC>`"]
pub type ERASEPAGEPARTIAL = crate::Reg<erasepagepartial::ERASEPAGEPARTIAL_SPEC>;
#[doc = "Register for partial erase of a page in code area"]
pub mod erasepagepartial;
#[doc = "ERASEPAGEPARTIALCFG (rw) register accessor: an alias for `Reg<ERASEPAGEPARTIALCFG_SPEC>`"]
pub type ERASEPAGEPARTIALCFG = crate::Reg<erasepagepartialcfg::ERASEPAGEPARTIALCFG_SPEC>;
#[doc = "Register for partial erase configuration"]
pub mod erasepagepartialcfg;
#[doc = "ICACHECNF (rw) register accessor: an alias for `Reg<ICACHECNF_SPEC>`"]
pub type ICACHECNF = crate::Reg<icachecnf::ICACHECNF_SPEC>;
#[doc = "I-code cache configuration register"]
pub mod icachecnf;
#[doc = "IHIT (rw) register accessor: an alias for `Reg<IHIT_SPEC>`"]
pub type IHIT = crate::Reg<ihit::IHIT_SPEC>;
#[doc = "I-code cache hit counter"]
pub mod ihit;
#[doc = "IMISS (rw) register accessor: an alias for `Reg<IMISS_SPEC>`"]
pub type IMISS = crate::Reg<imiss::IMISS_SPEC>;
#[doc = "I-code cache miss counter"]
pub mod imiss;
