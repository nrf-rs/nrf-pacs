#[doc = "PTR register accessor: an alias for `Reg<PTR_SPEC>`"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "Start address of flash block to be erased"]
pub mod ptr;
#[doc = "LEN register accessor: an alias for `Reg<LEN_SPEC>`"]
pub type LEN = crate::Reg<len::LEN_SPEC>;
#[doc = "Size of block to be erased."]
pub mod len;
