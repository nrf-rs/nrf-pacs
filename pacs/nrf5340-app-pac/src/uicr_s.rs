#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    approtect: Approtect,
    _reserved1: [u8; 0x0c],
    vreghvout: Vreghvout,
    hfxocnt: Hfxocnt,
    _reserved3: [u8; 0x04],
    secureapprotect: Secureapprotect,
    eraseprotect: Eraseprotect,
    tinstance: Tinstance,
    nfcpins: Nfcpins,
    _reserved7: [u8; 0xd4],
    otp: [Otp; 192],
    keyslot: Keyslot,
}
impl RegisterBlock {
    #[doc = "0x00 - Access port protection"]
    #[inline(always)]
    pub const fn approtect(&self) -> &Approtect {
        &self.approtect
    }
    #[doc = "0x10 - Output voltage from the high voltage (VREGH) regulator stage. The maximum output voltage from this stage is given as VDDH - VREGHDROP."]
    #[inline(always)]
    pub const fn vreghvout(&self) -> &Vreghvout {
        &self.vreghvout
    }
    #[doc = "0x14 - HFXO startup counter"]
    #[inline(always)]
    pub const fn hfxocnt(&self) -> &Hfxocnt {
        &self.hfxocnt
    }
    #[doc = "0x1c - Secure access port protection"]
    #[inline(always)]
    pub const fn secureapprotect(&self) -> &Secureapprotect {
        &self.secureapprotect
    }
    #[doc = "0x20 - Erase protection"]
    #[inline(always)]
    pub const fn eraseprotect(&self) -> &Eraseprotect {
        &self.eraseprotect
    }
    #[doc = "0x24 - SW-DP Target instance"]
    #[inline(always)]
    pub const fn tinstance(&self) -> &Tinstance {
        &self.tinstance
    }
    #[doc = "0x28 - Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
    #[inline(always)]
    pub const fn nfcpins(&self) -> &Nfcpins {
        &self.nfcpins
    }
    #[doc = "0x100..0x400 - Description collection: One time programmable memory"]
    #[inline(always)]
    pub const fn otp(&self, n: usize) -> &Otp {
        &self.otp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x400 - Description collection: One time programmable memory"]
    #[inline(always)]
    pub fn otp_iter(&self) -> impl Iterator<Item = &Otp> {
        self.otp.iter()
    }
    #[doc = "0x400..0x1000 - Unspecified"]
    #[inline(always)]
    pub const fn keyslot(&self) -> &Keyslot {
        &self.keyslot
    }
}
#[doc = "APPROTECT (rw) register accessor: Access port protection\n\nYou can [`read`](crate::Reg::read) this register and get [`approtect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`approtect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@approtect`] module"]
#[doc(alias = "APPROTECT")]
pub type Approtect = crate::Reg<approtect::ApprotectSpec>;
#[doc = "Access port protection"]
pub mod approtect;
#[doc = "VREGHVOUT (rw) register accessor: Output voltage from the high voltage (VREGH) regulator stage. The maximum output voltage from this stage is given as VDDH - VREGHDROP.\n\nYou can [`read`](crate::Reg::read) this register and get [`vreghvout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreghvout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vreghvout`] module"]
#[doc(alias = "VREGHVOUT")]
pub type Vreghvout = crate::Reg<vreghvout::VreghvoutSpec>;
#[doc = "Output voltage from the high voltage (VREGH) regulator stage. The maximum output voltage from this stage is given as VDDH - VREGHDROP."]
pub mod vreghvout;
#[doc = "HFXOCNT (rw) register accessor: HFXO startup counter\n\nYou can [`read`](crate::Reg::read) this register and get [`hfxocnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxocnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxocnt`] module"]
#[doc(alias = "HFXOCNT")]
pub type Hfxocnt = crate::Reg<hfxocnt::HfxocntSpec>;
#[doc = "HFXO startup counter"]
pub mod hfxocnt;
#[doc = "SECUREAPPROTECT (rw) register accessor: Secure access port protection\n\nYou can [`read`](crate::Reg::read) this register and get [`secureapprotect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureapprotect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureapprotect`] module"]
#[doc(alias = "SECUREAPPROTECT")]
pub type Secureapprotect = crate::Reg<secureapprotect::SecureapprotectSpec>;
#[doc = "Secure access port protection"]
pub mod secureapprotect;
#[doc = "ERASEPROTECT (rw) register accessor: Erase protection\n\nYou can [`read`](crate::Reg::read) this register and get [`eraseprotect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eraseprotect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eraseprotect`] module"]
#[doc(alias = "ERASEPROTECT")]
pub type Eraseprotect = crate::Reg<eraseprotect::EraseprotectSpec>;
#[doc = "Erase protection"]
pub mod eraseprotect;
#[doc = "TINSTANCE (rw) register accessor: SW-DP Target instance\n\nYou can [`read`](crate::Reg::read) this register and get [`tinstance::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tinstance::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tinstance`] module"]
#[doc(alias = "TINSTANCE")]
pub type Tinstance = crate::Reg<tinstance::TinstanceSpec>;
#[doc = "SW-DP Target instance"]
pub mod tinstance;
#[doc = "NFCPINS (rw) register accessor: Setting of pins dedicated to NFC functionality: NFC antenna or GPIO\n\nYou can [`read`](crate::Reg::read) this register and get [`nfcpins::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nfcpins::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nfcpins`] module"]
#[doc(alias = "NFCPINS")]
pub type Nfcpins = crate::Reg<nfcpins::NfcpinsSpec>;
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
pub mod nfcpins;
#[doc = "OTP (rw) register accessor: Description collection: One time programmable memory\n\nYou can [`read`](crate::Reg::read) this register and get [`otp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp`] module"]
#[doc(alias = "OTP")]
pub type Otp = crate::Reg<otp::OtpSpec>;
#[doc = "Description collection: One time programmable memory"]
pub mod otp;
#[doc = "Unspecified"]
pub use self::keyslot::Keyslot;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod keyslot;
