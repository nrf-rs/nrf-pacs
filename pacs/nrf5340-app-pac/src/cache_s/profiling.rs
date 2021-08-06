#[doc = "IHIT register accessor: an alias for `Reg<IHIT_SPEC>`"]
pub type IHIT = crate::Reg<ihit::IHIT_SPEC>;
#[doc = "Description cluster: Instruction fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod ihit;
#[doc = "IMISS register accessor: an alias for `Reg<IMISS_SPEC>`"]
pub type IMISS = crate::Reg<imiss::IMISS_SPEC>;
#[doc = "Description cluster: Instruction fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod imiss;
#[doc = "DHIT register accessor: an alias for `Reg<DHIT_SPEC>`"]
pub type DHIT = crate::Reg<dhit::DHIT_SPEC>;
#[doc = "Description cluster: Data fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod dhit;
#[doc = "DMISS register accessor: an alias for `Reg<DMISS_SPEC>`"]
pub type DMISS = crate::Reg<dmiss::DMISS_SPEC>;
#[doc = "Description cluster: Data fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod dmiss;
