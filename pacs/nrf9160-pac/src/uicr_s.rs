#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access port protection"]
    pub approtect: crate::Reg<approtect::APPROTECT_SPEC>,
    _reserved1: [u8; 0x10],
    #[doc = "0x14 - Oscillator control"]
    pub xosc32m: crate::Reg<xosc32m::XOSC32M_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x1c - HFXO clock source selection"]
    pub hfxosrc: crate::Reg<hfxosrc::HFXOSRC_SPEC>,
    #[doc = "0x20 - HFXO startup counter"]
    pub hfxocnt: crate::Reg<hfxocnt::HFXOCNT_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x2c - Secure access port protection"]
    pub secureapprotect: crate::Reg<secureapprotect::SECUREAPPROTECT_SPEC>,
    #[doc = "0x30 - Erase protection"]
    pub eraseprotect: crate::Reg<eraseprotect::ERASEPROTECT_SPEC>,
    _reserved6: [u8; 0xd4],
    #[doc = "0x108..0x400 - Description collection: One time programmable memory"]
    pub otp: [crate::Reg<otp::OTP_SPEC>; 190],
    #[doc = "0x400..0x1000 - Unspecified"]
    pub keyslot: KEYSLOT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct KEYSLOT {
    #[doc = "0x00..0x400 - Unspecified"]
    pub config: [self::keyslot::CONFIG; 128],
    #[doc = "0x400..0xc00 - Unspecified"]
    pub key: [self::keyslot::KEY; 128],
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod keyslot;
#[doc = "APPROTECT register accessor: an alias for `Reg<APPROTECT_SPEC>`"]
pub type APPROTECT = crate::Reg<approtect::APPROTECT_SPEC>;
#[doc = "Access port protection"]
pub mod approtect;
#[doc = "XOSC32M register accessor: an alias for `Reg<XOSC32M_SPEC>`"]
pub type XOSC32M = crate::Reg<xosc32m::XOSC32M_SPEC>;
#[doc = "Oscillator control"]
pub mod xosc32m;
#[doc = "HFXOSRC register accessor: an alias for `Reg<HFXOSRC_SPEC>`"]
pub type HFXOSRC = crate::Reg<hfxosrc::HFXOSRC_SPEC>;
#[doc = "HFXO clock source selection"]
pub mod hfxosrc;
#[doc = "HFXOCNT register accessor: an alias for `Reg<HFXOCNT_SPEC>`"]
pub type HFXOCNT = crate::Reg<hfxocnt::HFXOCNT_SPEC>;
#[doc = "HFXO startup counter"]
pub mod hfxocnt;
#[doc = "SECUREAPPROTECT register accessor: an alias for `Reg<SECUREAPPROTECT_SPEC>`"]
pub type SECUREAPPROTECT = crate::Reg<secureapprotect::SECUREAPPROTECT_SPEC>;
#[doc = "Secure access port protection"]
pub mod secureapprotect;
#[doc = "ERASEPROTECT register accessor: an alias for `Reg<ERASEPROTECT_SPEC>`"]
pub type ERASEPROTECT = crate::Reg<eraseprotect::ERASEPROTECT_SPEC>;
#[doc = "Erase protection"]
pub mod eraseprotect;
#[doc = "OTP register accessor: an alias for `Reg<OTP_SPEC>`"]
pub type OTP = crate::Reg<otp::OTP_SPEC>;
#[doc = "Description collection: One time programmable memory"]
pub mod otp;
