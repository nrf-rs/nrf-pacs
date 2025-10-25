#[repr(C)]
#[doc = "Peripheral events."]
#[doc(alias = "EVENTS_CH")]
pub struct EventsCh {
    limith: Limith,
    limitl: Limitl,
}
impl EventsCh {
    #[doc = "0x00 - Description cluster: Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
    #[inline(always)]
    pub const fn limith(&self) -> &Limith {
        &self.limith
    }
    #[doc = "0x04 - Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW"]
    #[inline(always)]
    pub const fn limitl(&self) -> &Limitl {
        &self.limitl
    }
}
#[doc = "LIMITH (rw) register accessor: Description cluster: Last results is equal or above CH\\[n\\].LIMIT.HIGH\n\nYou can [`read`](crate::Reg::read) this register and get [`limith::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limith::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@limith`] module"]
#[doc(alias = "LIMITH")]
pub type Limith = crate::Reg<limith::LimithSpec>;
#[doc = "Description cluster: Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
pub mod limith;
#[doc = "LIMITL (rw) register accessor: Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW\n\nYou can [`read`](crate::Reg::read) this register and get [`limitl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limitl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@limitl`] module"]
#[doc(alias = "LIMITL")]
pub type Limitl = crate::Reg<limitl::LimitlSpec>;
#[doc = "Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW"]
pub mod limitl;
