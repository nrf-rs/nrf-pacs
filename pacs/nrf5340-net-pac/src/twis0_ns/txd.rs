#[doc = "PTR register accessor: an alias for `Reg<PTR_SPEC>`"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "TXD Data pointer"]
pub mod ptr;
#[doc = "MAXCNT register accessor: an alias for `Reg<MAXCNT_SPEC>`"]
pub type MAXCNT = crate::Reg<maxcnt::MAXCNT_SPEC>;
#[doc = "Maximum number of bytes in TXD buffer"]
pub mod maxcnt;
#[doc = "AMOUNT register accessor: an alias for `Reg<AMOUNT_SPEC>`"]
pub type AMOUNT = crate::Reg<amount::AMOUNT_SPEC>;
#[doc = "Number of bytes transferred in the last TXD transaction"]
pub mod amount;
#[doc = "LIST register accessor: an alias for `Reg<LIST_SPEC>`"]
pub type LIST = crate::Reg<list::LIST_SPEC>;
#[doc = "EasyDMA list type"]
pub mod list;
