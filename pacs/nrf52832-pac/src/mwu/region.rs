#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "REGION")]
pub struct Region {
    start: Start,
    end: End,
}
impl Region {
    #[doc = "0x00 - Description cluster\\[0\\]: Start address for region 0"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x04 - Description cluster\\[0\\]: End address of region 0"]
    #[inline(always)]
    pub const fn end(&self) -> &End {
        &self.end
    }
}
#[doc = "START (rw) register accessor: Description cluster\\[0\\]: Start address for region 0\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "Description cluster\\[0\\]: Start address for region 0"]
pub mod start;
#[doc = "END (rw) register accessor: Description cluster\\[0\\]: End address of region 0\n\nYou can [`read`](crate::Reg::read) this register and get [`end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@end`] module"]
#[doc(alias = "END")]
pub type End = crate::Reg<end::EndSpec>;
#[doc = "Description cluster\\[0\\]: End address of region 0"]
pub mod end;
