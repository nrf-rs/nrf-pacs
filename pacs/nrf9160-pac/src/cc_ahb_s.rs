#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0b00],
    ahbm_singles: AhbmSingles,
    ahbm_hprot: AhbmHprot,
    ahbm_hmastlock: AhbmHmastlock,
    ahbm_hnonsec: AhbmHnonsec,
}
impl RegisterBlock {
    #[doc = "0xb00 - This register forces the AHB transactions from CRYPTOCELL master to be always singles."]
    #[inline(always)]
    pub const fn ahbm_singles(&self) -> &AhbmSingles {
        &self.ahbm_singles
    }
    #[doc = "0xb04 - This register holds the AHB HPROT value"]
    #[inline(always)]
    pub const fn ahbm_hprot(&self) -> &AhbmHprot {
        &self.ahbm_hprot
    }
    #[doc = "0xb08 - This register holds AHB HMASTLOCK value"]
    #[inline(always)]
    pub const fn ahbm_hmastlock(&self) -> &AhbmHmastlock {
        &self.ahbm_hmastlock
    }
    #[doc = "0xb0c - This register holds AHB HNONSEC value"]
    #[inline(always)]
    pub const fn ahbm_hnonsec(&self) -> &AhbmHnonsec {
        &self.ahbm_hnonsec
    }
}
#[doc = "AHBM_SINGLES (rw) register accessor: This register forces the AHB transactions from CRYPTOCELL master to be always singles.\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbm_singles::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbm_singles::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbm_singles`] module"]
#[doc(alias = "AHBM_SINGLES")]
pub type AhbmSingles = crate::Reg<ahbm_singles::AhbmSinglesSpec>;
#[doc = "This register forces the AHB transactions from CRYPTOCELL master to be always singles."]
pub mod ahbm_singles;
#[doc = "AHBM_HPROT (rw) register accessor: This register holds the AHB HPROT value\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbm_hprot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbm_hprot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbm_hprot`] module"]
#[doc(alias = "AHBM_HPROT")]
pub type AhbmHprot = crate::Reg<ahbm_hprot::AhbmHprotSpec>;
#[doc = "This register holds the AHB HPROT value"]
pub mod ahbm_hprot;
#[doc = "AHBM_HMASTLOCK (rw) register accessor: This register holds AHB HMASTLOCK value\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbm_hmastlock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbm_hmastlock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbm_hmastlock`] module"]
#[doc(alias = "AHBM_HMASTLOCK")]
pub type AhbmHmastlock = crate::Reg<ahbm_hmastlock::AhbmHmastlockSpec>;
#[doc = "This register holds AHB HMASTLOCK value"]
pub mod ahbm_hmastlock;
#[doc = "AHBM_HNONSEC (rw) register accessor: This register holds AHB HNONSEC value\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbm_hnonsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbm_hnonsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahbm_hnonsec`] module"]
#[doc(alias = "AHBM_HNONSEC")]
pub type AhbmHnonsec = crate::Reg<ahbm_hnonsec::AhbmHnonsecSpec>;
#[doc = "This register holds AHB HNONSEC value"]
pub mod ahbm_hnonsec;
