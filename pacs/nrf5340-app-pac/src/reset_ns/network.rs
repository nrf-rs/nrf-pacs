#[repr(C)]
#[doc = "ULP network core control"]
#[doc(alias = "NETWORK")]
pub struct Network {
    _reserved0: [u8; 0x04],
    forceoff: Forceoff,
}
impl Network {
    #[doc = "0x04 - Force network core off"]
    #[inline(always)]
    pub const fn forceoff(&self) -> &Forceoff {
        &self.forceoff
    }
}
#[doc = "FORCEOFF (rw) register accessor: Force network core off\n\nYou can [`read`](crate::Reg::read) this register and get [`forceoff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`forceoff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@forceoff`] module"]
#[doc(alias = "FORCEOFF")]
pub type Forceoff = crate::Reg<forceoff::ForceoffSpec>;
#[doc = "Force network core off"]
pub mod forceoff;
