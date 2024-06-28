#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    approtect: Approtect,
    _reserved1: [u8; 0x10],
    xosc32m: Xosc32m,
    _reserved2: [u8; 0x04],
    hfxosrc: Hfxosrc,
    hfxocnt: Hfxocnt,
    appnvmcpofguard: Appnvmcpofguard,
    pmicconf: Pmicconf,
    secureapprotect: Secureapprotect,
    eraseprotect: Eraseprotect,
    _reserved8: [u8; 0xd4],
    otp: [Otp; 190],
    keyslot: Keyslot,
}
impl RegisterBlock {
    #[doc = "0x00 - Access port protection"]
    #[inline(always)]
    pub const fn approtect(&self) -> &Approtect {
        &self.approtect
    }
    #[doc = "0x14 - Oscillator control"]
    #[inline(always)]
    pub const fn xosc32m(&self) -> &Xosc32m {
        &self.xosc32m
    }
    #[doc = "0x1c - HFXO clock source selection"]
    #[inline(always)]
    pub const fn hfxosrc(&self) -> &Hfxosrc {
        &self.hfxosrc
    }
    #[doc = "0x20 - HFXO startup counter"]
    #[inline(always)]
    pub const fn hfxocnt(&self) -> &Hfxocnt {
        &self.hfxocnt
    }
    #[doc = "0x24 - Enable blocking NVM WRITE and aborting NVM ERASE for Application NVM in POFWARN condition ."]
    #[inline(always)]
    pub const fn appnvmcpofguard(&self) -> &Appnvmcpofguard {
        &self.appnvmcpofguard
    }
    #[doc = "0x28 - Polarity of PMIC polarity configuration signals."]
    #[inline(always)]
    pub const fn pmicconf(&self) -> &Pmicconf {
        &self.pmicconf
    }
    #[doc = "0x2c - Secure access port protection"]
    #[inline(always)]
    pub const fn secureapprotect(&self) -> &Secureapprotect {
        &self.secureapprotect
    }
    #[doc = "0x30 - Erase protection"]
    #[inline(always)]
    pub const fn eraseprotect(&self) -> &Eraseprotect {
        &self.eraseprotect
    }
    #[doc = "0x108..0x400 - Description collection: One time programmable memory"]
    #[inline(always)]
    pub const fn otp(&self, n: usize) -> &Otp {
        &self.otp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x108..0x400 - Description collection: One time programmable memory"]
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
#[doc = "APPROTECT (rw) register accessor: Access port protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`approtect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`approtect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@approtect`]
module"]
#[doc(alias = "APPROTECT")]
pub type Approtect = crate::Reg<approtect::ApprotectSpec>;
#[doc = "Access port protection"]
pub mod approtect;
#[doc = "XOSC32M (rw) register accessor: Oscillator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xosc32m::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xosc32m::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xosc32m`]
module"]
#[doc(alias = "XOSC32M")]
pub type Xosc32m = crate::Reg<xosc32m::Xosc32mSpec>;
#[doc = "Oscillator control"]
pub mod xosc32m;
#[doc = "HFXOSRC (rw) register accessor: HFXO clock source selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxosrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxosrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxosrc`]
module"]
#[doc(alias = "HFXOSRC")]
pub type Hfxosrc = crate::Reg<hfxosrc::HfxosrcSpec>;
#[doc = "HFXO clock source selection"]
pub mod hfxosrc;
#[doc = "HFXOCNT (rw) register accessor: HFXO startup counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxocnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxocnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxocnt`]
module"]
#[doc(alias = "HFXOCNT")]
pub type Hfxocnt = crate::Reg<hfxocnt::HfxocntSpec>;
#[doc = "HFXO startup counter"]
pub mod hfxocnt;
#[doc = "APPNVMCPOFGUARD (rw) register accessor: Enable blocking NVM WRITE and aborting NVM ERASE for Application NVM in POFWARN condition .\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`appnvmcpofguard::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`appnvmcpofguard::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@appnvmcpofguard`]
module"]
#[doc(alias = "APPNVMCPOFGUARD")]
pub type Appnvmcpofguard = crate::Reg<appnvmcpofguard::AppnvmcpofguardSpec>;
#[doc = "Enable blocking NVM WRITE and aborting NVM ERASE for Application NVM in POFWARN condition ."]
pub mod appnvmcpofguard;
#[doc = "PMICCONF (rw) register accessor: Polarity of PMIC polarity configuration signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmicconf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmicconf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmicconf`]
module"]
#[doc(alias = "PMICCONF")]
pub type Pmicconf = crate::Reg<pmicconf::PmicconfSpec>;
#[doc = "Polarity of PMIC polarity configuration signals."]
pub mod pmicconf;
#[doc = "SECUREAPPROTECT (rw) register accessor: Secure access port protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secureapprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secureapprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secureapprotect`]
module"]
#[doc(alias = "SECUREAPPROTECT")]
pub type Secureapprotect = crate::Reg<secureapprotect::SecureapprotectSpec>;
#[doc = "Secure access port protection"]
pub mod secureapprotect;
#[doc = "ERASEPROTECT (rw) register accessor: Erase protection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eraseprotect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eraseprotect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eraseprotect`]
module"]
#[doc(alias = "ERASEPROTECT")]
pub type Eraseprotect = crate::Reg<eraseprotect::EraseprotectSpec>;
#[doc = "Erase protection"]
pub mod eraseprotect;
#[doc = "OTP (rw) register accessor: Description collection: One time programmable memory\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otp`]
module"]
#[doc(alias = "OTP")]
pub type Otp = crate::Reg<otp::OtpSpec>;
#[doc = "Description collection: One time programmable memory"]
pub mod otp;
#[doc = "Unspecified"]
pub use self::keyslot::Keyslot;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod keyslot;
