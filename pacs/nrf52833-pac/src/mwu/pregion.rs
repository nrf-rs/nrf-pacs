#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PREGION")]
pub struct Pregion {
    start: Start,
    end: End,
    subs: Subs,
}
impl Pregion {
    #[doc = "0x00 - Description cluster: Reserved for future use"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x04 - Description cluster: Reserved for future use"]
    #[inline(always)]
    pub const fn end(&self) -> &End {
        &self.end
    }
    #[doc = "0x08 - Description cluster: Subregions of region n"]
    #[inline(always)]
    pub const fn subs(&self) -> &Subs {
        &self.subs
    }
}
#[doc = "START (r) register accessor: Description cluster: Reserved for future use\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "Description cluster: Reserved for future use"]
pub mod start;
#[doc = "END (r) register accessor: Description cluster: Reserved for future use\n\nYou can [`read`](crate::Reg::read) this register and get [`end::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@end`] module"]
#[doc(alias = "END")]
pub type End = crate::Reg<end::EndSpec>;
#[doc = "Description cluster: Reserved for future use"]
pub mod end;
#[doc = "SUBS (rw) register accessor: Description cluster: Subregions of region n\n\nYou can [`read`](crate::Reg::read) this register and get [`subs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`subs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subs`] module"]
#[doc(alias = "SUBS")]
pub type Subs = crate::Reg<subs::SubsSpec>;
#[doc = "Description cluster: Subregions of region n"]
pub mod subs;
