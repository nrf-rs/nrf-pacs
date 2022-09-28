#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access port protection"]
    pub approtect: APPROTECT,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Output voltage from the high voltage (VREGH) regulator stage. The maximum output voltage from this stage is given as VDDH - VREGHDROP."]
    pub vreghvout: VREGHVOUT,
    #[doc = "0x14 - HFXO startup counter"]
    pub hfxocnt: HFXOCNT,
    _reserved3: [u8; 0x04],
    #[doc = "0x1c - Secure access port protection"]
    pub secureapprotect: SECUREAPPROTECT,
    #[doc = "0x20 - Erase protection"]
    pub eraseprotect: ERASEPROTECT,
    #[doc = "0x24 - SW-DP Target instance"]
    pub tinstance: TINSTANCE,
    #[doc = "0x28 - Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
    pub nfcpins: NFCPINS,
    _reserved7: [u8; 0xd4],
    #[doc = "0x100..0x400 - Description collection: One time programmable memory"]
    pub otp: [OTP; 192],
    #[doc = "0x400..0x1000 - Unspecified"]
    pub keyslot: KEYSLOT,
}
#[doc = "APPROTECT (rw) register accessor: an alias for `Reg<APPROTECT_SPEC>`"]
pub type APPROTECT = crate::Reg<approtect::APPROTECT_SPEC>;
#[doc = "Access port protection"]
pub mod approtect;
#[doc = "VREGHVOUT (rw) register accessor: an alias for `Reg<VREGHVOUT_SPEC>`"]
pub type VREGHVOUT = crate::Reg<vreghvout::VREGHVOUT_SPEC>;
#[doc = "Output voltage from the high voltage (VREGH) regulator stage. The maximum output voltage from this stage is given as VDDH - VREGHDROP."]
pub mod vreghvout;
#[doc = "HFXOCNT (rw) register accessor: an alias for `Reg<HFXOCNT_SPEC>`"]
pub type HFXOCNT = crate::Reg<hfxocnt::HFXOCNT_SPEC>;
#[doc = "HFXO startup counter"]
pub mod hfxocnt;
#[doc = "SECUREAPPROTECT (rw) register accessor: an alias for `Reg<SECUREAPPROTECT_SPEC>`"]
pub type SECUREAPPROTECT = crate::Reg<secureapprotect::SECUREAPPROTECT_SPEC>;
#[doc = "Secure access port protection"]
pub mod secureapprotect;
#[doc = "ERASEPROTECT (rw) register accessor: an alias for `Reg<ERASEPROTECT_SPEC>`"]
pub type ERASEPROTECT = crate::Reg<eraseprotect::ERASEPROTECT_SPEC>;
#[doc = "Erase protection"]
pub mod eraseprotect;
#[doc = "TINSTANCE (rw) register accessor: an alias for `Reg<TINSTANCE_SPEC>`"]
pub type TINSTANCE = crate::Reg<tinstance::TINSTANCE_SPEC>;
#[doc = "SW-DP Target instance"]
pub mod tinstance;
#[doc = "NFCPINS (rw) register accessor: an alias for `Reg<NFCPINS_SPEC>`"]
pub type NFCPINS = crate::Reg<nfcpins::NFCPINS_SPEC>;
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
pub mod nfcpins;
#[doc = "OTP (rw) register accessor: an alias for `Reg<OTP_SPEC>`"]
pub type OTP = crate::Reg<otp::OTP_SPEC>;
#[doc = "Description collection: One time programmable memory"]
pub mod otp;
#[doc = "Unspecified"]
pub use keyslot::KEYSLOT;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod keyslot;
