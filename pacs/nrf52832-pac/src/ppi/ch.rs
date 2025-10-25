#[repr(C)]
#[doc = "PPI Channel"]
#[doc(alias = "CH")]
pub struct Ch {
    eep: Eep,
    tep: Tep,
}
impl Ch {
    #[doc = "0x00 - Description cluster\\[0\\]: Channel 0 event end-point"]
    #[inline(always)]
    pub const fn eep(&self) -> &Eep {
        &self.eep
    }
    #[doc = "0x04 - Description cluster\\[0\\]: Channel 0 task end-point"]
    #[inline(always)]
    pub const fn tep(&self) -> &Tep {
        &self.tep
    }
}
#[doc = "EEP (rw) register accessor: Description cluster\\[0\\]: Channel 0 event end-point\n\nYou can [`read`](crate::Reg::read) this register and get [`eep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eep`] module"]
#[doc(alias = "EEP")]
pub type Eep = crate::Reg<eep::EepSpec>;
#[doc = "Description cluster\\[0\\]: Channel 0 event end-point"]
pub mod eep;
#[doc = "TEP (rw) register accessor: Description cluster\\[0\\]: Channel 0 task end-point\n\nYou can [`read`](crate::Reg::read) this register and get [`tep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tep`] module"]
#[doc(alias = "TEP")]
pub type Tep = crate::Reg<tep::TepSpec>;
#[doc = "Description cluster\\[0\\]: Channel 0 task end-point"]
pub mod tep;
