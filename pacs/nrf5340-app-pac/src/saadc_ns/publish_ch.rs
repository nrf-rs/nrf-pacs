#[repr(C)]
#[doc = "Publish configuration for events"]
#[doc(alias = "PUBLISH_CH")]
pub struct PublishCh {
    limith: Limith,
    limitl: Limitl,
}
impl PublishCh {
    #[doc = "0x00 - Description cluster: Publish configuration for event CH\\[n\\].LIMITH"]
    #[inline(always)]
    pub const fn limith(&self) -> &Limith {
        &self.limith
    }
    #[doc = "0x04 - Description cluster: Publish configuration for event CH\\[n\\].LIMITL"]
    #[inline(always)]
    pub const fn limitl(&self) -> &Limitl {
        &self.limitl
    }
}
#[doc = "LIMITH (rw) register accessor: Description cluster: Publish configuration for event CH\\[n\\].LIMITH\n\nYou can [`read`](crate::Reg::read) this register and get [`limith::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limith::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@limith`] module"]
#[doc(alias = "LIMITH")]
pub type Limith = crate::Reg<limith::LimithSpec>;
#[doc = "Description cluster: Publish configuration for event CH\\[n\\].LIMITH"]
pub mod limith;
#[doc = "LIMITL (rw) register accessor: Description cluster: Publish configuration for event CH\\[n\\].LIMITL\n\nYou can [`read`](crate::Reg::read) this register and get [`limitl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limitl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@limitl`] module"]
#[doc(alias = "LIMITL")]
pub type Limitl = crate::Reg<limitl::LimitlSpec>;
#[doc = "Description cluster: Publish configuration for event CH\\[n\\].LIMITL"]
pub mod limitl;
