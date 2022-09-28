#[doc = r"Register block"]
#[repr(C)]
pub struct EPOUT {
    #[doc = "0x00 - Description cluster: Data pointer"]
    pub ptr: PTR,
    #[doc = "0x04 - Description cluster: Maximum number of bytes to transfer"]
    pub maxcnt: MAXCNT,
    #[doc = "0x08 - Description cluster: Number of bytes transferred in the last transaction"]
    pub amount: AMOUNT,
}
#[doc = "PTR (rw) register accessor: an alias for `Reg<PTR_SPEC>`"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "Description cluster: Data pointer"]
pub mod ptr;
#[doc = "MAXCNT (rw) register accessor: an alias for `Reg<MAXCNT_SPEC>`"]
pub type MAXCNT = crate::Reg<maxcnt::MAXCNT_SPEC>;
#[doc = "Description cluster: Maximum number of bytes to transfer"]
pub mod maxcnt;
#[doc = "AMOUNT (r) register accessor: an alias for `Reg<AMOUNT_SPEC>`"]
pub type AMOUNT = crate::Reg<amount::AMOUNT_SPEC>;
#[doc = "Description cluster: Number of bytes transferred in the last transaction"]
pub mod amount;
