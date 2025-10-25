#[repr(C)]
#[doc = "Fork"]
#[doc(alias = "FORK")]
pub struct Fork {
    tep: Tep,
}
impl Fork {
    #[doc = "0x00 - Description cluster: Channel n task endpoint"]
    #[inline(always)]
    pub const fn tep(&self) -> &Tep {
        &self.tep
    }
}
#[doc = "TEP (rw) register accessor: Description cluster: Channel n task endpoint\n\nYou can [`read`](crate::Reg::read) this register and get [`tep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tep`] module"]
#[doc(alias = "TEP")]
pub type Tep = crate::Reg<tep::TepSpec>;
#[doc = "Description cluster: Channel n task endpoint"]
pub mod tep;
