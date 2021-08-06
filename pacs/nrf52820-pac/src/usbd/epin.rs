#[doc = "PTR register accessor: an alias for `Reg<PTR_SPEC>`"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "Description cluster: Data pointer"]
pub mod ptr;
#[doc = "MAXCNT register accessor: an alias for `Reg<MAXCNT_SPEC>`"]
pub type MAXCNT = crate::Reg<maxcnt::MAXCNT_SPEC>;
#[doc = "Description cluster: Maximum number of bytes to transfer"]
pub mod maxcnt;
#[doc = "AMOUNT register accessor: an alias for `Reg<AMOUNT_SPEC>`"]
pub type AMOUNT = crate::Reg<amount::AMOUNT_SPEC>;
#[doc = "Description cluster: Number of bytes transferred in the last transaction"]
pub mod amount;
