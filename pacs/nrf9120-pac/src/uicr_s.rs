#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access port protection"]
    pub approtect: APPROTECT,
    _reserved1: [u8; 0x10],
    #[doc = "0x14 - Oscillator control"]
    pub xosc32m: XOSC32M,
    _reserved2: [u8; 0x04],
    #[doc = "0x1c - HFXO clock source selection"]
    pub hfxosrc: HFXOSRC,
    #[doc = "0x20 - HFXO startup counter"]
    pub hfxocnt: HFXOCNT,
    #[doc = "0x24 - Enable blocking NVM WRITE and aborting NVM ERASE for Application NVM in POFWARN condition ."]
    pub appnvmcpofguard: APPNVMCPOFGUARD,
    #[doc = "0x28 - Polarity of PMIC polarity configuration signals."]
    pub pmicconf: PMICCONF,
    #[doc = "0x2c - Secure access port protection"]
    pub secureapprotect: SECUREAPPROTECT,
    #[doc = "0x30 - Erase protection"]
    pub eraseprotect: ERASEPROTECT,
    _reserved8: [u8; 0xd4],
    #[doc = "0x108..0x400 - Description collection: One time programmable memory"]
    pub otp: [OTP; 190],
    #[doc = "0x400..0x1000 - Unspecified"]
    pub keyslot: KEYSLOT,
}
#[doc = "APPROTECT (rw) register accessor: an alias for `Reg<APPROTECT_SPEC>`"]
pub type APPROTECT = crate::Reg<approtect::APPROTECT_SPEC>;
#[doc = "Access port protection"]
pub mod approtect;
#[doc = "XOSC32M (rw) register accessor: an alias for `Reg<XOSC32M_SPEC>`"]
pub type XOSC32M = crate::Reg<xosc32m::XOSC32M_SPEC>;
#[doc = "Oscillator control"]
pub mod xosc32m;
#[doc = "HFXOSRC (rw) register accessor: an alias for `Reg<HFXOSRC_SPEC>`"]
pub type HFXOSRC = crate::Reg<hfxosrc::HFXOSRC_SPEC>;
#[doc = "HFXO clock source selection"]
pub mod hfxosrc;
#[doc = "HFXOCNT (rw) register accessor: an alias for `Reg<HFXOCNT_SPEC>`"]
pub type HFXOCNT = crate::Reg<hfxocnt::HFXOCNT_SPEC>;
#[doc = "HFXO startup counter"]
pub mod hfxocnt;
#[doc = "APPNVMCPOFGUARD (rw) register accessor: an alias for `Reg<APPNVMCPOFGUARD_SPEC>`"]
pub type APPNVMCPOFGUARD = crate::Reg<appnvmcpofguard::APPNVMCPOFGUARD_SPEC>;
#[doc = "Enable blocking NVM WRITE and aborting NVM ERASE for Application NVM in POFWARN condition ."]
pub mod appnvmcpofguard;
#[doc = "PMICCONF (rw) register accessor: an alias for `Reg<PMICCONF_SPEC>`"]
pub type PMICCONF = crate::Reg<pmicconf::PMICCONF_SPEC>;
#[doc = "Polarity of PMIC polarity configuration signals."]
pub mod pmicconf;
#[doc = "SECUREAPPROTECT (rw) register accessor: an alias for `Reg<SECUREAPPROTECT_SPEC>`"]
pub type SECUREAPPROTECT = crate::Reg<secureapprotect::SECUREAPPROTECT_SPEC>;
#[doc = "Secure access port protection"]
pub mod secureapprotect;
#[doc = "ERASEPROTECT (rw) register accessor: an alias for `Reg<ERASEPROTECT_SPEC>`"]
pub type ERASEPROTECT = crate::Reg<eraseprotect::ERASEPROTECT_SPEC>;
#[doc = "Erase protection"]
pub mod eraseprotect;
#[doc = "OTP (rw) register accessor: an alias for `Reg<OTP_SPEC>`"]
pub type OTP = crate::Reg<otp::OTP_SPEC>;
#[doc = "Description collection: One time programmable memory"]
pub mod otp;
#[doc = "Unspecified"]
pub use keyslot::KEYSLOT;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod keyslot;
