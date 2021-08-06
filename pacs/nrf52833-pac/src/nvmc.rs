#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400 - Ready flag"]
    pub ready: crate::Reg<ready::READY_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x408 - Ready flag"]
    pub readynext: crate::Reg<readynext::READYNEXT_SPEC>,
    _reserved2: [u8; 0xf8],
    #[doc = "0x504 - Configuration register"]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved_3_erasepage: [u8; 0x04],
    #[doc = "0x50c - Register for erasing all non-volatile user memory"]
    pub eraseall: crate::Reg<eraseall::ERASEALL_SPEC>,
    #[doc = "0x510 - Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
    pub erasepcr0: crate::Reg<erasepcr0::ERASEPCR0_SPEC>,
    #[doc = "0x514 - Register for erasing user information configuration registers"]
    pub eraseuicr: crate::Reg<eraseuicr::ERASEUICR_SPEC>,
    #[doc = "0x518 - Register for partial erase of a page in code area"]
    pub erasepagepartial: crate::Reg<erasepagepartial::ERASEPAGEPARTIAL_SPEC>,
    #[doc = "0x51c - Register for partial erase configuration"]
    pub erasepagepartialcfg: crate::Reg<erasepagepartialcfg::ERASEPAGEPARTIALCFG_SPEC>,
    _reserved9: [u8; 0x20],
    #[doc = "0x540 - I-code cache configuration register"]
    pub icachecnf: crate::Reg<icachecnf::ICACHECNF_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x548 - I-code cache hit counter"]
    pub ihit: crate::Reg<ihit::IHIT_SPEC>,
    #[doc = "0x54c - I-code cache miss counter"]
    pub imiss: crate::Reg<imiss::IMISS_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x508 - Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
    #[inline(always)]
    pub fn erasepcr1(&self) -> &crate::Reg<erasepcr1::ERASEPCR1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1288usize)
                as *const crate::Reg<erasepcr1::ERASEPCR1_SPEC>)
        }
    }
    #[doc = "0x508 - Register for erasing a page in code area"]
    #[inline(always)]
    pub fn erasepage(&self) -> &crate::Reg<erasepage::ERASEPAGE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1288usize)
                as *const crate::Reg<erasepage::ERASEPAGE_SPEC>)
        }
    }
}
#[doc = "READY register accessor: an alias for `Reg<READY_SPEC>`"]
pub type READY = crate::Reg<ready::READY_SPEC>;
#[doc = "Ready flag"]
pub mod ready;
#[doc = "READYNEXT register accessor: an alias for `Reg<READYNEXT_SPEC>`"]
pub type READYNEXT = crate::Reg<readynext::READYNEXT_SPEC>;
#[doc = "Ready flag"]
pub mod readynext;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "ERASEPAGE register accessor: an alias for `Reg<ERASEPAGE_SPEC>`"]
pub type ERASEPAGE = crate::Reg<erasepage::ERASEPAGE_SPEC>;
#[doc = "Register for erasing a page in code area"]
pub mod erasepage;
#[doc = "ERASEPCR1 register accessor: an alias for `Reg<ERASEPCR1_SPEC>`"]
pub type ERASEPCR1 = crate::Reg<erasepcr1::ERASEPCR1_SPEC>;
#[doc = "Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
pub mod erasepcr1;
#[doc = "ERASEALL register accessor: an alias for `Reg<ERASEALL_SPEC>`"]
pub type ERASEALL = crate::Reg<eraseall::ERASEALL_SPEC>;
#[doc = "Register for erasing all non-volatile user memory"]
pub mod eraseall;
#[doc = "ERASEPCR0 register accessor: an alias for `Reg<ERASEPCR0_SPEC>`"]
pub type ERASEPCR0 = crate::Reg<erasepcr0::ERASEPCR0_SPEC>;
#[doc = "Deprecated register - Register for erasing a page in code area, equivalent to ERASEPAGE"]
pub mod erasepcr0;
#[doc = "ERASEUICR register accessor: an alias for `Reg<ERASEUICR_SPEC>`"]
pub type ERASEUICR = crate::Reg<eraseuicr::ERASEUICR_SPEC>;
#[doc = "Register for erasing user information configuration registers"]
pub mod eraseuicr;
#[doc = "ERASEPAGEPARTIAL register accessor: an alias for `Reg<ERASEPAGEPARTIAL_SPEC>`"]
pub type ERASEPAGEPARTIAL = crate::Reg<erasepagepartial::ERASEPAGEPARTIAL_SPEC>;
#[doc = "Register for partial erase of a page in code area"]
pub mod erasepagepartial;
#[doc = "ERASEPAGEPARTIALCFG register accessor: an alias for `Reg<ERASEPAGEPARTIALCFG_SPEC>`"]
pub type ERASEPAGEPARTIALCFG = crate::Reg<erasepagepartialcfg::ERASEPAGEPARTIALCFG_SPEC>;
#[doc = "Register for partial erase configuration"]
pub mod erasepagepartialcfg;
#[doc = "ICACHECNF register accessor: an alias for `Reg<ICACHECNF_SPEC>`"]
pub type ICACHECNF = crate::Reg<icachecnf::ICACHECNF_SPEC>;
#[doc = "I-code cache configuration register"]
pub mod icachecnf;
#[doc = "IHIT register accessor: an alias for `Reg<IHIT_SPEC>`"]
pub type IHIT = crate::Reg<ihit::IHIT_SPEC>;
#[doc = "I-code cache hit counter"]
pub mod ihit;
#[doc = "IMISS register accessor: an alias for `Reg<IMISS_SPEC>`"]
pub type IMISS = crate::Reg<imiss::IMISS_SPEC>;
#[doc = "I-code cache miss counter"]
pub mod imiss;
