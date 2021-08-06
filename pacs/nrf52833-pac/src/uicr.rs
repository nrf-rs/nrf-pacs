#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x14],
    #[doc = "0x14..0x48 - Description collection: Reserved for Nordic firmware design"]
    pub nrffw: [crate::Reg<nrffw::NRFFW_SPEC>; 13],
    _reserved1: [u8; 0x08],
    #[doc = "0x50..0x80 - Description collection: Reserved for Nordic hardware design"]
    pub nrfhw: [crate::Reg<nrfhw::NRFHW_SPEC>; 12],
    #[doc = "0x80..0x100 - Description collection: Reserved for customer"]
    pub customer: [crate::Reg<customer::CUSTOMER_SPEC>; 32],
    _reserved3: [u8; 0x0100],
    #[doc = "0x200..0x208 - Description collection: Mapping of the nRESET function (see POWER chapter for details)"]
    pub pselreset: [crate::Reg<pselreset::PSELRESET_SPEC>; 2],
    #[doc = "0x208 - Access port protection"]
    pub approtect: crate::Reg<approtect::APPROTECT_SPEC>,
    #[doc = "0x20c - Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
    pub nfcpins: crate::Reg<nfcpins::NFCPINS_SPEC>,
    #[doc = "0x210 - Processor debug control"]
    pub debugctrl: crate::Reg<debugctrl::DEBUGCTRL_SPEC>,
    _reserved7: [u8; 0xf0],
    #[doc = "0x304 - Output voltage from REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - VREG0DROP."]
    pub regout0: crate::Reg<regout0::REGOUT0_SPEC>,
}
#[doc = "NRFFW register accessor: an alias for `Reg<NRFFW_SPEC>`"]
pub type NRFFW = crate::Reg<nrffw::NRFFW_SPEC>;
#[doc = "Description collection: Reserved for Nordic firmware design"]
pub mod nrffw;
#[doc = "NRFHW register accessor: an alias for `Reg<NRFHW_SPEC>`"]
pub type NRFHW = crate::Reg<nrfhw::NRFHW_SPEC>;
#[doc = "Description collection: Reserved for Nordic hardware design"]
pub mod nrfhw;
#[doc = "CUSTOMER register accessor: an alias for `Reg<CUSTOMER_SPEC>`"]
pub type CUSTOMER = crate::Reg<customer::CUSTOMER_SPEC>;
#[doc = "Description collection: Reserved for customer"]
pub mod customer;
#[doc = "PSELRESET register accessor: an alias for `Reg<PSELRESET_SPEC>`"]
pub type PSELRESET = crate::Reg<pselreset::PSELRESET_SPEC>;
#[doc = "Description collection: Mapping of the nRESET function (see POWER chapter for details)"]
pub mod pselreset;
#[doc = "APPROTECT register accessor: an alias for `Reg<APPROTECT_SPEC>`"]
pub type APPROTECT = crate::Reg<approtect::APPROTECT_SPEC>;
#[doc = "Access port protection"]
pub mod approtect;
#[doc = "NFCPINS register accessor: an alias for `Reg<NFCPINS_SPEC>`"]
pub type NFCPINS = crate::Reg<nfcpins::NFCPINS_SPEC>;
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
pub mod nfcpins;
#[doc = "DEBUGCTRL register accessor: an alias for `Reg<DEBUGCTRL_SPEC>`"]
pub type DEBUGCTRL = crate::Reg<debugctrl::DEBUGCTRL_SPEC>;
#[doc = "Processor debug control"]
pub mod debugctrl;
#[doc = "REGOUT0 register accessor: an alias for `Reg<REGOUT0_SPEC>`"]
pub type REGOUT0 = crate::Reg<regout0::REGOUT0_SPEC>;
#[doc = "Output voltage from REG0 regulator stage. The maximum output voltage from this stage is given as VDDH - VREG0DROP."]
pub mod regout0;
