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
    _reserved3: [u8; 0x04],
    #[doc = "0x50c - Register for erasing all non-volatile user memory"]
    pub eraseall: crate::Reg<eraseall::ERASEALL_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x51c - Register for partial erase configuration"]
    pub erasepagepartialcfg: crate::Reg<erasepagepartialcfg::ERASEPAGEPARTIALCFG_SPEC>,
    _reserved5: [u8; 0x20],
    #[doc = "0x540 - I-code cache configuration register"]
    pub icachecnf: crate::Reg<icachecnf::ICACHECNF_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x548 - I-code cache hit counter"]
    pub ihit: crate::Reg<ihit::IHIT_SPEC>,
    #[doc = "0x54c - I-code cache miss counter"]
    pub imiss: crate::Reg<imiss::IMISS_SPEC>,
    _reserved8: [u8; 0x34],
    #[doc = "0x584 - Unspecified"]
    pub configns: crate::Reg<configns::CONFIGNS_SPEC>,
    #[doc = "0x588 - Non-secure APPROTECT enable register"]
    pub writeuicrns: crate::Reg<writeuicrns::WRITEUICRNS_SPEC>,
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
#[doc = "ERASEALL register accessor: an alias for `Reg<ERASEALL_SPEC>`"]
pub type ERASEALL = crate::Reg<eraseall::ERASEALL_SPEC>;
#[doc = "Register for erasing all non-volatile user memory"]
pub mod eraseall;
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
#[doc = "CONFIGNS register accessor: an alias for `Reg<CONFIGNS_SPEC>`"]
pub type CONFIGNS = crate::Reg<configns::CONFIGNS_SPEC>;
#[doc = "Unspecified"]
pub mod configns;
#[doc = "WRITEUICRNS register accessor: an alias for `Reg<WRITEUICRNS_SPEC>`"]
pub type WRITEUICRNS = crate::Reg<writeuicrns::WRITEUICRNS_SPEC>;
#[doc = "Non-secure APPROTECT enable register"]
pub mod writeuicrns;
