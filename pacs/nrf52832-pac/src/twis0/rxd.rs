#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - RXD Data pointer"]
    pub ptr: PTR,
    #[doc = "0x04 - Maximum number of bytes in RXD buffer"]
    pub maxcnt: MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last RXD transaction"]
    pub amount: AMOUNT,
}
#[doc = "PTR (rw) register accessor: an alias for `Reg<PTR_SPEC>`"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "RXD Data pointer"]
pub mod ptr;
#[doc = "MAXCNT (rw) register accessor: an alias for `Reg<MAXCNT_SPEC>`"]
pub type MAXCNT = crate::Reg<maxcnt::MAXCNT_SPEC>;
#[doc = "Maximum number of bytes in RXD buffer"]
pub mod maxcnt;
#[doc = "AMOUNT (r) register accessor: an alias for `Reg<AMOUNT_SPEC>`"]
pub type AMOUNT = crate::Reg<amount::AMOUNT_SPEC>;
#[doc = "Number of bytes transferred in the last RXD transaction"]
pub mod amount;
