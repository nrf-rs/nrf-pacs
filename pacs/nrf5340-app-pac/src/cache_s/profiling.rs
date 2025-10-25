#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PROFILING")]
pub struct Profiling {
    ihit: Ihit,
    imiss: Imiss,
    dhit: Dhit,
    dmiss: Dmiss,
}
impl Profiling {
    #[doc = "0x00 - Description cluster: Instruction fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    #[inline(always)]
    pub const fn ihit(&self) -> &Ihit {
        &self.ihit
    }
    #[doc = "0x04 - Description cluster: Instruction fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    #[inline(always)]
    pub const fn imiss(&self) -> &Imiss {
        &self.imiss
    }
    #[doc = "0x08 - Description cluster: Data fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    #[inline(always)]
    pub const fn dhit(&self) -> &Dhit {
        &self.dhit
    }
    #[doc = "0x0c - Description cluster: Data fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    #[inline(always)]
    pub const fn dmiss(&self) -> &Dmiss {
        &self.dmiss
    }
}
#[doc = "IHIT (r) register accessor: Description cluster: Instruction fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nYou can [`read`](crate::Reg::read) this register and get [`ihit::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ihit`] module"]
#[doc(alias = "IHIT")]
pub type Ihit = crate::Reg<ihit::IhitSpec>;
#[doc = "Description cluster: Instruction fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod ihit;
#[doc = "IMISS (r) register accessor: Description cluster: Instruction fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nYou can [`read`](crate::Reg::read) this register and get [`imiss::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imiss`] module"]
#[doc(alias = "IMISS")]
pub type Imiss = crate::Reg<imiss::ImissSpec>;
#[doc = "Description cluster: Instruction fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod imiss;
#[doc = "DHIT (r) register accessor: Description cluster: Data fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nYou can [`read`](crate::Reg::read) this register and get [`dhit::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhit`] module"]
#[doc(alias = "DHIT")]
pub type Dhit = crate::Reg<dhit::DhitSpec>;
#[doc = "Description cluster: Data fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod dhit;
#[doc = "DMISS (r) register accessor: Description cluster: Data fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmiss::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmiss`] module"]
#[doc(alias = "DMISS")]
pub type Dmiss = crate::Reg<dmiss::DmissSpec>;
#[doc = "Description cluster: Data fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod dmiss;
