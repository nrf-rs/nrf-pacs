#[doc = "PTR register accessor: an alias for `Reg<PTR_SPEC>`"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "RAM address pointer to write samples to with EasyDMA"]
pub mod ptr;
#[doc = "MAXCNT register accessor: an alias for `Reg<MAXCNT_SPEC>`"]
pub type MAXCNT = crate::Reg<maxcnt::MAXCNT_SPEC>;
#[doc = "Number of samples to allocate memory for in EasyDMA mode"]
pub mod maxcnt;
