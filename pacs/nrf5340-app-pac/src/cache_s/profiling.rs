#[doc = r"Register block"]
#[repr(C)]
pub struct PROFILING {
    #[doc = "0x00 - Description cluster: Instruction fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    pub ihit: IHIT,
    #[doc = "0x04 - Description cluster: Instruction fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    pub imiss: IMISS,
    #[doc = "0x08 - Description cluster: Data fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    pub dhit: DHIT,
    #[doc = "0x0c - Description cluster: Data fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
    pub dmiss: DMISS,
}
#[doc = "IHIT (r) register accessor: an alias for `Reg<IHIT_SPEC>`"]
pub type IHIT = crate::Reg<ihit::IHIT_SPEC>;
#[doc = "Description cluster: Instruction fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod ihit;
#[doc = "IMISS (r) register accessor: an alias for `Reg<IMISS_SPEC>`"]
pub type IMISS = crate::Reg<imiss::IMISS_SPEC>;
#[doc = "Description cluster: Instruction fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod imiss;
#[doc = "DHIT (r) register accessor: an alias for `Reg<DHIT_SPEC>`"]
pub type DHIT = crate::Reg<dhit::DHIT_SPEC>;
#[doc = "Description cluster: Data fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod dhit;
#[doc = "DMISS (r) register accessor: an alias for `Reg<DMISS_SPEC>`"]
pub type DMISS = crate::Reg<dmiss::DMISS_SPEC>;
#[doc = "Description cluster: Data fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod dmiss;
